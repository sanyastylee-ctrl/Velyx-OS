# Velyx OS Preview Install Flow

Velyx OS Preview is now presented as a Velyx-native install experience rather than a manual runtime bootstrap.

## Primary user path

Use:

```bash
velyx-installer
```

The installer is a preview-stage TUI wizard that wraps the existing install backend:

- `Install Velyx`
- `Repair Existing Install`
- `Try / Diagnostics`

## Installer steps

The installer flow covers:

1. Welcome
2. System check
3. Install mode
4. AI mode
5. Model selection mode
6. Backend setup
7. Summary
8. Install or repair

## What the installer configures

Behind the scenes it prepares:

- `~/.velyx/install_handoff.json`
- `~/.velyx/first_boot_state.json`
- AI mode
- model routing mode
- backend preference
- default space
- predictive mode

It then calls the install backend:

- `scripts/velyx-install.sh`

or repair backend:

- `velyx-recovery`

## Preview install target

The current preview install still uses the existing Velyx runtime layout:

- install prefix:
  `${VELYX_INSTALL_PREFIX:-$HOME/.local/share/velyx}`
- user state:
  `${VELYX_STATE_DIR:-$HOME/.velyx}`

This is still a preview-stage installer, not a full consumer ISO workflow, but the user-facing path is now Velyx-native.

## Explicit bare-metal target disk install

For destructive SSD install, the installer now supports an explicit target disk:

```bash
sudo env \
  VELYX_BASE_ROOTFS=/path/to/base-rootfs \
  VELYX_BIN_DIR=/path/to/target/release \
  VELYX_SHELL_BINARY=/path/to/velyx-shell \
  bash scripts/velyx-install.sh \
    --target-disk /dev/sda \
    --artifact-dir /path/to/dist/velyx-os-preview-<build> \
    --yes-wipe
```

Properties of this path:

- `TARGET_DISK` is mandatory
- partitioning happens only on that disk
- EFI is created only on that disk
- `grub-install` uses only that disk and that EFI mount
- `--no-nvram` is used to avoid modifying foreign EFI boot entries
- installer fails fast if `/boot/efi` is not mounted from the target disk
