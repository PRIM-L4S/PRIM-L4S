# Tell linux that iperf-server is reachable through the router
ROUTER_IP="172.20.1.11"
IPERF_SERVER_SUBNET="172.20.2.0/24"
IPERF_SERVER_IP="172.20.2.10"

ip route add $IPERF_SERVER_SUBNET via $ROUTER_IP

# Required to improve L4S.
NETIF="eth0"
# some preparations for having better paced traffic and reduce bursts for each network interface $NETIF that sends L4S traffic
# Avoid processing 64K packets in the kernel, which will send those packets in a burst independent of the pacing (lro only for newer NICS and kernels that support it):
ethtool -K $NETIF tso off gso off gro off lro off
# fq qdisc needs to be configured on clients and server NICS (instead of fq_codel; fq is the only one that supports the pacing)
tc qdisc replace dev $NETIF root handle 1: fq limit 20480 flow_limit 10240

echo "Client ready"

# Iperf3 client running indefinitely, sending 1 second bursts every 5 seconds
# towards the iperf-server container
# --connect-timeout is in ms
# list available TCP congestion control algorithms:
# sysctl net.ipv4.tcp_available_congestion_control
#  --linux-congestion cubic doesn't seem to work :(
while true; do iperf3 -c $IPERF_SERVER_IP --time 1 --bitrate 100M --connect-timeout 3000; sleep 5; done