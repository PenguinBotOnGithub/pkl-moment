use warp::{Filter, Rejection, Reply};

pub fn assets_route() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let static_dir = warp::fs::dir("assets/static");

    let signatures_dir = warp::path("assets")
        .and(warp::path("signatures"))
        .and(warp::fs::dir("assets/signatures"));

    static_dir.or(signatures_dir)
}
