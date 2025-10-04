# Iperf3 tests on docker

I set the target bitrate in `entrypoint_client.sh` to 10 Mb/s.

To create some bufferbloat at the `router`, it might be intresting to limit the throughput to something slower on it.

## Inspect with wireshark

- The subnet for the iperf client is `172.20.1.0/24`, the one of the iperf server is `172.20.2.0/24`. Find the associated interfaces with:

```sh
ip a | grep -B 2 "172.20"
```

E.g.: `br-f5139c9911d7` and `br-f2bc663406af`

- Record packets

```sh
sudo tcpdump -i <interface> -s 65535 -w dump.pcap
```

Or use `-i any` to record on all interfaces.

- Stop the recording with `Ctrl+C`. ⚠️ The file size can become very big very quick!!
