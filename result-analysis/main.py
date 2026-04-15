from tqdm import tqdm

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
CC2 = ["cubic", "bbr"]
OTHER_PARAMS = [
    "1000mbit+10ms+1ms",
    "100mbit+10ms+1ms",
    "50mbit+10ms+1ms",
    "10mbit+10ms+1ms",
]


def main():
    experiments = load_experiments_from_csv(EXPERIMENT_RESULTS_CSV)

    for cc2, other_params in tqdm(
        [(cc2, other_params) for cc2 in CC2 for other_params in OTHER_PARAMS]
    ):
        download_and_save_two_cc_comparison(
            experiments, METRICS, CC1, cc2, other_params
        )


if __name__ == "__main__":
    main()
