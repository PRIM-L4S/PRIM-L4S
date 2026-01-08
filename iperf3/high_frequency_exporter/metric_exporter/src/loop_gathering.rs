use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use tokio::time::sleep;

use crate::socket_statistics::{SockStatError, get_socket_statistics};
use crate::{constants::INTERVAL_GATHERING, data_store::MetricDataStore};

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
                let mut storage = data_storage.lock().unwrap();

                match stats.get_u64_statistic("cwnd") {
                    Ok(cwnd) => storage.cwnd.push(now, cwnd),
                    Err(err) => println!(" > Socket statistics failed (cwnd): {}", err),
                }

                match stats.get_u64_statistic("bytes_sent") {
                    Ok(bytes_sent) => storage.bytes_sent.push(now, bytes_sent),
                    Err(err) => println!(" > Socket statistics failed (bytes_sent): {}", err),
                }

                storage.recv_q.push(now, stats.recv_q);
                storage.send_q.push(now, stats.send_q);

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
