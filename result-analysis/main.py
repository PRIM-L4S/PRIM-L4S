import matplotlib.pyplot as plt

from src.experiments_download import (
    load_experiments_from_csv,
    experiments_download,
)
from src.visualization import plot_experiment


EXPERIMENT_RESULTS_CSV = "./test_results.csv"


def main():
    results = experiments_download(load_experiments_from_csv(EXPERIMENT_RESULTS_CSV))

    for result in results:
        plot_experiment(result, "ss_rtt")

    plt.show()


if __name__ == "__main__":
    main()
