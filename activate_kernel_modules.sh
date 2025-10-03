# sudo modprobe sch_dualpi2
sudo modprobe tcp_prague

# Enable ECN
sudo sysctl -w net.ipv4.tcp_ecn=1

# List available congestion control algorithms
# sysctl net.ipv4.tcp_available_congestion_control

# Set default congestion control algorithm to Prague
sudo sysctl -w net.ipv4.tcp_congestion_control=prague
