use std::time::{Duration, SystemTime, UNIX_EPOCH};

use tokio::time::Instant;

use crate::{
    constants::INTERVAL_IPERF,
    iperf::{self, make_iperf3_benchmark},
};

/// Calculate the Instant of the next iperf3 benchmark run
/// by aligning to INTERVAL_IPERF
fn get_date_next_run() -> Instant {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let duration_before_next_run = INTERVAL_IPERF - (now % INTERVAL_IPERF);

    Instant::now() + Duration::from_secs(duration_before_next_run)
}

/// Runs iperf3 benchmarks in a loop at intervals defined by INTERVAL_IPERF
pub async fn loop_iperf(config: &iperf::Iperf3Config) {
    loop {
        // TODO: Gather statistics about the iperf3 run
        if let Err(err) = make_iperf3_benchmark(config).await {
            println!(" > Iperf3 benchmark failed: {}", err);
        }

        tokio::time::sleep_until(get_date_next_run()).await;
    }
}
