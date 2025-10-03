# Iperf3 tests on docker

## Inspect with wireshark

- Find the subnet of the docker network

```sh
docker network inspect iperf3_iperf-net | grep Subnet
```

E.g: `172.18.0.0/16`

- Find the associated interface

```sh
ip a | grep -B 2 "172.18"
```

E.g.: `br-ce12e4ae0a72`

- Record packets

```sh
sudo tcpdump -i <interface> -s 65535 -w dump.pcap
```

- Stop the recording with `Ctrl+C`. ⚠️ The file size can become very big very quick!!
