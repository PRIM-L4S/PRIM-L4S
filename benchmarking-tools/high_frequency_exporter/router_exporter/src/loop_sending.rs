use std::sync::Arc;

use metric_data_store::MetricDataToImport;
use tokio::{sync::Mutex, time::sleep};

use crate::{constants::INTERVAL_SENDING, data_store::MetricDataStore};

/// Sends gathered metric data to the metric server at regular intervals
pub async fn loop_sending(data_storage: Arc<Mutex<Vec<MetricDataStore>>>, metric_server_url: &str) {
    let http_client = reqwest::Client::new();
    let api_url = format!("{}/api/v1/import", metric_server_url);

    loop {
        let mut storage = data_storage.lock().await;

        let formatted_data_list: Vec<String> = storage
            .iter_mut()
            .map(|entry| {
                let data = entry.to_import_format();
                // We clear the storage regardless of the sending result
                // This can cause data loss, but also prevents data buildup
                // so that's a trade-off we accept here
                entry.clear();
                data
            })
            .collect();

        drop(storage);

        for formatted_data in formatted_data_list {
            match http_client.post(&api_url).body(formatted_data).send().await {
                Err(err) => {
                    println!(
                        " > Failed to send metrics: {}. Tip: the url might be incorrect or the server is unreachable.",
                        err
                    );
                }
                Ok(res) => {
                    if !res.status().is_success() {
                        println!(
                            "Failed to send metrics, server responded with status: {}",
                            res.status()
                        );
                        continue;
                    }
                }
            }
        }

        sleep(INTERVAL_SENDING).await;
    }
}
