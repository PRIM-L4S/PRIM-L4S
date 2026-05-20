import polars as pl

from src.data_types import (
    ExperimentWithResultsAndNbrCCs,
    TwoCCCurveConfig,
    TwoCCGraphConfig,
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
        label=cc,
        color=CC_COLORS.get(cc, "tab:blue"),
        compute=compute_function,
    )


def generate_clients_medians_graph_config(
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
        label=metric,
        color=QUEUE_COLORS.get(metric, "tab:blue"),
        compute=compute_function,
    )
