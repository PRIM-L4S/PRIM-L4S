
# Tell linux that iperf-server is reachable through the router
ROUTER_IP="172.20.1.11"
IPERF_SERVER_IP="172.20.2.10"

ip route add $IPERF_SERVER_IP via $ROUTER_IP

echo "Client ready"

# Iperf3 client running indefinitely, sending 1 second bursts every 5 seconds
# towards the iperf-server container
# --connect-timeout is in ms
while true; do iperf3 -c $IPERF_SERVER_IP --time 1 --bitrate 10M --connect-timeout 3000; sleep 5; done