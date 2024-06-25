use std::{collections::HashMap, num::ParseIntError, sync::Arc};

use diesel_async::AsyncPgConnection;
use models::permohonan::{CreatePermohonan, Permohonan, UpdatePermohonan};
use models::permohonan_student::{CreatePermohonanStudent, PermohonanStudent};
use models::signature::Signature;
use models::types::UserRole;
use parking_lot::Mutex;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use warp::{
    reject::{self, Rejection},
    reply::{self, Reply},
    Filter,
};

use crate::auth::{with_auth_with_claims, JwtClaims};
use crate::error::handle_fk_data_not_exists;
use crate::pdf::gen_permohonan_chromium;
use crate::{
    error::{ClientError, InternalError},
    with_db, with_json, AddStudentRequest, ApiResponse, GenPdfRequest,
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

    let get_permohonan_students_route = permohonan
        .and(warp::path::param::<i32>())
        .and(warp::path("student"))
        .and(warp::path::end())
        .and(warp::get())
        .and(with_auth_with_claims(false, jwt_key.clone(), db.clone()))
        .and(with_db(db.clone()))
        .and_then(get_permohonan_students);

    let add_permohonan_student_route = permohonan
        .and(warp::path::param::<i32>())
        .and(warp::path("student"))
        .and(warp::path("add"))
        .and(warp::path::end())
        .and(with_auth_with_claims(false, jwt_key.clone(), db.clone()))
        .and(with_json())
        .and(with_db(db.clone()))
        .and_then(add_permohonan_student);

    let remove_permohonan_student_route = permohonan
        .and(warp::path::param::<i32>())
        .and(warp::path("student"))
        .and(warp::path::param::<i32>())
        .and(warp::path("remove"))
        .and(warp::path::end())
        .and(warp::delete())
        .and(with_auth_with_claims(false, jwt_key.clone(), db.clone()))
        .and(with_db(db.clone()))
        .and_then(remove_permohonan_student);

    let verify_permohonan_route = permohonan
        .and(warp::path::param::<i32>())
        .and(warp::path("verify"))
        .and(warp::path::end())
        .and(warp::patch())
        .and(with_auth_with_claims(true, jwt_key.clone(), db.clone()))
        .and(with_db(db.clone()))
        .and_then(verify_permohonan);

    let unverify_permohonan_route = permohonan
        .and(warp::path::param::<i32>())
        .and(warp::path("unverify"))
        .and(warp::path::end())
        .and(warp::patch())
        .and(with_auth_with_claims(true, jwt_key.clone(), db.clone()))
        .and(with_db(db.clone()))
        .and_then(unverify_permohonan);

    let pdf_permohonan_route = permohonan
        .and(warp::path::param::<i32>())
        .and(warp::path("pdf"))
        .and(warp::path::end())
        .and(warp::get())
        .and(with_auth_with_claims(false, jwt_key.clone(), db.clone()))
        .and(with_db(db.clone()))
        .and_then(gen_permohonan_pdf);

    get_permohonans_route
        .or(create_permohonan_route)
        .or(read_permohonan_route)
        .or(update_permohonan_route)
        .or(delete_permohonan_route)
        .or(get_permohonan_students_route)
        .or(add_permohonan_student_route)
        .or(remove_permohonan_student_route)
        .or(verify_permohonan_route)
        .or(unverify_permohonan_route)
        .or(pdf_permohonan_route)
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
    match &claims.role {
        UserRole::Admin => {
            let by_user = queries.get("user");
            match by_user {
                None => {
                    let permohonans = Permohonan::paginate_brief(&mut db, page, page_size, None)
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
                        Permohonan::paginate_brief(&mut db, page, page_size, Some(by_user))
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
        UserRole::Advisor => {
            let permohonans = Permohonan::paginate_brief(&mut db, page, page_size, Some(claims.id))
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
    match &claims.role {
        UserRole::Admin => {
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
        UserRole::Advisor => {
            payload.user_id = Some(claims.id);

            payload.verified = Some(false);
            payload.verified_date = None;
        }
    }

    let mut db = db.lock();
    let result = Permohonan::create(&mut db, &payload, claims.id)
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
        match &claims.role {
            UserRole::Admin => Ok(reply::json(&ApiResponse::ok("success".to_owned(), v))),
            UserRole::Advisor => {
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

    let Some(v) = Permohonan::get_owner_id(&mut db, id)
        .await
        .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?
    else {
        return Err(reject::custom(ClientError::NotFound(
            "permohonan not found".to_owned(),
        )));
    };

    if let UserRole::Advisor = &claims.role {
        if v != claims.id {
            return Err(reject::custom(ClientError::Authorization(
                "insufficient privilege to update other users data".to_owned(),
            )));
        }

        if let Some(_) = payload.user_id {
            return Err(reject::custom(ClientError::Authorization(
                "insufficient privilege to update data ownership".to_owned(),
            )));
        }
    }

    payload.verified = Some(false);
    payload.verified_date = Some(None);

    let result = Permohonan::update(&mut db, id, &payload, claims.id)
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
        match &claims.role {
            UserRole::Admin => (),
            UserRole::Advisor => {
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

    let result = Permohonan::delete(&mut db, id, claims.id)
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

async fn get_permohonan_students(
    id: i32,
    claims: JwtClaims,
    db: Arc<Mutex<AsyncPgConnection>>,
) -> Result<impl Reply, Rejection> {
    let mut db = db.lock();
    let result = PermohonanStudent::filter_by_letter_and_return_letter_id(&mut db, id)
        .await
        .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?;

    match result {
        Some(v) => match &claims.role {
            UserRole::Admin => Ok(reply::json(&ApiResponse::ok("success".to_owned(), v.1))),
            UserRole::Advisor => {
                if v.0 != claims.id {
                    return Err(reject::custom(ClientError::Authorization(
                        "insufficient privilege to view others data".to_owned(),
                    )));
                }

                Ok(reply::json(&ApiResponse::ok("success".to_owned(), v.1)))
            }
        },
        None => Err(reject::custom(ClientError::NotFound(
            "permohonan not found".to_string(),
        ))),
    }
}

async fn add_permohonan_student(
    id: i32,
    claims: JwtClaims,
    payload: AddStudentRequest,
    db: Arc<Mutex<AsyncPgConnection>>,
) -> Result<impl Reply, Rejection> {
    let mut db = db.lock();
    let letter_owner = Permohonan::get_owner_id(&mut db, id)
        .await
        .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?;

    if let Some(n) = letter_owner {
        match &claims.role {
            UserRole::Admin => {
                let res = PermohonanStudent::create(
                    &mut db,
                    &CreatePermohonanStudent {
                        permohonan_id: id,
                        student_id: payload.student_id,
                    },
                    claims.id,
                )
                .await
                .map_err(handle_fk_data_not_exists)?;

                Ok(reply::json(&ApiResponse::ok("success".to_owned(), res)))
            }
            UserRole::Advisor => {
                if n != claims.id {
                    return Err(reject::custom(ClientError::Authorization(
                        "insufficient privilege to modify others data".to_owned(),
                    )));
                }

                let res = PermohonanStudent::create(
                    &mut db,
                    &CreatePermohonanStudent {
                        permohonan_id: id,
                        student_id: payload.student_id,
                    },
                    claims.id,
                )
                .await
                .map_err(handle_fk_data_not_exists)?;

                Ok(reply::json(&ApiResponse::ok("success".to_owned(), res)))
            }
        }
    } else {
        Err(reject::custom(ClientError::NotFound(
            "permohonan not found".to_owned(),
        )))
    }
}

async fn remove_permohonan_student(
    id: i32,
    student_id: i32,
    claims: JwtClaims,
    db: Arc<Mutex<AsyncPgConnection>>,
) -> Result<impl Reply, Rejection> {
    let mut db = db.lock();
    let Some(n) = Permohonan::get_owner_id(&mut db, id)
        .await
        .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?
    else {
        return Err(reject::custom(ClientError::NotFound(
            "permohonan not found".to_owned(),
        )));
    };

    match &claims.role {
        UserRole::Admin => {
            let res = PermohonanStudent::delete_by_student_and_letter_id(
                &mut db, student_id, id, claims.id,
            )
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
        UserRole::Advisor => {
            if n != claims.id {
                return Err(reject::custom(ClientError::Authorization(
                    "insufficient privilege to modify others data".to_owned(),
                )));
            }

            let res = PermohonanStudent::delete_by_student_and_letter_id(
                &mut db, student_id, id, claims.id,
            )
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

async fn verify_permohonan(
    id: i32,
    claims: JwtClaims,
    db: Arc<Mutex<AsyncPgConnection>>,
) -> Result<impl Reply, Rejection> {
    let mut db = db.lock();
    let res = Permohonan::verify(&mut db, id, claims.id)
        .await
        .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?;

    if let Some(v) = res {
        Ok(reply::json(&ApiResponse::ok("success".to_owned(), v)))
    } else {
        Err(reject::custom(ClientError::NotFound(
            "permohonan not found".to_owned(),
        )))
    }
}
async fn unverify_permohonan(
    id: i32,
    claims: JwtClaims,
    db: Arc<Mutex<AsyncPgConnection>>,
) -> Result<impl Reply, Rejection> {
    let mut db = db.lock();
    let res = Permohonan::unverify(&mut db, id, claims.id)
        .await
        .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?;

    if let Some(v) = res {
        Ok(reply::json(&ApiResponse::ok("success".to_owned(), v)))
    } else {
        Err(reject::custom(ClientError::NotFound(
            "permohonan not found".to_owned(),
        )))
    }
}

async fn gen_permohonan_pdf(
    id: i32,
    claims: JwtClaims,
    db: Arc<Mutex<AsyncPgConnection>>,
) -> Result<impl Reply, Rejection> {
    let mut db = db.lock();
    let Some(n) = Permohonan::get_owner_id(&mut db, id)
        .await
        .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?
    else {
        return Err(reject::custom(ClientError::NotFound(
            "permohonan not found".to_owned(),
        )));
    };

    if let UserRole::Advisor = &claims.role {
        if n != claims.id {
            return Err(reject::custom(ClientError::Authorization(
                "insufficient privilege to view others data".to_owned(),
            )));
        }
    }

    let detail = Permohonan::read_with_joins(&mut db, id)
        .await
        .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?;
    let Some(detail) = detail else {
        return Err(reject::custom(ClientError::NotFound(
            "permohonan not found".to_owned(),
        )));
    };

    if !&detail.verified {
        return Err(reject::custom(ClientError::Authorization(
            "permohonan not verified".to_string(),
        )));
    }

    let buffer = gen_permohonan_chromium(&detail).await?;

    let file = fs::File::create(format!(
        "assets/pdf/{}.pdf",
        chrono::Local::now().to_string()
    ))
    .await
    .ok();
    if let Some(mut v) = file {
        v.write_all(&buffer.clone()).await.ok();
    }

    Ok(reply::with_header(
        buffer,
        "Content-Type",
        "application/pdf",
    ))
}
