use crate::auth::{with_auth_with_claims, JwtClaims};
use crate::{
    auth::with_auth,
    error::{ClientError, InternalError},
    with_db, with_json, ApiResponse,
};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use chrono::Datelike;
use diesel_async::scoped_futures::ScopedFutureExt;
use diesel_async::{AsyncConnection, AsyncPgConnection};
use models::student::{CreateStudent, Student, UpdateStudent};
use models::types::UserRole;
use models::user::{CreateUser, User};
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, num::ParseIntError, sync::Arc};
use warp::{
    reject::{self, Rejection},
    reply::{self, Reply},
    Filter,
};

#[derive(Debug, Clone, Deserialize)]
struct CreateStudentPayload {
    pub name: String,
    pub class_id: i32,
    pub nis: String,
}

#[derive(Debug, Clone, Serialize)]
struct CreateStudentResponse {
    student: Student,
    username: String,
    password: String,
}

pub fn students_routes(
    jwt_key: String,
    db: Arc<Mutex<AsyncPgConnection>>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let student = warp::any().and(warp::path("student"));

    let get_students_route = student
        .and(warp::path::end())
        .and(warp::get())
        .and(with_auth(false, jwt_key.clone(), db.clone()))
        .untuple_one()
        .and(warp::query::query::<HashMap<String, String>>())
        .and(with_db(db.clone()))
        .and_then(get_students);

    let create_student_route = student
        .and(warp::path("create"))
        .and(warp::path::end())
        .and(warp::post())
        .and(with_auth_with_claims(false, jwt_key.clone(), db.clone()))
        .and(with_json())
        .and(with_db(db.clone()))
        .and_then(create_student);

    let read_student_route = student
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(warp::get())
        .and(with_auth(false, jwt_key.clone(), db.clone()).untuple_one())
        .and(with_db(db.clone()))
        .and_then(read_student);

    let update_student_route = student
        .and(warp::path::param::<i32>())
        .and(warp::path("update"))
        .and(warp::path::end())
        .and(warp::patch())
        .and(with_auth_with_claims(false, jwt_key.clone(), db.clone()))
        .and(with_json())
        .and(with_db(db.clone()))
        .and_then(update_student);

    let delete_student_route = student
        .and(warp::path::param::<i32>())
        .and(warp::path("delete"))
        .and(warp::path::end())
        .and(warp::delete())
        .and(with_auth_with_claims(true, jwt_key.clone(), db.clone()))
        .and(with_db(db.clone()))
        .and_then(delete_student);

    get_students_route
        .or(create_student_route)
        .or(read_student_route)
        .or(update_student_route)
        .or(delete_student_route)
}

async fn get_students(
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

async fn create_student(
    claims: JwtClaims,
    payload: CreateStudentPayload,
    db: Arc<Mutex<AsyncPgConnection>>,
) -> Result<impl Reply, Rejection> {
    let year = chrono::Local::now().year();
    let mut username = payload.name.trim().to_string();
    username.retain(|c| !c.is_whitespace());
    let mut username = username.to_lowercase();
    username = format!("{}{}", year, username);
    let c_name = username.clone();

    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(&username.as_bytes(), &salt)
        .map_err(|e| reject::custom(InternalError::ArgonError(e.to_string())))?
        .to_string();

    let create_user = CreateUser {
        username: username,
        password: hash,
        role: UserRole::Student,
    };

    let mut db = db.lock();
    let result = db
        .transaction(|conn| {
            async move {
                let user = User::create(conn, &create_user, *&claims.id).await?;

                let result = Student::create(
                    conn,
                    &CreateStudent {
                        name: payload.name.trim().to_owned(),
                        class_id: payload.class_id,
                        nis: payload.nis.trim().to_owned(),
                        user_id: user.id,
                    },
                    claims.id,
                )
                .await?;

                Ok(result)
            }
            .scope_boxed()
        })
        .await
        .map_err(|e: diesel::result::Error| {
            reject::custom(InternalError::DatabaseError(e.to_string()))
        })?;

    Ok(reply::json(&ApiResponse::ok(
        "success".to_owned(),
        CreateStudentResponse {
            student: result,
            username: c_name.clone(),
            password: c_name.clone(),
        },
    )))
}

async fn read_student(id: i32, db: Arc<Mutex<AsyncPgConnection>>) -> Result<impl Reply, Rejection> {
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

async fn update_student(
    id: i32,
    claims: JwtClaims,
    payload: UpdateStudent,
    db: Arc<Mutex<AsyncPgConnection>>,
) -> Result<impl Reply, Rejection> {
    let mut db = db.lock();
    let result = Student::update(&mut db, id, &payload, claims.id)
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

async fn delete_student(
    id: i32,
    claims: JwtClaims,
    db: Arc<Mutex<AsyncPgConnection>>,
) -> Result<impl Reply, Rejection> {
    let mut db = db.lock();
    let result = Student::delete(&mut db, id, claims.id)
        .await
        .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?;

    if result > 0 {
        Ok(reply::json(&ApiResponse::ok("success".to_owned(), result)))
    } else {
        Err(reject::custom(ClientError::NotFound(
            "student not found".to_owned(),
        )))
    }
}
