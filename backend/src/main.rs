use std::sync::Arc;

use migration::Migrator;
use migration::MigratorTrait;
use sea_orm::ConnectOptions;
use sea_orm::Database;
use shuttle_runtime::Error;
use warp::Filter;
use warp::Reply;

#[shuttle_runtime::main]
async fn warp(
    #[shuttle_shared_db::Postgres(
        local_uri = "postgres://postgres:postgres@localhost:5432/pkl_moment"
    )]
    conn: String,
) -> shuttle_warp::ShuttleWarp<(impl Reply,)> {
    let conn_options = ConnectOptions::new(conn);
    let db = Database::connect(conn_options)
        .await
        .map_err(|e| Error::Database(e.to_string()))?;

    Migrator::up(&db, None)
        .await
        .map_err(|e| Error::Database(format!("Error running migrations: {e}")))?;

    let db = Arc::new(db);
    let db_filter = warp::any().map(move || Arc::clone(&db)).boxed();

    let route = warp::any().map(|| "Hello, World!");
    Ok(route.boxed().into())
}
