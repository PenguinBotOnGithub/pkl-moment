use chrono::Datelike;
use diesel_async::AsyncPgConnection;
use models::wave::{CreateWave, Wave};
use parking_lot::Mutex;
use std::{collections::HashMap, num::ParseIntError, sync::Arc};
use warp::reject;
use warp::reject::Rejection;
use warp::reply;
use warp::reply::Reply;
use warp::Filter;

use crate::auth::with_auth;
use crate::error::ClientError;
use crate::error::InternalError;
use crate::with_db;
use crate::ApiResponse;

pub fn waves_routes(
    jwt_key: String,
    db: Arc<Mutex<AsyncPgConnection>>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let wave = warp::any().and(warp::path("wave"));

    let get_waves_route = wave
        .and(warp::path::end())
        .and(warp::get())
        .and(with_auth(false, jwt_key.clone(), db.clone()))
        .untuple_one()
        .and(warp::query::<HashMap<String, String>>())
        .and(with_db(db.clone()))
        .and_then(get_waves);

    let read_wave_route = wave
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(warp::get())
        .and(with_auth(false, jwt_key.clone(), db.clone()).untuple_one())
        .and(with_db(db.clone()))
        .and_then(read_wave);

    get_waves_route.or(read_wave_route)
}

async fn get_waves(
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

pub async fn current_wave(
    db: &mut AsyncPgConnection,
    user_id: i32,
) -> Result<Wave, diesel::result::Error> {
    let now = chrono::Local::now();
    let school_year = if now.month() >= 7 {
        (now.year() as i16, (now.year() + 1) as i16)
    } else {
        ((now.year() - 1) as i16, now.year() as i16)
    };

    let wave = Wave::find_by_school_year(db, school_year).await?;

    if let Some(v) = wave {
        return Ok(v);
    }

    let wave = Wave::create(
        db,
        &CreateWave {
            start_year: school_year.0,
            end_year: school_year.1,
        },
        user_id,
    )
    .await?;

    Ok(wave)
}

async fn read_wave(id: i32, db: Arc<Mutex<AsyncPgConnection>>) -> Result<impl Reply, Rejection> {
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
