use std::time::Duration;

/// Interval between sending metric data to the server
pub const INTERVAL_SENDING: Duration = Duration::from_secs(10);

/// Interval between gathering metric data
pub const INTERVAL_GATHERING: Duration = Duration::from_millis(1);

/// Interval between the **start** of each iperf3 benchmarks
pub const INTERVAL_IPERF: Duration = Duration::from_secs(15);

/// Duration of each iperf3 benchmark. Must be less than INTERVAL_IPERF.
pub const DURATION_IPERF: Duration = Duration::from_secs(10);
