use std::{collections::HashMap, sync::Arc, time::Duration};

use chrono::Utc;
use folketinget_api_models::ft::domain::models::entity_types;
use log::{info, warn};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use tokio::sync::RwLock;
use warp::{Filter, Rejection, Reply};

const METRIC_COLLECTION_FREQUENCY: Duration = Duration::from_secs(60);

fn count_resources(pool: &Pool<SqliteConnectionManager>, resource_type: &str) -> Option<usize> {
    pool.get()
        .unwrap()
        .query_row(
            &format!(
                "SELECT COUNT(id) FROM (SELECT DISTINCT id FROM {0});",
                resource_type
            ),
            [],
            |row| row.get(0),
        )
        .ok()
}

pub async fn update_metrics(
    metrics: Arc<RwLock<HashMap<&'static str, usize>>>,
    pool: Pool<SqliteConnectionManager>,
) {
    loop {
        let start = Utc::now();
        for (typename, _) in entity_types() {
            if let Some(count) = count_resources(&pool, typename) {
                metrics
                    .write()
                    .await
                    .entry(typename)
                    .and_modify(|f| *f = count)
                    .or_insert(count);
            }
        }

        let cycle_time = Utc::now().signed_duration_since(start).to_std().unwrap();
        if cycle_time < METRIC_COLLECTION_FREQUENCY {
            info!(
                "finished metric update loop, sleeping for {}s",
                (METRIC_COLLECTION_FREQUENCY - cycle_time).as_secs()
            );
            tokio::time::sleep(METRIC_COLLECTION_FREQUENCY - cycle_time).await;
        } else {
            warn!("metric collection loop took longer than {} seconds, restarting new loop immediately", METRIC_COLLECTION_FREQUENCY.as_secs());
        }
    }
}

pub fn metric_routes(
    metrics: Arc<RwLock<HashMap<&'static str, usize>>>,
) -> impl warp::Filter<Extract = impl Reply, Error = Rejection> + Clone {
    use std::fmt::Write;

    warp::path!("metrics")
        .and(warp::get())
        .and(warp::path::end().map(move || metrics.clone()))
        .then(
            |metrics: Arc<RwLock<HashMap<&'static str, usize>>>| async move {
                info!("metrics fetched");

                let mut output = String::new();
                for (metric, count) in metrics.read().await.iter() {
                    writeln!(
                        &mut output,
                        "items_cached_total{{resource_type=\"{}\"}} {}",
                        metric, count
                    )
                    .unwrap();
                }

                Ok(warp::http::Response::builder()
                    .header("Content-Type", "text/plain; charset=utf-8")
                    .status(200)
                    .body(output))
            },
        )
}
