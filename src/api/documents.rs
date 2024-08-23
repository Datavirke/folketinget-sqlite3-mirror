use chrono::{DateTime, Utc};
use log::info;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use serde::{Deserialize, Serialize};
use warp::{Filter, Rejection, Reply};

use crate::api::DATABASE_DATETIME_FORMAT;

use super::parse_from_local_to_utc;

#[derive(Debug, Serialize)]
struct Document {
    pub id: i32,
    pub last_updated: DateTime<Utc>,
    pub title: String,
}

#[derive(Debug, Deserialize)]
struct DocumentRequest {
    pub since: DateTime<Utc>,
}

fn fetch_documents_newer_than(
    pool: Pool<SqliteConnectionManager>,
    since: DateTime<Utc>,
) -> Vec<Document> {
    let conn = pool.get().unwrap();
    let mut stmt = conn.prepare( "SELECT id, opdateringsdato, titel FROM Dokument WHERE datetime(opdateringsdato) > datetime(:since) ORDER BY opdateringsdato ASC LIMIT 100").unwrap();

    let timezone = since
        .with_timezone(&chrono_tz::Europe::Copenhagen)
        .format(DATABASE_DATETIME_FORMAT)
        .to_string();
    info!("tz: {}", timezone);
    let docs = stmt
        .query_and_then(
            &[(":since", &timezone)],
            |row| -> Result<Document, rusqlite::Error> {
                Ok(Document {
                    id: row.get(0).unwrap(),
                    last_updated: parse_from_local_to_utc(&row.get::<usize, String>(1).unwrap())
                        .unwrap(),
                    title: row.get(2).unwrap(),
                })
            },
        )
        .unwrap()
        .map(Result::unwrap)
        .collect();

    docs
}

// localhost:3030/api/documents?since=2021-11-18T21:58:13.000Z
pub fn document_routes(
    pool: Pool<SqliteConnectionManager>,
) -> impl warp::Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "documents")
        .and(warp::get())
        .and(warp::query::<DocumentRequest>())
        .and(warp::path::end().map(move || pool.clone()))
        .then(|request: DocumentRequest, pool| async move {
            let docs = fetch_documents_newer_than(pool, request.since);

            warp::reply::json(&docs)
        })
}
