import os
import re

import numpy as np
import polars as pl
import matplotlib.pyplot as plt
import matplotlib.ticker as mticker
from tqdm import tqdm

from .data_types import (
    Experiment,
    ExperimentWithResults,
    ExperimentWithResultsAndNbrCCs,
    TwoCCCurveConfig,
    TwoCCGraphConfig,
)

from src.experiments_download import experiments_download

from src.utils import format_y_axis_as_scientific_notation

COLORS = {
    "prague": "tab:blue",
    "cubic": "tab:orange",
    "bbr": "tab:green",
}

REQUIRE_ZOOM_THRESHOLD = 5.0
ZOOM_PADDING_RATIO = 0.05
TOTAL_NBR_CONTAINERS = 10


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


def _filter_two_cc_relevant_experiments(
    experiments: list[Experiment],
    graph_configs: list[TwoCCGraphConfig],
) -> list[Experiment]:
    """
    Filters the experiments to keep only those that are relevant for creating the graphs defined by graph_configs.
    """
    relevant_experiments = []

    required_scenarios = set()
    for graph_config in graph_configs:
        cc1 = graph_config["cc1"]
        cc2 = graph_config["cc2"]
        other_params = graph_config["other_params"]

        required_scenarios.add((cc1, cc2, other_params))

    for cc1, cc2, other_params in required_scenarios:
        pattern = _get_two_cc_scenario_pattern(cc1, cc2, other_params)

        # total_nbr_cc = -1
        for experiment in experiments:
            match = _match_pattern(pattern, experiment["scenario"], cc1)
            if not match:
                continue
            nbr_cc1, nbr_cc2 = match

            if nbr_cc1 + nbr_cc2 != TOTAL_NBR_CONTAINERS:
                tqdm.write(
                    f"[ ⚠️  ] The total number of containers is not {TOTAL_NBR_CONTAINERS} in scenario '{experiment['scenario']}' (got {nbr_cc1 + nbr_cc2}). Skipped."
                )
                continue

            relevant_experiments.append(experiment)

        if len(relevant_experiments) == 0:
            raise ValueError(
                f"No experiments found for cc1='{cc1}', cc2='{cc2}', and other_params='{other_params}'."
            )

    return relevant_experiments


def _extract_all_necessary_metrics(
    graph_configs: list[TwoCCGraphConfig],
) -> list[str]:
    """
    Extracts the list of all necessary metrics from the graph configurations.
    """
    metrics = set()
    for graph_config in graph_configs:
        for metric in graph_config["required_metrics"]:
            metrics.add(metric)

    return list(metrics)


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
    graph_config: TwoCCGraphConfig,
) -> tuple[np.ndarray, list[np.ndarray]]:
    """
    Compares the results of two congestion control algorithms (cc1 and cc2) for a given metric and other parameters.
    """

    experiments_with_results_and_nbr_ccs = _map_experiments_with_results(
        experiments,
        graph_config["cc1"],
        graph_config["cc2"],
        graph_config["other_params"],
    )

    experiments_with_results_and_nbr_ccs = sorted(
        experiments_with_results_and_nbr_ccs,
        key=lambda exp: exp["number_of_cc1"],
    )

    share_cc1 = np.zeros(len(experiments_with_results_and_nbr_ccs))
    curve_values = []
    for i in range(len(graph_config["curves"])):
        curve_values.append(np.zeros(len(experiments_with_results_and_nbr_ccs)))

    for i, experiment in enumerate(experiments_with_results_and_nbr_ccs):
        for j, curve_config in enumerate(graph_config["curves"]):
            compute_curve_values = curve_config["compute"]
            curve_values[j][i] = compute_curve_values(experiment)

        share_cc1[i] = experiment["number_of_cc1"] / (
            experiment["number_of_cc1"] + experiment["number_of_cc2"]
        )

    return share_cc1, curve_values


def _graph_config_group_by_ccs_and_params(
    graph_configs: list[TwoCCGraphConfig],
) -> dict[tuple[str, str, str], list[TwoCCGraphConfig]]:
    """
    Groups the graph configurations by their cc1, cc2, and other_params values.
    """
    grouped_configs = {}
    for graph_config in graph_configs:
        key = (
            graph_config["cc1"],
            graph_config["cc2"],
            graph_config["other_params"],
        )
        if key not in grouped_configs:
            grouped_configs[key] = []
        grouped_configs[key].append(graph_config)

    return grouped_configs


def _graph_foldername(cc1: str, cc2: str, other_params: str) -> str:
    return f"figures/Two CC comparison/{other_params}/{cc1}+{cc2}"


def _graph_filename(
    cc1: str,
    cc2: str,
    other_params: str,
    graph_short_name: str,
    zoomed_on: str | None = None,
) -> str:
    if zoomed_on is not None:
        suffix = f" (zoomed on {zoomed_on})"
    else:
        suffix = ""

    return f"{_graph_foldername(cc1, cc2, other_params)}/{graph_short_name}{suffix}.png"


def download_and_save_two_cc_comparison(
    experiments: list[Experiment],
    graph_configs: list[TwoCCGraphConfig],
):
    """
    Completes the experiments with results by adding the number of cc1 and cc2 containers to each experiment.

    Saves two plots: one with the full y-axis range, and one zoomed in around the cc1 values.
    """

    grouped_graph_configs = _graph_config_group_by_ccs_and_params(graph_configs)

    # We group by cc1/cc2/other_params to chunck the downloading in smaller pieces
    # to avoid OOM
    for sub_graph_configs in tqdm(grouped_graph_configs.values()):
        relevant_experiments = _filter_two_cc_relevant_experiments(
            experiments,
            sub_graph_configs,
        )

        required_metrics = _extract_all_necessary_metrics(sub_graph_configs)

        relevant_experiments_with_results = experiments_download(
            relevant_experiments, required_metrics
        )

        for graph_config in tqdm(
            sub_graph_configs,
            leave=False,
        ):
            cc1 = graph_config["cc1"]
            cc2 = graph_config["cc2"]
            other_params = graph_config["other_params"]

            share_cc1, curve_values = _two_cc_comparison(
                relevant_experiments_with_results, graph_config
            )

            plt.figure(figsize=(10, 6))
            for j, curve_config in enumerate(graph_config["curves"]):
                plt.plot(
                    share_cc1,
                    curve_values[j],
                    label=curve_config["label"],
                    marker="o",
                    color=COLORS.get(curve_config["label"], "tab:blue"),
                )

            plt.title(graph_config["title"])
            plt.xlabel("Share of clients using " + cc1)
            plt.gca().xaxis.set_major_formatter(mticker.PercentFormatter(xmax=1.0))
            plt.ylabel(graph_config["yaxis_label"])
            format_y_axis_as_scientific_notation()
            plt.legend()
            plt.grid(True)
            plt.tight_layout()

            # Create the directory if it doesn't exist
            os.makedirs(_graph_foldername(cc1, cc2, other_params), exist_ok=True)

            plt.savefig(
                _graph_filename(cc1, cc2, other_params, graph_config["short_name"])
            )

            # Compute zoomed versions if required
            min_curves = [np.nanmin(curve) for curve in curve_values]
            max_curves = [np.nanmax(curve) for curve in curve_values]
            max_range = max(
                [max_curves[i] - min_curves[i] for i in range(len(curve_values))]
            )

            for i in range(len(curve_values)):
                curve_range = max_curves[i] - min_curves[i]
                if REQUIRE_ZOOM_THRESHOLD * curve_range < max_range:
                    plt.ylim(
                        min_curves[i] - curve_range * ZOOM_PADDING_RATIO,
                        max_curves[i] + curve_range * ZOOM_PADDING_RATIO,
                    )

                    plt.savefig(
                        _graph_filename(
                            cc1,
                            cc2,
                            other_params,
                            graph_config["short_name"],
                            zoomed_on=graph_config["curves"][i]["label"],
                        )
                    )

            plt.close()


def generate_simple_client_median_curve_config(
    cc: str, metric: str
) -> TwoCCCurveConfig:
    return TwoCCCurveConfig(
        label=cc,
        color=COLORS.get(cc, "tab:blue"),
        compute=lambda exp: exp["results"]
        .filter(pl.col("metric") == metric)
        .filter(pl.col("congestion") == cc)
        .median()["value"][0],
    )


def generate_simple_clients_medians_graph_config(
    cc1: str, cc2: str, other_params: str, metric: str
) -> TwoCCGraphConfig:

    return TwoCCGraphConfig(
        short_name=metric,
        title=f"Comparing the clients' {metric} when using {cc1} and {cc2} with parameters {other_params}",
        yaxis_label=f"Median of {metric} accross clients of each CC",
        cc1=cc1,
        cc2=cc2,
        other_params=other_params,
        required_metrics=[metric],
        curves=[
            generate_simple_client_median_curve_config(cc1, metric),
            generate_simple_client_median_curve_config(cc2, metric),
        ],
    )
