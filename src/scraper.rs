use std::time::Duration;

use chrono::NaiveDateTime;
use folketinget_api_models::{ft::domain::models::entity_types, OpenDataType};
use hyper::{client::HttpConnector, Client, Error};
use hyper_openssl::HttpsConnector;
use log::{debug, info};
use odata_simple_client::{Comparison, Connector, DataSource, Filter, Page};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::Connection;
use tokio::time::sleep;

const LOOP_SLEEP_DURATION: Duration = Duration::from_secs(5);

fn get_entity_by_name(typename: &str) -> &'static [(&'static str, OpenDataType)] {
    entity_types()
        .into_iter()
        .find(|(entity, _)| entity == &typename)
        .unwrap()
        .1
}

fn format_insert_statement(typename: &str) -> String {
    let properties = get_entity_by_name(typename);

    let indices = properties
        .iter()
        .map(|(name, _)| name.to_string())
        .collect::<Vec<_>>()
        .join(", ");

    let values = properties
        .iter()
        .map(|(name, _)| format!(":{}", name))
        .collect::<Vec<_>>()
        .join(", ");

    format!("INSERT INTO {}({}) VALUES({})", typename, indices, values)
}

fn serialize(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::Null => "NULL".to_string(),
        serde_json::Value::Bool(b) => b.to_string(),
        serde_json::Value::Number(n) => n.to_string(),
        serde_json::Value::String(s) => s.to_owned(),
        serde_json::Value::Array(_) => unreachable!(),
        serde_json::Value::Object(_) => unreachable!(),
    }
}

fn insert(
    conn: &Connection,
    typename: &str,
    data: &serde_json::Value,
) -> Result<usize, rusqlite::Error> {
    let statement = format_insert_statement(typename);

    debug!("inserting {:#?}", data);
    let mut stmt = conn.prepare(&statement)?;

    let properties = get_entity_by_name(typename);
    for (property, _) in properties {
        let idx = stmt.parameter_index(&format!(":{}", property))?.unwrap();
        stmt.raw_bind_parameter(idx, serialize(data.get(property).unwrap()))?;
    }
    stmt.raw_execute()
}

fn client() -> Client<HttpsConnector<HttpConnector>> {
    let client: Client<HttpsConnector<HttpConnector>> =
        Client::builder().build(HttpsConnector::<HttpConnector>::new().unwrap());

    client
}

async fn get_next<C: Connector>(
    datasource: &DataSource<C>,
    resource_type: &str,
    start: Option<NaiveDateTime>,
) -> Result<Page<serde_json::Value>, odata_simple_client::Error> {
    let start = start.unwrap_or(NaiveDateTime::from_timestamp(0, 0));
    debug!("fetching new {} starting from {}", resource_type, start);

    datasource
        .get_as::<serde_json::Value>(
            resource_type,
            Some(
                Filter::default()
                    .order_by("opdateringsdato", None)
                    .filter(
                        "opdateringsdato",
                        Comparison::GreaterThan,
                        &format!("datetime'{}T{}'", start.date(), start.time()),
                    )
                    .inline_count("allpages".to_string()),
            ),
        )
        .await
}

async fn mirror_next<C: Connector>(
    datasource: &DataSource<C>,
    resource_type: &str,
    pool: &Pool<SqliteConnectionManager>,
) -> Result<Page<serde_json::Value>, Error> {
    let (count, max_id): (Option<u32>, Option<NaiveDateTime>) = pool
        .get()
        .unwrap()
        .query_row(
            &format!(
                "SELECT COUNT(id), MAX(opdateringsdato) FROM {0};",
                resource_type
            ),
            [],
            |row| {
                Ok((
                    row.get(0).ok(),
                    row.get::<usize, String>(1)
                        .ok()
                        .map(|ts| NaiveDateTime::parse_from_str(&ts, "%Y-%m-%dT%H:%M:%S%.3f").ok())
                        .flatten(),
                ))
            },
        )
        .unwrap();

    debug!(
        "maximum opdateringsdato for {}: {:?}",
        resource_type, &max_id
    );
    let page = get_next(&datasource, resource_type, max_id).await.unwrap();

    for value in &page.value {
        insert(&pool.get().unwrap(), resource_type, &value).unwrap();
    }

    let remaining: u32 = page.count.as_deref().unwrap_or("0").parse().unwrap();

    if page.value.len() == 0 {
        debug!(
            "finished mirroring {} ({} total)",
            count.unwrap_or(0),
            resource_type
        );
    } else {
        info!(
            "finished {} more {}, {} remaining ({}%)",
            page.value.len(),
            resource_type,
            remaining,
            100 - (remaining * 100) / (remaining + count.unwrap_or(0))
        );
    }
    Ok(page)
}

pub async fn synchronize(pool: Pool<SqliteConnectionManager>) {
    let ft = DataSource::new(client(), "oda.ft.dk").unwrap();

    loop {
        for (typename, _) in entity_types() {
            // Sambehandlinger don't actually appear to exist in the API, even though it appears
            // in the model. Returns 404 if accessed
            if *typename == "Sambehandlinger" {
                continue;
            }

            loop {
                if mirror_next(&ft, typename, &pool).await.unwrap().value.len() == 0 {
                    break;
                }
            }
        }

        info!(
            "completed synchronization cycle, sleeping for {} seconds",
            LOOP_SLEEP_DURATION.as_secs()
        );
        sleep(LOOP_SLEEP_DURATION).await;
    }
}
