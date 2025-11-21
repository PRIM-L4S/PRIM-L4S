# setup

## create qcow2 image with ssh

- From [debian official cloud images](https://cdimage.debian.org/images/cloud/trixie/20251117-2299/) download `debian-13-nocloud-amd64-20251117-2299.qcow2`
- Launch the VM : `qemu-system-x86_64 -m 1G -smp 2 -accel hvf -accel tcg -hda debian-13-nocloud-amd64-20251117-2299.qcow2 -boot d`
- Login as user `root` (no password)
- Follow the instructions below to setup the image

## common image

```shell
apt update
apt install -y openssh-server
mkdir -p /root/.ssh
echo "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIM6xRI5rlrvv73Lb2OuUp9U+hZEPWJAhIq4F43eDcJ+0 root@l4sdemo" >> /root/.ssh/authorized_keys
echo "PermitRootLogin yes" >> /etc/ssh/sshd_config
service ssh reload
```

## nodes setup and launch

Duplicate the common image to create the following images:
- router.qcow2
- clientA.qcow2
- clientB.qcow2
- serverA.qcow2
- serverB.qcow2

Launch the demo with:
```shell
docker compose up
```

## on each node

```shell
echo router|clientA|clientB|serverA|serverB > /etc/hostname
```

## config specific to router

```shell
echo "net.ipv4.ip_forward=1" >> /etc/sysctl.conf
```

## ip routing config

```shell
docker exec -it l4sdemo-clientA ip r add 192.168.2.0/24 via 192.168.1.2
docker exec -it l4sdemo-clientB ip r add 192.168.2.0/24 via 192.168.1.2
```
