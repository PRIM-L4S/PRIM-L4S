use std::{
    ops::Add,
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use eyre::Result;
use serde_json::{Map, Value};
use tokio::sync::Mutex;

use crate::{constants::DURATION_IPERF, data_store::MetricDataStore};

pub struct Iperf3Config {
    pub sender_port: u16,
    pub destination_address: String,
    pub destination_port: u16,
}

fn get_u64(data: &Map<String, Value>, key: &str) -> u64 {
    data.get(key).and_then(|x| x.as_u64()).unwrap_or_default()
}

fn get_f64(data: &Map<String, Value>, key: &str) -> f64 {
    data.get(key).and_then(|x| x.as_f64()).unwrap_or_default()
}

fn get_duration(data: &Map<String, Value>, key: &str) -> Duration {
    data.get(key)
        .and_then(|x| x.as_f64())
        .and_then(|secs| Duration::try_from_secs_f64(secs).ok())
        .unwrap_or_default()
}

pub async fn make_iperf3_benchmark(
    config: &Iperf3Config,
    data_storage: Arc<Mutex<MetricDataStore>>,
) -> Result<()> {
    let t0 = SystemTime::now();

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
            "iperf3 command failed with status '{}' and stderr: {}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let Value::Object(obj) = serde_json::from_str(&stdout)? else {
        return Ok(());
    };

    let Some(intervals) = obj.get("intervals").and_then(|x| x.as_array()) else {
        return Ok(());
    };

    let mut storage = data_storage.lock().await;

    for interval in intervals {
        let Some(data) = interval
            .as_object()
            .and_then(|hm| hm.get("sum"))
            .and_then(|val| val.as_object())
        else {
            return Ok(());
        };

        let timestamp = t0
            .add(get_duration(data, "seconds"))
            .duration_since(UNIX_EPOCH)
            .expect("The system time is before the UNIX EPOCH")
            .as_millis();

        storage.iperf_bytes.push(timestamp, get_u64(data, "bytes"));
        storage
            .iperf_bits_per_second
            .push(timestamp, get_f64(data, "bits_per_second") as u64);
        storage
            .iperf_retransmits
            .push(timestamp, get_u64(data, "retransmits"));
        storage
            .iperf_snd_cwnd
            .push(timestamp, get_u64(data, "snd_cwnd"));
        storage
            .iperf_snd_wnd
            .push(timestamp, get_u64(data, "snd_wnd"));
        storage.iperf_rtt.push(timestamp, get_u64(data, "rtt"));
        storage
            .iperf_rttvar
            .push(timestamp, get_u64(data, "rttvar"));
        storage.iperf_pmtu.push(timestamp, get_u64(data, "pmtu"));
    }

    Ok(())
}
