use warp::{
    reject::{self, Rejection},
    reply::Reply,
};

use crate::error::InternalError;

pub async fn get_companies() -> Result<impl Reply, Rejection> {
    Err::<String, Rejection>(reject::custom(InternalError::NotImplemented(
        "this feature has not been implemented yet; please contact the administrator or developer"
            .to_owned(),
    )))
}
pub async fn create_company() -> Result<impl Reply, Rejection> {
    Err::<String, Rejection>(reject::custom(InternalError::NotImplemented(
        "this feature has not been implemented yet; please contact the administrator or developer"
            .to_owned(),
    )))
}
pub async fn read_company() -> Result<impl Reply, Rejection> {
    Err::<String, Rejection>(reject::custom(InternalError::NotImplemented(
        "this feature has not been implemented yet; please contact the administrator or developer"
            .to_owned(),
    )))
}
pub async fn update_company() -> Result<impl Reply, Rejection> {
    Err::<String, Rejection>(reject::custom(InternalError::NotImplemented(
        "this feature has not been implemented yet; please contact the administrator or developer"
            .to_owned(),
    )))
}
pub async fn delete_company() -> Result<impl Reply, Rejection> {
    Err::<String, Rejection>(reject::custom(InternalError::NotImplemented(
        "companies deletion hasn't been implemented".to_owned(),
    )))
}
