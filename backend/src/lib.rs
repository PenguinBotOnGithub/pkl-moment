use warp::{Filter, Rejection, Reply};

pub fn assets_route() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let static_dir = warp::fs::dir("assets/static");

    let signatures_dir = warp::path("assets")
        .and(warp::path("signatures"))
        .and(warp::fs::dir("assets/signatures"));

    let photos_dir = warp::path("assets")
        .and(warp::path("photos"))
        .and(warp::fs::dir("assets/photos"));

    let mou_dir = warp::path("assets")
        .and(warp::path("mou"))
        .and(warp::fs::dir("assets/mou"));

    static_dir.or(signatures_dir).or(photos_dir).or(mou_dir)
}

pub fn with_cors() -> warp::cors::Cors {
    warp::cors()
        .allow_origins([
            "http://localhost:5173",
            "http://127.0.0.1:5173",
            "https://warp-pkl-moment.shuttleapp.rs",
        ])
        .allow_headers([
            "Accept",
            "Content-Type",
            "Authorization",
            "User-Agent",
            "Sec-Fetch-Mode",
            "Referer",
            "Origin",
            "Access-Control-Request-Method",
            "Access-Control-Request-Headers",
        ])
        .allow_methods(["GET", "POST", "PATCH", "DELETE"])
        .build()
}

pub fn with_dev_cors() -> warp::cors::Cors {
    warp::cors()
        .allow_any_origin()
        .allow_headers([
            "Accept",
            "Content-Type",
            "Authorization",
            "User-Agent",
            "Sec-Fetch-Mode",
            "Referer",
            "Origin",
            "Access-Control-Request-Method",
            "Access-Control-Request-Headers",
        ])
        .expose_headers(["Access-Control-Allow-Origin"])
        .allow_methods(["GET", "POST", "PATCH", "DELETE"])
        .build()
}
