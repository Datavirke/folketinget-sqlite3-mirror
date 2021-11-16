use chrono::Utc;
use log::info;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{backup::Backup, Connection};

pub fn create_backup(pool: Pool<SqliteConnectionManager>) -> Result<String, rusqlite::Error> {
    let target_name = Utc::now()
        .format("folketinget_%Y-%m-%dT%H-%M-%S.sqlite3")
        .to_string();
    info!("creating backup: {}", target_name);

    let mut target = Connection::open(&target_name).expect("unable to open backup database");

    let src = pool.get().unwrap();
    let backup = Backup::new(&src, &mut target)?;

    backup.run_to_completion(100, std::time::Duration::from_millis(1), None)?;

    Ok(target_name)
}
