use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use tokio::sync::Mutex;
use tokio::time::sleep;

use crate::socket_statistics::{GetSocketStatError, SockStatError, get_socket_statistics};
use crate::{constants::INTERVAL_GATHERING, data_store::MetricDataStore};

/// Gathers socket statistics in a loop at high frequency
pub async fn loop_gathering(
    data_storage: Arc<Mutex<MetricDataStore>>,
    sender_port: u16,
    destination_port: u16,
) {
    loop {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("The system time is before the UNIX EPOCH")
            .as_millis();

        let socket_stats = get_socket_statistics(sender_port, destination_port).await;

        match socket_stats {
            Ok(stats) => {
                let mut storage = data_storage.lock().await;

                match stats.get_u64_statistic("cwnd") {
                    Ok(cwnd) => storage.ss_cwnd.push(now, cwnd),
                    Err(GetSocketStatError::StatisticNotFound) => (), // silently skip if not found
                    Err(err) => println!(" > Socket statistics failed (cwnd): {}", err),
                }

                match stats.get_u64_statistic("bytes_sent") {
                    Ok(bytes_sent) => storage.ss_bytes_sent.push(now, bytes_sent),
                    Err(GetSocketStatError::StatisticNotFound) => (), // silently skip if not found
                    Err(err) => println!(" > Socket statistics failed (bytes_sent): {}", err),
                }

                match stats.get_f64_f64_statistic("rtt", "/") {
                    Ok((rtt, rttvar)) => {
                        // convert float ms to integer µs
                        storage.ss_rtt.push(now, (rtt * 1000.) as u64);
                        storage.ss_rttvar.push(now, (rttvar * 1000.) as u64);
                    }
                    Err(GetSocketStatError::StatisticNotFound) => (), // silently skip if not found
                    Err(err) => println!(" > Socket statistics failed (rtt+rttvar): {}", err),
                }

                storage.ss_recv_q.push(now, stats.recv_q);
                storage.ss_send_q.push(now, stats.send_q);

                drop(storage);
            }
            // This one is expected between measurements
            // We skip siliently to avoid cluttering the output
            // (that's high frequency)
            Err(SockStatError::NoMatchingSocket) => (),
            Err(err) => {
                println!(" > Socket statistics failed: {}", err);
            }
        }

        sleep(Duration::from_micros(INTERVAL_GATHERING)).await;
    }
}
