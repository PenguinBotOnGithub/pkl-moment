use std::{collections::HashMap, num::ParseIntError, sync::Arc};

use diesel_async::AsyncPgConnection;
use models::log::{CreateLog, Log};
use parking_lot::Mutex;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use tracing::{debug, error};
use warp::{
    reject::{self, Rejection},
    reply::{self, Reply},
    Buf, Filter,
};

use models::signature::{CreateSignature, Signature, UpdateSignature};
use models::types::{Operation, TableRef};

use crate::auth::{with_auth_with_claims, JwtClaims};
use crate::{
    auth::with_auth,
    error::{ClientError, InternalError},
    with_db, with_image_upload, with_json, ApiResponse,
};

pub fn signatures_routes(
    jwt_key: String,
    db: Arc<Mutex<AsyncPgConnection>>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let signature = warp::any().and(warp::path("signature"));

    let get_signatures_route = signature
        .and(warp::path::end())
        .and(warp::get())
        .and(with_auth(false, jwt_key.clone(), db.clone()))
        .untuple_one()
        .and(warp::query::query::<HashMap<String, String>>())
        .and(with_db(db.clone()))
        .and_then(get_signatures);

    let create_signature_route = signature
        .and(warp::path("create"))
        .and(warp::path::end())
        .and(warp::post())
        .and(with_auth_with_claims(true, jwt_key.clone(), db.clone()))
        .and(with_json())
        .and(with_db(db.clone()))
        .and_then(create_signature);

    let read_signature_route = signature
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(warp::get())
        .and(with_auth(false, jwt_key.clone(), db.clone()).untuple_one())
        .and(with_db(db.clone()))
        .and_then(read_signature);

    let update_signature_route = signature
        .and(warp::path::param::<i32>())
        .and(warp::path("update"))
        .and(warp::path::end())
        .and(warp::patch())
        .and(with_auth_with_claims(true, jwt_key.clone(), db.clone()))
        .and(with_json())
        .and(with_db(db.clone()))
        .and_then(update_signature);

    let delete_signature_route = signature
        .and(warp::path::param::<i32>())
        .and(warp::path("delete"))
        .and(warp::path::end())
        .and(warp::delete())
        .and(with_auth_with_claims(true, jwt_key.clone(), db.clone()))
        .and(with_db(db.clone()))
        .and_then(delete_signature);

    let upload_signature_route = signature
        .and(warp::path::param::<i32>())
        .and(warp::path("upload"))
        .and(warp::path::end())
        .and(warp::post())
        .and(with_auth_with_claims(true, jwt_key.clone(), db.clone()))
        .and(with_image_upload())
        .and(with_db(db.clone()))
        .and_then(upload_signature);

    get_signatures_route
        .or(create_signature_route)
        .or(read_signature_route)
        .or(update_signature_route)
        .or(delete_signature_route)
        .or(upload_signature_route)
}

async fn get_signatures(
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
    let signatures = Signature::paginate(&mut db, page, page_size)
        .await
        .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?;

    Ok(reply::json(&ApiResponse::ok(
        "success".to_owned(),
        signatures,
    )))
}

async fn upload_signature(
    id: i32,
    claims: JwtClaims,
    mut body: impl Buf,
    db: Arc<Mutex<AsyncPgConnection>>,
) -> Result<impl Reply, Rejection> {
    let mut db_ = db.lock();
    let signature = Signature::read(&mut db_, id)
        .await
        .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?;

    drop(db_);

    if let None = signature {
        return Err(reject::custom(ClientError::NotFound(
            "signature not found".to_owned(),
        )));
    }

    let mut img: Vec<u8> = Vec::with_capacity(1024 * 5000);
    while body.has_remaining() {
        let chunk = body.chunk();
        img.extend_from_slice(chunk);
        let count = chunk.len();
        body.advance(count);
    }
    debug!("{:?}", img);

    let mut file = fs::File::create(format!("assets/signatures/{}", signature.unwrap().id))
        .await
        .map_err(|e| reject::custom(InternalError::FilesystemError(e.to_string())))?;
    file.write_all(&img)
        .await
        .map_err(|e| reject::custom(InternalError::FilesystemError(e.to_string())))?;

    let mut db = db.lock();
    if let Err(e) = Log::create(
        &mut db,
        &CreateLog {
            operation_type: Operation::Upload,
            table_affected: TableRef::Signature,
            snapshot: None,
            user_id: claims.id,
        },
    )
    .await
    {
        error!("error logging action: {}", e.to_string());
    }

    Ok(reply::json(&ApiResponse::ok(
        "success".to_owned(),
        None::<u8>,
    )))
}

async fn create_signature(
    claims: JwtClaims,
    payload: CreateSignature,
    db: Arc<Mutex<AsyncPgConnection>>,
) -> Result<impl Reply, Rejection> {
    let mut db = db.lock();
    let result = Signature::create(&mut db, &payload, claims.id)
        .await
        .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?;

    Ok(reply::json(&ApiResponse::ok("success".to_owned(), result)))
}

async fn read_signature(
    id: i32,
    db: Arc<Mutex<AsyncPgConnection>>,
) -> Result<impl Reply, Rejection> {
    let mut db = db.lock();
    let signature = Signature::read(&mut db, id)
        .await
        .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?;

    if let Some(v) = signature {
        Ok(reply::json(&ApiResponse::ok("success".to_owned(), v)))
    } else {
        Err(reject::custom(ClientError::NotFound(
            "signature not found".to_owned(),
        )))
    }
}

async fn update_signature(
    id: i32,
    claims: JwtClaims,
    payload: UpdateSignature,
    db: Arc<Mutex<AsyncPgConnection>>,
) -> Result<impl Reply, Rejection> {
    let mut db = db.lock();
    let result = Signature::update(&mut db, id, &payload, claims.id)
        .await
        .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?;

    if let Some(v) = result {
        Ok(reply::json(&ApiResponse::ok("success".to_owned(), v)))
    } else {
        Err(reject::custom(ClientError::NotFound(
            "signature not found".to_owned(),
        )))
    }
}

async fn delete_signature(
    id: i32,
    claims: JwtClaims,
    db: Arc<Mutex<AsyncPgConnection>>,
) -> Result<impl Reply, Rejection> {
    let mut db = db.lock();
    let result = Signature::delete(&mut db, id, claims.id)
        .await
        .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?;

    if result > 0 {
        Ok(reply::json(&ApiResponse::ok("success".to_owned(), result)))
    } else {
        Err(reject::custom(ClientError::NotFound(
            "signature not found".to_owned(),
        )))
    }
}
