# Scenarii generator

Generates scenarii for the benchmarking tools. It takes as input a list of bandwidths, a list of congestion control algorithms, and a list of weights (for multi-client scenarios). It outputs a set of scenarii in the specified output folder.

## Usage

```sh
cargo run -- --help
```

## Usage example

```sh
cargo run -- --bandwidths 10,50,100,1000 --client cubic,bbr,prague --weights 1 -n 1 --output-folder scenarii/one-client

cargo run -- --bandwidths 10,50,100,1000 --client cubic,bbr,prague --weights 1 --output-folder scenarii/simple

cargo run -- --bandwidths 10,50,100,1000 --client cubic,bbr --client prague --weights 90-10,80-20,70-30,60-40,50-50,40-60,30-70,20-80,10-90 --output-folder scenarii/multi2

cargo run -- --bandwidths 10,50,100,1000 --client cubic --client bbr --weights 90-10,80-20,70-30,60-40,50-50,40-60,30-70,20-80,10-90 --output-folder scenarii/multi2-cubic-bbr

cargo run -- --bandwidths 10,50,100,1000 --client cubic --client bbr --client prague --weights 72-18-10,64-16-20,54-14-30,48-12-40,40-10-50,32-8-60,24-6-70,16-4-80 --output-folder scenarii/multi3

cargo run -- --bandwidths 10,50,100,1000 --client cubic --client bbr --client prague --weights 54-36-10,48-32-20,42-28-30,36-24-40,30-20-50,24-16-60,18-12-70,12-8-80 --output-folder scenarii/multi3
```
