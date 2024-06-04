use std::sync::Arc;

use api::error::handle_rejection;
use diesel_async::AsyncPgConnection;
use diesel_async_migrations::{embed_migrations, EmbeddedMigrations};
use parking_lot::Mutex;
use shuttle_runtime::SecretStore;
use warp::Filter;
use warp::Reply;
use warp_pkl_moment::{assets_route, with_cors};

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

    MIGRATIONS
        .run_pending_migrations(&mut db_connection)
        .await
        .map_err(|e| shuttle_runtime::Error::Database(format!("Error running migrations: {e}")))?;

    let arc_db = Arc::new(Mutex::new(db_connection));

    let route = assets_route()
        .or(api::routes::routes(arc_db, jwt_key))
        .with(with_cors())
        .recover(handle_rejection);

    Ok(route.boxed().into())
}
