# Result analysis

## Installation

- Install `uv` package manager:

```sh
brew install uv
```

- Install Black code formatter for VS Code:

```sh
code --install-extension ms-python.black-formatter
```

## Usage

- Copy and open data:

```sh
scp l4s2:git/PRIM-L4S/benchmarking-tools/serial_experiments_runner/results.csv .
ssh -L 8428:localhost:8428 l4s2
```

- Run

```sh
uv run main.py
```
