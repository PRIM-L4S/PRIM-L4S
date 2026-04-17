use std::time::Duration;

pub const TIME_BETWEEN_SCENARIOS: Duration = Duration::from_secs(5);
pub const RUN_TIME: Duration = Duration::from_mins(4);
pub const MAX_UP_RETRIES: usize = 10;
pub const UP_RETRY_WAIT: Duration = Duration::from_secs(10);
// Assuming 10s to clean up and start the scenario
pub const SCENARIO_SETUP_TIME: Duration = Duration::from_secs(10);
