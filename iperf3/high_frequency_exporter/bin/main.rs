use clap::Parser;
use eyre::Result;

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
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let args = AppArgs::parse();

    let http_client = reqwest::Client::new();

    let data = generate_fake_data::generate_fake_metrics();
    let formatted_data = data.to_import_format();

    let res = http_client
        .post(format!("{}/api/v1/import", args.victoria_server_url))
        .body(formatted_data)
        .send()
        .await?;

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());

    let body = res.text().await?;
    println!("Body:\n{}", body);
    Ok(())
}
