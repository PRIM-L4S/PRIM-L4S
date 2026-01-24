use std::sync::Arc;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use tokio::sync::Mutex;

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

    // let mut time_worked = Duration::default();
    // let mut time_worked_useless = Duration::default();
    // let mut time_slept = Duration::default();
    // let mut last_reset_time = Instant::now();
    // let mut last_sample_count = 0;

    loop {
        let work_start = Instant::now();

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

                number_of_samples += 1;
                // time_worked += work_start.elapsed();
            }
            // This error is expected when the iperf3 benchmark is not running yet
            Err(SockStatError::NoMatchingSocket) => {
                // time_worked_useless += work_start.elapsed();
            }
            Err(err) => {
                println!(" > Socket statistics failed: {}", err);

                // time_worked_useless += work_start.elapsed();
                // println!(
                //     " > Info: until socket break, I sampled {} times, work time: {} ms, sleep time: {} ms, useless work time (waiting for socket to be ready): {} ms, wall time since last reset: {} ms",
                //     number_of_samples - last_sample_count,
                //     time_worked.as_millis(),
                //     time_slept.as_millis(),
                //     time_worked_useless.as_millis(),
                //     last_reset_time.elapsed().as_millis()
                // );
                // last_sample_count = number_of_samples;
                // time_slept = Duration::default();
                // time_worked = Duration::default();
                // time_worked_useless = Duration::default();
                // last_reset_time = Instant::now();
            }
        }

        let duration_to_sleep = INTERVAL_GATHERING.saturating_sub(work_start.elapsed());
        // time_slept += duration_to_sleep;

        // if duration_to_sleep.is_zero() {
        //     println!(" > Warning: Gathering loop is taking longer than the intended interval");
        //     continue;
        // }

        spin_sleep::sleep(duration_to_sleep);
    }
}
