# Serial Experiments Runner

Use this tool to run several benchmarks at once and store data in the corresponding ```results.csv``` file.

### How to run

```cargo run scenario1 scenario2```

Runs ```scenario1.json``` and ```scenario2.json```. The files need to be in ```docker-testbed/scenarii```. Scenarii run 120s by default, this may be changed in future updates.

You can then check the results out on VictoriaMetrics using the timestamps in ```results.csv```.

