use std::{collections::HashMap, num::ParseIntError, sync::Arc};

use diesel_async::AsyncPgConnection;
use models::types::UserRole;
use parking_lot::Mutex;
use serde::Deserialize;
use warp::reject;
use warp::reject::Rejection;
use warp::reply;
use warp::reply::Reply;
use warp::Filter;

use models::tenure::Tenure;

use crate::auth::JwtClaims;
use crate::auth::{with_auth, with_auth_with_claims};
use crate::error::ClientError;
use crate::error::InternalError;
use crate::ApiResponse;
use crate::{with_db, with_json};

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct AddAdvisorPayload {
    advisor_type: String,
    advisor_id: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct RemoveAdvisorPayload {
    advisor_type: String,
}

pub fn tenures_routes(
    jwt_key: String,
    db: Arc<Mutex<AsyncPgConnection>>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let tenure = warp::any().and(warp::path("tenure"));

    let get_tenures_route = tenure
        .and(warp::path::end())
        .and(warp::get())
        .and(with_auth(false, jwt_key.clone(), db.clone()))
        .untuple_one()
        .and(warp::query::query::<HashMap<String, String>>())
        .and(with_db(db.clone()))
        .and_then(get_tenures);

    let read_tenure_route = tenure
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(warp::get())
        .and(with_auth(false, jwt_key.clone(), db.clone()).untuple_one())
        .and(with_db(db.clone()))
        .and_then(read_tenure);

    let add_advisor_route = tenure
        .and(warp::path::param::<i32>())
        .and(warp::path("advisor"))
        .and(warp::path("add"))
        .and(warp::path::end())
        .and(warp::patch())
        .and(with_auth_with_claims(false, jwt_key.clone(), db.clone()))
        .and(with_json())
        .and(with_db(db.clone()))
        .and_then(add_advisor);

    let remove_advisor_route = tenure
        .and(warp::path::param::<i32>())
        .and(warp::path("advisor"))
        .and(warp::path("remove"))
        .and(warp::path::end())
        .and(warp::patch())
        .and(with_auth_with_claims(false, jwt_key.clone(), db.clone()))
        .and(with_json())
        .and(with_db(db.clone()))
        .and_then(remove_advisor);

    let my_tenures_route = tenure
        .and(warp::path("my"))
        .and(warp::path::end())
        .and(warp::get())
        .and(with_auth_with_claims(false, jwt_key.clone(), db.clone()))
        .and(with_db(db.clone()))
        .and_then(my_tenures);

    get_tenures_route
        .or(read_tenure_route)
        .or(add_advisor_route)
        .or(remove_advisor_route)
        .or(my_tenures_route)
}

async fn get_tenures(
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
    let result = Tenure::paginate(&mut db, page, page_size)
        .await
        .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?;

    Ok(reply::json(&ApiResponse::ok("success".to_owned(), result)))
}

async fn read_tenure(id: i32, db: Arc<Mutex<AsyncPgConnection>>) -> Result<impl Reply, Rejection> {
    let mut db = db.lock();
    let result = Tenure::read(&mut db, id)
        .await
        .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?;

    match result {
        Some(v) => Ok(reply::json(&ApiResponse::ok("success".to_owned(), v))),
        None => Err(reject::custom(ClientError::NotFound(
            "tenure not found".to_owned(),
        ))),
    }
}

async fn add_advisor(
    id: i32,
    claims: JwtClaims,
    payload: AddAdvisorPayload,
    db: Arc<Mutex<AsyncPgConnection>>,
) -> Result<impl Reply, Rejection> {
    match *&claims.role {
        UserRole::Secretary | UserRole::Coordinator => {}
        _ => {
            return Err(reject::custom(ClientError::Authorization(
                "user is not authorized to assign advisors".to_string(),
            )));
        }
    }

    let mut db = db.lock();
    let res = match &payload.advisor_type[..] {
        "school" => Tenure::add_advisor_sch(&mut db, id, payload.advisor_id, *&claims.id)
            .await
            .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?,
        "dudi" => Tenure::add_advisor_dudi(&mut db, id, payload.advisor_id, *&claims.id)
            .await
            .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?,
        _ => {
            return Err(reject::custom(ClientError::InvalidInput(
                "invalid advisor type".to_owned(),
            )))
        }
    };

    if res == -1 {
        return Err(reject::custom(ClientError::InvalidInput(
            "submitted advisor user does not have the required role".to_owned(),
        )));
    }

    if res == 0 {
        return Err(reject::custom(ClientError::NotFound(
            "advisor or tenure data not found".to_owned(),
        )));
    }

    Ok(reply::json(&ApiResponse::ok("success".to_owned(), res)))
}

async fn remove_advisor(
    id: i32,
    claims: JwtClaims,
    payload: RemoveAdvisorPayload,
    db: Arc<Mutex<AsyncPgConnection>>,
) -> Result<impl Reply, Rejection> {
    match *&claims.role {
        UserRole::Secretary | UserRole::Coordinator => {}
        _ => {
            return Err(reject::custom(ClientError::Authorization(
                "user is not authorized to dismiss advisors".to_string(),
            )));
        }
    }

    let mut db = db.lock();
    let res = match &payload.advisor_type[..] {
        "school" => Tenure::remove_advisor_sch(&mut db, id, *&claims.id)
            .await
            .map_err(|e| InternalError::DatabaseError(e.to_string()))?,
        "dudi" => Tenure::remove_advisor_dudi(&mut db, id, *&claims.id)
            .await
            .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?,
        _ => {
            return Err(reject::custom(ClientError::InvalidInput(
                "invalid advisor type".to_owned(),
            )))
        }
    };

    if res == 0 {
        return Err(reject::custom(ClientError::NotFound(
            "tenure data not found".to_owned(),
        )));
    }

    Ok(reply::json(&ApiResponse::ok("success".to_owned(), res)))
}

async fn my_tenures(
    claims: JwtClaims,
    db: Arc<Mutex<AsyncPgConnection>>,
) -> Result<impl Reply, Rejection> {
    let mut db = db.lock();
    let result = Tenure::get_tenures_by_user(&mut db, *&claims.id)
        .await
        .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?;

    Ok(reply::json(&ApiResponse::ok("success".to_owned(), result)))
}
