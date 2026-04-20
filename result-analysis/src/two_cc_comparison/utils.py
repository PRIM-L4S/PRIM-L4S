from src.data_types import (
    ExperimentWithResults,
    ExperimentWithResultsAndNbrCCs,
    TwoCCGraphConfig,
)
from src.two_cc_comparison.scenario_pattern import (
    get_two_cc_scenario_pattern,
    match_pattern,
)


def graph_foldername(cc1: str, cc2: str, other_params: str) -> str:
    return f"figures/Two CC comparison/{other_params}/{cc1}+{cc2}"


def graph_filename(
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

    return f"{graph_foldername(cc1, cc2, other_params)}/{graph_short_name}{suffix}.png"


def map_experiments_with_results(
    experiments: list[ExperimentWithResults],
    cc1: str,
    cc2: str,
    other_params: str,
) -> list[ExperimentWithResultsAndNbrCCs]:
    """
    Adds the number of cc1 and cc2 containers to each experiment.
    """

    pattern = get_two_cc_scenario_pattern(cc1, cc2, other_params)

    res = []
    for experiment in experiments:
        match = match_pattern(pattern, experiment["scenario"], cc1)
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


def extract_all_necessary_metrics(
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


def graph_config_group_by_ccs_and_params(
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
