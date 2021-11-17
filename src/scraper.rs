use std::{sync::Arc, time::Duration};

use chrono::{NaiveDateTime, Utc};
use folketinget_api_models::{ft::domain::models::entity_types, OpenDataType};
use futures::{future::join_all, Future};
use governor::{
    clock::QuantaClock,
    state::{InMemoryState, NotKeyed},
    Quota, RateLimiter,
};
use hyper::{client::HttpConnector, Client, Error};
use hyper_openssl::HttpsConnector;
use log::{debug, info};
use odata_simple_client::{
    Comparison, Connector, DataSource, Direction, InlineCount, ListRequest, Page,
};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::Connection;

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
    rate_limiter: &Arc<RateLimiter<NotKeyed, InMemoryState, QuantaClock>>,
) -> Result<Page<serde_json::Value>, odata_simple_client::Error> {
    let start = start.unwrap_or(NaiveDateTime::from_timestamp(0, 0));
    debug!("fetching new {} starting from {}", resource_type, start);

    let request = ListRequest::new(resource_type)
        .order_by("opdateringsdato", Direction::Ascending)
        .filter(
            "opdateringsdato",
            Comparison::GreaterThan,
            &format!("datetime'{}'", start.format("%Y-%m-%dT%H:%M:%S%.3f")),
        )
        .inline_count(InlineCount::AllPages);

    rate_limiter.until_ready().await;
    datasource.fetch_paged::<serde_json::Value>(request).await
}

async fn mirror_next<C: Connector>(
    datasource: &DataSource<C>,
    resource_type: &str,
    pool: &Pool<SqliteConnectionManager>,
    rate_limiter: &Arc<RateLimiter<NotKeyed, InMemoryState, QuantaClock>>,
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
    let page = get_next(&datasource, resource_type, max_id, &rate_limiter)
        .await
        .unwrap();

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

fn mirror_all<C: Connector>(
    datasource: &DataSource<C>,
    resource_type: &str,
    pool: &Pool<SqliteConnectionManager>,
    rate_limiter: &Arc<RateLimiter<NotKeyed, InMemoryState, QuantaClock>>,
) -> impl Future<Output = Result<usize, Error>> {
    let datasource = datasource.clone();
    let resource_type = resource_type.to_owned();
    let pool = pool.clone();
    let rate_limiter = rate_limiter.clone();

    async move {
        let mut total = 0;
        loop {
            let mirrored = mirror_next(&datasource, &resource_type, &pool, &rate_limiter)
                .await
                .unwrap();

            total += mirrored.value.len();
            if mirrored.value.is_empty() {
                break;
            }
        }

        return Ok(total);
    }
}

pub async fn synchronize(pool: Pool<SqliteConnectionManager>) {
    let ft = DataSource::new(client(), "oda.ft.dk", Some("/api".to_string())).unwrap();
    let rate_limiter = std::sync::Arc::new(RateLimiter::direct(Quota::per_second(
        nonzero_ext::nonzero!(5u32),
    )));

    loop {
        let start = Utc::now();

        let mut syncs = Vec::new();
        for (typename, _) in entity_types() {
            // Sambehandlinger don't actually appear to exist in the API, even though it appears
            // in the model. Returns 404 if accessed
            if *typename == "Sambehandlinger" {
                continue;
            }

            syncs.push(tokio::spawn(mirror_all(
                &ft,
                &typename,
                &pool,
                &rate_limiter,
            )));
        }
        join_all(syncs).await;

        let cycle_time = Utc::now()
            .signed_duration_since(start)
            .to_std()
            .unwrap_or(Duration::from_nanos(1));

        info!(
            "completed synchronization cycle in {}ms",
            cycle_time.as_millis(),
        );
    }
}
