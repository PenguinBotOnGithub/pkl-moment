use std::sync::Arc;

use diesel_async::AsyncPgConnection;
use parking_lot::Mutex;
use warp::{filters::BoxedFilter, reject::Rejection, Filter, Reply};

use crate::{auth::login_handler, with_json};

pub fn routes(
    with_db: impl FnOnce() -> BoxedFilter<(Arc<Mutex<AsyncPgConnection>>,)>,
    with_jwt_key: impl FnOnce() -> BoxedFilter<(String,)>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let api = warp::path("api");
    let auth = api.and(warp::path("auth"));

    let login_route = auth
        .and(warp::path("login"))
        .and(warp::path::end())
        .and(warp::post())
        .and(with_jwt_key())
        .and(with_db())
        .and(with_json())
        .and_then(login_handler);

    let iwak = api
        .and(warp::path::end())
        .and(warp::any())
        .then(|| async { "Iwak 🐟🐟🐟☭☭☭" })
        .or(login_route);

    iwak
}
