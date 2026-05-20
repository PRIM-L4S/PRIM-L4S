import os
import textwrap

import matplotlib.pyplot as plt
import matplotlib.ticker as mticker
import numpy as np
from tqdm import tqdm

from src.data_types import Experiment, TwoCCGraphConfig
from src.experiments_download import experiments_download
from src.two_cc_comparison.two_cc_comparison import (
    filter_two_cc_relevant_experiments,
    two_cc_comparison,
)
from src.two_cc_comparison.utils import (
    extract_all_necessary_metrics,
    graph_config_group_by_ccs_and_params,
    graph_filename,
    graph_foldername,
)
from src.utils import format_y_axis_as_scientific_notation

REQUIRE_ZOOM_THRESHOLD = 5.0
ZOOM_PADDING_RATIO = 0.05
TITLE_WRAP_THRESHOLD = 100


def download_and_save_two_cc_comparison(
    experiments: list[Experiment],
    graph_configs: list[TwoCCGraphConfig],
):
    """
    Completes the experiments with results by adding the number of cc1 and cc2 containers to each experiment.

    Saves two plots: one with the full y-axis range, and one zoomed in around the cc1 values.
    """

    grouped_graph_configs = graph_config_group_by_ccs_and_params(graph_configs)

    # We group by cc1/cc2/other_params to chunck the downloading in smaller pieces
    # to avoid OOM
    for sub_graph_configs in tqdm(grouped_graph_configs.values()):
        relevant_experiments = filter_two_cc_relevant_experiments(
            experiments,
            sub_graph_configs,
        )

        required_metrics = extract_all_necessary_metrics(sub_graph_configs)

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

            share_cc1, curve_values, curve_errors = two_cc_comparison(
                relevant_experiments_with_results, graph_config
            )

            plt.figure(figsize=(10, 6))
            for j, curve_config in enumerate(graph_config["curves"]):
                # yerr creates a vertical error bar of height yerr under and yerr above the point.
                plt.errorbar(
                    share_cc1,
                    curve_values[j],
                    yerr=curve_errors[j],
                    label=curve_config["label"],
                    marker="o",
                    color=curve_config["color"],
                )

            plt.title(
                "\n".join(
                    textwrap.wrap(graph_config["title"], width=TITLE_WRAP_THRESHOLD)
                )
            )
            plt.xlabel("Share of clients using " + cc1)
            plt.gca().xaxis.set_major_formatter(mticker.PercentFormatter(xmax=1.0))
            plt.ylabel(graph_config["yaxis_label"])
            format_y_axis_as_scientific_notation()
            plt.legend()
            plt.grid(True)
            plt.tight_layout()

            # Create the directory if it doesn't exist
            os.makedirs(graph_foldername(cc1, cc2, other_params), exist_ok=True)

            plt.savefig(
                graph_filename(cc1, cc2, other_params, graph_config["short_name"])
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
                    zoomed_title = (
                        graph_config["title"]
                        + f" (zoomed on {graph_config['curves'][i]['label']})"
                    )
                    plt.title(
                        "\n".join(
                            textwrap.wrap(zoomed_title, width=TITLE_WRAP_THRESHOLD)
                        )
                    )

                    plt.savefig(
                        graph_filename(
                            cc1,
                            cc2,
                            other_params,
                            graph_config["short_name"],
                            zoomed_on=graph_config["curves"][i]["label"],
                        )
                    )

            plt.close()
