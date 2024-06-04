use std::{collections::HashMap, num::ParseIntError, sync::Arc};

use diesel_async::AsyncPgConnection;
use models::permohonan::{CreatePermohonan, Permohonan, UpdatePermohonan};
use parking_lot::Mutex;
use warp::{
    reject::{self, Rejection},
    reply::{self, Reply},
    Filter,
};

use crate::auth::{with_auth_with_claims, JwtClaims};
use crate::error::handle_fk_data_not_exists;
use crate::{
    error::{ClientError, InternalError},
    with_db, with_json, ApiResponse,
};

pub fn permohonans_routes(
    jwt_key: String,
    db: Arc<Mutex<AsyncPgConnection>>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let permohonan = warp::any().and(warp::path("permohonan"));

    let get_permohonans_route = permohonan
        .and(warp::path::end())
        .and(warp::get())
        .and(with_auth_with_claims(false, jwt_key.clone(), db.clone()))
        .and(warp::query::query::<HashMap<String, String>>())
        .and(with_db(db.clone()))
        .and_then(get_permohonans);

    let create_permohonan_route = permohonan
        .and(warp::path("create"))
        .and(warp::path::end())
        .and(warp::post())
        .and(with_auth_with_claims(false, jwt_key.clone(), db.clone()))
        .and(with_json())
        .and(with_db(db.clone()))
        .and_then(create_permohonan);

    let read_permohonan_route = permohonan
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(warp::get())
        .and(with_auth_with_claims(false, jwt_key.clone(), db.clone()))
        .and(with_db(db.clone()))
        .and_then(read_permohonan);

    let update_permohonan_route = permohonan
        .and(warp::path::param::<i32>())
        .and(warp::path("update"))
        .and(warp::path::end())
        .and(warp::patch())
        .and(with_auth_with_claims(false, jwt_key.clone(), db.clone()))
        .and(with_json())
        .and(with_db(db.clone()))
        .and_then(update_permohonan);

    let delete_permohonan_route = permohonan
        .and(warp::path::param::<i32>())
        .and(warp::path("delete"))
        .and(warp::path::end())
        .and(warp::delete())
        .and(with_auth_with_claims(false, jwt_key.clone(), db.clone()))
        .and(with_db(db.clone()))
        .and_then(delete_permohonan);

    get_permohonans_route
        .or(create_permohonan_route)
        .or(read_permohonan_route)
        .or(update_permohonan_route)
        .or(delete_permohonan_route)
}

async fn get_permohonans(
    claims: JwtClaims,
    queries: HashMap<String, String>,
    db: Arc<Mutex<AsyncPgConnection>>,
) -> Result<impl Reply, Rejection> {
    let (page, page_size) = match (queries.get("page"), queries.get("size")) {
        (None, None) => (0, 20),
        (None, Some(v)) => (
            0,
            v.parse().map_err(|e: ParseIntError| {
                reject::custom(ClientError::InvalidInput(e.to_string()))
            })?,
        ),
        (Some(v), None) => (
            v.parse().map_err(|e: ParseIntError| {
                reject::custom(ClientError::InvalidInput(e.to_string()))
            })?,
            20,
        ),
        (Some(v1), Some(v2)) => (
            v1.parse().map_err(|e: ParseIntError| {
                reject::custom(ClientError::InvalidInput(e.to_string()))
            })?,
            v2.parse().map_err(|e: ParseIntError| {
                reject::custom(ClientError::InvalidInput(e.to_string()))
            })?,
        ),
    };

    let mut db = db.lock();
    match &claims.role[..] {
        "admin" => {
            let by_user = queries.get("user");
            match by_user {
                None => {
                    let permohonans = Permohonan::paginate(&mut db, page, page_size)
                        .await
                        .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?;

                    Ok(reply::json(&ApiResponse::ok(
                        "success".to_owned(),
                        permohonans,
                    )))
                }
                Some(v) => {
                    let by_user = v.parse::<i32>().map_err(|e| {
                        reject::custom(ClientError::InvalidInput(format!(
                            "invalid user id: {}",
                            e.to_string()
                        )))
                    })?;

                    let permohonans =
                        Permohonan::paginate_by_user(&mut db, by_user, page, page_size)
                            .await
                            .map_err(|e| {
                                reject::custom(InternalError::DatabaseError(e.to_string()))
                            })?;

                    Ok(reply::json(&ApiResponse::ok(
                        "success".to_owned(),
                        permohonans,
                    )))
                }
            }
        }
        _ => {
            let permohonans = Permohonan::paginate_by_user(&mut db, claims.id, page, page_size)
                .await
                .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?;

            Ok(reply::json(&ApiResponse::ok(
                "success".to_owned(),
                permohonans,
            )))
        }
    }
}

async fn create_permohonan(
    claims: JwtClaims,
    mut payload: CreatePermohonan,
    db: Arc<Mutex<AsyncPgConnection>>,
) -> Result<impl Reply, Rejection> {
    match &claims.role[..] {
        "admin" => {
            if let None = payload.user_id {
                payload.user_id = Some(claims.id);
            }

            if payload.verified {
                payload.verified_date = Some(chrono::Local::now().date_naive());
            }
        }
        _ => {
            payload.user_id = Some(claims.id);

            if payload.verified {
                payload.verified = false;
                payload.verified_date = None;
            }
        }
    }

    let mut db = db.lock();
    let result = Permohonan::create(&mut db, &payload)
        .await
        .map_err(handle_fk_data_not_exists)?;

    Ok(reply::json(&ApiResponse::ok("success".to_owned(), result)))
}

async fn read_permohonan(
    id: i32,
    claims: JwtClaims,
    db: Arc<Mutex<AsyncPgConnection>>,
) -> Result<impl Reply, Rejection> {
    let mut db = db.lock();
    let permohonan = Permohonan::read_with_joins(&mut db, id)
        .await
        .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?;

    if let Some(v) = permohonan {
        match &claims.role[..] {
            "admin" => Ok(reply::json(&ApiResponse::ok("success".to_owned(), v))),
            _ => {
                if v.user.id != claims.id {
                    return Err(reject::custom(ClientError::Authorization(
                        "insufficient privilege to view other users data".to_owned(),
                    )));
                }

                Ok(reply::json(&ApiResponse::ok("success".to_owned(), v)))
            }
        }
    } else {
        Err(reject::custom(ClientError::NotFound(
            "permohonan not found".to_owned(),
        )))
    }
}

async fn update_permohonan(
    id: i32,
    claims: JwtClaims,
    mut payload: UpdatePermohonan,
    db: Arc<Mutex<AsyncPgConnection>>,
) -> Result<impl Reply, Rejection> {
    let mut db = db.lock();

    let letter = Permohonan::read(&mut db, id)
        .await
        .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?;

    if let Some(v) = letter {
        match &claims.role[..] {
            "admin" => {
                if let Some(b) = payload.verified {
                    if b {
                        if v.verified {
                            ()
                        } else {
                            payload.verified_date = Some(chrono::Local::now().date_naive());
                        }
                    } else {
                        if v.verified {
                            payload.verified_date = None;
                        } else {
                            ()
                        }
                    }
                }
            }
            _ => {
                if v.user_id != claims.id {
                    return Err(reject::custom(ClientError::Authorization(
                        "insufficient privilege to update other users data".to_owned(),
                    )));
                }

                if let Some(_) = payload.verified {
                    return Err(reject::custom(ClientError::Authorization(
                        "insufficient privilege to verify data".to_owned(),
                    )));
                }
            }
        }
    } else {
        return Err(reject::custom(ClientError::NotFound(
            "permohonan not found".to_owned(),
        )));
    }

    let result = Permohonan::update(&mut db, id, &payload)
        .await
        .map_err(handle_fk_data_not_exists)?;

    if let Some(v) = result {
        Ok(reply::json(&ApiResponse::ok("success".to_owned(), v)))
    } else {
        Err(reject::custom(ClientError::NotFound(
            "permohonan not found".to_owned(),
        )))
    }
}

async fn delete_permohonan(
    id: i32,
    claims: JwtClaims,
    db: Arc<Mutex<AsyncPgConnection>>,
) -> Result<impl Reply, Rejection> {
    let mut db = db.lock();

    let letter = Permohonan::read(&mut db, id)
        .await
        .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?;

    if let Some(v) = letter {
        match &claims.role[..] {
            "admin" => (),
            _ => {
                if v.user_id != claims.id {
                    return Err(reject::custom(ClientError::Authorization(
                        "insufficient privilege to delete other users data".to_owned(),
                    )));
                }
            }
        }
    } else {
        return Err(reject::custom(ClientError::NotFound(
            "permohonan not found".to_owned(),
        )));
    }

    let result = Permohonan::delete(&mut db, id)
        .await
        .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?;

    if result > 0 {
        Ok(reply::json(&ApiResponse::ok("success".to_owned(), result)))
    } else {
        Err(reject::custom(ClientError::NotFound(
            "permohonan not found".to_owned(),
        )))
    }
}
