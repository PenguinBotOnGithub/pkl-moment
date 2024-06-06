use std::{collections::HashMap, num::ParseIntError, sync::Arc};

use diesel_async::AsyncPgConnection;
use models::pengantaran::{CreatePengantaran, Pengantaran, UpdatePengantaran};
use models::pengantaran_student::CreatePengantaranStudent;
use models::pengantaran_student::PengantaranStudent;
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
    with_db, with_json, AddStudentRequest, ApiResponse,
};

pub fn pengantarans_routes(
    jwt_key: String,
    db: Arc<Mutex<AsyncPgConnection>>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let pengantaran = warp::any().and(warp::path("pengantaran"));

    let get_pengantarans_route = pengantaran
        .and(warp::path::end())
        .and(warp::get())
        .and(with_auth_with_claims(false, jwt_key.clone(), db.clone()))
        .and(warp::query::query::<HashMap<String, String>>())
        .and(with_db(db.clone()))
        .and_then(get_pengantarans);

    let create_pengantaran_route = pengantaran
        .and(warp::path("create"))
        .and(warp::path::end())
        .and(warp::post())
        .and(with_auth_with_claims(false, jwt_key.clone(), db.clone()))
        .and(with_json())
        .and(with_db(db.clone()))
        .and_then(create_pengantaran);

    let read_pengantaran_route = pengantaran
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(warp::get())
        .and(with_auth_with_claims(false, jwt_key.clone(), db.clone()))
        .and(with_db(db.clone()))
        .and_then(read_pengantaran);

    let update_pengantaran_route = pengantaran
        .and(warp::path::param::<i32>())
        .and(warp::path("update"))
        .and(warp::path::end())
        .and(warp::patch())
        .and(with_auth_with_claims(false, jwt_key.clone(), db.clone()))
        .and(with_json())
        .and(with_db(db.clone()))
        .and_then(update_pengantaran);

    let delete_pengantaran_route = pengantaran
        .and(warp::path::param::<i32>())
        .and(warp::path("delete"))
        .and(warp::path::end())
        .and(warp::delete())
        .and(with_auth_with_claims(false, jwt_key.clone(), db.clone()))
        .and(with_db(db.clone()))
        .and_then(delete_pengantaran);

    let get_pengantaran_students_route = pengantaran
        .and(warp::path::param::<i32>())
        .and(warp::path("student"))
        .and(warp::path::end())
        .and(warp::get())
        .and(with_auth_with_claims(false, jwt_key.clone(), db.clone()))
        .and(with_db(db.clone()))
        .and_then(get_pengantaran_students);

    let add_pengantaran_student_route = pengantaran
        .and(warp::path::param::<i32>())
        .and(warp::path("student"))
        .and(warp::path("add"))
        .and(warp::path::end())
        .and(warp::post())
        .and(with_auth_with_claims(false, jwt_key.clone(), db.clone()))
        .and(with_json())
        .and(with_db(db.clone()))
        .and_then(add_pengantaran_student);

    let remove_pengantaran_student_route = pengantaran
        .and(warp::path::param::<i32>())
        .and(warp::path("student"))
        .and(warp::path::param::<i32>())
        .and(warp::path("remove"))
        .and(warp::path::end())
        .and(warp::delete())
        .and(with_auth_with_claims(false, jwt_key.clone(), db.clone()))
        .and(with_db(db.clone()))
        .and_then(remove_pengantaran_student);

    get_pengantarans_route
        .or(create_pengantaran_route)
        .or(read_pengantaran_route)
        .or(update_pengantaran_route)
        .or(delete_pengantaran_route)
        .or(get_pengantaran_students_route)
        .or(add_pengantaran_student_route)
        .or(remove_pengantaran_student_route)
}

async fn get_pengantarans(
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
                    let pengantarans = Pengantaran::paginate_brief(&mut db, page, page_size, None)
                        .await
                        .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?;

                    Ok(reply::json(&ApiResponse::ok(
                        "success".to_owned(),
                        pengantarans,
                    )))
                }
                Some(v) => {
                    let by_user = v.parse::<i32>().map_err(|e| {
                        reject::custom(ClientError::InvalidInput(format!(
                            "invalid user id: {}",
                            e.to_string()
                        )))
                    })?;

                    let pengantarans =
                        Pengantaran::paginate_brief(&mut db, page, page_size, Some(by_user))
                            .await
                            .map_err(|e| {
                                reject::custom(InternalError::DatabaseError(e.to_string()))
                            })?;

                    Ok(reply::json(&ApiResponse::ok(
                        "success".to_owned(),
                        pengantarans,
                    )))
                }
            }
        }
        _ => {
            let pengantarans =
                Pengantaran::paginate_brief(&mut db, page, page_size, Some(claims.id))
                    .await
                    .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?;

            Ok(reply::json(&ApiResponse::ok(
                "success".to_owned(),
                pengantarans,
            )))
        }
    }
}

async fn create_pengantaran(
    claims: JwtClaims,
    mut payload: CreatePengantaran,
    db: Arc<Mutex<AsyncPgConnection>>,
) -> Result<impl Reply, Rejection> {
    match &claims.role[..] {
        "admin" => {
            if let None = payload.user_id {
                payload.user_id = Some(claims.id);
            }

            if let Some(b) = payload.verified {
                if b {
                    payload.verified_date = Some(chrono::Local::now().date_naive());
                } else {
                    payload.verified_date = None;
                }
            } else {
                payload.verified = Some(false);
                payload.verified_date = None;
            }
        }
        _ => {
            payload.user_id = Some(claims.id);

            payload.verified = Some(false);
            payload.verified_date = None;
        }
    }

    let mut db = db.lock();
    let result = Pengantaran::create(&mut db, &payload)
        .await
        .map_err(handle_fk_data_not_exists)?;

    Ok(reply::json(&ApiResponse::ok("success".to_owned(), result)))
}

async fn read_pengantaran(
    id: i32,
    claims: JwtClaims,
    db: Arc<Mutex<AsyncPgConnection>>,
) -> Result<impl Reply, Rejection> {
    let mut db = db.lock();
    let pengantaran = Pengantaran::read_with_joins(&mut db, id)
        .await
        .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?;

    if let Some(v) = pengantaran {
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
            "pengantaran not found".to_owned(),
        )))
    }
}

async fn update_pengantaran(
    id: i32,
    claims: JwtClaims,
    mut payload: UpdatePengantaran,
    db: Arc<Mutex<AsyncPgConnection>>,
) -> Result<impl Reply, Rejection> {
    let mut db = db.lock();

    let Some(v) = Pengantaran::read(&mut db, id)
        .await
        .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?
    else {
        return Err(reject::custom(ClientError::NotFound(
            "pengantaran not found".to_owned(),
        )));
    };

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

            if let Some(_) = payload.user_id {
                return Err(reject::custom(ClientError::Authorization(
                    "insufficient privilege to verify data".to_owned(),
                )));
            }
        }
    }

    let result = Pengantaran::update(&mut db, id, &payload)
        .await
        .map_err(handle_fk_data_not_exists)?;

    if let Some(v) = result {
        Ok(reply::json(&ApiResponse::ok("success".to_owned(), v)))
    } else {
        Err(reject::custom(ClientError::NotFound(
            "pengantaran not found".to_owned(),
        )))
    }
}

async fn delete_pengantaran(
    id: i32,
    claims: JwtClaims,
    db: Arc<Mutex<AsyncPgConnection>>,
) -> Result<impl Reply, Rejection> {
    let mut db = db.lock();

    let letter = Pengantaran::read(&mut db, id)
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
            "pengantaran not found".to_owned(),
        )));
    }

    let result = Pengantaran::delete(&mut db, id)
        .await
        .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?;

    if result > 0 {
        Ok(reply::json(&ApiResponse::ok("success".to_owned(), result)))
    } else {
        Err(reject::custom(ClientError::NotFound(
            "pengantaran not found".to_owned(),
        )))
    }
}

async fn get_pengantaran_students(
    id: i32,
    claims: JwtClaims,
    db: Arc<Mutex<AsyncPgConnection>>,
) -> Result<impl Reply, Rejection> {
    let mut db = db.lock();
    let result = PengantaranStudent::filter_by_letter_and_return_letter_id(&mut db, id)
        .await
        .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?;

    match result {
        Some(v) => match &claims.role[..] {
            "admin" => Ok(reply::json(&ApiResponse::ok("success".to_owned(), v.1))),
            _ => {
                if v.0 != claims.id {
                    return Err(reject::custom(ClientError::Authorization(
                        "insufficient privilege to view others data".to_owned(),
                    )));
                }

                Ok(reply::json(&ApiResponse::ok("success".to_owned(), v.1)))
            }
        },
        None => Err(reject::custom(ClientError::NotFound(
            "pengantaran not found".to_string(),
        ))),
    }
}

async fn add_pengantaran_student(
    id: i32,
    claims: JwtClaims,
    payload: AddStudentRequest,
    db: Arc<Mutex<AsyncPgConnection>>,
) -> Result<impl Reply, Rejection> {
    let mut db = db.lock();
    let letter_owner = Pengantaran::get_owner_id(&mut db, id)
        .await
        .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?;

    if let Some(n) = letter_owner {
        match &claims.role[..] {
            "admin" => {
                let res = PengantaranStudent::create(
                    &mut db,
                    &CreatePengantaranStudent {
                        pengantaran_id: id,
                        student_id: payload.student_id,
                    },
                )
                .await
                .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?;

                Ok(reply::json(&ApiResponse::ok("success".to_owned(), res)))
            }
            _ => {
                if n != claims.id {
                    return Err(reject::custom(ClientError::Authorization(
                        "insufficient privilege to modify others data".to_owned(),
                    )));
                }

                let res = PengantaranStudent::create(
                    &mut db,
                    &CreatePengantaranStudent {
                        pengantaran_id: id,
                        student_id: payload.student_id,
                    },
                )
                .await
                .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?;

                Ok(reply::json(&ApiResponse::ok("success".to_owned(), res)))
            }
        }
    } else {
        Err(reject::custom(ClientError::NotFound(
            "pengantaran not found".to_owned(),
        )))
    }
}

async fn remove_pengantaran_student(
    id: i32,
    student_id: i32,
    claims: JwtClaims,
    db: Arc<Mutex<AsyncPgConnection>>,
) -> Result<impl Reply, Rejection> {
    let mut db = db.lock();
    let Some(n) = Pengantaran::get_owner_id(&mut db, id)
        .await
        .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?
    else {
        return Err(reject::custom(ClientError::NotFound(
            "pengantaran not found".to_owned(),
        )));
    };

    match &claims.role[..] {
        "admin" => {
            let res = PengantaranStudent::delete_by_student_and_letter_id(&mut db, student_id, id)
                .await
                .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?;

            if res > 0 {
                Ok(reply::json(&ApiResponse::ok("success".to_owned(), res)))
            } else {
                Err(reject::custom(ClientError::NotFound(
                    "student not found".to_owned(),
                )))
            }
        }
        _ => {
            if n != claims.id {
                return Err(reject::custom(ClientError::Authorization(
                    "insufficient privilege to modify others data".to_owned(),
                )));
            }

            let res = PengantaranStudent::delete_by_student_and_letter_id(&mut db, student_id, id)
                .await
                .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?;

            if res > 0 {
                Ok(reply::json(&ApiResponse::ok("success".to_owned(), res)))
            } else {
                Err(reject::custom(ClientError::NotFound(
                    "student not found".to_owned(),
                )))
            }
        }
    }
}
