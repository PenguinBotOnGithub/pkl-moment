use std::sync::Arc;

use diesel_async::AsyncPgConnection;
use parking_lot::Mutex;
use warp::{reject::Rejection, Filter, Reply};

use crate::{
    auth::{login_handler, register_handler, with_auth},
    with_db, with_json, with_jwt_key,
};

pub fn routes(
    db: Arc<Mutex<AsyncPgConnection>>,
    jwt_key: String,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let api = warp::path("api");
    let auth = api.and(warp::path("auth"));

    let login_route = auth
        .and(warp::path("login"))
        .and(warp::path::end())
        .and(warp::post())
        .and(with_jwt_key(jwt_key.clone()))
        .and(with_db(db.clone()))
        .and(with_json())
        .and_then(login_handler);

    let register_route = auth
        .and(warp::path("register"))
        .and(warp::path::end())
        .and(warp::post())
        .and(with_auth(true, jwt_key.clone(), db.clone()))
        .untuple_one()
        .and(with_json())
        .and(with_db(db.clone()))
        .and_then(register_handler);

    let iwak = api
        .and(warp::path::end())
        .and(warp::any())
        .then(|| async { "Iwak 🐟🐟🐟☭☭☭" });

    iwak.or(login_route).or(register_route)
}
