use metric_data_store::MetricDataFormat;
use metric_data_store_derive::ToImportFormat;

#[derive(ToImportFormat, Debug)]
pub struct MetricDataStore {
    pub cwnd: MetricDataFormat,
    pub bytes_sent: MetricDataFormat,
    pub rtt: MetricDataFormat,
    pub rttvar: MetricDataFormat,
    pub number_of_benchmarks: MetricDataFormat,
}
