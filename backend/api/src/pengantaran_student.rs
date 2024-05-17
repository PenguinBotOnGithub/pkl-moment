use warp::{
    reject::{self, Rejection},
    reply::Reply,
};

use crate::error::InternalError;

pub async fn get_pengantaran_students() -> Result<impl Reply, Rejection> {
    Err::<String, Rejection>(reject::custom(InternalError::NotImplemented(
        "this feature has not been implemented yet; please contact the administrator or developer"
            .to_owned(),
    )))
}
pub async fn create_pengantaran_student() -> Result<impl Reply, Rejection> {
    Err::<String, Rejection>(reject::custom(InternalError::NotImplemented(
        "this feature has not been implemented yet; please contact the administrator or developer"
            .to_owned(),
    )))
}
pub async fn read_pengantaran_student() -> Result<impl Reply, Rejection> {
    Err::<String, Rejection>(reject::custom(InternalError::NotImplemented(
        "this feature has not been implemented yet; please contact the administrator or developer"
            .to_owned(),
    )))
}
pub async fn update_pengantaran_student() -> Result<impl Reply, Rejection> {
    Err::<String, Rejection>(reject::custom(InternalError::NotImplemented(
        "this feature has not been implemented yet; please contact the administrator or developer"
            .to_owned(),
    )))
}
pub async fn delete_pengantaran_student() -> Result<impl Reply, Rejection> {
    Err::<String, Rejection>(reject::custom(InternalError::NotImplemented(
        "companies deletion hasn't been implemented".to_owned(),
    )))
}
