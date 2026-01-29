# Socket statistics

A library to retrieve socket statistics on Linux systems.

## Testing

```sh
cargo test
```

## Benchmarking

```sh
cargo run --release --bin bench_get_socket_infos
cargo run --release --bin bench_update_fd
```
