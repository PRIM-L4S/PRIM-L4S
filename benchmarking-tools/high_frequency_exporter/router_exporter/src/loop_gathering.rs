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

                for if_stats in stats {
                    let Some(if_storage) = storage.get_mut(if_stats.index) else {
                        continue;
                    };
                    push_metric_u64(&mut if_storage.qdisc_bytes, now, if_stats.bytes);
                    push_metric_u32(&mut if_storage.qdisc_packets, now, if_stats.packets);
                    push_metric_u32(&mut if_storage.qdisc_qlen, now, if_stats.qlen);
                    push_metric_u32(&mut if_storage.qdisc_backlog, now, if_stats.backlog);
                    push_metric_u32(&mut if_storage.qdisc_drops, now, if_stats.drops);
                    push_metric_u32(&mut if_storage.qdisc_requeues, now, if_stats.requeues);
                    push_metric_u32(&mut if_storage.qdisc_overlimits, now, if_stats.overlimits);
                    push_metric_u32(&mut if_storage.qdisc_prob, now, if_stats.prob);
                    push_metric_u32(&mut if_storage.qdisc_delay_c, now, if_stats.delay_c);
                    push_metric_u32(&mut if_storage.qdisc_delay_l, now, if_stats.delay_l);
                    push_metric_u32(
                        &mut if_storage.qdisc_packets_in_c,
                        now,
                        if_stats.packets_in_c,
                    );
                    push_metric_u32(
                        &mut if_storage.qdisc_packets_in_l,
                        now,
                        if_stats.packets_in_l,
                    );
                    push_metric_u32(&mut if_storage.qdisc_maxq, now, if_stats.maxq);
                    push_metric_u32(&mut if_storage.qdisc_ecn_mark, now, if_stats.ecn_mark);
                    if_storage
                        .qdisc_number_of_samples
                        .push(now, number_of_samples);
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
