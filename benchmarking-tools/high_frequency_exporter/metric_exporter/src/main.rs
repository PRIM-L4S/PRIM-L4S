use std::sync::Arc;

use eyre::Result;
use tokio::join;
use tokio::sync::Mutex;

use crate::data_store::MetricDataStore;
use crate::utils::{env_str, env_u16};

mod constants;
mod data_store;
mod iperf;
mod loop_gathering;
mod loop_iperf;
mod loop_sending;
mod socket_statistics;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let client_name = env_str("CLIENT_NAME")?;
    let metric_server_url = env_str("METRIC_SERVER_URL")?;
    let sender_port = env_u16("SENDER_PORT")?;
    let destination_address = env_str("DESTINATION_ADDRESS")?;
    let destination_port = env_u16("DESTINATION_PORT")?;

    // Handle termination signals gracefully
    // Allows for faster shutdown of containers
    ctrlc::set_handler(|| {
        println!("Received termination signal, exiting...");
        std::process::exit(0);
    })?;

    let data_store = Arc::new(Mutex::new(MetricDataStore::new(client_name)));

    let iperf3_config = iperf::Iperf3Config {
        sender_port,
        destination_address,
        destination_port,
    };

    join!(
        loop_sending::loop_sending(data_store.clone(), &metric_server_url),
        loop_gathering::loop_gathering(data_store.clone(), sender_port, destination_port),
        loop_iperf::loop_iperf(data_store, &iperf3_config),
    );

    Ok(())
}
