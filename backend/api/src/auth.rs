use std::sync::Arc;

use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use chrono::{Duration, Utc};
use diesel_async::AsyncPgConnection;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use models::{
    invalidated_jwt::InvalidatedJwt,
    user::{CreateUser, User},
};
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use tracing::debug;
use warp::{
    reject::{self, Rejection},
    reply::{self, Reply},
    Filter,
};

use crate::{
    error::{ClientError, InternalError},
    with_db, with_jwt_key, ApiResponse,
};

const BEARER: &'static str = "Bearer ";

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    pub id: i32,
    pub name: String,
    pub role: String,
    pub exp: i64,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    username: String,
    password: String,
    role: String,
}

pub async fn login_handler(
    jwt_key: String,
    db: Arc<Mutex<AsyncPgConnection>>,
    payload: LoginRequest,
) -> Result<impl Reply, Rejection> {
    let mut db = db.lock();
    let username = payload.username.trim();
    let user = User::find_by_username(&mut db, username)
        .await
        .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?;

    if user.is_none() {
        return Err(reject::custom(ClientError::NotFound(
            "user not found".to_owned(),
        )));
    }

    if user.as_ref().unwrap().username != payload.username {
        return Err(reject::custom(ClientError::NotFound(
            "user not found".to_owned(),
        )));
    }

    let jwt = match Argon2::default().verify_password(
        payload.password.as_bytes(),
        &PasswordHash::new(&user.as_ref().unwrap().password)
            .map_err(|e| reject::custom(InternalError::ArgonError(e.to_string())))?,
    ) {
        Ok(_) => {
            let exp = Utc::now()
                .checked_add_signed(Duration::hours(3))
                .ok_or(reject::custom(InternalError::ChronoError(
                    "invalid timestamp".to_owned(),
                )))?
                .timestamp();

            let claims = JwtClaims {
                id: user.as_ref().unwrap().id,
                name: user.as_ref().unwrap().username.clone(),
                role: match &user.as_ref().unwrap().role {
                    models::types::UserRole::Admin => "admin".to_owned(),
                    models::types::UserRole::Advisor => "advisor".to_owned(),
                },
                exp: exp,
            };

            let header = Header::new(jsonwebtoken::Algorithm::HS512);
            jsonwebtoken::encode(
                &header,
                &claims,
                &EncodingKey::from_secret(jwt_key.as_bytes()),
            )
            .map_err(|e| reject::custom(InternalError::JwtError(e.to_string())))?
        }
        Err(e) => match e {
            argon2::password_hash::Error::Password => {
                debug!("argon2: {:?}", e);
                return Err(reject::custom(ClientError::Authentication(e.to_string())));
            }
            _ => return Err(reject::custom(InternalError::ArgonError(e.to_string()))),
        },
    };

    Ok(reply::json(&ApiResponse::ok("logged in".to_owned(), jwt)))
}

pub async fn register_handler(
    payload: RegisterRequest,
    db: Arc<Mutex<AsyncPgConnection>>,
) -> Result<impl Reply, Rejection> {
    let salt = SaltString::generate(&mut OsRng);
    let argon = Argon2::default();
    let hash = argon
        .hash_password(payload.password.trim().as_bytes(), &salt)
        .map_err(|e| reject::custom(InternalError::ArgonError(e.to_string())))?
        .to_string();

    let user = CreateUser {
        username: payload.username.trim().to_owned(),
        password: hash,
        role: match &payload.role[..] {
            "admin" => models::types::UserRole::Admin,
            "advisor" => models::types::UserRole::Advisor,
            _ => {
                return Err(reject::custom(ClientError::InvalidInput(
                    "field 'role' must be either 'admin' or 'advisor'".to_owned(),
                )))
            }
        },
    };

    let mut db = db.lock();
    let result = User::create(&mut db, &user)
        .await
        .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?;

    Ok(reply::json(&ApiResponse::ok(
        "registered".to_owned(),
        result,
    )))
}

pub fn with_auth_with_claims(
    require_admin: bool,
    jwt_key: String,
    db: Arc<Mutex<AsyncPgConnection>>,
) -> impl Filter<Extract = (JwtClaims,), Error = Rejection> + Clone {
    warp::header::<String>("Authorization")
        .and(warp::any().map(move || require_admin))
        .and(with_jwt_key(jwt_key))
        .and(with_db(db))
        .and_then(
            |token: String,
             require_admin: bool,
             secret: String,
             db: Arc<Mutex<AsyncPgConnection>>| async move {
                let token = if token.trim().starts_with(BEARER) {
                    &token[7..]
                } else {
                    &token
                };

                let decoded = jsonwebtoken::decode::<JwtClaims>(
                    &token,
                    &DecodingKey::from_secret(secret.as_bytes()),
                    &Validation::new(jsonwebtoken::Algorithm::HS512),
                )
                .map_err(|_| {
                    reject::custom(ClientError::Authentication(
                        "jwt signature invalid or validation failed".to_owned(),
                    ))
                })?;

                let mut db = db.lock();
                let blacklist = InvalidatedJwt::find_by_token(&mut db, &token)
                    .await
                    .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?;

                if blacklist.is_some() {
                    return Err(reject::custom(ClientError::Authorization(
                        "token is blacklisted".to_owned(),
                    )));
                }

                if require_admin {
                    if decoded.claims.name == "admin" {
                        Ok::<JwtClaims, Rejection>(decoded.claims)
                    } else {
                        Err(reject::custom(ClientError::Authorization(
                            "insufficient permission".to_owned(),
                        )))
                    }
                } else {
                    Ok::<JwtClaims, Rejection>(decoded.claims)
                }
            },
        )
}

pub fn with_auth(
    require_admin: bool,
    jwt_key: String,
    db: Arc<Mutex<AsyncPgConnection>>,
) -> impl Filter<Extract = ((),), Error = Rejection> + Clone {
    warp::header::<String>("Authorization")
        .and(warp::any().map(move || require_admin))
        .and(with_jwt_key(jwt_key))
        .and(with_db(db))
        .and_then(
            |token: String,
             require_admin: bool,
             secret: String,
             db: Arc<Mutex<AsyncPgConnection>>| async move {
                let token = if token.trim().starts_with(BEARER) {
                    &token[7..]
                } else {
                    &token
                };

                let decoded = jsonwebtoken::decode::<JwtClaims>(
                    &token,
                    &DecodingKey::from_secret(secret.as_bytes()),
                    &Validation::new(jsonwebtoken::Algorithm::HS512),
                )
                .map_err(|e| {
                    reject::custom(ClientError::Authentication(format!(
                        "jwt signature invalid or validation failed: {e}"
                    )))
                })?;

                let mut db = db.lock();
                let blacklist = InvalidatedJwt::find_by_token(&mut db, &token)
                    .await
                    .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?;

                if blacklist.is_some() {
                    return Err(reject::custom(ClientError::Authorization(
                        "token is blacklisted".to_owned(),
                    )));
                }

                if require_admin {
                    if decoded.claims.name == "admin" {
                        Ok(())
                    } else {
                        Err(reject::custom(ClientError::Authorization(
                            "insufficient permission".to_owned(),
                        )))
                    }
                } else {
                    Ok(())
                }
            },
        )
}

pub async fn refresh_token_handler(
    claims: JwtClaims,
    jwt_key: String,
    db: Arc<Mutex<AsyncPgConnection>>,
) -> Result<impl Reply, Rejection> {
    let mut db = db.lock();
    let user = User::read(&mut db, claims.id)
        .await
        .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?;

    if user.is_none() {
        return Err(reject::custom(ClientError::NotFound(
            "user not found in database".to_owned(),
        )));
    }

    let exp = Utc::now()
        .checked_add_signed(Duration::hours(3))
        .ok_or(reject::custom(InternalError::ChronoError(
            "invalid timestamp".to_owned(),
        )))?
        .timestamp();

    let claims = JwtClaims {
        id: claims.id,
        name: user.as_ref().unwrap().username.clone(),
        role: match user.unwrap().role {
            models::types::UserRole::Admin => "admin".to_owned(),
            models::types::UserRole::Advisor => "advisor".to_owned(),
        },
        exp: exp,
    };

    let header = Header::new(jsonwebtoken::Algorithm::HS512);
    let jwt = jsonwebtoken::encode(
        &header,
        &claims,
        &EncodingKey::from_secret(jwt_key.as_bytes()),
    )
    .map_err(|e| reject::custom(InternalError::JwtError(e.to_string())))?;

    Ok(reply::json(&ApiResponse::ok(
        "refreshed token".to_owned(),
        jwt,
    )))
}
