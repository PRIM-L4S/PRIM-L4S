# Required to improve L4S. See entrypoint_client.sh for comments on those lines.
NETIF="eth0"
ethtool -K $NETIF tso off gso off gro off lro off
tc qdisc replace dev $NETIF root handle 1: fq limit 20480 flow_limit 10240

echo "Server ready"

iperf3 -s