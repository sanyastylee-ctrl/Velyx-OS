# Velyx OS Runtime Usage

## Install once

From a fresh Ubuntu checkout:

```bash
cd ~/Velyx-OS
./scripts/velyx-install.sh
```

Install result:

- runtime prefix in `${VELYX_INSTALL_PREFIX:-$HOME/.local/share/velyx}`
- persistent user state in `${VELYX_STATE_DIR:-$HOME/.velyx}`
- helper commands in `~/.local/bin`
- user systemd units in `~/.config/systemd/user`

Useful commands after install:

- `velyx-status`
- `velyx-version`
- `velyx-logs.sh`
- `velyx-restart.sh`
- `velyx-update`
- `velyx-recovery`
- `velyx-app`
- `velyx-space`
- `velyx-intent`

## Daily runtime workflow

Typical development/runtime workflow:

```bash
cd ~/Velyx-OS
git pull
velyx-update
velyx-status
```

Explicit equivalent:

```bash
cd ~/Velyx-OS
git pull
velyx-update --source-root "$PWD"
velyx-status
```

This is the standard dev update path:

1. update repository contents with `git pull`
2. re-stage and validate runtime with `velyx-update`
3. verify with `velyx-status`

## Network update workflow

Remote update without using the local working tree:

```bash
velyx-update --source github
```

Or:

```bash
velyx-update --source https://github.com/sanyastylee-ctrl/Velyx-OS.git
```

Or archive source:

```bash
velyx-update --source https://example.com/Velyx-OS-release.tar.gz
```

Network update:

- fetches the source into a temporary directory
- validates payload
- applies the same staged update flow
- keeps user state intact
- falls back to the local source root if remote fetch is unavailable and a local checkout is present

## Verification after install or update

CLI checks:

```bash
velyx-version
velyx-status
```

Shell-level checks:

- shell starts automatically
- spaces are visible
- intents are visible
- active space is shown
- apps can be launched from current space

Useful runtime commands:

```bash
velyx-space list
velyx-space current
velyx-intent list
velyx-intent current
velyx-app list
```

## Recovery

Manual recovery:

```bash
velyx-recovery
```

Restart user runtime without full recovery:

```bash
velyx-restart.sh
```

If the previous update failed badly:

- `update_state.json` may set `recovery_needed=true`
- session bootstrap attempts auto-recovery

## Logs

High-level helper:

```bash
velyx-logs.sh
```

Important files:

- `~/.velyx/update.log`
- `~/.velyx/update_state.json`
- `~/.velyx/session_manager_audit.log`
- `~/.velyx/launcher_history.log`
- `~/.velyx/sandbox_audit.log`
- `~/.velyx/shell_mvp.log`
- `~/.velyx/app_registry.log`
- `~/.velyx/spaces.log`
- `~/.velyx/intents.log`

User journal:

```bash
journalctl --user -b
```

## Ubuntu test scenario

Clean install:

```bash
cd ~/Velyx-OS
./scripts/velyx-install.sh
velyx-status
```

Update from local repo:

```bash
git pull
velyx-update
velyx-status
```

Explicit equivalent:

```bash
git pull
velyx-update --source-root "$PWD"
velyx-status
```

Check shell/runtime:

- shell starts
- spaces work
- intents work
- active space is visible
- current version is visible in shell and `velyx-version`

Remote update:

```bash
velyx-update --source github
```

## Hardening expectations

Current scripts are intended to be safe for repeated use:

- `velyx-install.sh` can be re-run to refresh install and units
- `velyx-update` uses staged apply and rollback
- `velyx-recovery` restores last known good runtime
- all major flows write logs into `~/.velyx`

## Related docs

- [install-bootstrap.md](./install-bootstrap.md)
- [update-recovery.md](./update-recovery.md)
- [app-model.md](./app-model.md)
- [spaces-system.md](./spaces-system.md)
- [intents-system.md](./intents-system.md)
