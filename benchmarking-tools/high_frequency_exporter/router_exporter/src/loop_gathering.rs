use std::sync::Arc;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use metric_data_store::MetricDataFormat;
use tokio::sync::Mutex;

use crate::{constants::INTERVAL_GATHERING, data_store::MetricDataStore};
use qdisc_statistics::{self, QdiscStatistics};

fn push_metric_u64(data_format: &mut MetricDataFormat, now: u128, value: Option<u64>) {
    if let Some(value) = value {
        data_format.push(now, value);
    }
}

fn push_metric_u32(data_format: &mut MetricDataFormat, now: u128, value: Option<u32>) {
    if let Some(value) = value {
        data_format.push(now, value as u64);
    }
}

/// Gathers socket statistics in a loop at high frequency
pub async fn loop_gathering(data_storage: Arc<Mutex<Vec<MetricDataStore>>>) {
    let qdisc_stats = QdiscStatistics::new().await;
    let mut number_of_samples: u64 = 0;

    loop {
        let work_start = Instant::now();

        let tcp_infos = qdisc_stats.poll().await;

        match tcp_infos {
            Ok(stats) => {
                let mut storage = data_storage.lock().await;

                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("The system time is before the UNIX EPOCH")
                    .as_millis();

                for (storage, stats) in storage.iter_mut().zip(stats) {
                    push_metric_u64(&mut storage.qdisc_bytes, now, stats.bytes);
                    push_metric_u32(&mut storage.qdisc_packets, now, stats.packets);
                    push_metric_u32(&mut storage.qdisc_qlen, now, stats.qlen);
                    push_metric_u32(&mut storage.qdisc_backlog, now, stats.backlog);
                    push_metric_u32(&mut storage.qdisc_drops, now, stats.drops);
                    push_metric_u32(&mut storage.qdisc_requeues, now, stats.requeues);
                    push_metric_u32(&mut storage.qdisc_overlimits, now, stats.overlimits);
                    storage.qdisc_number_of_samples.push(now, number_of_samples);
                }

                drop(storage);

                number_of_samples += 1;
            }
            Err(err) => {
                println!(" > Qdisc statistics failed: {}", err);
            }
        }

        let duration_to_sleep = INTERVAL_GATHERING.saturating_sub(work_start.elapsed());
        spin_sleep::sleep(duration_to_sleep);
    }
}
