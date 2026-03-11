use std::time::Duration;

/// Interval between gathering metric data
pub const INTERVAL_GATHERING: Duration = Duration::from_millis(1);

/// Interval between sending metric data to the server
pub const INTERVAL_SENDING: Duration = Duration::from_secs(10);
