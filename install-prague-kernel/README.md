# Installing the modified linux kernel

- Install the kernel

```sh
./install_kernel.sh
```

- If there is an issue of dependencies, you can try to fix it with:

```sh
sudo apt --fix-broken install
```

- Get the path of the menu entry. For that, we must find the name of the Linux Prague GRUB menu entry and, if necessary, the name of the sub menu it is into.
  - List the menu entries

    ```sh
    sudo grep menuentry /boot/grub/grub.cfg
    ```

  - Note that `Ubuntu, with Linux [...]-prague-[...]` is in a sub menu: `Advanced options for Ubuntu`. We will need to put that sub menu in the path.

  - The synthax of the path is the following: `[sub_menu_name]>[menu_entry_name]`. Those name are not the one on the left (e.g. `Ubuntu, with Linux [...]-prague-[...]`) but the one on the right (e.g `'gnulinux-6.12.54-0b264c55b-l4steam-117-advanced-f20d3202-7d8b-4659-b4d3-5a2b4fe6260a`)

  - In our case, this gives us the path: `gnulinux-advanced-f20d3202-7d8b-4659-b4d3-5a2b4fe6260a>gnulinux-6.12.54-0b264c55b-l4steam-117-advanced-f20d3202-7d8b-4659-b4d3-5a2b4fe6260a`

- Put this path as default in GRUB. Modify `GRUB_DEFAULT` to the path (e.g. `GRUB_DEFAULT='gnulinux-advanced-f20d3202-7d8b-4659-b4d3-5a2b4fe6260a>gnulinux-6.12.54-0b264c55b-l4steam-117-advanced-f20d3202-7d8b-4659-b4d3-5a2b4fe6260a'`)

```sh
sudo nano /etc/default/grub
```

- Update grub config and reboot

```sh
sudo update-grub
sudo reboot
```

- Check that the loaded kernel is the correct one:

```sh
uname -r
```

- Install dependencies

```sh
./install_dependencies.sh
```

- Activate the modules. ⚠️ We might need to execute thios command at every restart.

```sh
./activate_kernel_modules.sh
```
