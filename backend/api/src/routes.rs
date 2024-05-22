use std::{collections::HashMap, sync::Arc};

use diesel_async::AsyncPgConnection;
use parking_lot::Mutex;
use warp::{reject::Rejection, Filter, Reply};

use crate::{
    auth::{auth_routes, with_auth},
    company::{create_company, delete_company, get_companies, read_company, update_company},
    student::students_routes,
    wave::waves_routes,
    with_db, with_json,
};

pub fn routes(
    db: Arc<Mutex<AsyncPgConnection>>,
    jwt_key: String,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let api = warp::path("api");
    let company = api.and(warp::path("company"));

    // Company

    let get_companies_route = company
        .and(warp::path::end())
        .and(warp::get())
        .and(with_auth(false, jwt_key.clone(), db.clone()))
        .untuple_one()
        .and(warp::query::query::<HashMap<String, String>>())
        .and(with_db(db.clone()))
        .and_then(get_companies);

    let create_company_route = company
        .and(warp::path("create"))
        .and(warp::path::end())
        .and(warp::post())
        .and(with_auth(true, jwt_key.clone(), db.clone()))
        .untuple_one()
        .and(with_json())
        .and(with_db(db.clone()))
        .and_then(create_company);

    let read_company_route = company
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(warp::get())
        .and(with_auth(false, jwt_key.clone(), db.clone()).untuple_one())
        .and(with_db(db.clone()))
        .and_then(read_company);

    let update_company_route = company
        .and(warp::path::param::<i32>())
        .and(warp::path("update"))
        .and(warp::path::end())
        .and(warp::patch())
        .and(with_auth(true, jwt_key.clone(), db.clone()).untuple_one())
        .and(with_json())
        .and(with_db(db.clone()))
        .and_then(update_company);

    let delete_company_route = company
        .and(warp::path::param::<i32>())
        .and(warp::path("delete"))
        .and(warp::path::end())
        .and(warp::delete())
        .and(with_auth(true, jwt_key.clone(), db.clone()).untuple_one())
        .and(with_db(db.clone()))
        .and_then(delete_company);

    let companies_route = get_companies_route
        .or(create_company_route)
        .or(read_company_route)
        .or(update_company_route)
        .or(delete_company_route);

    let root = api
        .and(warp::path::end())
        .and(warp::any())
        .then(|| async { "Hello, World!" });

    root.or(api.and(auth_routes(jwt_key.clone(), db.clone())))
        .or(api.and(waves_routes(jwt_key.clone(), db.clone())))
        .or(api.and(students_routes(jwt_key.clone(), db.clone())))
        .or(companies_route)
}
