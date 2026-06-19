from typing import Literal

TITLES_ON_GRAPHS = True

GRAPH_LANGUAGE: Literal["english", "french"] = "english"

match GRAPH_LANGUAGE:
    case "english":
        METRIC_DISPLAY_NAME = {
            "qdisc_delay": "qdisc delay",
            "ss_bytes_acked": "number of bytes acked",
            "ss_bytes_retrans": "number of bytes retransmitted",
            "ss_bytes_sent": "number of bytes sent",
            "ss_delivery_rate": "throughput (in B/s)",
            "ss_rtt": "RTT (in µs)",
            "ss_rttvar": "jitter (in µs)",
            "ss_snd_cwnd": "congestion window size (in units of MSS)",
            "ss_snd_ssthresh": "slow start threshold (in units of MSS)",
        }

        LABEL_DISPLAY_NAME = {
            "prague": "Prague",
            "cubic": "Cubic",
            "bbr": "BBR",
            "qdisc_delay_l": "Qdisc delay L",
            "qdisc_delay_c": "Qdisc delay C",
        }
    case "french":
        METRIC_DISPLAY_NAME = {
            "qdisc_delay": "du délai qdisc",
            "ss_bytes_acked": "du nombre d'octets ackés",
            "ss_bytes_retrans": "du nombre d'octets retransmis",
            "ss_bytes_sent": "du nombre d'octets envoyés",
            "ss_delivery_rate": "du débit (en B/s)",
            "ss_rtt": "de la latence (en µs)",
            "ss_rttvar": "de la gigue (en µs)",
            "ss_snd_cwnd": "de la taille de la fenêtre de congestion (en unités de MSS)",
            "ss_snd_ssthresh": "du seuil de slow start (en unités de MSS)",
        }

        LABEL_DISPLAY_NAME = {
            "prague": "Prague",
            "cubic": "Cubic",
            "bbr": "BBR",
            "qdisc_delay_l": "Délai qdisc L",
            "qdisc_delay_c": "Délai qdisc C",
        }
