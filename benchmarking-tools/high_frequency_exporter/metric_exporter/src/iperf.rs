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

#[derive(Default)]
struct Iperf3Data {
    since_start: Duration,
    bytes: u64,
    bps: f64,
    retransmits: u64,
    snd_cwnd: u64,
    snd_wnd: u64,
    rtt: u64,
    rttvar: u64,
    pmtu: u64,
}

impl Iperf3Data {
    fn from(hm: &Map<String, Value>) -> Option<Self> {
        Some(Self {
            since_start: Duration::from_secs_f64(hm.get("seconds")?.as_f64()?),
            bytes: hm.get("bytes")?.as_u64()?,
            bps: hm.get("bits_per_second")?.as_f64()?,
            retransmits: hm.get("retransmits")?.as_u64()?,
            snd_cwnd: hm.get("snd_cwnd")?.as_u64()?,
            snd_wnd: hm.get("snd_wnd")?.as_u64()?,
            rtt: hm.get("rtt")?.as_u64()?,
            rttvar: hm.get("rttvar")?.as_u64()?,
            pmtu: hm.get("pmtu")?.as_u64()?,
        })
    }
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
        let data = interval
            .as_object()
            .and_then(|hm| hm.get("sum"))
            .and_then(|val| val.as_object())
            .and_then(|hm| Iperf3Data::from(hm))
            .unwrap_or(Iperf3Data::default());

        let timestamp = t0
            .add(data.since_start)
            .duration_since(UNIX_EPOCH)
            .expect("The system time is before the UNIX EPOCH")
            .as_millis();

        storage.iperf_bytes.push(timestamp, data.bytes);
        storage.iperf_bps.push(timestamp, data.bps as u64);
        storage.iperf_retransmits.push(timestamp, data.retransmits);
        storage.iperf_snd_cwnd.push(timestamp, data.snd_cwnd);
        storage.iperf_snd_wnd.push(timestamp, data.snd_wnd);
        storage.iperf_rtt.push(timestamp, data.rtt);
        storage.iperf_rttvar.push(timestamp, data.rttvar);
        storage.iperf_pmtu.push(timestamp, data.pmtu);
    }

    Ok(())
}
