use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use tokio::time::sleep;

use crate::socket_statistics::get_socket_statistics;
use crate::{constants::INTERVAL_GATHERING, data_store::MetricDataStore};

pub async fn loop_gathering(
    data_storage: Arc<Mutex<MetricDataStore>>,
    sender_port: u16,
    destination_port: u16,
) {
    let mut fake_value = 0;

    loop {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("The system time is before the UNIX EPOCH")
            .as_millis();

        let socket_stats = get_socket_statistics(sender_port, destination_port).await;

        match socket_stats {
            Ok(stats) => {
                println!(" > Gathered socket statistics\n{:#?}", stats);

                // TODO: Replace with actual gathered values
                let mut storage = data_storage.lock().unwrap();

                storage.cwnd.push(now, fake_value);
                storage.bytes_received.push(now, 100 + fake_value);
                storage.bytes_sent.push(now, 200 + fake_value);

                drop(storage);
            }
            Err(err) => {
                println!(" > Socket statistics failed: {}", err);
            }
        }

        sleep(Duration::from_secs(INTERVAL_GATHERING)).await;

        fake_value += 1;
    }
}
