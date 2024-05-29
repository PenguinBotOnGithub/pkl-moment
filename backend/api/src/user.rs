use std::{collections::HashMap, num::ParseIntError, sync::Arc};

use diesel_async::AsyncPgConnection;
use parking_lot::Mutex;
use warp::{
    reject::{self, Rejection},
    reply::{self, Reply},
    Filter,
};

use models::user::{UpdateUser, User};

use crate::error::handle_fk_depended_data_delete;
use crate::{
    auth::with_auth,
    error::{ClientError, InternalError},
    with_db, with_json, ApiResponse,
};

pub fn users_routes(
    jwt_key: String,
    db: Arc<Mutex<AsyncPgConnection>>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let user = warp::any().and(warp::path("user"));

    let get_users_route = user
        .and(warp::path::end())
        .and(warp::get())
        .and(with_auth(true, jwt_key.clone(), db.clone()))
        .untuple_one()
        .and(warp::query::query::<HashMap<String, String>>())
        .and(with_db(db.clone()))
        .and_then(get_users);

    let read_user_route = user
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(warp::get())
        .and(with_auth(true, jwt_key.clone(), db.clone()).untuple_one())
        .and(with_db(db.clone()))
        .and_then(read_user);

    let update_user_route = user
        .and(warp::path::param::<i32>())
        .and(warp::path("update"))
        .and(warp::path::end())
        .and(warp::patch())
        .and(with_auth(true, jwt_key.clone(), db.clone()).untuple_one())
        .and(with_json())
        .and(with_db(db.clone()))
        .and_then(update_user);

    let delete_user_route = user
        .and(warp::path::param::<i32>())
        .and(warp::path("delete"))
        .and(warp::path::end())
        .and(warp::delete())
        .and(with_auth(true, jwt_key.clone(), db.clone()).untuple_one())
        .and(with_db(db.clone()))
        .and_then(delete_user);

    get_users_route
        .or(read_user_route)
        .or(update_user_route)
        .or(delete_user_route)
}

async fn get_users(
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
    let users = User::paginate(&mut db, page, page_size)
        .await
        .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?;

    Ok(reply::json(&ApiResponse::ok("success".to_owned(), users)))
}

async fn read_user(id: i32, db: Arc<Mutex<AsyncPgConnection>>) -> Result<impl Reply, Rejection> {
    let mut db = db.lock();
    let user = User::read(&mut db, id)
        .await
        .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?;

    if let Some(v) = user {
        Ok(reply::json(&ApiResponse::ok("success".to_owned(), v)))
    } else {
        Err(reject::custom(ClientError::NotFound(
            "user not found".to_owned(),
        )))
    }
}

async fn update_user(
    id: i32,
    payload: UpdateUser,
    db: Arc<Mutex<AsyncPgConnection>>,
) -> Result<impl Reply, Rejection> {
    let mut db = db.lock();
    let result = User::update(&mut db, id, &payload).await.map_err(|e| {
        if let diesel::result::Error::DatabaseError(v1, _) = &e {
            if let diesel::result::DatabaseErrorKind::UniqueViolation = v1 {
                return reject::custom(ClientError::Conflict("username already taken".to_owned()));
            }
        }

        reject::custom(InternalError::DatabaseError(e.to_string()))
    })?;

    if let Some(v) = result {
        Ok(reply::json(&ApiResponse::ok("success".to_owned(), v)))
    } else {
        Err(reject::custom(ClientError::NotFound(
            "user not found".to_owned(),
        )))
    }
}

async fn delete_user(id: i32, db: Arc<Mutex<AsyncPgConnection>>) -> Result<impl Reply, Rejection> {
    let mut db = db.lock();
    let result = User::delete(&mut db, id)
        .await
        .map_err(handle_fk_depended_data_delete)?;

    if result > 0 {
        Ok(reply::json(&ApiResponse::ok("success".to_owned(), result)))
    } else {
        Err(reject::custom(ClientError::NotFound(
            "user not found".to_owned(),
        )))
    }
}
