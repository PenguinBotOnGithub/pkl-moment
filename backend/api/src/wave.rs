use std::{collections::HashMap, num::ParseIntError, sync::Arc};

use diesel_async::AsyncPgConnection;
use models::wave::{CreateWave, Wave};
use parking_lot::Mutex;
use serde::Deserialize;
use warp::{
    reject::{self, Rejection},
    reply::{self, Reply},
};

use crate::{
    error::{ClientError, InternalError},
    ApiResponse,
};

#[derive(Debug, Deserialize)]
pub struct WaveRequest {
    start_date: Option<chrono::NaiveDate>,
    end_date: Option<chrono::NaiveDate>,
}

pub async fn get_waves(
    queries: HashMap<String, String>,
    db: Arc<Mutex<AsyncPgConnection>>,
) -> Result<impl Reply, Rejection> {
    let mut db = db.lock();

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

    let waves = Wave::paginate(&mut db, page, page_size)
        .await
        .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?;

    Ok(reply::json(&ApiResponse::ok("success".to_string(), waves)))
}

pub async fn create_wave(
    payload: WaveRequest,
    db: Arc<Mutex<AsyncPgConnection>>,
) -> Result<impl Reply, Rejection> {
    if payload.start_date.is_none() {
        return Err(reject::custom(ClientError::InvalidInput(
            "start_date field is required".to_owned(),
        )));
    }

    if payload.end_date.is_none() {
        return Err(reject::custom(ClientError::InvalidInput(
            "start_date field is required".to_owned(),
        )));
    }

    let new_wave = CreateWave {
        start_date: payload.start_date.unwrap(),
        end_date: payload.end_date.unwrap(),
    };

    let mut db = db.lock();
    let result = Wave::create(&mut db, &new_wave)
        .await
        .map_err(|e| InternalError::DatabaseError(e.to_string()))?;

    Ok(reply::json(&ApiResponse::ok("success".to_owned(), result)))
}

pub async fn read_wave(
    id: i32,
    db: Arc<Mutex<AsyncPgConnection>>,
) -> Result<impl Reply, Rejection> {
    let mut db = db.lock();
    let wave = Wave::read(&mut db, id)
        .await
        .map_err(|e| InternalError::DatabaseError(e.to_string()))?;

    if let Some(v) = wave {
        Ok(reply::json(&ApiResponse::ok("success".to_owned(), v)))
    } else {
        Err(reject::custom(ClientError::NotFound(
            "wave not found".to_owned(),
        )))
    }
}
pub async fn update_wave() -> Result<impl Reply, Rejection> {
    Err::<String, Rejection>(reject::custom(InternalError::NotImplemented(
        "this feature has not been implemented yet; please contact the administrator or developer"
            .to_owned(),
    )))
}
pub async fn delete_wave() -> Result<impl Reply, Rejection> {
    Err::<String, Rejection>(reject::custom(InternalError::NotImplemented(
        "companies deletion hasn't been implemented".to_owned(),
    )))
}
