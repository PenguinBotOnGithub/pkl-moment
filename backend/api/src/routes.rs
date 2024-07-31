use std::sync::Arc;

use diesel_async::AsyncPgConnection;
use parking_lot::Mutex;
use tracing::log::debug;
use warp::{reject::Rejection, Filter, Reply};

use crate::auth::auth_routes;
use crate::class::classes_routes;
use crate::company::companies_routes;
use crate::department::departments_routes;
use crate::journal::journals_routes;
use crate::letters::letters_routes;
use crate::log::logs_routes;
use crate::signature::signatures_routes;
use crate::student::students_routes;
use crate::user::users_routes;
use crate::wave::waves_routes;

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
        .or(api.and(classes_routes(jwt_key.clone(), db.clone())))
        .or(api.and(departments_routes(jwt_key.clone(), db.clone())))
        .or(api.and(companies_routes(jwt_key.clone(), db.clone())))
        .or(api.and(letters_routes(jwt_key.clone(), db.clone())))
        .or(api.and(signatures_routes(jwt_key.clone(), db.clone())))
        .or(api.and(users_routes(jwt_key.clone(), db.clone())))
        .or(api.and(logs_routes(jwt_key.clone(), db.clone())))
        .or(api.and(journals_routes(jwt_key.clone(), db.clone())))
}
