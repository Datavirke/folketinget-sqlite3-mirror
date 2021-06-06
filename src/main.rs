use std::time::Duration;

use folketinget_api_models::{ft::domain::models::entity_types, OpenDataType};
use hyper::{client::HttpConnector, Client, Error};
use hyper_openssl::HttpsConnector;
use log::{debug, info};
use odata_simple_client::{Comparison, Connector, DataSource, Filter, Page};
use rusqlite::Connection;
use tokio::time::sleep;

/// Create SQL batch command for creating all the databases necessary to hold the Folketinget API data
fn create_schema() -> String {
    entity_types()
        .iter()
        .map(|(entity, properties)| {
            let properties = properties
                .iter()
                .map(|(name, description)| {
                    format!(
                        "\t{} {}",
                        name,
                        match *description {
                            OpenDataType::Binary { nullable, key } => format!(
                                "BLOB{}{}",
                                if nullable { " NULL" } else { " NOT NULL" },
                                if key { " PRIMARY KEY" } else { "" }
                            ),
                            OpenDataType::Boolean { nullable, key } => format!(
                                "BOOLEAN{}{}",
                                if nullable { " NULL" } else { " NOT NULL" },
                                if key { " PRIMARY KEY" } else { "" }
                            ),
                            OpenDataType::Byte { nullable, key } => format!(
                                "BYTE{}{}",
                                if nullable { " NULL" } else { " NOT NULL" },
                                if key { " PRIMARY KEY" } else { "" }
                            ),
                            OpenDataType::DateTime { nullable, key } => format!(
                                "DATETIME(3){}{}",
                                if nullable { " NULL" } else { " NOT NULL" },
                                if key { " PRIMARY KEY" } else { "" }
                            ),
                            OpenDataType::DateTimeOffset { nullable, key } => format!(
                                "TEXT{}{}",
                                if nullable { " NULL" } else { " NOT NULL" },
                                if key { " PRIMARY KEY" } else { "" }
                            ),
                            OpenDataType::Decimal { nullable, key } => format!(
                                "DECIMAL{}{}",
                                if nullable { " NULL" } else { " NOT NULL" },
                                if key { " PRIMARY KEY" } else { "" }
                            ),
                            OpenDataType::Double { nullable, key } => format!(
                                "DOUBLE{}{}",
                                if nullable { " NULL" } else { " NOT NULL" },
                                if key { " PRIMARY KEY" } else { "" }
                            ),
                            OpenDataType::Int16 { nullable, key } => format!(
                                "INTEGER{}{}",
                                if nullable { " NULL" } else { " NOT NULL" },
                                if key { " PRIMARY KEY" } else { "" }
                            ),
                            OpenDataType::Int32 { nullable, key } => format!(
                                "INTEGER{}{}",
                                if nullable { " NULL" } else { " NOT NULL" },
                                if key { " PRIMARY KEY" } else { "" }
                            ),
                            OpenDataType::String { nullable, key } => format!(
                                "TEXT{}{}",
                                if nullable { " NULL" } else { " NOT NULL" },
                                if key { " PRIMARY KEY" } else { "" }
                            ),
                        }
                    )
                })
                .collect::<Vec<_>>()
                .join(",\n");

            format!(
                "CREATE TABLE IF NOT EXISTS {} (\n{}\n);\n",
                entity, properties
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

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

fn get_key_name(typename: &str) -> &'static str {
    get_entity_by_name(typename)
        .iter()
        .find(|(_, description)| match description {
            OpenDataType::Binary {
                nullable: _,
                key: true,
            } => true,
            OpenDataType::Boolean {
                nullable: _,
                key: true,
            } => true,
            OpenDataType::Byte {
                nullable: _,
                key: true,
            } => true,
            OpenDataType::DateTime {
                nullable: _,
                key: true,
            } => true,
            OpenDataType::DateTimeOffset {
                nullable: _,
                key: true,
            } => true,
            OpenDataType::Decimal {
                nullable: _,
                key: true,
            } => true,
            OpenDataType::Double {
                nullable: _,
                key: true,
            } => true,
            OpenDataType::Int16 {
                nullable: _,
                key: true,
            } => true,
            OpenDataType::Int32 {
                nullable: _,
                key: true,
            } => true,
            OpenDataType::String {
                nullable: _,
                key: true,
            } => true,
            _ => false,
        })
        .unwrap()
        .0
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
    start: Option<u32>,
) -> Result<Page<serde_json::Value>, odata_simple_client::Error> {
    let start = start.unwrap_or(0);
    debug!("fetching new {} starting from {}", resource_type, start);
    datasource
        .get_as::<serde_json::Value>(
            resource_type,
            Some(
                Filter::default()
                    .order_by("id", None)
                    .filter("id", Comparison::GreaterThan, &start.to_string())
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
    let (count, max_id): (Option<u32>, Option<u32>) = db
        .query_row(
            &format!(
                "SELECT COUNT({1}), MAX({1}) FROM {0};",
                resource_type,
                get_key_name(resource_type)
            ),
            [],
            |row| Ok((row.get(0).ok(), row.get(1).ok())),
        )
        .unwrap();

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

    let db = Connection::open("folketinget.sqlite3").expect("unable to open database");
    let schema = create_schema();
    db.execute_batch(&schema).unwrap();

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
