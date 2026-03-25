import matplotlib.pyplot as plt

from src.experiments_download import (
    load_experiments_from_csv,
    experiments_download,
)
from src.two_cc_comparison import (
    filter_two_cc_relevant_experiments,
    plot_two_cc_comparison,
)


EXPERIMENT_RESULTS_CSV = "./results.csv"

METRIC = "ss_rtt"
CC1 = "prague"
CC2 = "cubic"
OTHER_PARAMS = "1000mbit+10ms+1ms"


def main():
    experiments = load_experiments_from_csv(EXPERIMENT_RESULTS_CSV)

    relevant_experiments = filter_two_cc_relevant_experiments(
        experiments, cc1=CC1, cc2=CC2, other_params=OTHER_PARAMS
    )

    relevant_experiments_with_results = experiments_download(
        relevant_experiments, [METRIC]
    )

    plot_two_cc_comparison(
        relevant_experiments_with_results,
        metric=METRIC,
        cc1=CC1,
        cc2=CC2,
        other_params=OTHER_PARAMS,
    )

    plt.show()


if __name__ == "__main__":
    main()
