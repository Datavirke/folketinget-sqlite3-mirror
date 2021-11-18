use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite_migration::{Migrations, M};
use warp::Filter;

mod backup;
mod config;
mod metrics;
mod scraper;

fn initialize_database(sqlite_path: &str) -> Pool<SqliteConnectionManager> {
    let pool = r2d2::Pool::new(SqliteConnectionManager::file(sqlite_path))
        .expect("failed to open database");

    pool.get().expect("failed to open database");

    let migrations = Migrations::new(vec![M::up(include_str!(
        "../migrations/01-create-tables.sql"
    ))]);

    migrations
        .to_latest(&mut pool.get().unwrap())
        .expect("failed to run migrations");

    pool
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let settings = config::Settings::new();
    pretty_env_logger::init_timed();

    let pool = initialize_database(&settings.sqlite.path);

    tokio::spawn({
        {
            let pool = pool.clone();
            async move {
                warp::serve(
                    backup::backup_routes(pool.clone()).or(metrics::metric_routes(pool.clone())),
                )
                .run(([0, 0, 0, 0], 3030))
                .await
            }
        }
    });

    tokio::spawn(async move { scraper::synchronize(&settings, pool.clone()).await })
        .await
        .unwrap();
}
