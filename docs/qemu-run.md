# QEMU Run Guide

## Предпосылки

Нужны:

- Linux host или Linux build environment
- `qemu-system-x86_64`
- `mkfs.ext4`
- базовый Linux rootfs с `systemd`
- собранные Velyx binaries

## Быстрый путь

```bash
export VELYX_BASE_ROOTFS=/path/to/base-rootfs
export VELYX_BIN_DIR=/path/to/velyx-binaries
export VELYX_SHELL_BINARY=/path/to/velyx-shell
export VELYX_KERNEL=/boot/vmlinuz-linux
export VELYX_INITRD=/boot/initramfs-linux.img

./scripts/build-image.sh
./scripts/run-qemu.sh ./build/image/velyx-rootfs.img
```

## Headless запуск

```bash
VELYX_QEMU_MODE=headless ./scripts/run-qemu.sh ./build/image/velyx-rootfs.img
```

## Что проверять внутри VM

- `systemctl status velyx-boot.target`
- `systemctl status velyx-firstboot.service`
- `systemctl --user status velyx-session-manager.service`
- `systemctl --user status velyx-session.target`
- `journalctl -b`
- `journalctl --user -b`
- `/var/log/velyx/first_boot.log`
- `/home/velyx/.velyx/session_manager_audit.log`
