use std::{
    f64::consts::PI,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::data_format::MetricDataFormat;

const NUM_SAMPLES: u64 = 1000;

pub fn generate_fake_metrics() -> Vec<MetricDataFormat> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("The system time is before the UNIX EPOCH");

    let mut metric1 = MetricDataFormat::new("test_sinus", "value1", "value2");
    let mut metric2 = MetricDataFormat::new("test_cosinus", "value2", "value2");

    for i in 0..NUM_SAMPLES {
        let timestamp = now.as_millis() - (NUM_SAMPLES as u128) + (i as u128);

        metric1.push(
            timestamp,
            (1000.0 * (1.0 + (2.0 * PI * i as f64 / NUM_SAMPLES as f64).sin())) as u64,
        );
        metric2.push(
            timestamp,
            (1000.0 * (1.0 + (2.0 * PI * i as f64 / NUM_SAMPLES as f64).cos())) as u64,
        );
    }

    vec![metric1, metric2]
}
