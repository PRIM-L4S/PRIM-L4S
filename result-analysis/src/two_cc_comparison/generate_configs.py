import polars as pl

from src.data_types import TwoCCCurveConfig, TwoCCGraphConfig

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
    return TwoCCCurveConfig(
        label=cc,
        color=CC_COLORS.get(cc, "tab:blue"),
        compute=lambda exp: exp["results"]
        .filter(pl.col("metric") == metric)
        .filter(pl.col("congestion") == cc)
        .median()["value"][0],
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
    return TwoCCCurveConfig(
        label=metric,
        color=QUEUE_COLORS.get(metric, "tab:blue"),
        compute=lambda exp: exp["results"]
        .filter(pl.col("metric") == metric)
        .median()["value"][0],
    )
