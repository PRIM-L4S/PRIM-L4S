import json
import requests
import time

from datetime import datetime, timezone
from typing import TypedDict

import pandas as pd

VICTORIA_METRICS_URL = "http://localhost:8428"

METRICS = [
    "ss_snd_cwnd",
    "ss_snd_ssthresh",
    "ss_bytes_sent",
#     "ss_bytes_retrans",
#     "ss_bytes_acked",
#     "ss_delivery_rate",
#     "ss_rtt",
#     "ss_rttvar",
#    # "ss_number_of_samples",
#     "iperf_bytes",
#     "iperf_bits_per_second",
#     "iperf_retransmits",
#     "iperf_snd_cwnd",
#     "iperf_snd_wnd",
#     "iperf_rtt",
#     "iperf_rttvar",
#     "iperf_pmtu",
#     "hfe_number_of_benchmarks",
]

# JSON format type
class Labels(TypedDict):
    __name__: str
    host: str
    congestion: str

class MetricData(TypedDict):
    metric: Labels
    timestamps: list[int]
    values: list[str]


def download_metrics(start: datetime, end: datetime) -> pd.DataFrame:
    """
    Download raw metrics defined in METRICS from VictoriaMetrics between start and end.
    Returns a pandas DataFrame containing all metrics data.
    """
    start_time = time.time()
    records = []

    for metric in METRICS:
        print(f"Downloading metric: {metric}... ", end="", flush=True)
        resp = requests.get(
            f"{VICTORIA_METRICS_URL}/api/v1/export",
            params={
                "match[]": f'{{__name__="{metric}"}}',
                "start": start.astimezone(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
                "end":   end.astimezone(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
            },
            stream=True,
        )
        print("Downloaded. ", end="", flush=True)
        resp.raise_for_status()

        for line in resp.iter_lines():
            if not line:
                continue
            metric_data: MetricData = json.loads(line)
            labels = metric_data["metric"]
            
            timestamps = metric_data.get("timestamps", [])
            values = metric_data.get("values", [])
            
            if not timestamps:
                continue

            series_df = pd.DataFrame({
                "timestamp": timestamps,
                "value": pd.to_numeric(values, downcast='float')
            })
            
            # Broadcast scalar values
            series_df["host"] = labels["host"]
            series_df["congestion"] = labels["congestion"]
            series_df["metric"] = labels["__name__"]
            
            records.append(series_df)

        print("Imported.", flush=True)

    if records:
        df = pd.concat(records, ignore_index=True)
        df["timestamp"] = pd.to_datetime(df["timestamp"], unit="ms")
    else:
        raise ValueError("No metrics data found for the specified time range.")

    end_time = time.time()
    print(f"Finished downloading and processing metrics in {end_time - start_time:.2f} seconds.")
        
    return df
