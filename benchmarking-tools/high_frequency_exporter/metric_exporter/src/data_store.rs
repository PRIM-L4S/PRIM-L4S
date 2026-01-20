use metric_data_store::MetricDataFormat;
use metric_data_store_derive::ToImportFormat;

#[derive(ToImportFormat, Debug)]
pub struct MetricDataStore {
    // == Metrics gathered from socket statistics with high frequency ==
    pub ss_cwnd: MetricDataFormat,
    pub ss_bytes_sent: MetricDataFormat,
    pub ss_rtt: MetricDataFormat,
    pub ss_rttvar: MetricDataFormat,
    /// Number of bytes delivered marked as L4S available
    pub ss_delivered_e1_bytes: MetricDataFormat,
    pub ss_delivered_e0_bytes: MetricDataFormat,
    /// Number of bytes delivered marked as Congestion Experienced
    pub ss_delivered_ce_bytes: MetricDataFormat,
    /// Number of bytes received marked as L4S available
    pub ss_received_e1_bytes: MetricDataFormat,
    pub ss_received_e0_bytes: MetricDataFormat,
    /// Number of bytes received marked as Congestion Experienced
    pub ss_received_ce_bytes: MetricDataFormat,

    // == Metrics gathered from iperf3 results with low frequency ==
    pub iperf_bytes: MetricDataFormat,
    pub iperf_bits_per_second: MetricDataFormat,
    pub iperf_retransmits: MetricDataFormat,
    pub iperf_snd_cwnd: MetricDataFormat,
    pub iperf_snd_wnd: MetricDataFormat,
    pub iperf_rtt: MetricDataFormat,
    pub iperf_rttvar: MetricDataFormat,
    pub iperf_pmtu: MetricDataFormat,

    // == Additional metrics ==
    pub hfe_number_of_benchmarks: MetricDataFormat,
    /// A temporary metric to assess the speed of data gathering
    pub ss_number_of_samples: MetricDataFormat,
}
