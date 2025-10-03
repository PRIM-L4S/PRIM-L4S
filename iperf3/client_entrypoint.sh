# Tell linux that iperf-server is reachable through the router
# TODO

# Iperf3 client running indefinitely, sending 1 second bursts every 5 seconds
# towards the iperf-server container
while true; do iperf3 -c iperf-server --time 1; sleep 5; done