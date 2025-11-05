# Iperf3 tests on docker

## Requierments

- Install `docker`

- Install the L4S kernel patch (see [../README.md](../README.md))

- Create a venv and install jinja2:

```sh
make install
```

## Usage

- Create templated files and start docker containers

```sh
make up
```

- Observe the logs:

```sh
make logs
```

- Stop docker containers

```sh
make down
```

## Access prometheus web UI


- Open SSH tunnel with forwarding:

```sh
ssh -L 9000:localhost:9000 l4s
```

- Open http://localhost:9000 in your browser.

## Creating bufferbloat

I set the target bitrate in `entrypoint_client.sh` to 100 Mb/s.

To create some bufferbloat at the `router`, I limited the throughput of the router to `100 kbits/s`.

## Inspect with wireshark

- The subnet for the iperf client is `172.20.1.0/24`, the one of the iperf server is `172.20.2.0/24`. Find the associated interfaces with:

```sh
ip a | grep -B 2 "172.20"
```

E.g.: `br-f5139c9911d7` and `br-f2bc663406af`

- Record packets

```sh
sudo tcpdump -i any -s 65535 -w dump.pcap
```

Or use `-i <interface>` to record only a specific interface.

- Stop the recording with `Ctrl+C`. ⚠️ The file size can become very big very quick!!

- Open the `.pcap` in Wireshark. You can use the filter `ip.addr == 1.1.1.1 or (ip.addr >= 172.20.0.0 and ip.addr <= 172.20.255.255 and ip.dsfield.ecn == 1)` to only keep the relevent traffic. `ip.dsfield.ecn` refers to Explicit Congestion Notification. A value of `1` means that we use L4S, and `3` (`11`) means that we are experiencing a congestion.

- If we change rate limiting throughout the experient, we make a ping to `1.1.1.1` when we add a rate limit to the experience, so that it can be seen on wireshark.

## Traceroute from the client

```
traceroute to 172.20.2.10 (172.20.2.10), 30 hops max, 60 byte packets
 1  router.iperf3_iperf-client-net (172.20.1.11)  0.293 ms  0.044 ms  0.042 ms
 2  172.20.2.10 (172.20.2.10)  0.311 ms  0.177 ms  0.184 ms
```

