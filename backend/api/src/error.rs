use std::convert::Infallible;

use thiserror::Error;
use tracing::error;
use warp::filters::body::BodyDeserializeError;
use warp::reject::{InvalidHeader, MissingHeader, Reject, Rejection, UnsupportedMediaType};
use warp::{http::StatusCode, reject, reject::MethodNotAllowed, reply::Reply};

use crate::ApiResponse;

#[derive(Debug, Error)]
pub enum InternalError {
    #[error("database error")]
    DatabaseError(String),
    #[error("hashing error")]
    ArgonError(String),
    #[error("jwt error")]
    JwtError(String),
    #[error("not implemented")]
    NotImplemented(String),
    #[error("chrono time error")]
    ChronoError(String),
    #[error("filesystem error")]
    FilesystemError(String),
}

impl Reject for InternalError {}

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("internal data conflict")]
    Conflict(String),
    #[error("authorization related error")]
    Authorization(String),
    #[error("resource not found")]
    NotFound(String),
    #[error("authentication related error")]
    Authentication(String),
    #[error("invalid user input")]
    InvalidInput(String),
}

impl Reject for ClientError {}

pub fn handle_fk_data_not_exists(e: diesel::result::Error) -> Rejection {
    if let diesel::result::Error::DatabaseError(v1, v2) = &e {
        if let diesel::result::DatabaseErrorKind::ForeignKeyViolation = v1 {
            return reject::custom(ClientError::NotFound(format!(
                "the row the foreign key points to doesn't exists: {:?}",
                v2.constraint_name()
                    .unwrap_or("none; please contact administrator or developer for further info")
            )));
        }
    }
    reject::custom(InternalError::DatabaseError(e.to_string()))
}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let (code, message) = if err.is_not_found() {
        (StatusCode::NOT_FOUND, "not found".to_owned())
    } else if let Some(e) = err.find::<MissingHeader>() {
        let e = e.to_string();
        (StatusCode::BAD_REQUEST, format!("missing header: {e}"))
    } else if let Some(e) = err.find::<InvalidHeader>() {
        let e = e.to_string();
        (StatusCode::BAD_REQUEST, format!("invalid header: {e}"))
    } else if let Some(e) = err.find::<ClientError>() {
        match e {
            ClientError::Conflict(e) => (StatusCode::CONFLICT, e.to_owned()),
            ClientError::Authorization(e) => (StatusCode::UNAUTHORIZED, e.to_owned()),
            ClientError::NotFound(e) => (StatusCode::NOT_FOUND, e.to_owned()),
            ClientError::Authentication(e) => (StatusCode::UNAUTHORIZED, e.to_owned()),
            ClientError::InvalidInput(e) => (StatusCode::BAD_REQUEST, e.to_owned()),
        }
    } else if let Some(e) = err.find::<InternalError>() {
        match e {
            InternalError::DatabaseError(e) => {
                error!("database error: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "database error".to_owned(),
                )
            }
            InternalError::ArgonError(e) => {
                error!("hashing error: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "hashing error".to_owned(),
                )
            }
            InternalError::JwtError(e) => {
                error!("jwt error: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "jwt error".to_owned())
            }
            InternalError::NotImplemented(e) => {
                error!("unimplemented error: {}", e);
                (StatusCode::NOT_IMPLEMENTED, e.to_owned())
            }
            InternalError::ChronoError(e) => {
                error!("chrono time error: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_owned())
            }
            InternalError::FilesystemError(e) => {
                error!("filesystem error: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_owned())
            }
        }
    } else if err.find::<MethodNotAllowed>().is_some() {
        (
            StatusCode::METHOD_NOT_ALLOWED,
            "method not allowed".to_owned(),
        )
    } else if let Some(e) = err.find::<BodyDeserializeError>() {
        (StatusCode::BAD_REQUEST, e.to_string())
    } else if let Some(e) = err.find::<UnsupportedMediaType>() {
        (StatusCode::BAD_REQUEST, e.to_string())
    } else {
        error!("unhandled rejection {:?}", err);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "unhandled rejection; please contact administrator/developer".to_owned(),
        )
    };

    Ok(warp::reply::with_status(
        warp::reply::json(&ApiResponse::<Infallible>::error(message)),
        code,
    ))
}
