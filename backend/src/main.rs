use diesel_async::AsyncPgConnection;
use diesel_async_migrations::{embed_migrations, EmbeddedMigrations};
use warp::Filter;
use warp::Reply;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

#[shuttle_runtime::main]
async fn warp(
    #[shuttle_shared_db::Postgres(local_uri = "postgres://postgres:postgres@localhost/pkl_moment")]
    mut db_connection: AsyncPgConnection,
) -> shuttle_warp::ShuttleWarp<(impl Reply,)> {
    MIGRATIONS
        .run_pending_migrations(&mut db_connection)
        .await
        .map_err(|e| shuttle_runtime::Error::Database(format!("Error running migrations: {e}")))?;

    let route = warp::any().map(|| "Hello, World!");
    Ok(route.boxed().into())
}
