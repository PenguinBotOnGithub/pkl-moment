use crate::auth::{with_auth_with_claims, JwtClaims};
use crate::error::handle_fk_depended_data_delete;
use crate::{
    error::{ClientError, InternalError},
    with_db, with_json, ApiResponse,
};
use diesel_async::AsyncPgConnection;
use models::journal::{CreateJournal, Journal, UpdateJournal};
use models::tenure::Tenure;
use models::types::UserRole;
use parking_lot::Mutex;
use std::{collections::HashMap, num::ParseIntError, sync::Arc};
use warp::{
    reject::{self, Rejection},
    reply::{self, Reply},
    Filter,
};

pub fn journals_routes(
    jwt_key: String,
    db: Arc<Mutex<AsyncPgConnection>>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let journal = warp::any().and(warp::path("journal"));

    let get_journals_route = journal
        .and(warp::path::end())
        .and(warp::get())
        .and(with_auth_with_claims(false, jwt_key.clone(), db.clone()))
        .and(warp::query::query::<HashMap<String, String>>())
        .and(with_db(db.clone()))
        .and_then(get_journals);

    let create_journal_route = journal
        .and(warp::path("create"))
        .and(warp::path::end())
        .and(warp::post())
        .and(with_auth_with_claims(false, jwt_key.clone(), db.clone()))
        .and(with_json())
        .and(with_db(db.clone()))
        .and_then(create_journal);

    let read_journal_route = journal
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(warp::get())
        .and(with_auth_with_claims(false, jwt_key.clone(), db.clone()))
        .and(with_db(db.clone()))
        .and_then(read_journal);

    let update_journal_route = journal
        .and(warp::path::param::<i32>())
        .and(warp::path("update"))
        .and(warp::path::end())
        .and(warp::patch())
        .and(with_auth_with_claims(false, jwt_key.clone(), db.clone()))
        .and(with_json())
        .and(with_db(db.clone()))
        .and_then(update_journal);

    let delete_journal_route = journal
        .and(warp::path::param::<i32>())
        .and(warp::path("delete"))
        .and(warp::path::end())
        .and(warp::delete())
        .and(with_auth_with_claims(true, jwt_key.clone(), db.clone()))
        .and(with_db(db.clone()))
        .and_then(delete_journal);

    let verify_journal_route = journal
        .and(warp::path::param::<i32>())
        .and(warp::path("verify"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(warp::patch())
        .and(with_auth_with_claims(false, jwt_key.clone(), db.clone()))
        .and(with_db(db.clone()))
        .and_then(verify_journal);

    get_journals_route
        .or(create_journal_route)
        .or(read_journal_route)
        .or(update_journal_route)
        .or(delete_journal_route)
        .or(verify_journal_route)
}

async fn get_journals(
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
    let result = match *&claims.role {
        UserRole::Secretary => Journal::paginate(&mut db, page, page_size)
            .await
            .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?,
        UserRole::Coordinator => {
            return Err(reject::custom(ClientError::Authorization(
                "user is not authorized to view journal entries".to_owned(),
            )));
        }
        UserRole::AdvisorSchool | UserRole::AdvisorDudi => {
            Journal::paginate_by_advisor(&mut db, page, page_size, *&claims.id)
                .await
                .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?
        }
        UserRole::Student => Journal::paginate_by_student(&mut db, page, page_size, *&claims.id)
            .await
            .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?,
    };

    Ok(reply::json(&ApiResponse::ok("success".to_owned(), result)))
}

async fn create_journal(
    claims: JwtClaims,
    payload: CreateJournal,
    db: Arc<Mutex<AsyncPgConnection>>,
) -> Result<impl Reply, Rejection> {
    match *&claims.role {
        UserRole::AdvisorSchool | UserRole::AdvisorDudi | UserRole::Coordinator => {
            return Err(reject::custom(ClientError::Authorization(
                "user is not authorized to create journal entry".to_owned(),
            )));
        }
        _ => {}
    }

    let mut db = db.lock();
    let result = Journal::create_checked(&mut db, &payload, claims.id)
        .await
        .map_err(|e| match e.downcast::<diesel::result::Error>() {
            Ok(e) => reject::custom(InternalError::DatabaseError(e.to_string())),
            Err(e) => reject::custom(ClientError::InvalidInput(e.to_string())),
        })?;

    Ok(reply::json(&ApiResponse::ok("success".to_owned(), result)))
}

async fn read_journal(
    id: i32,
    claims: JwtClaims,
    db: Arc<Mutex<AsyncPgConnection>>,
) -> Result<impl Reply, Rejection> {
    let mut db = db.lock();

    let tenure = Journal::return_tenure(&mut db, id)
        .await
        .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?;
    let Some(tenure) = tenure else {
        return Err(reject::custom(InternalError::DatabaseError(
            "data not found".to_owned(),
        )));
    };

    match *&claims.role {
        UserRole::Coordinator => {
            return Err(reject::custom(ClientError::Authorization(
                "user is not allowed to view journal entries".to_owned(),
            )));
        }
        UserRole::AdvisorSchool => {
            let Some(adv) = tenure.advsch_id else {
                return Err(reject::custom(ClientError::Authorization(
                    "advisors can only view journal entries from student assigned to them"
                        .to_owned(),
                )));
            };

            if adv != *&claims.id {
                return Err(reject::custom(ClientError::Authorization(
                    "advisors can only view journal entries from student assigned to them"
                        .to_owned(),
                )));
            }
        }
        UserRole::AdvisorDudi => {
            let Some(adv) = tenure.advdudi_id else {
                return Err(reject::custom(ClientError::Authorization(
                    "advisors can only view journal entries from student assigned to them"
                        .to_owned(),
                )));
            };

            if adv != *&claims.id {
                return Err(reject::custom(ClientError::Authorization(
                    "advisors can only view journal entries from student assigned to them"
                        .to_owned(),
                )));
            }
        }
        UserRole::Student => {
            let user = Tenure::return_student_user(&mut db, &tenure)
                .await
                .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?;
            let Some(user) = user else {
                return Err(reject::custom(ClientError::Authorization(
                    "user not authorized to view others data".to_owned(),
                )));
            };

            if user.id != *&claims.id {
                return Err(reject::custom(ClientError::Authorization(
                    "user not authorized to view others data".to_owned(),
                )));
            }
        }
        _ => {}
    }

    let result = Journal::read_joined(&mut db, id)
        .await
        .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?;

    match result {
        Some(v) => Ok(reply::json(&ApiResponse::ok("success".to_owned(), v))),
        None => Err(reject::custom(ClientError::NotFound(
            "journal not found".to_owned(),
        ))),
    }
}

async fn update_journal(
    id: i32,
    claims: JwtClaims,
    payload: UpdateJournal,
    db: Arc<Mutex<AsyncPgConnection>>,
) -> Result<impl Reply, Rejection> {
    if let Some(_) = *&payload.verified_sch {
        return Err(reject::custom(ClientError::InvalidInput(
            "verifying journal entries through this endpoint is prohibited".to_owned(),
        )));
    }

    if let Some(_) = *&payload.verified_dudi {
        return Err(reject::custom(ClientError::InvalidInput(
            "verifying journal entries through this endpoint is prohibited".to_owned(),
        )));
    }

    let mut db = db.lock();

    let owner = Journal::get_owner_id(&mut db, id)
        .await
        .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?;
    let Some(owner) = owner else {
        return Err(reject::custom(ClientError::NotFound(
            "journal not found".to_owned(),
        )));
    };

    let verified = Journal::get_verified_status(&mut db, id)
        .await
        .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?;
    let Some((v_sch, v_dudi)) = verified else {
        return Err(reject::custom(ClientError::NotFound(
            "journal entry not found".to_owned(),
        )));
    };

    match &claims.role {
        UserRole::Secretary => {}
        UserRole::Coordinator => {
            return Err(reject::custom(ClientError::Authorization(
                "user is not allowed to manipulate journal entry".to_owned(),
            )));
        }
        UserRole::AdvisorSchool => {
            if v_sch && v_dudi {
                return Err(reject::custom(ClientError::Authorization(
                    "user does not have permission to modify verified journal entries".to_owned(),
                )));
            }

            let adv = Journal::get_advisors(&mut db, id)
                .await
                .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?;
            let Some((sch, _)) = adv else {
                return Err(reject::custom(ClientError::NotFound(
                    "journal entry not found".to_owned(),
                )));
            };

            match sch {
                None => {
                    return Err(reject::custom(ClientError::Authorization(
                        "advisor is not assigned to this student".to_owned(),
                    )));
                }
                Some(n) => {
                    if n != *&claims.id {
                        return Err(reject::custom(ClientError::Authorization(
                            "advisor is not assigned to this student".to_owned(),
                        )));
                    }
                }
            }
        }
        UserRole::AdvisorDudi => {
            if v_sch && v_dudi {
                return Err(reject::custom(ClientError::Authorization(
                    "user does not have permission to modify verified journal entries".to_owned(),
                )));
            }

            let adv = Journal::get_advisors(&mut db, id)
                .await
                .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?;
            let Some((_, dudi)) = adv else {
                return Err(reject::custom(ClientError::NotFound(
                    "journal entry not found".to_owned(),
                )));
            };

            match dudi {
                None => {
                    return Err(reject::custom(ClientError::Authorization(
                        "advisor is not assigned to this student".to_owned(),
                    )));
                }
                Some(n) => {
                    if n != *&claims.id {
                        return Err(reject::custom(ClientError::Authorization(
                            "advisor is not assigned to this student".to_owned(),
                        )));
                    }
                }
            }
        }
        UserRole::Student => {
            if v_sch && v_dudi {
                return Err(reject::custom(ClientError::Authorization(
                    "user does not have permission to modify verified journal entries".to_owned(),
                )));
            }

            if owner != *&claims.id {
                return Err(reject::custom(ClientError::Authorization(
                    "insufficient privilege to modify other users' data".to_owned(),
                )));
            }
        }
    }

    let result = Journal::update(&mut db, id, &payload, claims.id)
        .await
        .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?;

    match result {
        Some(v) => Ok(reply::json(&ApiResponse::ok("success".to_owned(), v))),
        None => Err(reject::custom(ClientError::NotFound(
            "journal not found".to_owned(),
        ))),
    }
}

async fn verify_journal(
    id: i32,
    v_type: String,
    claims: JwtClaims,
    db: Arc<Mutex<AsyncPgConnection>>,
) -> Result<impl Reply, Rejection> {
    enum VerificationType {
        School,
        Dudi,
    }

    let v_type = match &v_type[..] {
        "school" => VerificationType::School,
        "dudi" => VerificationType::Dudi,
        _ => return Err(reject::not_found()),
    };

    let mut db = db.lock();

    let owners = Journal::get_advisors(&mut db, id)
        .await
        .map_err(|e| InternalError::DatabaseError(e.to_string()))?;
    let Some((adv_sch, adv_dudi)) = owners else {
        return Err(reject::custom(ClientError::NotFound(
            "journal entry not found".to_owned(),
        )));
    };

    let res = match *&claims.role {
        UserRole::Secretary => match v_type {
            VerificationType::School => {
                Journal::verify_journal(&mut db, id, (true, false), *&claims.id)
                    .await
                    .map_err(|e| InternalError::DatabaseError(e.to_string()))?
            }
            VerificationType::Dudi => {
                Journal::verify_journal(&mut db, id, (true, false), *&claims.id)
                    .await
                    .map_err(|e| InternalError::DatabaseError(e.to_string()))?
            }
        },
        UserRole::Coordinator => {
            return Err(reject::custom(ClientError::Authorization(
                "user is not authorized to manipulate journal entries".to_owned(),
            )));
        }
        UserRole::AdvisorSchool => {
            if let VerificationType::Dudi = v_type {
                return Err(reject::custom(ClientError::Authorization(
                    "user is not allowed to authenticate on behalf of other advisors".to_owned(),
                )));
            }

            let Some(adv) = adv_sch else {
                return Err(reject::custom(ClientError::Authorization(
                    "user is not allowed to authenticate on behalf of other advisors".to_owned(),
                )));
            };

            if *&claims.id != adv {
                return Err(reject::custom(ClientError::Authorization(
                    "user is not allowed to authenticate on behalf of other advisors".to_owned(),
                )));
            }

            Journal::verify_journal(&mut db, id, (true, false), *&claims.id)
                .await
                .map_err(|e| InternalError::DatabaseError(e.to_string()))?
        }
        UserRole::AdvisorDudi => {
            if let VerificationType::School = v_type {
                return Err(reject::custom(ClientError::Authorization(
                    "user is not allowed to authenticate on behalf of other advisors".to_owned(),
                )));
            }

            let Some(adv) = adv_dudi else {
                return Err(reject::custom(ClientError::Authorization(
                    "user is not allowed to authenticate on behalf of other advisors".to_owned(),
                )));
            };

            if *&claims.id != adv {
                return Err(reject::custom(ClientError::Authorization(
                    "user is not allowed to authenticate on behalf of other advisors".to_owned(),
                )));
            }

            Journal::verify_journal(&mut db, id, (false, true), *&claims.id)
                .await
                .map_err(|e| InternalError::DatabaseError(e.to_string()))?
        }
        UserRole::Student => {
            return Err(reject::custom(ClientError::Authorization(
                "user is not authorized to manipulate journal entries".to_owned(),
            )));
        }
    };

    Ok(reply::json(&ApiResponse::ok("success".to_owned(), res)))
}

async fn delete_journal(
    id: i32,
    claims: JwtClaims,
    db: Arc<Mutex<AsyncPgConnection>>,
) -> Result<impl Reply, Rejection> {
    let mut db = db.lock();
    let result = Journal::delete(&mut db, id, claims.id)
        .await
        .map_err(handle_fk_depended_data_delete)?;

    if result > 0 {
        Ok(reply::json(&ApiResponse::ok("success".to_owned(), result)))
    } else {
        Err(reject::custom(ClientError::NotFound(
            "journal not found".to_owned(),
        )))
    }
}
