use metric_data_store::{MetricDataFormat, ToImportFormat};

#[derive(ToImportFormat, Debug)]
pub struct MetricDataStore {
    // === TCP_INFO metrics ===
    /// Instantaneous congestion window size
    pub ss_snd_cwnd: MetricDataFormat,
    /// Slow start threshold
    pub ss_snd_ssthresh: MetricDataFormat,

    /// Total number of bytes sent (including retransmissions) (RFC4898 tcpEStatsPerfHCDataOctetsOut)
    pub ss_bytes_sent: MetricDataFormat,
    /// Total number of retransmitted bytes (RFC4898 tcpEStatsPerfOctetsRetrans)
    pub ss_bytes_retrans: MetricDataFormat,
    /// Total number of octets for which cumulative acknowledgments have been received (RFC4898 tcpEStatsAppHCThruOctetsAcked)
    pub ss_bytes_acked: MetricDataFormat,
    /// Instantaneous troughput in bytes per second
    pub ss_delivery_rate: MetricDataFormat,

    /// Instantaneous round-trip time in microseconds
    pub ss_rtt: MetricDataFormat,
    /// Instantaneous round-trip time variation in microseconds
    pub ss_rttvar: MetricDataFormat,
    /// A temporary metric to assess the speed of data gathering
    pub ss_number_of_samples: MetricDataFormat,

    // === iperf3 metrics ===
    pub iperf_bytes: MetricDataFormat,
    pub iperf_bits_per_second: MetricDataFormat,
    pub iperf_retransmits: MetricDataFormat,
    pub iperf_snd_cwnd: MetricDataFormat,
    pub iperf_snd_wnd: MetricDataFormat,
    pub iperf_rtt: MetricDataFormat,
    pub iperf_rttvar: MetricDataFormat,
    pub iperf_pmtu: MetricDataFormat,

    // === Benchmark metadata ===
    pub hfe_number_of_benchmarks: MetricDataFormat,
}
