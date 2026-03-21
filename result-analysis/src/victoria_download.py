import json
import requests
import time

from datetime import datetime, timezone
from typing import TypedDict

import pandas as pd

VICTORIA_METRICS_URL = "http://localhost:8428"

ALL_METRICS = [
    "ss_snd_cwnd",
    "ss_snd_ssthresh",
    "ss_bytes_sent",
    "ss_bytes_retrans",
    "ss_bytes_acked",
    "ss_delivery_rate",
    "ss_rtt",
    "ss_rttvar",
    # "ss_number_of_samples",
    "iperf_bytes",
    "iperf_bits_per_second",
    "iperf_retransmits",
    "iperf_snd_cwnd",
    "iperf_snd_wnd",
    "iperf_rtt",
    "iperf_rttvar",
    "iperf_pmtu",
    "hfe_number_of_benchmarks",
]


# JSON format type
class Labels(TypedDict):
    __name__: str
    host: str
    congestion: str


class MetricData(TypedDict):
    metric: Labels
    timestamps: list[int]
    values: list[int]


def download_metrics(
    start: datetime, end: datetime, metrics: list[str] = ALL_METRICS
) -> pd.DataFrame:
    """
    Download raw metrics from VictoriaMetrics between start and end.
    Returns a pandas DataFrame containing all metrics data.
    """
    start_time = time.time()

    # Send a single request for all metrics
    params = [
        ("start", start.astimezone(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")),
        ("end", end.astimezone(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")),
    ]
    for metric in metrics:
        params.append(("match[]", f'{{__name__="{metric}"}}'))

    print("Downloading metrics... ", end="", flush=True)
    resp = requests.get(
        f"{VICTORIA_METRICS_URL}/api/v1/export",
        params=params,
        stream=True,
    )
    resp.raise_for_status()
    print("Downloaded.\nProcessing data... ", end="", flush=True)

    records = []

    for line in resp.iter_lines():
        if not line:
            continue

        metric_data: MetricData = json.loads(line)
        timestamps = metric_data.get("timestamps", [])
        values = metric_data.get("values", [])

        if not timestamps:
            continue

        labels = metric_data.get("metric", {})

        chunk_df = pd.DataFrame({"timestamp": timestamps, "value": values})

        chunk_df["host"] = labels.get("host", "")
        chunk_df["congestion"] = labels.get("congestion", "")
        chunk_df["metric"] = labels.get("__name__", "")

        records.append(chunk_df)

    print("Processed.", flush=True)

    if not records:
        raise ValueError("No metrics data found for the specified time range.")

    df = pd.concat(records, ignore_index=True)
    df["timestamp"] = pd.to_datetime(df["timestamp"], unit="ms")

    end_time = time.time()
    print(
        f"Finished downloading and processing metrics in {end_time - start_time:.2f} seconds."
    )

    return df
