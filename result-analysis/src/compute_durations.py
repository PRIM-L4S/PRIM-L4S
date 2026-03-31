import polars as pl

# Computes the duration of each experiment in seconds and saves it to a new CSV file.

df = pl.read_csv("results.csv")

df = df.with_columns(
    [
        pl.col("Launch time")
        .str.to_datetime(time_zone="UTC", strict=False)
        .alias("Launch time"),
        pl.col("End time")
        .str.to_datetime(time_zone="UTC", strict=False)
        .alias("End time"),
    ]
)

df = df.with_columns(
    (pl.col("End time") - pl.col("Launch time")).dt.total_seconds().alias("Duration")
)

df.write_csv("results.with_durations.csv")
