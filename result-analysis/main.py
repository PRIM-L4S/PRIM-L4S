from src.experiments_download import load_experiments_from_csv

from src.two_cc_comparison.generate_configs import (
    generate_router_median_curve_config,
    generate_clients_medians_graph_config,
)
from src.two_cc_comparison.download_and_save import (
    download_and_save_two_cc_comparison,
)


EXPERIMENT_RESULTS_CSV = "./results.csv"

METRICS = [
    "ss_snd_cwnd",
    "ss_snd_ssthresh",
    "ss_bytes_sent",
    "ss_bytes_retrans",
    "ss_bytes_acked",
    "ss_delivery_rate",
    "ss_rtt",
    "ss_rttvar",
]
CC1 = ["prague"]
CC2 = ["cubic", "bbr"]
OTHER_PARAMS = [
    "1000mbit+10ms+1ms",
    "100mbit+10ms+1ms",
    "50mbit+10ms+1ms",
    "10mbit+10ms+1ms",
]


def main():
    experiments = load_experiments_from_csv(EXPERIMENT_RESULTS_CSV)

    graphs_to_generate = []
    # Clients' metrics graphs
    for cc1 in CC1:
        for cc2 in CC2:
            for other_params in OTHER_PARAMS:
                for metric in METRICS:
                    graphs_to_generate.append(
                        generate_clients_medians_graph_config(
                            cc1, cc2, other_params, metric
                        )
                    )

    # Router's queues metrics graphs
    for cc1 in CC1:
        for cc2 in CC2:
            for other_params in OTHER_PARAMS:
                graphs_to_generate.append(
                    {
                        "short_name": "qdisc_delay",
                        "title": f"Comparing the delays of the router's queues when clients use {CC1[0]} and {CC2[0]} with parameters {OTHER_PARAMS[0]}",
                        "yaxis_label": f"Median delay in the queue (in μs?)",
                        "cc1": cc1,
                        "cc2": cc2,
                        "other_params": other_params,
                        "required_metrics": ["qdisc_delay_c", "qdisc_delay_l"],
                        "curves": [
                            generate_router_median_curve_config("qdisc_delay_c"),
                            generate_router_median_curve_config("qdisc_delay_l"),
                        ],
                    }
                )

    download_and_save_two_cc_comparison(experiments, graphs_to_generate)


if __name__ == "__main__":
    main()
