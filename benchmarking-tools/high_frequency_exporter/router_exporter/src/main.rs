use std::sync::Arc;

use eyre::Result;
use qdisc_statistics::QdiscStatistics;
use tokio::join;
use tokio::sync::Mutex;

use crate::data_store::MetricDataStore;
use crate::utils::env_str;

mod constants;
mod data_store;
mod loop_gathering;
mod loop_sending;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let metric_server_url = env_str("METRIC_SERVER_URL")?;

    // Handle termination signals gracefully
    // Allows for faster shutdown of containers
    ctrlc::set_handler(|| {
        println!("Received termination signal, exiting...");
        std::process::exit(0);
    })?;

    let qdisc_stats = QdiscStatistics::new().await;

    let interface_names = qdisc_stats.get_interface_names().await?;

    let data_store = Arc::new(Mutex::new(
        interface_names
            .into_iter()
            .map(|name| MetricDataStore::new(name))
            .collect(),
    ));

    tokio::spawn(loop_gathering::loop_gathering(data_store.clone()));

    join!(loop_sending::loop_sending(
        data_store.clone(),
        &metric_server_url
    ));

    Ok(())
}
