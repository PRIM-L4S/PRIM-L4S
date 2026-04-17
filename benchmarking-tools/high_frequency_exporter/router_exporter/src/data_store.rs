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
    pub qdisc_prob: MetricDataFormat,
    pub qdisc_delay_c: MetricDataFormat,
    pub qdisc_delay_l: MetricDataFormat,
    pub qdisc_packets_in_c: MetricDataFormat,
    pub qdisc_packets_in_l: MetricDataFormat,
    pub qdisc_maxq: MetricDataFormat,
    pub qdisc_ecn_mark: MetricDataFormat,
    pub qdisc_number_of_samples: MetricDataFormat,
}
