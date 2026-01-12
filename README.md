# PRIM - L4S

> Authors: Emre Ucar, Thomas Sauvage, Timothée Fisher

This repository contains the methods we developed to observe the behaviour of various congestion control algorithms. Our work focused on the behaviour of L4S and TCP Prague in different scenarios, such as in networks with Reno clients.

These productions were made as part of our project PRIM at Télécom Paris.

## Folder structure

| Folder                   | Comments                                                                                                             |
| ------------------------ | -------------------------------------------------------------------------------------------------------------------- |
| `benchmarking-tools/`    | The various tools we develop or use to gather metrics in our benchmarks                                              |
| `docker-testbed/`        | A containerized testbed used to observe the behavior of different congestion control algorithms in various scenarios |
| `install-prague-kernel/` | Some comments and advice on how to install the kernel modules necessary to run TCP Prague                            |
