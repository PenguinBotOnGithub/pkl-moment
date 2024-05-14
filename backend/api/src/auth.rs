use std::sync::Arc;

use argon2::{Argon2, PasswordHash, PasswordVerifier};
use chrono::{Duration, Utc};
use diesel_async::AsyncPgConnection;
use jsonwebtoken::{EncodingKey, Header};
use models::user::User;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use tracing::{debug, error};
use warp::{
    reject::{self, Rejection},
    reply::{self, Reply},
};

use crate::{
    error::{ClientError, InternalError},
    ApiResponse,
};

const BEARER: &'static str = "Bearer ";

#[derive(Debug, Serialize, Deserialize)]
struct JwtClaims {
    sub: String,
    role: String,
    exp: i64,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
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
                .checked_sub_signed(Duration::hours(3))
                .ok_or(reject::custom(InternalError::ChronoError(
                    "invalid timestamp".to_owned(),
                )))?
                .timestamp();

            let claims = JwtClaims {
                sub: user.as_ref().unwrap().username.clone(),
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
                return Err(reject::custom(ClientError::AuthenticationFailed(
                    e.to_string(),
                )));
            }
            _ => return Err(reject::custom(InternalError::ArgonError(e.to_string()))),
        },
    };

    Ok(reply::json(&ApiResponse::ok("logged in".to_owned(), jwt)))
}

pub async fn register_handler() -> Result<impl Reply, Rejection> {
    Ok(reply::json(&ApiResponse::ok("registered".to_owned(), "f")))
}

/* pub async fn with_auth() -> impl Filter<Extract = (User,), Error = Rejection> + Clone {
    warp::header::<String>("Authorization")
        .and_then(|s: String| async {
            if &s[0..6] == "Bearer " {

            }

        })
} */
