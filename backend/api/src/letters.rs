use std::{collections::HashMap, num::ParseIntError, sync::Arc};

use diesel_async::AsyncPgConnection;
use models::letters::{CreateLetter, Letter, UpdateLetter};
use models::letters_student::{CreateLettersStudent, LettersStudent};
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
use crate::error::{handle_fk_data_not_exists, handle_fk_not_exists_unique_violation};
use crate::pdf::{gen_penarikan_chromium, gen_pengantaran_chromium, gen_permohonan_chromium};
use crate::{
    error::{ClientError, InternalError},
    wave, with_db, with_json, AddStudentRequest, ApiResponse,
};

pub fn letters_routes(
    jwt_key: String,
    db: Arc<Mutex<AsyncPgConnection>>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let letters = warp::any().and(warp::path("letters"));

    let get_letters_route = letters
        .and(warp::path::end())
        .and(warp::get())
        .and(with_auth_with_claims(false, jwt_key.clone(), db.clone()))
        .and(warp::query::query::<HashMap<String, String>>())
        .and(with_db(db.clone()))
        .and_then(get_letters);

    let create_letters_route = letters
        .and(warp::path("create"))
        .and(warp::path::end())
        .and(warp::post())
        .and(with_auth_with_claims(false, jwt_key.clone(), db.clone()))
        .and(with_json())
        .and(with_db(db.clone()))
        .and_then(create_letters);

    let read_letters_route = letters
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(warp::get())
        .and(with_auth_with_claims(false, jwt_key.clone(), db.clone()))
        .and(with_db(db.clone()))
        .and_then(read_letters);

    let update_letters_route = letters
        .and(warp::path::param::<i32>())
        .and(warp::path("update"))
        .and(warp::path::end())
        .and(warp::patch())
        .and(with_auth_with_claims(false, jwt_key.clone(), db.clone()))
        .and(with_json())
        .and(with_db(db.clone()))
        .and_then(update_letters);

    let delete_letters_route = letters
        .and(warp::path::param::<i32>())
        .and(warp::path("delete"))
        .and(warp::path::end())
        .and(warp::delete())
        .and(with_auth_with_claims(false, jwt_key.clone(), db.clone()))
        .and(with_db(db.clone()))
        .and_then(delete_letters);

    let get_letters_students_route = letters
        .and(warp::path::param::<i32>())
        .and(warp::path("student"))
        .and(warp::path::end())
        .and(warp::get())
        .and(with_auth_with_claims(false, jwt_key.clone(), db.clone()))
        .and(with_db(db.clone()))
        .and_then(get_letters_students);

    let add_letters_student_route = letters
        .and(warp::path::param::<i32>())
        .and(warp::path("student"))
        .and(warp::path("add"))
        .and(warp::path::end())
        .and(warp::post())
        .and(with_auth_with_claims(false, jwt_key.clone(), db.clone()))
        .and(with_json())
        .and(with_db(db.clone()))
        .and_then(add_letters_student);

    let remove_letters_student_route = letters
        .and(warp::path::param::<i32>())
        .and(warp::path("student"))
        .and(warp::path::param::<i32>())
        .and(warp::path("remove"))
        .and(warp::path::end())
        .and(warp::delete())
        .and(with_auth_with_claims(false, jwt_key.clone(), db.clone()))
        .and(with_db(db.clone()))
        .and_then(remove_letters_student);

    let verify_letters_route = letters
        .and(warp::path::param::<i32>())
        .and(warp::path("verify"))
        .and(warp::path::end())
        .and(warp::patch())
        .and(with_auth_with_claims(true, jwt_key.clone(), db.clone()))
        .and(with_db(db.clone()))
        .and_then(verify_letters);

    let pdf_letters_route = letters
        .and(warp::path::param::<i32>())
        .and(warp::path("pdf"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(warp::get())
        .and(with_auth_with_claims(false, jwt_key.clone(), db.clone()))
        .and(with_db(db.clone()))
        .and_then(gen_letters_pdf);

    get_letters_route
        .or(create_letters_route)
        .or(read_letters_route)
        .or(update_letters_route)
        .or(delete_letters_route)
        .or(get_letters_students_route)
        .or(add_letters_student_route)
        .or(remove_letters_student_route)
        .or(verify_letters_route)
        .or(pdf_letters_route)
}

async fn get_letters(
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
        UserRole::Secretary => {
            let by_user = queries.get("user");
            match by_user {
                None => {
                    let letters = Letter::paginate_brief(&mut db, page, page_size, None)
                        .await
                        .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?;

                    Ok(reply::json(&ApiResponse::ok("success".to_owned(), letters)))
                }
                Some(v) => {
                    let by_user = v.parse::<i32>().map_err(|e| {
                        reject::custom(ClientError::InvalidInput(format!(
                            "invalid user id: {}",
                            e.to_string()
                        )))
                    })?;

                    let letters = Letter::paginate_brief(&mut db, page, page_size, Some(by_user))
                        .await
                        .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?;

                    Ok(reply::json(&ApiResponse::ok("success".to_owned(), letters)))
                }
            }
        }
        UserRole::Coordinator => {
            let letters = Letter::paginate_brief(&mut db, page, page_size, Some(claims.id))
                .await
                .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?;

            Ok(reply::json(&ApiResponse::ok("success".to_owned(), letters)))
        }
        _ => Err(reject::custom(ClientError::Authorization(
            "user not authorized to administrate letters".to_owned(),
        ))),
    }
}

async fn create_letters(
    claims: JwtClaims,
    mut payload: CreateLetter,
    db: Arc<Mutex<AsyncPgConnection>>,
) -> Result<impl Reply, Rejection> {
    match &claims.role {
        UserRole::Secretary => {
            if let None = payload.user_id {
                payload.user_id = Some(claims.id);
            }

            if let Some(b) = payload.verified {
                if b {
                    payload.verified_at = Some(chrono::Utc::now());
                } else {
                    payload.verified_at = None;
                }
            } else {
                payload.verified = Some(false);
                payload.verified_at = None;
            }
        }
        UserRole::Coordinator => {
            payload.user_id = Some(claims.id);

            payload.verified = Some(false);
            payload.verified_at = None;
        }
        _ => {
            return Err(reject::custom(ClientError::Authorization(
                "user not authorized to administrate letters".to_owned(),
            )))
        }
    }

    let mut db = db.lock();

    let wave = wave::current_wave(&mut db, claims.id)
        .await
        .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?;
    payload.wave_id = Some(wave.id);

    let result = Letter::create(&mut db, &payload, claims.id)
        .await
        .map_err(handle_fk_data_not_exists)?;

    Ok(reply::json(&ApiResponse::ok("success".to_owned(), result)))
}

async fn read_letters(
    id: i32,
    claims: JwtClaims,
    db: Arc<Mutex<AsyncPgConnection>>,
) -> Result<impl Reply, Rejection> {
    let mut db = db.lock();
    let letters = Letter::read_with_joins(&mut db, id)
        .await
        .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?;

    if let Some(v) = letters {
        match &claims.role {
            UserRole::Secretary => Ok(reply::json(&ApiResponse::ok("success".to_owned(), v))),
            UserRole::Coordinator => {
                if v.user.id != claims.id {
                    return Err(reject::custom(ClientError::Authorization(
                        "insufficient privilege to view other users data".to_owned(),
                    )));
                }

                Ok(reply::json(&ApiResponse::ok("success".to_owned(), v)))
            }
            _ => {
                return Err(reject::custom(ClientError::Authorization(
                    "user not authorized to administrate letters".to_owned(),
                )))
            }
        }
    } else {
        Err(reject::custom(ClientError::NotFound(
            "letters not found".to_owned(),
        )))
    }
}

async fn update_letters(
    id: i32,
    claims: JwtClaims,
    mut payload: UpdateLetter,
    db: Arc<Mutex<AsyncPgConnection>>,
) -> Result<impl Reply, Rejection> {
    let mut db = db.lock();

    let Some(b) = Letter::get_verification_status(&mut db, id)
        .await
        .map_err(|e| InternalError::DatabaseError(e.to_string()))?
    else {
        return Err(reject::custom(ClientError::NotFound(
            "letters not found".to_owned(),
        )));
    };

    let Some(v) = Letter::get_owner_id(&mut db, id)
        .await
        .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?
    else {
        return Err(reject::custom(ClientError::NotFound(
            "letters not found".to_owned(),
        )));
    };

    if let UserRole::Coordinator = &claims.role {
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

        if b {
            return Err(reject::custom(ClientError::Authorization(
                "insufficient privilege to update letters data after verified".to_owned(),
            )));
        }
    }

    if b {
        payload.verified = None;
        payload.verified_at = None;
    } else {
        payload.verified = Some(false);
        payload.verified_at = Some(None);
    }
    payload.wave_id = None;

    let result = Letter::update(&mut db, id, &payload, claims.id)
        .await
        .map_err(handle_fk_data_not_exists)?;

    if let Some(v) = result {
        Ok(reply::json(&ApiResponse::ok("success".to_owned(), v)))
    } else {
        Err(reject::custom(ClientError::NotFound(
            "letters not found".to_owned(),
        )))
    }
}

async fn delete_letters(
    id: i32,
    claims: JwtClaims,
    db: Arc<Mutex<AsyncPgConnection>>,
) -> Result<impl Reply, Rejection> {
    let mut db = db.lock();

    let Some(b) = Letter::get_verification_status(&mut db, id)
        .await
        .map_err(|e| InternalError::DatabaseError(e.to_string()))?
    else {
        return Err(reject::custom(ClientError::NotFound(
            "letters not found".to_owned(),
        )));
    };
    if b {
        return Err(reject::custom(ClientError::Conflict(
            "letters data can not be deleted after reaching verified status".to_owned(),
        )));
    }

    let letter = Letter::read(&mut db, id)
        .await
        .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?;

    if let Some(v) = letter {
        match &claims.role {
            UserRole::Secretary => (),
            UserRole::Coordinator => {
                if v.user_id != claims.id {
                    return Err(reject::custom(ClientError::Authorization(
                        "insufficient privilege to delete other users data".to_owned(),
                    )));
                }
            }
            _ => {
                return Err(reject::custom(ClientError::Authorization(
                    "user not authorized to administrate letters".to_owned(),
                )))
            }
        }
    } else {
        return Err(reject::custom(ClientError::NotFound(
            "letters not found".to_owned(),
        )));
    }

    let result = Letter::delete(&mut db, id, claims.id)
        .await
        .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?;

    if result > 0 {
        Ok(reply::json(&ApiResponse::ok("success".to_owned(), result)))
    } else {
        Err(reject::custom(ClientError::NotFound(
            "letters not found".to_owned(),
        )))
    }
}

async fn get_letters_students(
    id: i32,
    claims: JwtClaims,
    db: Arc<Mutex<AsyncPgConnection>>,
) -> Result<impl Reply, Rejection> {
    let mut db = db.lock();
    let owner = Letter::get_owner_id(&mut db, id)
        .await
        .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?;
    let Some(owner) = owner else {
        return Err(reject::custom(ClientError::NotFound(
            "letters not found".to_owned(),
        )));
    };

    let result = LettersStudent::filter_by_letter(&mut db, id)
        .await
        .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?;

    match result {
        Some(v) => match &claims.role {
            UserRole::Secretary => Ok(reply::json(&ApiResponse::ok("success".to_owned(), v))),
            UserRole::Coordinator => {
                if owner != claims.id {
                    return Err(reject::custom(ClientError::Authorization(
                        "insufficient privilege to view others data".to_owned(),
                    )));
                }

                Ok(reply::json(&ApiResponse::ok("success".to_owned(), v)))
            }
            _ => {
                return Err(reject::custom(ClientError::Authorization(
                    "user not authorized to administrate letters".to_owned(),
                )))
            }
        },
        None => Err(reject::custom(ClientError::NotFound(
            "letters not found".to_string(),
        ))),
    }
}

async fn add_letters_student(
    id: i32,
    claims: JwtClaims,
    payload: AddStudentRequest,
    db: Arc<Mutex<AsyncPgConnection>>,
) -> Result<impl Reply, Rejection> {
    let mut db = db.lock();
    let letter_owner = Letter::get_owner_id(&mut db, id)
        .await
        .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?;

    if let Some(n) = letter_owner {
        match &claims.role {
            UserRole::Secretary => {
                let res = LettersStudent::create(
                    &mut db,
                    &CreateLettersStudent {
                        letters_id: id,
                        student_id: payload.student_id,
                    },
                    claims.id,
                )
                .await
                .map_err(handle_fk_not_exists_unique_violation)?;

                Ok(reply::json(&ApiResponse::ok("success".to_owned(), res)))
            }
            UserRole::Coordinator => {
                if n != claims.id {
                    return Err(reject::custom(ClientError::Authorization(
                        "insufficient privilege to modify others data".to_owned(),
                    )));
                }

                let res = LettersStudent::create(
                    &mut db,
                    &CreateLettersStudent {
                        letters_id: id,
                        student_id: payload.student_id,
                    },
                    claims.id,
                )
                .await
                .map_err(handle_fk_not_exists_unique_violation)?;

                Ok(reply::json(&ApiResponse::ok("success".to_owned(), res)))
            }
            _ => {
                return Err(reject::custom(ClientError::Authorization(
                    "user not authorized to administrate letters".to_owned(),
                )))
            }
        }
    } else {
        Err(reject::custom(ClientError::NotFound(
            "letters not found".to_owned(),
        )))
    }
}

async fn remove_letters_student(
    id: i32,
    student_id: i32,
    claims: JwtClaims,
    db: Arc<Mutex<AsyncPgConnection>>,
) -> Result<impl Reply, Rejection> {
    let mut db = db.lock();
    let Some(n) = Letter::get_owner_id(&mut db, id)
        .await
        .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?
    else {
        return Err(reject::custom(ClientError::NotFound(
            "letters not found".to_owned(),
        )));
    };

    match &claims.role {
        UserRole::Secretary => {
            let res =
                LettersStudent::delete_by_student_letter_id(&mut db, student_id, id, claims.id)
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
        UserRole::Coordinator => {
            if n != claims.id {
                return Err(reject::custom(ClientError::Authorization(
                    "insufficient privilege to modify others data".to_owned(),
                )));
            }

            let res =
                LettersStudent::delete_by_student_letter_id(&mut db, student_id, id, claims.id)
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
            return Err(reject::custom(ClientError::Authorization(
                "user not authorized to administrate letters".to_owned(),
            )))
        }
    }
}

async fn verify_letters(
    id: i32,
    claims: JwtClaims,
    db: Arc<Mutex<AsyncPgConnection>>,
) -> Result<impl Reply, Rejection> {
    let mut db = db.lock();
    let res = Letter::verify_letter(&mut db, id, claims.id)
        .await
        .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?;

    if res > 0 {
        Ok(reply::json(&ApiResponse::ok("success".to_owned(), res)))
    } else {
        Err(reject::custom(ClientError::NotFound(
            "letters not found".to_owned(),
        )))
    }
}

async fn gen_letters_pdf(
    id: i32,
    letter_type: String,
    claims: JwtClaims,
    db: Arc<Mutex<AsyncPgConnection>>,
) -> Result<impl Reply, Rejection> {
    enum LetterType {
        Permohonan,
        Pengantaran,
        Penarikan,
    }
    let letter_type = match &letter_type[..] {
        "permohonan" => LetterType::Permohonan,
        "pengantaran" => LetterType::Pengantaran,
        "penarikan" => LetterType::Penarikan,
        _ => return Err(reject::not_found()),
    };

    let mut db = db.lock();
    let Some(n) = Letter::get_owner_id(&mut db, id)
        .await
        .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?
    else {
        return Err(reject::custom(ClientError::NotFound(
            "letters not found".to_owned(),
        )));
    };

    if let UserRole::Coordinator = &claims.role {
        if n != claims.id {
            return Err(reject::custom(ClientError::Authorization(
                "insufficient privilege to view others data".to_owned(),
            )));
        }
    }

    let detail = Letter::read_with_joins(&mut db, id)
        .await
        .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?;
    let Some(detail) = detail else {
        return Err(reject::custom(ClientError::NotFound(
            "letters not found".to_owned(),
        )));
    };

    if !&detail.verified {
        return Err(reject::custom(ClientError::Authorization(
            "letters not verified".to_string(),
        )));
    }

    let buffer = match letter_type {
        LetterType::Permohonan => {
            gen_permohonan_chromium(
                &detail,
                Letter::get_letter_order(&mut db, detail.id, detail.wave.id)
                    .await
                    .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?,
            )
            .await?
        }
        LetterType::Pengantaran => {
            gen_pengantaran_chromium(
                &detail,
                Letter::get_letter_order(&mut db, detail.id, detail.wave.id)
                    .await
                    .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?,
            )
            .await?
        }
        LetterType::Penarikan => {
            gen_penarikan_chromium(
                &detail,
                Letter::get_letter_order(&mut db, detail.id, detail.wave.id)
                    .await
                    .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?,
            )
            .await?
        }
    };

    let file = fs::File::create(format!(
        "assets/pdf/{}.pdf",
        chrono::Local::now().to_string()
    ))
    .await
    .ok();
    if let Some(mut v) = file {
        v.write_all(&buffer).await.ok();
    }

    Ok(reply::with_header(
        buffer,
        "Content-Type",
        "application/pdf",
    ))
}
