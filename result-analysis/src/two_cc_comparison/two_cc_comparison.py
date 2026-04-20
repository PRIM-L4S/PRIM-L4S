import numpy as np
from tqdm import tqdm

from src.two_cc_comparison.scenario_pattern import (
    get_two_cc_scenario_pattern,
    match_pattern,
)
from src.two_cc_comparison.utils import (
    map_experiments_with_results,
)

from src.data_types import (
    Experiment,
    ExperimentWithResults,
    TwoCCGraphConfig,
)


TOTAL_NBR_CONTAINERS = 10


def filter_two_cc_relevant_experiments(
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
        pattern = get_two_cc_scenario_pattern(cc1, cc2, other_params)

        # total_nbr_cc = -1
        for experiment in experiments:
            match = match_pattern(pattern, experiment["scenario"], cc1)
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


def two_cc_comparison(
    experiments: list[ExperimentWithResults],
    graph_config: TwoCCGraphConfig,
) -> tuple[np.ndarray, list[np.ndarray]]:
    """
    Compares the results of two congestion control algorithms (cc1 and cc2) for a given metric and other parameters.
    """

    experiments_with_results_and_nbr_ccs = map_experiments_with_results(
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
