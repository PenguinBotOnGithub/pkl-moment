use warp::{
    reject::{self, Rejection},
    reply::Reply,
};

use crate::error::InternalError;

pub async fn get_pengantarans() -> Result<impl Reply, Rejection> {
    Err::<String, Rejection>(reject::custom(InternalError::NotImplemented(
        "this feature has not been implemented yet; please contact the administrator or developer"
            .to_owned(),
    )))
}
pub async fn create_pengantaran() -> Result<impl Reply, Rejection> {
    Err::<String, Rejection>(reject::custom(InternalError::NotImplemented(
        "this feature has not been implemented yet; please contact the administrator or developer"
            .to_owned(),
    )))
}
pub async fn read_pengantaran() -> Result<impl Reply, Rejection> {
    Err::<String, Rejection>(reject::custom(InternalError::NotImplemented(
        "this feature has not been implemented yet; please contact the administrator or developer"
            .to_owned(),
    )))
}
pub async fn update_pengantaran() -> Result<impl Reply, Rejection> {
    Err::<String, Rejection>(reject::custom(InternalError::NotImplemented(
        "this feature has not been implemented yet; please contact the administrator or developer"
            .to_owned(),
    )))
}
pub async fn delete_pengantaran() -> Result<impl Reply, Rejection> {
    Err::<String, Rejection>(reject::custom(InternalError::NotImplemented(
        "companies deletion hasn't been implemented".to_owned(),
    )))
}
