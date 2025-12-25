use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::{constants::INTERVAL_GATHERING, data_store::MetricDataStore};

pub async fn loop_gathering(data_storage: Arc<Mutex<MetricDataStore>>) {
    let mut fake_cwnd_value = 0;

    loop {
        // TODO: Implement the actual gathering logic here
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("The system time is before the UNIX EPOCH")
            .as_millis();

        let mut storage = data_storage.lock().unwrap();

        storage.cwnd.push(now, fake_cwnd_value);
        storage.bytes_received.push(now, 100 + fake_cwnd_value);
        storage.bytes_sent.push(now, 200 + fake_cwnd_value);

        println!("Gathered new data point for cwnd at {}", now);

        drop(storage);

        tokio::time::sleep(tokio::time::Duration::from_secs(INTERVAL_GATHERING)).await;

        fake_cwnd_value += 1;
    }
}
