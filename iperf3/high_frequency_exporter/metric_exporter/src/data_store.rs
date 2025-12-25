use metric_data_store::MetricDataFormat;
use metric_data_store_derive::ToImportFormat;

#[derive(ToImportFormat, Debug)]
pub struct MetricDataStore {
    cwnd: MetricDataFormat,
    bytes_sent: MetricDataFormat,
    bytes_received: MetricDataFormat,
}
