# Installing the modified linux kernel

- Install the kernel

```sh
./install_kernel.sh
```

- Get the path of the menu entry. For that, we must find the name of the Linux Prague GRUB menu entry and, if necessary, the name of the sub menu it is into.

  - List the menu entries

    ```sh
    sudo grep menuentry /boot/grub/grub.cfg
    ```

  - Note that `Ubuntu, with Linux [...]-prague-[...]` is in a sub menu: `Advanced options for Ubuntu`. We will need to put that sub menu in the path.

  - The synthax of the path is the following: `[sub_menu_name]>[menu_entry_name]`. Those name are not the one on the left (e.g. `Ubuntu, with Linux [...]-prague-[...]`) but the one on the right (e.g `'gnulinux-5.15.72-48b3db6b4-prague-111-advanced-b852d8d2-8460-44aa-8998-23e4f04d73cf`)

  - In our case, this gives us the path: `gnulinux-advanced-b852d8d2-8460-44aa-8998-23e4f04d73cf>gnulinux-5.15.72-48b3db6b4-prague-111-advanced-b852d8d2-8460-44aa-8998-23e4f04d73cf`

- Put this path as default in GRUB. Modify `GRUB_DEFAULT` to the path (e.g. `GRUB_DEFAULT='gnulinux-advanced-b852d8d2-8460-44aa-8998-23e4f04d73cf>gnulinux-5.15.72-48b3db6b4-prague-111-advanced-b852d8d2-8460-44aa-8998-23e4f04d73cf'`)

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
