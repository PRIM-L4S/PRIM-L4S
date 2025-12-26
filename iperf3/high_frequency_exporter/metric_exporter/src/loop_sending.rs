use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use metric_data_store::MetricDataToImport;
use tokio::time::sleep;

use crate::{constants::INTERVAL_SENDING, data_store::MetricDataStore};

pub async fn loop_sending(data_storage: Arc<Mutex<MetricDataStore>>) {
    loop {
        // TODO: Implement the actual sending logic here
        let mut storage = data_storage.lock().unwrap();
        println!("Data that would have been sent: {:#?}", *storage);

        // After sending, we empty the stored data
        storage.clear();
        drop(storage);

        sleep(Duration::from_secs(INTERVAL_SENDING)).await;
    }
}
