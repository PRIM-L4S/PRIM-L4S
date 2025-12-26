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
    /// The URL of the Victoria Metrics server to which metrics will be sent.
    /// Should include the protocol (http:// or https://).
    /// Should NOT include the trailing slash.
    ///
    /// Example: http://victoriametrics:8428
    victoria_server_url: String,

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
        loop_sending::loop_sending(data_store.clone()),
        loop_gathering::loop_gathering(data_store, args.sender_port, args.destination_port),
    );

    Ok(())

    // let http_client = reqwest::Client::new();

    // let data = generate_fake_data::generate_fake_metrics();
    // let formatted_data = data.to_import_format();

    // let res = http_client
    //     .post(format!("{}/api/v1/import", args.victoria_server_url))
    //     .body(formatted_data)
    //     .send()
    //     .await?;

    // println!("Status: {}", res.status());
    // println!("Headers:\n{:#?}", res.headers());

    // let body = res.text().await?;
    // println!("Body:\n{}", body);
    // Ok(())
}
