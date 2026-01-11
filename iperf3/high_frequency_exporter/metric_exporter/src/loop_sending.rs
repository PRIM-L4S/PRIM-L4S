use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use metric_data_store::MetricDataToImport;
use tokio::time::sleep;

use crate::{constants::INTERVAL_SENDING, data_store::MetricDataStore};

/// Sends gathered metric data to the metric server at regular intervals
pub async fn loop_sending(data_storage: Arc<Mutex<MetricDataStore>>, metric_server_url: &str) {
    let http_client = reqwest::Client::new();
    let api_url = format!("{}/api/v1/import", metric_server_url);

    loop {
        let mut storage = data_storage.lock().unwrap();
        let formatted_data = storage.to_import_format();
        // We clear the storage regardless of the sending result
        // This can cause data loss, but also prevents data buildup
        // so that's a trade-off we accept here
        storage.clear();
        drop(storage);

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

        sleep(Duration::from_micros(INTERVAL_SENDING)).await;
    }
}
