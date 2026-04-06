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
- X11 utilities
- Python 3
- Qt 6 runtime

## Verification goal

The VM path should feel like `Velyx OS Preview`, not like “Ubuntu with an app on top”.

For the full installable VM scenario, use:

- [D:\Velyx OS\docs\vm-install.md](D:\Velyx%20OS\docs\vm-install.md)
