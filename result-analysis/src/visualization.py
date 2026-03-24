import matplotlib.pyplot as plt
import polars as pl

from .experiments_download import ExperimentWithResults


def plot_experiment(experiment: ExperimentWithResults, metric: str):
    """
    Plots the results of an experiment.
    """
    df_metric = experiment["results"].filter(pl.col("metric") == metric)

    plt.figure(figsize=(10, 6))
    for df_host in df_metric.partition_by("host"):
        if not df_host.is_empty():
            host_name = df_host["host"][0]
            plt.scatter(df_host["timestamp"], df_host["value"], label=host_name, s=1)

    plt.title(
        f'Client\'s {metric} through time, in scenario "{experiment["scenario"]}"'
    )
    plt.xlabel("Time")
    plt.ylabel(metric)
    plt.legend()
    plt.grid(True)
    plt.tight_layout()


def print_experiment_results(experiments: list[ExperimentWithResults]):
    """
    Prints the experiments list for debugging purposes.
    """
    for experiment in experiments:
        print(f"{experiment['scenario']}")
        print(f"    • Description: {experiment['description']}")
        print(f"    • Start Time: {experiment['start_time']}")
        print(f"    • End Time: {experiment['end_time']}")
        print(f"    • Results: {experiment['results'].head()}\n   ...")
        print()
