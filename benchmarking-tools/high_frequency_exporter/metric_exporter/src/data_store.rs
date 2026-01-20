use metric_data_store::MetricDataFormat;
use metric_data_store_derive::ToImportFormat;

#[derive(ToImportFormat, Debug)]
pub struct MetricDataStore {
    pub ss_cwnd: MetricDataFormat,
    pub ss_bytes_sent: MetricDataFormat,
    pub ss_rtt: MetricDataFormat,
    pub ss_rttvar: MetricDataFormat,
    // pub ss_recv_q: MetricDataFormat,
    // pub ss_send_q: MetricDataFormat,
    /// A temporary metric to assess the speed of data gathering
    pub ss_number_of_samples: MetricDataFormat,
    pub iperf_bytes: MetricDataFormat,
    pub iperf_bits_per_second: MetricDataFormat,
    pub iperf_retransmits: MetricDataFormat,
    pub iperf_snd_cwnd: MetricDataFormat,
    pub iperf_snd_wnd: MetricDataFormat,
    pub iperf_rtt: MetricDataFormat,
    pub iperf_rttvar: MetricDataFormat,
    pub iperf_pmtu: MetricDataFormat,
    pub hfe_number_of_benchmarks: MetricDataFormat,
}
