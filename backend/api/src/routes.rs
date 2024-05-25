use std::sync::Arc;

use diesel_async::AsyncPgConnection;
use parking_lot::Mutex;
use warp::{reject::Rejection, Filter, Reply};

use crate::{
    auth::auth_routes, company::companies_routes, permohonan::permohonans_routes,
    student::students_routes, wave::waves_routes,
};

pub fn routes(
    db: Arc<Mutex<AsyncPgConnection>>,
    jwt_key: String,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let api = warp::path("api");

    let root = api
        .and(warp::path::end())
        .and(warp::any())
        .then(|| async { "Hello, World!" });

    root.or(api.and(auth_routes(jwt_key.clone(), db.clone())))
        .or(api.and(waves_routes(jwt_key.clone(), db.clone())))
        .or(api.and(students_routes(jwt_key.clone(), db.clone())))
        .or(api.and(companies_routes(jwt_key.clone(), db.clone())))
        .or(api.and(permohonans_routes(jwt_key.clone(), db.clone())))
}
