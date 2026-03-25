from datetime import datetime
from typing import TypedDict

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
