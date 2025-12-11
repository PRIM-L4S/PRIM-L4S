sudo modprobe sch_dualpi2
sudo modprobe tcp_prague
sudo modprobe tcp_bbr

echo "sch_dualpi2" | sudo tee /etc/modules-load.d/dualpi2.conf
echo "tcp_prague" | sudo tee /etc/modules-load.d/prague.conf
echo "tcp_bbr" | sudo tee /etc/modules-load.d/bbr.conf

# TODO?
# some preparations for having better paced traffic and reduce bursts for each network interface $NETIF that sends L4S traffic
# Avoid processing 64K packets in the kernel, which will send those packets in a burst independent of the pacing (lro only for newer NICS and kernels that support it):
# sudo ethtool -K $NETIF tso off gso off gro off lro off

# fq qdisc needs to be configured on clients and server NICS (instead of fq_codel; fq is the only one that supports the pacing)
# sudo tc qdisc replace dev $NETIF root handle 1: fq limit 20480 flow_limit 10240

# Enable ECN
sudo sysctl -w net.ipv4.tcp_ecn=1

# List available congestion control algorithms
# sysctl net.ipv4.tcp_available_congestion_control

# Set default congestion control algorithm to Prague
sudo sysctl -w net.ipv4.tcp_congestion_control=prague

# Enable IP forwarding, used for our Containerized router experiments
# We don't really need to do it because it is 1 by default
sudo sysctl -w net.ipv4.ip_forward=1
