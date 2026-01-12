/// Interval (in µs) between sending metric data to the server
pub const INTERVAL_SENDING: u64 = 10_000_000; // 10 s

/// Interval (in µs) between gathering metric data
pub const INTERVAL_GATHERING: u64 = 1_000; // 1 ms

/// Interval (in s) between the **start** of each iperf3 benchmarks
pub const INTERVAL_IPERF: u64 = 15; // s

/// Duration (in s) of each iperf3 benchmark. Must be less than INTERVAL_IPERF.
pub const DURATION_IPERF: u64 = 10; // s
