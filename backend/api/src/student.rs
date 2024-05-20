use std::{collections::HashMap, num::ParseIntError, sync::Arc};

use diesel_async::AsyncPgConnection;
use models::student::{CreateStudent, Student, UpdateStudent};
use parking_lot::Mutex;
use warp::{
    reject::{self, Rejection},
    reply::{self, Reply},
};

use crate::{
    error::{ClientError, InternalError},
    ApiResponse,
};

pub async fn get_students(
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
    let students = Student::paginate(&mut db, page, page_size)
        .await
        .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?;

    Ok(reply::json(&ApiResponse::ok(
        "success".to_owned(),
        students,
    )))
}

pub async fn create_student(
    payload: CreateStudent,
    db: Arc<Mutex<AsyncPgConnection>>,
) -> Result<impl Reply, Rejection> {
    let mut db = db.lock();
    let result = Student::create(&mut db, &payload)
        .await
        .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?;

    Ok(reply::json(&ApiResponse::ok("success".to_owned(), result)))
}

pub async fn read_student(
    id: i32,
    db: Arc<Mutex<AsyncPgConnection>>,
) -> Result<impl Reply, Rejection> {
    let mut db = db.lock();
    let student = Student::read(&mut db, id)
        .await
        .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?;

    if let Some(v) = student {
        Ok(reply::json(&ApiResponse::ok("success".to_owned(), v)))
    } else {
        Err(reject::custom(ClientError::NotFound(
            "student not found".to_owned(),
        )))
    }
}

pub async fn update_student(
    id: i32,
    payload: UpdateStudent,
    db: Arc<Mutex<AsyncPgConnection>>,
) -> Result<impl Reply, Rejection> {
    let mut db = db.lock();
    let result = Student::update(&mut db, id, &payload)
        .await
        .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?;

    if let Some(v) = result {
        Ok(reply::json(&ApiResponse::ok("success".to_owned(), v)))
    } else {
        Err(reject::custom(ClientError::NotFound(
            "student not found".to_owned(),
        )))
    }
}
pub async fn delete_student() -> Result<impl Reply, Rejection> {
    Err::<String, Rejection>(reject::custom(InternalError::NotImplemented(
        "companies deletion hasn't been implemented".to_owned(),
    )))
}
