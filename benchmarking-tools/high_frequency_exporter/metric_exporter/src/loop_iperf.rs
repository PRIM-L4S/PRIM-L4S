use std::{
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use tokio::sync::Mutex;

use crate::{
    constants::INTERVAL_IPERF,
    data_store::MetricDataStore,
    iperf::{self, make_iperf3_benchmark},
};

const INTERVAL_IPERF_MICRO: u128 = 1_000_000 * (INTERVAL_IPERF as u128);

/// Calculates the duration to wait until the next iperf3 benchmark run
/// This ensures that benchmarks are run at consistent intervals
/// synchronized to INTERVAL_IPERF
fn get_duration_to_next_run() -> Duration {
    let now: u128 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_micros();

    let duration_before_next_run = INTERVAL_IPERF_MICRO - (now % INTERVAL_IPERF_MICRO);

    // This cast is safe as duration_before_next_run is smaller than INTERVAL_IPERF_MICRO
    // and INTERVAL_IPERF_MICRO is u128 but fits in u64
    Duration::from_micros(duration_before_next_run as u64)
}

/// Runs iperf3 benchmarks in a loop at intervals defined by INTERVAL_IPERF
pub async fn loop_iperf(data_storage: Arc<Mutex<MetricDataStore>>, config: &iperf::Iperf3Config) {
    let mut number_of_benchmarks: u64 = 0;

    loop {
        number_of_benchmarks += 1;

        let mut storage = data_storage.lock().await;

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("The system time is before the UNIX EPOCH")
            .as_millis();

        storage
            .hfe_number_of_benchmarks
            .push(now, number_of_benchmarks);

        drop(storage);

        // TODO: Gather statistics about the iperf3 run
        println!("Starting iperf3 benchmark");
        if let Err(err) = make_iperf3_benchmark(config).await {
            println!(" > Iperf3 benchmark failed: {}", err);
        }

        tokio::time::sleep(get_duration_to_next_run()).await;
    }
}
