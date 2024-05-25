use std::sync::Arc;

use diesel_async::AsyncPgConnection;
use parking_lot::Mutex;
use warp::{Filter, reject::Rejection, Reply};

use crate::{
    auth::auth_routes, company::companies_routes, permohonan::permohonans_routes,
    permohonan_student::permohonan_students_routes, student::students_routes, wave::waves_routes,
};
use crate::penarikan::penarikans_routes;
use crate::penarikan_student::penarikan_students_routes;
use crate::pengantaran::pengantarans_routes;
use crate::pengantaran_student::pengantaran_students_routes;

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
        .or(api.and(permohonan_students_routes(jwt_key.clone(), db.clone())))
        .or(api.and(pengantarans_routes(jwt_key.clone(), db.clone())))
        .or(api.and(pengantaran_students_routes(jwt_key.clone(), db.clone())))
        .or(api.and(penarikans_routes(jwt_key.clone(), db.clone())))
        .or(api.and(penarikan_students_routes(jwt_key.clone(), db.clone())))
}
