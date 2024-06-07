use std::{collections::HashMap, num::ParseIntError, sync::Arc};

use diesel_async::AsyncPgConnection;
use models::company::{Company, CreateCompany, UpdateCompany};
use parking_lot::Mutex;
use warp::{
    reject::{self, Rejection},
    reply::{self, Reply},
    Filter,
};

use crate::auth::{with_auth_with_claims, JwtClaims};
use crate::error::handle_fk_depended_data_delete;
use crate::{
    auth::with_auth,
    error::{ClientError, InternalError},
    with_db, with_json, ApiResponse,
};

pub fn companies_routes(
    jwt_key: String,
    db: Arc<Mutex<AsyncPgConnection>>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let company = warp::any().and(warp::path("company"));

    let get_companies_route = company
        .and(warp::path::end())
        .and(warp::get())
        .and(with_auth(false, jwt_key.clone(), db.clone()))
        .untuple_one()
        .and(warp::query::query::<HashMap<String, String>>())
        .and(with_db(db.clone()))
        .and_then(get_companies);

    let create_company_route = company
        .and(warp::path("create"))
        .and(warp::path::end())
        .and(warp::post())
        .and(with_auth_with_claims(true, jwt_key.clone(), db.clone()))
        .and(with_json())
        .and(with_db(db.clone()))
        .and_then(create_company);

    let read_company_route = company
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(warp::get())
        .and(with_auth(false, jwt_key.clone(), db.clone()).untuple_one())
        .and(with_db(db.clone()))
        .and_then(read_company);

    let update_company_route = company
        .and(warp::path::param::<i32>())
        .and(warp::path("update"))
        .and(warp::path::end())
        .and(warp::patch())
        .and(with_auth_with_claims(true, jwt_key.clone(), db.clone()))
        .and(with_json())
        .and(with_db(db.clone()))
        .and_then(update_company);

    let delete_company_route = company
        .and(warp::path::param::<i32>())
        .and(warp::path("delete"))
        .and(warp::path::end())
        .and(warp::delete())
        .and(with_auth_with_claims(true, jwt_key.clone(), db.clone()))
        .and(with_db(db.clone()))
        .and_then(delete_company);

    get_companies_route
        .or(create_company_route)
        .or(read_company_route)
        .or(update_company_route)
        .or(delete_company_route)
}

async fn get_companies(
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
    let result = Company::paginate(&mut db, page, page_size)
        .await
        .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?;

    Ok(reply::json(&ApiResponse::ok("success".to_owned(), result)))
}

async fn create_company(
    claims: JwtClaims,
    payload: CreateCompany,
    db: Arc<Mutex<AsyncPgConnection>>,
) -> Result<impl Reply, Rejection> {
    let mut db = db.lock();
    let result = Company::create(&mut db, &payload, claims.id)
        .await
        .map_err(|e| InternalError::DatabaseError(e.to_string()))?;

    Ok(reply::json(&ApiResponse::ok("success".to_owned(), result)))
}

async fn read_company(id: i32, db: Arc<Mutex<AsyncPgConnection>>) -> Result<impl Reply, Rejection> {
    let mut db = db.lock();
    let result = Company::read(&mut db, id)
        .await
        .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?;

    match result {
        Some(v) => Ok(reply::json(&ApiResponse::ok("success".to_owned(), v))),
        None => Err(reject::custom(ClientError::NotFound(
            "company not found".to_owned(),
        ))),
    }
}

async fn update_company(
    id: i32,
    claims: JwtClaims,
    payload: UpdateCompany,
    db: Arc<Mutex<AsyncPgConnection>>,
) -> Result<impl Reply, Rejection> {
    let mut db = db.lock();
    let result = Company::update(&mut db, id, &payload, claims.id)
        .await
        .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?;

    match result {
        Some(v) => Ok(reply::json(&ApiResponse::ok("success".to_owned(), v))),
        None => Err(reject::custom(ClientError::NotFound(
            "company not found".to_owned(),
        ))),
    }
}

async fn delete_company(
    id: i32,
    claims: JwtClaims,
    db: Arc<Mutex<AsyncPgConnection>>,
) -> Result<impl Reply, Rejection> {
    let mut db = db.lock();
    let result = Company::delete(&mut db, id, claims.id)
        .await
        .map_err(handle_fk_depended_data_delete)?;

    if result > 0 {
        Ok(reply::json(&ApiResponse::ok("success".to_owned(), result)))
    } else {
        Err(reject::custom(ClientError::NotFound(
            "company not found".to_owned(),
        )))
    }
}
