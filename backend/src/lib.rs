use warp::{Filter, Rejection, Reply};

pub fn assets_route() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let static_dir = warp::fs::dir("assets/static");

    let signatures_dir = warp::path("assets")
        .and(warp::path("signatures"))
        .and(warp::fs::dir("assets/signatures"));

    static_dir.or(signatures_dir)
}

pub fn with_cors() -> warp::cors::Cors {
    warp::cors()
        .allow_origins([
            "localhost:5173",
            "127.0.0.1:5173",
            "warp-pkl-moment.shuttleapp.rs",
        ])
        .allow_methods(["GET", "POST", "PATCH", "DELETE"])
        .build()
}
