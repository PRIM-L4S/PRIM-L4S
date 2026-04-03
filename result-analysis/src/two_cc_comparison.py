import re
import numpy as np
import polars as pl
import matplotlib.pyplot as plt
import matplotlib.ticker as mticker

from .data_types import (
    Experiment,
    ExperimentWithResults,
    ExperimentWithResultsAndNbrCCs,
)

from src.experiments_download import experiments_download

from src.utils import format_y_axis_as_scientific_notation

COLORS = {
    "prague": "tab:blue",
    "cubic": "tab:orange",
    "bbr": "tab:green",
}


def _get_two_cc_scenario_pattern(
    cc1: str,
    cc2: str,
    other_params: str,
) -> re.Pattern:
    return re.compile(
        rf"^(?:"
        rf"(?P<single_n>\d+)(?P<single_cc>{re.escape(cc1)}|{re.escape(cc2)})"
        rf"|"
        rf"(?P<left_n>\d+)(?P<left_cc>{re.escape(cc1)}|{re.escape(cc2)})\+"
        rf"(?P<right_n>\d+)(?P<right_cc>{re.escape(cc1)}|{re.escape(cc2)})"
        rf")@{re.escape(other_params)}$"
    )


def _match_pattern(
    pattern: re.Pattern, scenario: str, cc1: str
) -> tuple[int, int] | None:
    match = pattern.match(scenario)
    if not match:
        return None

    single_cc = match.group("single_cc")
    if single_cc:
        single_n = int(match.group("single_n"))
        if single_cc == cc1:
            return single_n, 0
        return 0, single_n

    left_cc = match.group("left_cc")
    right_cc = match.group("right_cc")

    # Reject cases like "3prague+7prague@..."
    if left_cc == right_cc:
        raise ValueError(
            f"Invalid scenario '{scenario}': both sides have the same congestion control algorithm '{left_cc}'."
        )

    left_n = int(match.group("left_n"))
    right_n = int(match.group("right_n"))

    # Always map counts to cc1/cc2 correctly, regardless of side.
    if left_cc == cc1:
        nbr_cc1, nbr_cc2 = left_n, right_n
    else:
        nbr_cc1, nbr_cc2 = right_n, left_n

    return nbr_cc1, nbr_cc2


def filter_two_cc_relevant_experiments(
    experiments: list[Experiment],
    cc1: str,
    cc2: str,
    other_params: str,
) -> list[Experiment]:
    """
    Filters the experiments to keep only those that are relevant for comparing two congestion control algorithms (cc1 and cc2) for given other parameters, and extracts the number of cc1 and cc2 containers in each experiment.
    """
    pattern = _get_two_cc_scenario_pattern(cc1, cc2, other_params)

    # total_nbr_cc = -1
    relevant_experiments = []
    for experiment in experiments:
        match = _match_pattern(pattern, experiment["scenario"], cc1)
        if not match:
            continue
        nbr_cc1, nbr_cc2 = match

        # That's what we shall do in the future
        # But until we fix the scenarii generator,
        # we do the following instead
        #
        # if total_nbr_cc == -1:
        #     total_nbr_cc = nbr_cc1 + nbr_cc2
        # elif total_nbr_cc != nbr_cc1 + nbr_cc2:
        #     raise ValueError(
        #         f"Total number of containers is not consistent across experiments: expected {total_nbr_cc}, got {nbr_cc1 + nbr_cc2} in scenario '{experiment['scenario']}'. Others: {[exp['scenario'] for exp in relevant_experiments]}."
        #     )

        if nbr_cc1 + nbr_cc2 != 10:
            print(
                f"[ ⚠️  ] The total number of containers is not 10 in scenario '{experiment['scenario']}' (got {nbr_cc1 + nbr_cc2}). Skipped."
            )
            continue

        relevant_experiments.append(experiment)

    if len(relevant_experiments) == 0:
        raise ValueError(
            f"No experiments found for cc1='{cc1}', cc2='{cc2}', and other_params='{other_params}'."
        )

    return relevant_experiments


def _map_experiments_with_results(
    experiments: list[ExperimentWithResults],
    cc1: str,
    cc2: str,
    other_params: str,
) -> list[ExperimentWithResultsAndNbrCCs]:
    """
    Adds the number of cc1 and cc2 containers to each experiment.
    """

    pattern = _get_two_cc_scenario_pattern(cc1, cc2, other_params)

    res = []
    for experiment in experiments:
        match = _match_pattern(pattern, experiment["scenario"], cc1)
        if not match:
            raise ValueError(
                f"The experiments given to plot_two_cc_comparison should already be filtered by filter_two_cc_relevant_experiments, but scenario '{experiment['scenario']}' does not match the expected pattern."
            )
        nbr_cc1, nbr_cc2 = match

        res.append(
            {
                "scenario": experiment["scenario"],
                "start_time": experiment["start_time"],
                "end_time": experiment["end_time"],
                "description": experiment["description"],
                "number_of_cc1": nbr_cc1,
                "number_of_cc2": nbr_cc2,
                "results": experiment["results"],
            }
        )

    return res


def _two_cc_comparison(
    experiments: list[ExperimentWithResults],
    metric: str,
    cc1: str,
    cc2: str,
    other_params: str,
) -> tuple[np.ndarray, np.ndarray, np.ndarray]:
    """
    Compares the results of two congestion control algorithms (cc1 and cc2) for a given metric and other parameters.
    """

    experiments_with_results_and_nbr_ccs = _map_experiments_with_results(
        experiments, cc1, cc2, other_params
    )

    experiments_with_results_and_nbr_ccs = sorted(
        experiments_with_results_and_nbr_ccs,
        key=lambda exp: exp["number_of_cc1"],
    )

    share_cc1 = np.zeros(len(experiments_with_results_and_nbr_ccs))
    medians_cc1 = np.zeros(len(experiments_with_results_and_nbr_ccs))
    medians_cc2 = np.zeros(len(experiments_with_results_and_nbr_ccs))

    for i, experiment in enumerate(experiments_with_results_and_nbr_ccs):
        df_metric = experiment["results"].filter(pl.col("metric") == metric)

        medians_cc1[i] = df_metric.filter(pl.col("congestion") == cc1).median()[
            "value"
        ][0]
        medians_cc2[i] = df_metric.filter(pl.col("congestion") == cc2).median()[
            "value"
        ][0]

        share_cc1[i] = experiment["number_of_cc1"] / (
            experiment["number_of_cc1"] + experiment["number_of_cc2"]
        )

    return share_cc1, medians_cc1, medians_cc2


def plot_two_cc_comparison(
    experiments: list[ExperimentWithResults],
    metric: str,
    cc1: str,
    cc2: str,
    other_params: str,
):
    """
    Plots the Comparing two congestion control algorithms (cc1 and cc2) for a given metric.
    """

    share_cc1, medians_cc1, medians_cc2 = _two_cc_comparison(
        experiments, metric, cc1, cc2, other_params
    )

    plt.figure(figsize=(10, 6))
    plt.plot(
        share_cc1, medians_cc1, label=cc1, marker="o", color=COLORS.get(cc1, "tab:blue")
    )
    plt.plot(
        share_cc1,
        medians_cc2,
        label=cc2,
        marker="o",
        color=COLORS.get(cc2, "tab:orange"),
    )
    plt.title(f"Comparing {cc1} and {cc2} for {metric}")
    plt.xlabel("Share of clients using " + cc1)
    plt.gca().xaxis.set_major_formatter(mticker.PercentFormatter(xmax=1.0))
    plt.ylabel(f"Median {metric}")
    format_y_axis_as_scientific_notation()
    plt.legend()
    plt.grid(True)
    plt.tight_layout()


def download_and_save_two_cc_comparison(
    experiments: list[Experiment],
    metrics: list[str],
    cc1: str,
    cc2: str,
    other_params: str,
):
    """
    Completes the experiments with results by adding the number of cc1 and cc2 containers to each experiment.

    Saves two plots: one with the full y-axis range, and one zoomed in around the cc1 values.
    """

    relevant_experiments = filter_two_cc_relevant_experiments(
        experiments, cc1=cc1, cc2=cc2, other_params=other_params
    )

    relevant_experiments_with_results = experiments_download(
        relevant_experiments, metrics
    )

    for metric in metrics:
        share_cc1, medians_cc1, medians_cc2 = _two_cc_comparison(
            relevant_experiments_with_results, metric, cc1, cc2, other_params
        )

        plt.figure(figsize=(10, 6))
        plt.plot(
            share_cc1,
            medians_cc1,
            label=cc1,
            marker="o",
            color=COLORS.get(cc1, "tab:blue"),
        )
        plt.plot(
            share_cc1,
            medians_cc2,
            label=cc2,
            marker="o",
            color=COLORS.get(cc2, "tab:orange"),
        )
        plt.title(f"Comparing {cc1} and {cc2} for {metric}")
        plt.xlabel("Share of clients using " + cc1)
        plt.gca().xaxis.set_major_formatter(mticker.PercentFormatter(xmax=1.0))
        plt.ylabel(f"Median {metric}")
        format_y_axis_as_scientific_notation()
        plt.legend()
        plt.grid(True)
        plt.tight_layout()

        plt.savefig(f"figures/2cc {cc1}+{cc2} {metric} {other_params}.png")

        min_median_cc1 = np.nanmin(medians_cc1)
        max_median_cc1 = np.nanmax(medians_cc1)
        padding = (max_median_cc1 - min_median_cc1) * 0.05

        plt.ylim(min_median_cc1 - padding, max_median_cc1 + padding)

        plt.savefig(f"figures/2cc {cc1}+{cc2} {metric} {other_params} (zoomed).png")
