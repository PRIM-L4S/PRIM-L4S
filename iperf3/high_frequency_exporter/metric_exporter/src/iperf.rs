use eyre::Result;

use crate::constants::DURATION_IPERF;

pub struct Iperf3Config {
    pub sender_port: u16,
    pub destination_address: String,
    pub destination_port: u16,
}

pub async fn make_iperf3_benchmark(config: &Iperf3Config) -> Result<()> {
    let output = tokio::process::Command::new("iperf3")
        .arg("--cport")
        .arg(config.sender_port.to_string())
        .arg("--client")
        .arg(&config.destination_address)
        .arg("--port")
        .arg(config.destination_port.to_string())
        .arg("--time")
        .arg(DURATION_IPERF.to_string()) // Run for 60 seconds
        .arg("--json") // Output in JSON format
        .output()
        .await?;

    if !output.status.success() {
        return Err(eyre::eyre!(
            "> Iperf3 command failed with status: {}",
            output.status
        ));
    }

    // let stdout = String::from_utf8_lossy(&output.stdout);
    // let json_output: serde_json::Value = serde_json::from_str(&stdout)?;

    // println!("> Iperf3 Benchmark Results: {:#}", json_output);

    Ok(())
}
