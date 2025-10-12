NETIF0="eth0"
NETIF1="eth1"

# Required to improve L4S. See entrypoint_client.sh for comments on those lines.
ethtool -K $NETIF0 tso off gso off gro off lro off
tc qdisc replace dev $NETIF0 root handle 1: fq limit 20480 flow_limit 10240

ethtool -K $NETIF1 tso off gso off gro off lro off
tc qdisc replace dev $NETIF1 root handle 2: fq limit 20480 flow_limit 10240

iptables -t nat -A POSTROUTING -o $NETIF1 -j MASQUERADE
iptables -A FORWARD -i $NETIF1 -o $NETIF0 -m state --state RELATED,ESTABLISHED -j ACCEPT
iptables -A FORWARD -i $NETIF0 -o $NETIF1 -j ACCEPT

echo "Router ready"

sleep 10

# Limit bandwidth on both interfaces
tc qdisc replace dev $NETIF0 root handle 1: fq limit 20480 flow_limit 10240 maxrate 100kbit
tc qdisc replace dev $NETIF1 root handle 2: fq limit 20480 flow_limit 10240 maxrate 100kbit

echo "Router slows down to 100 Kb/s"

tail -f /dev/null