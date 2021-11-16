use log::info;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite_migration::{Migrations, M};

use crate::backup::create_backup;

mod backup;
mod scraper;

fn initialize_database() -> Pool<SqliteConnectionManager> {
    let pool = r2d2::Pool::new(SqliteConnectionManager::file("folketinget.sqlite3"))
        .expect("failed to open database");

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
    pretty_env_logger::init_timed();

    let pool = initialize_database();

    info!(
        "took startup backup: {}",
        create_backup(pool.clone()).unwrap()
    );

    tokio::spawn(async move { scraper::synchronize(pool.clone()).await })
        .await
        .unwrap();
}
