use std::sync::Arc;

use diesel_async::AsyncPgConnection;
use parking_lot::Mutex;
use warp::{reject::Rejection, Filter, Reply};

use crate::{
    auth::{
        login_handler, refresh_token_handler, register_handler, with_auth, with_auth_with_claims,
    },
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

    let refresh_route = auth
        .and(warp::path("refresh"))
        .and(warp::path::end())
        .and(warp::get())
        .and(with_auth_with_claims(false, jwt_key.clone(), db.clone()))
        .and(with_jwt_key(jwt_key.clone()))
        .and(with_db(db.clone()))
        .and_then(refresh_token_handler);

    let root = api
        .and(warp::path::end())
        .and(warp::any())
        .then(|| async { "Iwak 🐟🐟🐟☭☭☭" });

    root.or(login_route).or(register_route).or(refresh_route)
}
