# Velyx VM Preview

Velyx OS Preview now includes an explicit VM deployment profile:

- [D:\Velyx OS\profiles\velyx-vm-preview.json](D:\Velyx%20OS\profiles\velyx-vm-preview.json)
- [D:\Velyx OS\profiles\velyx-vm-profile.json](D:\Velyx%20OS\profiles\velyx-vm-profile.json)

## Purpose

This profile defines the reproducible preview target for running Velyx as a VM-first environment where:

- Velyx Installer is the install entry
- Velyx First Boot runs before normal use
- Velyx Shell is the primary session
- Velyx Update and Velyx Recovery remain available

## Helper command

```bash
velyx-vm-profile profile
velyx-vm-profile validate
velyx-vm-preview profile
velyx-vm-preview check
```

## Expected VM shape

Recommended preview baseline:

- 8 GB RAM
- 4 vCPUs
- 24 GB disk
- basic accelerated graphics

Required user-space capabilities:

- `systemd`
- `dbus-user-session`
- `dbus-x11` or a working systemd user bus
- X11 or framebuffer shell runtime
- Mesa OpenGL / EGL runtime for GPU-capable X11 sessions
- Python 3
- Qt 6 runtime

## Base Rootfs Build

To prepare a VM-capable base rootfs for the preview image:

```bash
./scripts/build-base-rootfs.sh
export VELYX_BASE_ROOTFS=/path/to/base-rootfs
```

The base rootfs must include:

- `sudo` with correct root ownership and setuid
- `dbus-user-session`
- shell runtime libraries such as `libX11.so.6`
- Mesa runtime libraries such as `libgl1`, `libegl1`, `libgles2`, `libgbm1`, `libdrm2`
- `mesa-utils` for in-VM diagnostics
- Qt runtime files and plugins
- fonts required by the shell

## Graphics Modes

Velyx Shell now supports three graphics modes through `~/.config/velyx/velyx.env`:

```bash
VELYX_GRAPHICS_MODE=auto
```

Supported values:

- `auto`: prefer X11 + OpenGL, then fall back to X11 software, then to `linuxfb`
- `gpu`: force the preferred X11 + OpenGL path and log the failure reason if it cannot start
- `software`: force the safe software-rendered path

Useful debug flags:

```bash
VELYX_GRAPHICS_MODE=gpu
VELYX_SHELL_DEBUG=1
```

The runtime logs now report:

- requested graphics mode
- chosen graphics mode
- fallback reason, when fallback happens
- Qt scene graph backend
- OpenGL vendor / renderer / version when OpenGL is active

## First Interactive Boot Checks

Inside the VM, validate:

```bash
velyx-vm-profile validate
velyx-status
busctl --user --list
systemctl --user status velyx-shell.service
ldd /usr/bin/velyx-shell
glxinfo -B
stat /usr/bin/sudo /etc/sudo.conf
```

Expected result:

- login works
- `sudo` works
- user D-Bus session exists
- `velyx-shell` has no missing shared libraries
- `velyx-shell.service` becomes active
- Velyx Shell becomes the primary interactive session
- the journal contains `graphics backend api= ...`
- the journal contains `graphics opengl vendor= ... renderer= ...`

For quick graphics diagnostics:

```bash
journalctl -b | grep -E 'graphics backend api=|graphics opengl vendor=|shell-session-launch backend=x11|graphics_fallback'
```

## Verification goal

The VM path should feel like `Velyx OS Preview`, not like “Ubuntu with an app on top”.

For the full installable VM scenario, use:

- [D:\Velyx OS\docs\vm-install.md](D:\Velyx%20OS\docs\vm-install.md)
