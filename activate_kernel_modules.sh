# sudo modprobe sch_dualpi2
sudo modprobe tcp_prague

# Enable ECN
sudo sysctl -w net.ipv4.tcp_ecn=1

# List available congestion control algorithms
# sysctl net.ipv4.tcp_available_congestion_control

# Set default congestion control algorithm to Prague
sudo sysctl -w net.ipv4.tcp_congestion_control=prague

# Enable IP forwarding, used for our Containerized router experiments
# We don't really need to do it because it is 1 by default
sudo sysctl -w net.ipv4.ip_forward=1