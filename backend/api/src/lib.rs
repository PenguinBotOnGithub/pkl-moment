use std::{convert::Infallible, sync::Arc};

use diesel_async::AsyncPgConnection;
use parking_lot::Mutex;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use warp::{reject::Rejection, Filter};

pub mod auth;
pub mod company;
pub mod error;
pub mod routes;

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
