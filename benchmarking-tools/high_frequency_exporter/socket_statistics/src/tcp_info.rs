#[derive(Debug, Default)]
#[repr(C)]
// See:
// https://github.com/torvalds/linux/blob/b29fb8829bff243512bb8c8908fd39406f9fd4c3/include/uapi/linux/tcp.h#L247
pub struct TcpInfo {
    pub tcpi_state: u8,
    pub tcpi_ca_state: u8,
    pub tcpi_retransmits: u8,
    pub tcpi_probes: u8,
    pub tcpi_backoff: u8,
    pub tcpi_options: u8,
    /// This contains the bitfields `tcpi_snd_wscale` and `tcpi_rcv_wscale`.
    /// Each is 4 bits.
    pub tcpi_snd_rcv_wscale: u8,
    pub tcpi_rto: u32,
    pub tcpi_ato: u32,
    pub tcpi_snd_mss: u32,
    pub tcpi_rcv_mss: u32,
    pub tcpi_unacked: u32,
    pub tcpi_sacked: u32,
    pub tcpi_lost: u32,
    pub tcpi_retrans: u32,
    pub tcpi_fackets: u32,
    pub tcpi_last_data_sent: u32,
    pub tcpi_last_ack_sent: u32,
    pub tcpi_last_data_recv: u32,
    pub tcpi_last_ack_recv: u32,
    pub tcpi_pmtu: u32,
    pub tcpi_rcv_ssthresh: u32,
    pub tcpi_rtt: u32,
    pub tcpi_rttvar: u32,
    pub tcpi_snd_ssthresh: u32,
    pub tcpi_snd_cwnd: u32,
    pub tcpi_advmss: u32,
    pub tcpi_reordering: u32,
    pub tcpi_rcv_rtt: u32,
    pub tcpi_rcv_space: u32,
    pub tcpi_total_retrans: u32,

    pub tcpi_pacing_rate: u64,
    pub tcpi_max_pacing_rate: u64,
    pub tcpi_bytes_acked: u64, /* RFC4898 tcpEStatsAppHCThruOctetsAcked */
    pub tcpi_bytes_received: u64, /* RFC4898 tcpEStatsAppHCThruOctetsReceived */
    pub tcpi_segs_out: u32,    /* RFC4898 tcpEStatsPerfSegsOut */
    pub tcpi_segs_in: u32,     /* RFC4898 tcpEStatsPerfSegsIn */

    pub tcpi_notsent_bytes: u32,
    pub tcpi_min_rtt: u32,
    pub tcpi_data_segs_in: u32,  /* RFC4898 tcpEStatsDataSegsIn */
    pub tcpi_data_segs_out: u32, /* RFC4898 tcpEStatsDataSegsOut */

    pub tcpi_delivery_rate: u64,

    pub tcpi_busy_time: u64,      /* Time (usec) busy sending data */
    pub tcpi_rwnd_limited: u64,   /* Time (usec) limited by receive window */
    pub tcpi_sndbuf_limited: u64, /* Time (usec) limited by send buffer */

    pub tcpi_delivered: u32,
    pub tcpi_delivered_ce: u32,

    pub tcpi_bytes_sent: u64,    /* RFC4898 tcpEStatsPerfHCDataOctetsOut */
    pub tcpi_bytes_retrans: u64, /* RFC4898 tcpEStatsPerfOctetsRetrans */
    pub tcpi_dsack_dups: u32,    /* RFC4898 tcpEStatsStackDSACKDups */
    pub tcpi_reord_seen: u32,    /* reordering events seen */

    pub tcpi_rcv_ooopack: u32, /* Out-of-order packets received */

    pub tcpi_snd_wnd: u32, /* peer's advertised receive window after scaling (bytes) */
    pub tcpi_rcv_wnd: u32, /* local advertised receive window after scaling (bytes) */

    pub tcpi_rehash: u32, /* PLB or timeout triggered rehash attempts */

    pub tcpi_total_rto: u16, /* Total number of RTO timeouts, including SYN/SYN-ACK and recurring timeouts. */
    pub tcpi_total_rto_recoveries: u16, /* Total number of RTO recoveries, including any unfinished recovery. */
    pub tcpi_total_rto_time: u32, /* Total time spent in RTO recoveries in milliseconds, including any unfinished recovery. */
    // Everything below this doesn't exist on older kernels such as the one on L4S2 server
    pub tcpi_received_ce: u32,        /* # of CE marks received */
    pub tcpi_delivered_e1_bytes: u32, /* Accurate ECN byte counters */
    pub tcpi_delivered_e0_bytes: u32,
    pub tcpi_delivered_ce_bytes: u32,
    pub tcpi_received_e1_bytes: u32,
    pub tcpi_received_e0_bytes: u32,
    pub tcpi_received_ce_bytes: u32,
    pub tcpi_accecn_fail_mode: u16,
    pub tcpi_accecn_opt_seen: u16,
}
