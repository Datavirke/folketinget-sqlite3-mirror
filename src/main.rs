use std::{collections::HashMap, sync::Arc};

use futures::future::join_all;
use log::info;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite_migration::{Migrations, M};
use tokio::{
    select,
    sync::{
        watch::{self, Receiver},
        RwLock,
    },
};
use warp::Filter;

mod api;
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

async fn start_webserver(
    pool: Pool<SqliteConnectionManager>,
    metrics: Arc<RwLock<HashMap<&'static str, usize>>>,
    mut shutdown: Receiver<bool>,
) {
    select! {
        _ = warp::serve(
            backup::backup_routes(pool.clone())
            .or(metrics::metric_routes(metrics.clone()))
            .or(api::documents::document_routes(pool.clone())),
        ).run(([0, 0, 0, 0], 3030)) => {
            info!("Webserver quit");
        },
        _ = shutdown.changed() => {
            info!("Webserver received Graceful Shutdown request, exiting");
        }
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let settings = config::Settings::new();
    pretty_env_logger::init_timed();

    let (shutdown_signaller, shutdown) = watch::channel(false);

    let pool = initialize_database(&settings.sqlite.path);
    let metrics = Arc::new(RwLock::new(HashMap::new()));

    join_all([
        // Background task for publishing metrics
        tokio::spawn(metrics::update_metrics(metrics.clone(), pool.clone())),
        // Background synchronization task
        tokio::spawn(scraper::synchronize(
            &settings,
            pool.clone(),
            shutdown_signaller.subscribe(),
        )),
        // Webserver task providing access to the SQLite database, as well as the prometheus endpoint.
        tokio::spawn(start_webserver(pool.clone(), metrics.clone(), shutdown)),
        // Ctrl+C watcher.
        tokio::spawn(async move {
            tokio::signal::ctrl_c().await.unwrap();
            info!("Ctrl+C detected, initiating graceful shutdown");
            shutdown_signaller.send(true).unwrap();
        }),
    ])
    .await;

    info!("Graceful shutdown completed.");
}
