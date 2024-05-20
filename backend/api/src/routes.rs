use std::{collections::HashMap, sync::Arc};

use diesel_async::AsyncPgConnection;
use parking_lot::Mutex;
use warp::{reject::Rejection, Filter, Reply};

use crate::{
    auth::{
        login_handler, refresh_token_handler, register_handler, with_auth, with_auth_with_claims,
    },
    wave::{create_wave, get_waves, read_wave, update_wave},
    with_db, with_json, with_jwt_key,
};

pub fn routes(
    db: Arc<Mutex<AsyncPgConnection>>,
    jwt_key: String,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let api = warp::path("api");
    let auth = api.and(warp::path("auth"));
    let wave = api.and(warp::path("wave"));

    // Auth

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

    let auth_routes = login_route.or(register_route).or(refresh_route);

    // Wave

    let get_waves_route = wave
        .and(warp::path::end())
        .and(warp::get())
        .and(with_auth(false, jwt_key.clone(), db.clone()))
        .untuple_one()
        .and(warp::query::<HashMap<String, String>>())
        .and(with_db(db.clone()))
        .and_then(get_waves);

    let create_wave_route = wave
        .and(warp::path("create"))
        .and(warp::path::end())
        .and(warp::post())
        .and(with_auth(true, jwt_key.clone(), db.clone()))
        .untuple_one()
        .and(with_json())
        .and(with_db(db.clone()))
        .and_then(create_wave);

    let read_wave_route = wave
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(warp::get())
        .and(with_auth(false, jwt_key.clone(), db.clone()).untuple_one())
        .and(with_db(db.clone()))
        .and_then(read_wave);

    let update_wave_route = wave
        .and(warp::path::param::<i32>())
        .and(warp::path("update"))
        .and(warp::path::end())
        .and(with_auth(true, jwt_key.clone(), db.clone()).untuple_one())
        .and(with_json())
        .and(with_db(db.clone()))
        .and_then(update_wave);

    let waves_route = get_waves_route
        .or(create_wave_route)
        .or(read_wave_route)
        .or(update_wave_route);

    let root = api
        .and(warp::path::end())
        .and(warp::any())
        .then(|| async { "Iwak 🐟🐟🐟☭☭☭" });

    root.or(auth_routes).or(waves_route)
}
