FROM debian
RUN apt-get update && apt-get install -y tcpdump
CMD ["tcpdump", "-i", "eth0"]
