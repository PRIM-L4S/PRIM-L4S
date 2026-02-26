sudo apt install unzip

wget https://github.com/L4STeam/linux/releases/download/l4steam-6.12.y-build/l4s-l4steam-6.12.y.zip -O l4s-patch.zip

unzip l4s-patch.zip

sudo dpkg --install debian_build/*
sudo update-grub  # This should auto-detect the new kernel
