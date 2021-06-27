use std::time::Duration;

use chrono::NaiveDateTime;
use folketinget_api_models::{ft::domain::models::entity_types, OpenDataType};
use hyper::{client::HttpConnector, Client, Error};
use hyper_openssl::HttpsConnector;
use log::{debug, info};
use odata_simple_client::{Comparison, Connector, DataSource, Filter, Page};
use rusqlite::Connection;
use rusqlite_migration::{Migrations, M};
use tokio::time::sleep;

fn get_entity_by_name(typename: &str) -> &'static [(&'static str, OpenDataType)] {
    entity_types()
        .into_iter()
        .find(|(entity, _)| entity == &typename)
        .unwrap()
        .1
}

fn insert_statement(typename: &str) -> String {
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

fn insert(
    conn: &Connection,
    typename: &str,
    data: &serde_json::Value,
) -> Result<usize, rusqlite::Error> {
    let statement = insert_statement(typename);

    debug!("inserting {:#?}", data);
    let mut stmt = conn.prepare(&statement)?;

    let properties = get_entity_by_name(typename);
    for (property, _) in properties {
        let idx = stmt.parameter_index(&format!(":{}", property))?.unwrap();
        stmt.raw_bind_parameter(idx, data.get(property))?;
    }
    stmt.raw_execute()
}

fn client() -> Client<HttpsConnector<HttpConnector>> {
    let client: Client<HttpsConnector<HttpConnector>> =
        Client::builder().build(HttpsConnector::<HttpConnector>::new().unwrap());

    client
}

pub async fn get_next<C: Connector>(
    datasource: &DataSource<C>,
    resource_type: &str,
    start: Option<chrono::NaiveDateTime>,
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

pub async fn mirror_next<C: Connector>(
    datasource: &DataSource<C>,
    resource_type: &str,
    db: &Connection,
) -> Result<Page<serde_json::Value>, Error> {
    let (count, max_id): (Option<u32>, Option<NaiveDateTime>) = db
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
                        .map(|ts| {
                            NaiveDateTime::parse_from_str(&ts, "\"%Y-%m-%dT%H:%M:%S%.3f\"").ok()
                        })
                        .flatten(),
                ))
            },
        )
        .unwrap();

    info!(
        "maximum opdateringsdato for {}: {:?}",
        resource_type, &max_id
    );
    let page = get_next(&datasource, resource_type, max_id).await.unwrap();

    for value in &page.value {
        insert(db, resource_type, &value).unwrap();
    }

    let remaining: u32 = page.count.as_deref().unwrap_or("0").parse().unwrap();

    if page.value.len() == 0 {
        info!(
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

#[tokio::main(flavor = "current_thread")]
async fn main() {
    pretty_env_logger::init_timed();

    let mut db = Connection::open("folketinget.sqlite3").expect("unable to open database");

    let migrations = Migrations::new(vec![M::up(include_str!(
        "../migrations/01-create-tables.sql"
    ))]);

    migrations
        .to_latest(&mut db)
        .expect("failed to run migrations");

    let ft = DataSource::new(client(), "oda.ft.dk").unwrap();

    loop {
        for (typename, _) in entity_types() {
            // Sambehandlinger don't actually appear to exist in the API, even though it appears
            // in the model. Returns 404 if accessed
            if *typename == "Sambehandlinger" {
                continue;
            }

            loop {
                if mirror_next(&ft, typename, &db).await.unwrap().value.len() == 0 {
                    break;
                }
            }
        }

        sleep(Duration::from_secs(60)).await;
    }
}
