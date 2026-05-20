from datetime import datetime
from typing import Callable, TypedDict

import polars as pl


class Experiment(TypedDict):
    scenario: str
    start_time: datetime
    end_time: datetime
    description: str


class ExperimentWithResults(Experiment):
    results: pl.DataFrame


class ExperimentWithNbrCCs(Experiment):
    number_of_cc1: int
    number_of_cc2: int


class ExperimentWithResultsAndNbrCCs(ExperimentWithResults, ExperimentWithNbrCCs):
    pass


class TwoCCCurveConfig(TypedDict):
    """
    Each curve in a graph is defined by a TwoCCCurveConfig, which specifies how to compute the value of the curve for an experiment.
    """

    label: str
    color: str

    compute: Callable[[ExperimentWithResultsAndNbrCCs], tuple[float, float]]
    """
    A function that takes an experiment and returns the value of the curve for this experiment, as well as the error.
    """


class TwoCCGraphConfig(TypedDict):
    """
    Each generated graph is defined by a TwoCCGraphConfig.
    """

    short_name: str
    title: str
    yaxis_label: str

    cc1: str
    cc2: str
    other_params: str

    required_metrics: list[str]
    """ 
    The metrics that are required to compute the value of the curve for an experiment.
    This is used to download only the required metrics.
    """

    curves: list[TwoCCCurveConfig]
