import json
import requests
import time

from datetime import datetime, timezone
from typing import TypedDict

import polars as pl

VICTORIA_METRICS_URL = "http://localhost:8428"


class Labels(TypedDict):
    __name__: str
    host: str
    congestion: str


class MetricData(TypedDict):
    metric: Labels
    timestamps: list[int]
    values: list[int]


def download_metrics(
    start_time: datetime,
    end_time: datetime,
    metrics: list[str],
) -> pl.DataFrame:
    """
    Download raw metrics from VictoriaMetrics between start_time and end_time.

    Returns a DataFrame containing all metrics data.
    """
    # Send a single request for all metrics
    params = [
        ("start", start_time.astimezone(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")),
        ("end", end_time.astimezone(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")),
    ]
    for metric in metrics:
        params.append(("match[]", f'{{__name__="{metric}"}}'))

    resp = requests.get(
        f"{VICTORIA_METRICS_URL}/api/v1/export",
        params=params,
    )
    resp.raise_for_status()
    raw_content = resp.content

    records = []

    for line in raw_content.split(b"\n"):
        if not line:
            continue

        metric_data: MetricData = json.loads(line)
        timestamps = metric_data.get("timestamps", [])
        values = metric_data.get("values", [])

        if not timestamps:
            continue

        labels = metric_data.get("metric", {})

        chunk_df = pl.DataFrame(
            {"timestamp": timestamps, "value": values}
        ).with_columns(
            pl.lit(labels.get("host", "")).alias("host"),
            pl.lit(labels.get("congestion", "")).alias("congestion"),
            pl.lit(labels.get("__name__", "")).alias("metric"),
        )

        records.append(chunk_df)

    if not records:
        raise ValueError(
            f"No metrics data found for metrics {metrics} between {start_time.isoformat()} and {end_time.isoformat()}."
        )

    df = pl.concat(records)
    df = df.with_columns(pl.from_epoch("timestamp", time_unit="ms"))

    return df
