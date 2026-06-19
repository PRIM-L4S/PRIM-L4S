import polars as pl

from src.data_types import (
    ExperimentWithResultsAndNbrCCs,
    TwoCCCurveConfig,
    TwoCCGraphConfig,
)
from src.two_cc_comparison.constants import (
    METRIC_DISPLAY_NAME,
    LABEL_DISPLAY_NAME,
    GRAPH_LANGUAGE,
)

ERROR_T_ALPHA = 1.96  # for a confidence interval of 95%

CC_COLORS = {
    "prague": "tab:blue",
    "cubic": "tab:orange",
    "bbr": "tab:green",
}

QUEUE_COLORS = {
    "qdisc_delay_l": "tab:blue",
    "qdisc_delay_c": "tab:orange",
}


def generate_clients_medians_curve_config(cc: str, metric: str) -> TwoCCCurveConfig:
    def compute_function(exp: ExperimentWithResultsAndNbrCCs) -> tuple[float, float]:
        filtered = (
            exp["results"]
            .filter(pl.col("metric") == metric)
            .filter(pl.col("congestion") == cc)
        )
        if filtered.height == 0:
            return float("nan"), float("nan")

        median = filtered.median()["value"][0]
        # std includes Bessel's correction by default in polars
        std = filtered.std()["value"][0]
        error = ERROR_T_ALPHA * std / (filtered.height**0.5)

        return median, error

    return TwoCCCurveConfig(
        label=LABEL_DISPLAY_NAME.get(cc, cc),
        color=CC_COLORS.get(cc, "tab:blue"),
        compute=compute_function,
    )


def generate_clients_medians_graph_config(
    cc1: str, cc2: str, other_params: str, metric: str
) -> TwoCCGraphConfig:

    match GRAPH_LANGUAGE:
        case "english":
            title = f"Comparing the clients' {METRIC_DISPLAY_NAME.get(metric, metric)} when using {LABEL_DISPLAY_NAME.get(cc1, cc1)} and {LABEL_DISPLAY_NAME.get(cc2, cc2)} with parameters {other_params}"
            yaxis_label = f"Median of {METRIC_DISPLAY_NAME.get(metric, metric)} accross clients, grouped by congestion algorithm"
        case "french":
            title = f"Comparaison {METRIC_DISPLAY_NAME.get(metric, metric)} des clients utilisant {LABEL_DISPLAY_NAME.get(cc1, cc1)} et {LABEL_DISPLAY_NAME.get(cc2, cc2)} avec les paramètres {other_params}"
            yaxis_label = f"Médiane {METRIC_DISPLAY_NAME.get(metric, metric)} des clients, groupés par algorithme de congestion"

    return TwoCCGraphConfig(
        short_name=metric,
        title=title,
        yaxis_label=yaxis_label,
        cc1=cc1,
        cc2=cc2,
        other_params=other_params,
        required_metrics=[metric],
        curves=[
            generate_clients_medians_curve_config(cc1, metric),
            generate_clients_medians_curve_config(cc2, metric),
        ],
    )


def generate_router_median_curve_config(metric: str) -> TwoCCCurveConfig:
    def compute_function(exp: ExperimentWithResultsAndNbrCCs) -> tuple[float, float]:
        filtered = exp["results"].filter(pl.col("metric") == metric)
        if filtered.height == 0:
            return float("nan"), float("nan")

        median = filtered.median()["value"][0]
        # std includes Bessel's correction by default in polars
        std = filtered.std()["value"][0]
        error = ERROR_T_ALPHA * std / (filtered.height**0.5)

        return median, error

    return TwoCCCurveConfig(
        label=LABEL_DISPLAY_NAME.get(metric, metric),
        color=QUEUE_COLORS.get(metric, "tab:blue"),
        compute=compute_function,
    )
