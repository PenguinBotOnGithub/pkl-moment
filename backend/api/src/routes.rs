use std::sync::Arc;

use diesel_async::AsyncPgConnection;
use parking_lot::Mutex;
use tracing::log::debug;
use warp::{reject::Rejection, Filter, Reply};

use crate::user::users_routes;
use crate::{
    auth::auth_routes, company::companies_routes, penarikan::penarikans_routes,
    pengantaran::pengantarans_routes, permohonan::permohonans_routes, signature::signatures_routes,
    student::students_routes, wave::waves_routes,
};

pub fn routes(
    db: Arc<Mutex<AsyncPgConnection>>,
    jwt_key: String,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let api = warp::path("api");

    let root = api.and(warp::path::end()).and(warp::any()).then(|| async {
        debug!("Hello, World!");
        "Hello, World!"
    });

    root.or(api.and(auth_routes(jwt_key.clone(), db.clone())))
        .or(api.and(waves_routes(jwt_key.clone(), db.clone())))
        .or(api.and(students_routes(jwt_key.clone(), db.clone())))
        .or(api.and(companies_routes(jwt_key.clone(), db.clone())))
        .or(api.and(permohonans_routes(jwt_key.clone(), db.clone())))
        .or(api.and(pengantarans_routes(jwt_key.clone(), db.clone())))
        .or(api.and(penarikans_routes(jwt_key.clone(), db.clone())))
        .or(api.and(signatures_routes(jwt_key.clone(), db.clone())))
        .or(api.and(users_routes(jwt_key.clone(), db.clone())))
}
