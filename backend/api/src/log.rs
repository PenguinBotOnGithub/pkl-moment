use std::{collections::HashMap, num::ParseIntError, sync::Arc};

use diesel_async::AsyncPgConnection;
use parking_lot::Mutex;
use warp::{
    reject::{self, Rejection},
    reply::{self, Reply},
    Filter,
};

use models::log::Log;

use crate::{
    auth::with_auth,
    error::{ClientError, InternalError},
    with_db, ApiResponse,
};

pub fn logs_routes(
    jwt_key: String,
    db: Arc<Mutex<AsyncPgConnection>>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let log = warp::any().and(warp::path("logs"));

    let get_logs_route = log
        .and(warp::path::end())
        .and(warp::get())
        .and(with_auth(true, jwt_key.clone(), db.clone()))
        .untuple_one()
        .and(warp::query::<HashMap<String, String>>())
        .and(with_db(db.clone()))
        .and_then(get_logs);

    let read_log_route = log
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(warp::get())
        .and(with_auth(true, jwt_key.clone(), db.clone()).untuple_one())
        .and(with_db(db.clone()))
        .and_then(read_log);

    get_logs_route.or(read_log_route)
}

async fn get_logs(
    queries: HashMap<String, String>,
    db: Arc<Mutex<AsyncPgConnection>>,
) -> Result<impl Reply, Rejection> {
    let mut db = db.lock();

    let user_id =
        if let Some(s) = queries.get("user") {
            Some(s.parse().map_err(|e: ParseIntError| {
                reject::custom(ClientError::InvalidInput(e.to_string()))
            })?)
        } else {
            None
        };

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

    let logs = Log::paginate(&mut db, page, page_size, user_id)
        .await
        .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?;

    Ok(reply::json(&ApiResponse::ok("success".to_string(), logs)))
}

async fn read_log(id: i32, db: Arc<Mutex<AsyncPgConnection>>) -> Result<impl Reply, Rejection> {
    let mut db = db.lock();
    let log = Log::read(&mut db, id)
        .await
        .map_err(|e| InternalError::DatabaseError(e.to_string()))?;

    if let Some(v) = log {
        Ok(reply::json(&ApiResponse::ok("success".to_owned(), v)))
    } else {
        Err(reject::custom(ClientError::NotFound(
            "log not found".to_owned(),
        )))
    }
}
