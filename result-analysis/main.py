from src.experiments_download import (
    load_experiments_from_csv,
    experiments_download,
    print_experiment_results,
)

EXPERIMENT_RESULTS_CSV = "./results.csv"


def main():
    results = experiments_download(load_experiments_from_csv(EXPERIMENT_RESULTS_CSV))

    print_experiment_results(results)


if __name__ == "__main__":
    main()
