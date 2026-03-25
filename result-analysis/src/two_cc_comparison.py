import numpy as np
import polars as pl
import matplotlib.pyplot as plt

from .experiments_download import ExperimentWithResults


def get_two_cc_scenario_name(
    number_of_cc1: int,
    number_of_cc2: int,
    cc1: str,
    cc2: str,
    other_params: str,
) -> str:
    return f"{number_of_cc1}{cc1}+{number_of_cc2}{cc2}@{other_params}"


def two_cc_comparison(
    experiments: list[ExperimentWithResults],
    metric: str,
    cc1: str,
    cc2: str,
    total_containers: int,
    other_params: str,
) -> tuple[np.ndarray, np.ndarray]:
    """
    Compares the results of two congestion control algorithms (cc1 and cc2) for a given metric and other parameters.
    """

    medians_cc1 = np.zeros(total_containers)
    medians_cc2 = np.zeros(total_containers)

    for nbr_cc1 in range(1, total_containers + 1):
        nbr_cc2 = total_containers - nbr_cc1

        scenario_name = get_two_cc_scenario_name(
            nbr_cc1, nbr_cc2, cc1, cc2, other_params
        )
        experiment = next(
            (exp for exp in experiments if exp["scenario"] == scenario_name), None
        )

        if experiment is None:
            raise ValueError(f"Experiment with scenario '{scenario_name}' not found.")

        df_metric = experiment["results"].filter(pl.col("metric") == metric)

        medians_cc1[nbr_cc1 - 1] = df_metric.filter(
            pl.col("congestion") == cc1
        ).median()
        medians_cc2[nbr_cc1 - 1] = df_metric.filter(
            pl.col("congestion") == cc2
        ).median()

    return medians_cc1, medians_cc2


def plot_two_cc_comparison(
    experiments: list[ExperimentWithResults],
    metric: str,
    cc1: str,
    cc2: str,
    total_containers: int,
    other_params: str,
):
    """
    Plots the comparison of two congestion control algorithms (cc1 and cc2) for a given metric.
    """

    medians_cc1, medians_cc2 = two_cc_comparison(
        experiments, metric, cc1, cc2, total_containers, other_params
    )

    x = np.arange(1, total_containers + 1)
    x_share = x / total_containers

    plt.figure(figsize=(10, 6))
    plt.plot(x_share, medians_cc1, label=cc1, marker="o")
    plt.plot(x_share, medians_cc2, label=cc2, marker="o")
    plt.title(f"Comparison of {cc1} and {cc2} for {metric}")
    plt.xlabel("Share of containers using " + cc1)
    plt.ylabel(f"Median {metric}")
    plt.xticks(x_share)
    plt.legend()
    plt.grid(True)
    plt.tight_layout()
