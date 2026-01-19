use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use tokio::sync::Mutex;
use tokio::time::sleep;

use crate::socket_statistics::SocketStatistics;
use crate::{constants::INTERVAL_GATHERING, data_store::MetricDataStore};

/// Gathers socket statistics in a loop at high frequency
pub async fn loop_gathering(
    data_storage: Arc<Mutex<MetricDataStore>>,
    sender_port: u16,
    destination_port: u16,
) {
    let mut socket_stats = SocketStatistics::new(sender_port, destination_port);

    loop {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("The system time is before the UNIX EPOCH")
            .as_millis();

        let tcp_infos = socket_stats.get_socket_infos();

        match tcp_infos {
            Ok(stats) => {
                let mut storage = data_storage.lock().await;

                storage.cwnd.push(now, stats.tcpi_snd_cwnd as u64); // TODO: snd or rcv?
                storage.bytes_sent.push(now, stats.tcpi_bytes_sent);
                storage.rtt.push(now, stats.tcpi_rtt as u64);
                storage.rttvar.push(now, stats.tcpi_rttvar as u64);

                drop(storage);
            }
            Err(err) => {
                println!(" > Socket statistics failed: {}", err);
            }
        }

        sleep(Duration::from_micros(INTERVAL_GATHERING)).await;
    }
}
