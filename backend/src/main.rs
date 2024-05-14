use std::sync::Arc;

use diesel_async::AsyncPgConnection;
use diesel_async_migrations::{embed_migrations, EmbeddedMigrations};
use parking_lot::Mutex;
use shuttle_runtime::SecretStore;
use warp::Filter;
use warp::Reply;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

#[shuttle_runtime::main]
async fn warp(
    #[shuttle_shared_db::Postgres(local_uri = "postgres://postgres:postgres@localhost/pkl_moment")]
    mut db_connection: AsyncPgConnection,
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> shuttle_warp::ShuttleWarp<(impl Reply,)> {
    let jwt_key = secrets
        .get("JWT_SECRET")
        .expect("Failed to get jet key from secrets");
    let with_jwt_key = || warp::any().map(move || jwt_key.clone()).boxed();

    MIGRATIONS
        .run_pending_migrations(&mut db_connection)
        .await
        .map_err(|e| shuttle_runtime::Error::Database(format!("Error running migrations: {e}")))?;

    let arc_db = Arc::new(Mutex::new(db_connection));
    let with_db = || warp::any().map(move || Arc::clone(&arc_db)).boxed();

    let route = warp::any()
        .and(warp::path::end())
        .then(|| async { "Hello, World!" })
        .or(api::routes::routes(with_db, with_jwt_key));
    Ok(route.boxed().into())
}
