import matplotlib.pyplot as plt

from src.experiments_download import load_experiments_from_csv

from src.two_cc_comparison import download_and_save_two_cc_comparison


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
CC1 = "prague"
OTHER_PARAMS = "1000mbit+10ms+1ms"


def main():
    experiments = load_experiments_from_csv(EXPERIMENT_RESULTS_CSV)

    # download_and_save_two_cc_comparison(
    #     experiments, ["ss_rtt"], CC1, "cubic", OTHER_PARAMS
    # )
    # exit(0)

    for cc2 in ["cubic", "bbr"]:
        download_and_save_two_cc_comparison(
            experiments, METRICS, CC1, cc2, OTHER_PARAMS
        )


if __name__ == "__main__":
    main()
