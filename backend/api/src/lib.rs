use std::{convert::Infallible, sync::Arc};

use diesel_async::AsyncPgConnection;
use parking_lot::Mutex;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use warp::{reject, reject::Rejection, Buf, Filter};

use crate::error::ClientError;

pub mod auth;
pub mod company;
pub mod error;
pub mod penarikan;
pub mod penarikan_student;
pub mod pengantaran;
pub mod pengantaran_student;
pub mod permohonan;
pub mod permohonan_student;
pub mod routes;
pub mod signature;
pub mod student;
pub mod wave;

pub fn with_json<J>() -> impl Filter<Extract = (J,), Error = Rejection> + Clone
where
    J: DeserializeOwned,
    J: Send + Sync,
{
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub fn with_db(
    db: Arc<Mutex<AsyncPgConnection>>,
) -> impl Filter<Extract = (Arc<Mutex<AsyncPgConnection>>,), Error = Infallible> + Clone {
    warp::any().map(move || Arc::clone(&db))
}

pub fn with_jwt_key(
    jwt_key: String,
) -> impl Filter<Extract = (String,), Error = Infallible> + Clone {
    warp::any().map(move || jwt_key.clone())
}

pub fn with_image_upload() -> impl Filter<Extract = (impl Buf,), Error = Rejection> + Clone {
    warp::header::header::<String>("Content-Type")
        .and_then(|v: String| async move {
            if v.starts_with("image") {
                Ok(())
            } else {
                Err(reject::custom(ClientError::InvalidInput(
                    "unsupported media type".to_owned(),
                )))
            }
        })
        .untuple_one()
        .and(warp::body::content_length_limit(1024 * 5000))
        .and(warp::body::aggregate())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    status: &'static str,
    message: String,
    data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn ok(m: String, d: T) -> Self {
        ApiResponse {
            status: "success",
            message: m,
            data: Some(d),
        }
    }

    pub fn error(m: String) -> ApiResponse<u8> {
        ApiResponse {
            status: "error",
            message: m,
            // todo: fix u8 workaround !!!
            data: None::<u8>,
        }
    }
}
