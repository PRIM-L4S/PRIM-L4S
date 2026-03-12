mod error;
mod generate;
mod scenario;

use std::path::Path;

use clap::Parser;

#[derive(Debug, Parser)]
#[command()]
struct Cli {
    /// Comma-separated bandwidths in Mbps, e.g. 10,50,100,1000
    #[arg(long, value_delimiter = ',')]
    bandwidths: Vec<u32>,

    /// Repeatable client groups, e.g. --client cubic,bbr --client prague
    #[arg(long, action = clap::ArgAction::Append, value_parser = parse_client_group)]
    client: Vec<Vec<String>>,

    /// Comma-separated weight tuples, e.g. 10-90,20-80,30-70
    #[arg(long, value_delimiter = ',', value_parser = parse_weights)]
    weights: Vec<Vec<u32>>,

    /// Destination folder (defaults to cwd)
    #[arg(long)]
    output_folder: Option<String>,
}

fn parse_client_group(s: &str) -> Result<Vec<String>, String> {
    let values: Vec<String> = s
        .split(',')
        .map(str::trim)
        .filter(|x| !x.is_empty())
        .map(ToOwned::to_owned)
        .collect();

    if values.is_empty() {
        return Err("client list cannot be empty".to_string());
    }

    Ok(values)
}

fn parse_weights(input: &str) -> Result<Vec<u32>, String> {
    input
        .split('-')
        .map(|weight| {
            Ok(weight
                .parse::<u32>()
                .map_err(|_| format!("invalid weights list: {input}"))?)
        })
        .collect()
}

fn main() -> eyre::Result<()> {
    let cli = Cli::parse();
    // println!("{:#?}", cli);

    generate::generate(
        10,
        cli.bandwidths,
        cli.client,
        cli.weights,
        Path::new(&cli.output_folder.unwrap_or_default()),
    )
}
