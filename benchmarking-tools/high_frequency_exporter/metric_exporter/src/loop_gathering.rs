use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use tokio::sync::Mutex;
use tokio::time::sleep;

use crate::socket_statistics::{SockStatError, SocketStatistics};
use crate::{constants::INTERVAL_GATHERING, data_store::MetricDataStore};

/// Gathers socket statistics in a loop at high frequency
pub async fn loop_gathering(
    data_storage: Arc<Mutex<MetricDataStore>>,
    sender_port: u16,
    destination_port: u16,
) {
    let mut socket_stats = SocketStatistics::new(sender_port, destination_port);
    let mut number_of_samples: u64 = 0;

    loop {
        let start = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("The system time is before the UNIX EPOCH")
            .as_micros();

        let tcp_infos = socket_stats.get_socket_infos().await;

        match tcp_infos {
            Ok(stats) => {
                let mut storage = data_storage.lock().await;

                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("The system time is before the UNIX EPOCH")
                    .as_millis();

                storage.ss_cwnd.push(now, stats.tcpi_snd_cwnd as u64); // TODO: snd or rcv?
                storage.ss_bytes_sent.push(now, stats.tcpi_bytes_sent);
                storage.ss_rtt.push(now, stats.tcpi_rtt as u64);
                storage.ss_rttvar.push(now, stats.tcpi_rttvar as u64);
                storage.ss_number_of_samples.push(now, number_of_samples);

                drop(storage);
            }
            // This error is expected when the iperf3 benchmark is not running yet
            Err(SockStatError::NoMatchingSocket) => (),
            Err(err) => {
                println!(" > Socket statistics failed: {}", err);
            }
        }

        number_of_samples += 1;

        let end = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("The system time is before the UNIX EPOCH")
            .as_micros();

        let duration_to_sleep_us = INTERVAL_GATHERING.saturating_sub((end - start) as u64);

        // if duration_to_sleep_us == 0 {
        //     println!(" > Warning: Gathering loop is taking longer than the intended interval");
        //     continue;
        // }

        sleep(Duration::from_micros(duration_to_sleep_us)).await;
    }
}
