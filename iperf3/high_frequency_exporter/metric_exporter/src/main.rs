use std::sync::{Arc, Mutex};

use clap::Parser;
use eyre::Result;
use tokio::join;

use crate::data_store::MetricDataStore;

mod constants;
mod data_store;
mod loop_gathering;
mod loop_sending;
mod socket_statistics;

#[derive(Parser)]
#[clap(version, author, about)]
struct AppArgs {
    #[clap(long)]
    /// The URL of the metrics server to which metrics will be sent.
    /// Should include the protocol (http:// or https://).
    /// Should NOT include the trailing slash.
    ///
    /// Example: http://metrics-server:8428
    metric_server_url: String,

    #[clap(long)]
    /// Port number of the sending address from the studied socket
    /// If studying iperf3, this can be set with the `--cport` argument
    sender_port: u16,

    #[clap(long)]
    /// Port number of the receiving address from the studied socket
    /// If studying iperf3, this is the default port
    destination_port: u16,

    #[clap(long)]
    /// PName of the client
    /// Added as a label for the exported metrics
    host: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let args = AppArgs::parse();

    let data_store = Arc::new(Mutex::new(MetricDataStore::new(args.host)));

    join!(
        loop_sending::loop_sending(data_store.clone(), &args.metric_server_url),
        loop_gathering::loop_gathering(data_store, args.sender_port, args.destination_port),
    );

    Ok(())
}
