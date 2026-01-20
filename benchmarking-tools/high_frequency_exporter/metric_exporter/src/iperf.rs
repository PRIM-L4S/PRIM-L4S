use std::{
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use eyre::Result;
use metric_data_store::MetricDataFormat;
use serde_json::{Map, Value};
use tokio::sync::Mutex;

use crate::{constants::DURATION_IPERF, data_store::MetricDataStore};

pub struct Iperf3Config {
    pub sender_port: u16,
    pub destination_address: String,
    pub destination_port: u16,
}

fn get_u64(data: &Map<String, Value>, key: &str) -> Option<u64> {
    data.get(key).and_then(|x| x.as_u64())
}

fn get_f64(data: &Map<String, Value>, key: &str) -> Option<u64> {
    Some(data.get(key).and_then(|x| x.as_f64())? as u64)
}

fn get_duration(data: &Map<String, Value>, key: &str) -> Option<Duration> {
    data.get(key)
        .and_then(|x| x.as_f64())
        .and_then(|secs| Duration::try_from_secs_f64(secs).ok())
}

fn push_metric(data_format: &mut MetricDataFormat, now: u128, value: Option<u64>) {
    if let Some(value) = value {
        data_format.push(now, value);
    }
}

fn push_results(storage: &mut MetricDataStore, mut t0: SystemTime, stdout: &str) -> Result<()> {
    let Value::Object(obj) = serde_json::from_str(stdout)? else {
        return Ok(());
    };

    let Some(intervals) = obj.get("intervals").and_then(|x| x.as_array()) else {
        return Ok(());
    };

    for interval in intervals {
        let Some(data) = interval
            .as_object()
            .and_then(|hm| hm.get("streams"))
            .and_then(|val| val.as_array())
            .and_then(|arr| arr.first())
            .and_then(|val| val.as_object())
        else {
            continue;
        };

        let Some(seconds) = get_duration(data, "seconds") else {
            continue;
        };

        t0 += seconds;
        let now = t0
            .duration_since(UNIX_EPOCH)
            .expect("The system time is before the UNIX EPOCH")
            .as_millis();

        push_metric(&mut storage.iperf_bytes, now, get_u64(data, "bytes"));
        push_metric(
            &mut storage.iperf_bits_per_second,
            now,
            get_f64(data, "bits_per_second"),
        );
        push_metric(
            &mut storage.iperf_retransmits,
            now,
            get_u64(data, "retransmits"),
        );
        push_metric(&mut storage.iperf_snd_cwnd, now, get_u64(data, "snd_cwnd"));
        push_metric(&mut storage.iperf_snd_wnd, now, get_u64(data, "snd_wnd"));
        push_metric(&mut storage.iperf_rtt, now, get_u64(data, "rtt"));
        push_metric(&mut storage.iperf_rttvar, now, get_u64(data, "rttvar"));
        push_metric(&mut storage.iperf_pmtu, now, get_u64(data, "pmtu"));
    }

    Ok(())
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
    let mut storage = data_storage.lock().await;
    push_results(&mut storage, t0, &stdout)?;

    Ok(())
}
