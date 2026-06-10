# PRIM - L4S

> Authors: Emre Ucar, Thomas Sauvage, Timothée Fisher

This repository provides a framework for evaluating the behavior of congestion control algorithms (TCP Prague, Cubic, BBR, Reno) when interacting with a dualpi2 AQM router.

Developed as part of the PRIM project at Télécom Paris.

## Folder structure

| Folder                   | Comments                                                                                                             |
| ------------------------ | -------------------------------------------------------------------------------------------------------------------- |
| `benchmarking-tools/`    | The various tools we develop or use to gather metrics in our benchmarks                                              |
| `docker-testbed/`        | A containerized testbed used to observe the behavior of different congestion control algorithms in various scenarios |
| `install-prague-kernel/` | Some comments and advice on how to install the kernel modules necessary to run TCP Prague                            |
| `result-analysis/`       | The python program we developed to analyze the results of our benchmarks and produce the figures in our report       |
