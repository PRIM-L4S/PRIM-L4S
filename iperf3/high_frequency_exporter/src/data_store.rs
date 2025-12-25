use crate::{VictoriaMetricsFormatting, data_format::MetricDataFormat};

#[derive(VictoriaMetricsFormatting, Debug)]
pub struct MetricDataStore {
    cwnd: MetricDataFormat,
    bytes_sent: MetricDataFormat,
    bytes_received: MetricDataFormat,
}
