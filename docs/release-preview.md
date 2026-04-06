# Velyx OS Preview Release Build

## Overview

`Velyx OS Preview` now has a reproducible VM-oriented release target.

The build path is centered around:

- `./scripts/velyx-release-preview build`
- `./scripts/velyx-release-preview validate`
- `./scripts/velyx-vm-profile validate`

The goal is not a polished consumer ISO yet. The goal is a repeatable preview artifact that can be booted in a VM and exercised through:

- `Velyx Entry`
- `Velyx Installer`
- `Velyx First Boot`
- `Velyx Shell`

## Supported Output

Current supported artifact formats:

- `raw`
- `qcow2`

The release bundle also includes:

- `vmlinuz`
- `initrd.img` when available
- `manifest.json`
- `velyx-vm-profile.json`
- `velyx-release-profile.json`
- source snapshot tarball
- `run-preview.sh`

## Build Requirements

Build on a Linux host or Linux-compatible build environment with:

- `bash`
- `truncate`
- `mkfs.ext4`
- `tar`
- optional `qemu-img`
- optional `qemu-system-x86_64`

Environment:

- `VELYX_BASE_ROOTFS=/path/to/minimal-systemd-rootfs`
- `VELYX_BIN_DIR=/path/to/compiled-velyx-binaries`
- `VELYX_SHELL_BINARY=/path/to/velyx-shell` if shell is outside the binary dir

## Build Commands

Check host readiness:

```bash
./scripts/velyx-release-preview host-check --format both
```

Inspect release metadata:

```bash
./scripts/velyx-release-preview manifest
```

Build preview artifacts:

```bash
./scripts/velyx-release-preview build --format both
```

This creates a release directory under `dist/` with a name like:

```text
dist/velyx-os-preview-<version>-<build_id>/
```

## What The Build Includes

The preview image path now pulls in:

- runtime services
- `Velyx Shell`
- `Velyx Entry`
- `Velyx Installer`
- `Velyx First Boot`
- `Control Center`
- `Assistant`
- model runtime helpers
- `Dev Mode`
- update and recovery helpers

## VM Flow

1. Build the artifact.
2. Create a VM with the requirements from `velyx-vm-profile`.
3. Boot the generated image.
4. Enter `Velyx Entry`.
5. Choose `Install Velyx`.
6. Reboot after install.
7. Complete `Velyx First Boot`.
8. Continue into `Velyx Shell`.

## Validation

Validate the artifact on the host:

```bash
./scripts/velyx-release-preview validate dist/velyx-os-preview-<version>-<build_id>
```

Validate the installed VM:

```bash
velyx-status
velyx-version
velyx-vm-profile validate
velyx-logs.sh
journalctl --user -b
```

Functional checks inside the VM:

- shell starts automatically
- assistant is available
- model runtime is visible
- Control Center is available
- update path works
- Dev Mode can be enabled
- visual refine flow works
- rollback works

## Notes

This is still a developer preview release path.

It is not yet:

- a polished consumer installer ISO
- a full distro ecosystem
- a package-managed release channel

But it is now a concrete, reproducible VM-oriented `Velyx OS Preview` build path with a named artifact and validation flow.
