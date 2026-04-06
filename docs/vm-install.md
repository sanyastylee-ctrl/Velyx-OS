# Velyx OS VM Install

`Velyx OS Preview` can now be prepared as a reproducible VM-first environment with a single product-facing flow:

1. create VM
2. prepare base environment
3. run `velyx-entry`
4. choose `Install Velyx`
5. finish install
6. reboot
7. complete `Velyx First Boot`
8. continue into `Velyx Shell`

## 1. Create the VM

Recommended preview baseline:

- 8 GB RAM
- 4 vCPUs
- 24 GB disk
- basic accelerated graphics
- network enabled

Minimum usable baseline:

- 6 GB RAM
- 2 vCPUs
- 20 GB disk

## 2. Prepare the Base Environment

Required dependencies:

- `systemd`
- `dbus-user-session`
- `python3`
- `qt6`
- `xorg`
- `xprop`
- `wmctrl`
- `xdotool`
- `curl` or `wget`
- `git`

VM profile reference:

```bash
velyx-vm-profile profile
```

## 3. Start the Velyx Entry Flow

Use the Velyx entry point:

```bash
velyx-entry
```

Then choose:

- `Install Velyx`

This keeps the user in the Velyx-native path:

- `Velyx Entry`
- `Velyx Installer`
- `Velyx First Boot`
- `Velyx Shell`

## 4. Installer Path

Inside `Velyx Installer`:

1. review system check
2. review network check
3. confirm install mode
4. choose AI mode
5. choose model selection mode
6. choose backend
7. confirm install

If install fails:

- use `Repair / Diagnostics` from `velyx-entry`
- or run `velyx-diagnostics`
- or continue with `velyx-recovery`

## 5. First Boot

After reboot, `Velyx First Boot` should appear before normal shell use.

Complete:

1. welcome
2. system readiness
3. model setup
4. space selection
5. predictive / automation preferences
6. enter Velyx

## 6. Validation Checklist

After install, validate:

- shell starts
- assistant works
- model layer is available
- update works
- control center is visible
- dev mode can be enabled

Quick validation:

```bash
velyx-vm-profile validate
```

Operational diagnostics:

```bash
velyx-status
velyx-logs.sh
journalctl --user -b
```

## 7. Dev Mode Test Inside the Installed VM

Enable Dev Mode:

```bash
velyx-dev enable
```

Then inside `Velyx Shell`, ask:

`Make the buttons smaller`

Verify:

- patch is proposed
- approval path works
- shell restarts or reloads
- visual result is visible
- rollback works

Rollback:

```bash
velyx-dev rollback
```

## 8. Fail Cases

### Install fail

- return to `velyx-entry`
- choose `Repair / Diagnostics`

### Network fail

- run `velyx-network check`
- continue in local-only mode if needed

### Model fail

- choose `stub`
- or use `auto_hardware`
- then re-run `velyx-model detect-hardware`

### Shell fail

- use `velyx-diagnostics`
- use `velyx-recovery`
- then re-enter `Velyx Shell`

## 9. Verification Goal

A clean VM should feel like `Velyx OS Preview`, not like a host desktop with a shell layered on top.
