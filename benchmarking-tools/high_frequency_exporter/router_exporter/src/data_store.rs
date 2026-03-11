use metric_data_store::{MetricDataFormat, ToImportFormat};

#[derive(ToImportFormat, Debug)]
pub struct MetricDataStore {
    pub qdisc_bytes: MetricDataFormat,
    pub qdisc_packets: MetricDataFormat,
    pub qdisc_qlen: MetricDataFormat,
    pub qdisc_backlog: MetricDataFormat,
    pub qdisc_drops: MetricDataFormat,
    pub qdisc_requeues: MetricDataFormat,
    pub qdisc_overlimits: MetricDataFormat,
    pub qdisc_number_of_samples: MetricDataFormat,
}
