# Iperf3 tests on docker

## Inspect with wireshark

- Find the subnet of the docker network

```sh
docker network inspect iperf3_iperf-net | grep Subnet
```

- Find the associated interface

```sh
ip a
```

E.g.: `br-3f00e7b74404`

- Record packets

```sh
sudo tcpdump -i <interface> -s 65535 -w dump.pcap
```

- Stop the recording with `Ctrl+C`. ⚠️ The file size can become very big very quick!!
