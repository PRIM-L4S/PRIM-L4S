# Serial Experiments Runner

Use this tool to run several benchmarks at once and store data in the corresponding `results.csv` file.

### Usage

```sh
cargo run scenario1 scenario2
```

Runs `scenario1.json` and `scenario2.json`. The files need to be in `docker-testbed/scenarii`. Scenarii run 120s by default, this may be changed in future updates.

You can then check the results out on VictoriaMetrics using the timestamps in `results.csv`.

## Usage example

```sh
cargo run \
  10bbr@1000mbit+10ms+1ms \
  10bbr@100mbit+10ms+1ms \
  10bbr@10mbit+10ms+1ms \
  10bbr@50mbit+10ms+1ms \
  10cubic@1000mbit+10ms+1ms \
  10cubic@100mbit+10ms+1ms \
  10cubic@10mbit+10ms+1ms \
  10cubic@50mbit+10ms+1ms \
  10prague@1000mbit+10ms+1ms \
  10prague@100mbit+10ms+1ms \
  10prague@10mbit+10ms+1ms \
  10prague@50mbit+10ms+1ms \
  1bbr+1cubic+7prague@1000mbit+10ms+1ms \
  1bbr+1cubic+7prague@100mbit+10ms+1ms \
  1bbr+1cubic+7prague@10mbit+10ms+1ms \
  1bbr+1cubic+7prague@50mbit+10ms+1ms \
  1bbr+1prague+7cubic@1000mbit+10ms+1ms \
  1bbr+1prague+7cubic@100mbit+10ms+1ms \
  1bbr+1prague+7cubic@10mbit+10ms+1ms \
  1bbr+1prague+7cubic@50mbit+10ms+1ms \
  1bbr+2cubic+6prague@1000mbit+10ms+1ms \
  1bbr+2cubic+6prague@100mbit+10ms+1ms \
  1bbr+2cubic+6prague@10mbit+10ms+1ms \
  1bbr+2cubic+6prague@50mbit+10ms+1ms \
  1bbr+2prague+6cubic@1000mbit+10ms+1ms \
  1bbr+2prague+6cubic@100mbit+10ms+1ms \
  1bbr+2prague+6cubic@10mbit+10ms+1ms \
  1bbr+2prague+6cubic@50mbit+10ms+1ms \
  1bbr+3prague+5cubic@1000mbit+10ms+1ms \
  1bbr+3prague+5cubic@100mbit+10ms+1ms \
  1bbr+3prague+5cubic@10mbit+10ms+1ms \
  1bbr+3prague+5cubic@50mbit+10ms+1ms \
  1bbr+4cubic+4prague@1000mbit+10ms+1ms \
  1bbr+4cubic+4prague@100mbit+10ms+1ms \
  1bbr+4cubic+4prague@10mbit+10ms+1ms \
  1bbr+4cubic+4prague@50mbit+10ms+1ms \
  1bbr+4cubic+5prague@1000mbit+10ms+1ms \
  1bbr+4cubic+5prague@100mbit+10ms+1ms \
  1bbr+4cubic+5prague@10mbit+10ms+1ms \
  1bbr+4cubic+5prague@50mbit+10ms+1ms \
  1bbr+9prague@1000mbit+10ms+1ms \
  1bbr+9prague@100mbit+10ms+1ms \
  1bbr+9prague@10mbit+10ms+1ms \
  1bbr+9prague@50mbit+10ms+1ms \
  1cubic+8prague@1000mbit+10ms+1ms \
  1cubic+8prague@100mbit+10ms+1ms \
  1cubic+8prague@10mbit+10ms+1ms \
  1cubic+8prague@50mbit+10ms+1ms \
  1cubic+9prague@1000mbit+10ms+1ms \
  1cubic+9prague@100mbit+10ms+1ms \
  1cubic+9prague@10mbit+10ms+1ms \
  1cubic+9prague@50mbit+10ms+1ms \
  1prague+3bbr+5cubic@1000mbit+10ms+1ms \
  1prague+3bbr+5cubic@100mbit+10ms+1ms \
  1prague+3bbr+5cubic@10mbit+10ms+1ms \
  1prague+3bbr+5cubic@50mbit+10ms+1ms \
  1prague+9bbr@1000mbit+10ms+1ms \
  1prague+9bbr@100mbit+10ms+1ms \
  1prague+9bbr@10mbit+10ms+1ms \
  1prague+9bbr@50mbit+10ms+1ms \
  1prague+9cubic@1000mbit+10ms+1ms \
  1prague+9cubic@100mbit+10ms+1ms \
  1prague+9cubic@10mbit+10ms+1ms \
  1prague+9cubic@50mbit+10ms+1ms \
  2bbr+3cubic+4prague@1000mbit+10ms+1ms \
  2bbr+3cubic+4prague@100mbit+10ms+1ms \
  2bbr+3cubic+4prague@10mbit+10ms+1ms \
  2bbr+3cubic+4prague@50mbit+10ms+1ms \
  2bbr+3cubic+5prague@1000mbit+10ms+1ms \
  2bbr+3cubic+5prague@100mbit+10ms+1ms \
  2bbr+3cubic+5prague@10mbit+10ms+1ms \
  2bbr+3cubic+5prague@50mbit+10ms+1ms \
  2bbr+3prague+4cubic@1000mbit+10ms+1ms \
  2bbr+3prague+4cubic@100mbit+10ms+1ms \
  2bbr+3prague+4cubic@10mbit+10ms+1ms \
  2bbr+3prague+4cubic@50mbit+10ms+1ms \
  2bbr+8cubic@1000mbit+10ms+1ms \
  2bbr+8cubic@100mbit+10ms+1ms \
  2bbr+8cubic@10mbit+10ms+1ms \
  2bbr+8cubic@50mbit+10ms+1ms \
  2bbr+8prague@1000mbit+10ms+1ms \
  2bbr+8prague@100mbit+10ms+1ms \
  2bbr+8prague@10mbit+10ms+1ms \
  2bbr+8prague@50mbit+10ms+1ms \
  2cubic+7prague@1000mbit+10ms+1ms \
  2cubic+7prague@100mbit+10ms+1ms \
  2cubic+7prague@10mbit+10ms+1ms \
  2cubic+7prague@50mbit+10ms+1ms \
  2cubic+8prague@1000mbit+10ms+1ms \
  2cubic+8prague@100mbit+10ms+1ms \
  2cubic+8prague@10mbit+10ms+1ms \
  2cubic+8prague@50mbit+10ms+1ms \
  2prague+3bbr+4cubic@1000mbit+10ms+1ms \
  2prague+3bbr+4cubic@100mbit+10ms+1ms \
  2prague+3bbr+4cubic@10mbit+10ms+1ms \
  2prague+3bbr+4cubic@50mbit+10ms+1ms \
  2prague+8bbr@1000mbit+10ms+1ms \
  2prague+8bbr@100mbit+10ms+1ms \
  2prague+8bbr@10mbit+10ms+1ms \
  2prague+8bbr@50mbit+10ms+1ms \
  2prague+8cubic@1000mbit+10ms+1ms \
  2prague+8cubic@100mbit+10ms+1ms \
  2prague+8cubic@10mbit+10ms+1ms \
  2prague+8cubic@50mbit+10ms+1ms \
  3bbr+7prague@1000mbit+10ms+1ms \
  3bbr+7prague@100mbit+10ms+1ms \
  3bbr+7prague@10mbit+10ms+1ms \
  3bbr+7prague@50mbit+10ms+1ms \
  3cubic+6prague@1000mbit+10ms+1ms \
  3cubic+6prague@100mbit+10ms+1ms \
  3cubic+6prague@10mbit+10ms+1ms \
  3cubic+6prague@50mbit+10ms+1ms \
  3cubic+7prague@1000mbit+10ms+1ms \
  3cubic+7prague@100mbit+10ms+1ms \
  3cubic+7prague@10mbit+10ms+1ms \
  3cubic+7prague@50mbit+10ms+1ms \
  3prague+7bbr@1000mbit+10ms+1ms \
  3prague+7bbr@100mbit+10ms+1ms \
  3prague+7bbr@10mbit+10ms+1ms \
  3prague+7bbr@50mbit+10ms+1ms \
  3prague+7cubic@1000mbit+10ms+1ms \
  3prague+7cubic@100mbit+10ms+1ms \
  3prague+7cubic@10mbit+10ms+1ms \
  3prague+7cubic@50mbit+10ms+1ms \
  4bbr+6cubic@1000mbit+10ms+1ms \
  4bbr+6cubic@100mbit+10ms+1ms \
  4bbr+6cubic@10mbit+10ms+1ms \
  4bbr+6cubic@50mbit+10ms+1ms \
  4bbr+6prague@1000mbit+10ms+1ms \
  4bbr+6prague@100mbit+10ms+1ms \
  4bbr+6prague@10mbit+10ms+1ms \
  4bbr+6prague@50mbit+10ms+1ms \
  4cubic+6prague@1000mbit+10ms+1ms \
  4cubic+6prague@100mbit+10ms+1ms \
  4cubic+6prague@10mbit+10ms+1ms \
  4cubic+6prague@50mbit+10ms+1ms \
  4prague+6bbr@1000mbit+10ms+1ms \
  4prague+6bbr@100mbit+10ms+1ms \
  4prague+6bbr@10mbit+10ms+1ms \
  4prague+6bbr@50mbit+10ms+1ms \
  4prague+6cubic@1000mbit+10ms+1ms \
  4prague+6cubic@100mbit+10ms+1ms \
  4prague+6cubic@10mbit+10ms+1ms \
  4prague+6cubic@50mbit+10ms+1ms \
  5bbr+5prague@1000mbit+10ms+1ms \
  5bbr+5prague@100mbit+10ms+1ms \
  5bbr+5prague@10mbit+10ms+1ms \
  5bbr+5prague@50mbit+10ms+1ms \
  5cubic+5prague@1000mbit+10ms+1ms \
  5cubic+5prague@100mbit+10ms+1ms \
  5cubic+5prague@10mbit+10ms+1ms \
  5cubic+5prague@50mbit+10ms+1ms
```
