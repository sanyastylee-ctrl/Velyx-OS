# Velyx OS Install + Bootstrap

## Install flow

Velyx OS uses a user-level install model:

- install prefix: `${VELYX_PREFIX:-$HOME/.local/share/velyx}`
- user state: `${VELYX_STATE_DIR:-$HOME/.velyx}`

Primary entrypoint:

```bash
cd ~/Velyx-OS
./scripts/velyx-install.sh
```

The script is designed to be repeatable. Re-running it refreshes binaries, env and user units without changing the persistent user state layout.

## Install prefix layout

`${VELYX_INSTALL_PREFIX}` contains runtime artifacts:

- `bin/`
  - `velyx-session-manager`
  - `velyx-permissions-service`
  - `velyx-launcher-service`
  - `velyx-shell`
  - `velyx-update`
  - `velyx-recovery`
  - `velyx-version`
  - `velyx-app`
  - `velyx-space`
  - `velyx-intent`
- `libexec/`
  - `velyx-user-session-bootstrap`
  - `velyx-firstboot-dispatch`
  - `velyx-system-session-bootstrap`
  - `velyx-recovery-bootstrap`
- `share/app-manifests/`
  - system app manifests
- `share/version.txt`
  - installed runtime version metadata

## User state layout

`~/.velyx/` contains mutable state and logs:

- `update_state.json`
- `update.log`
- `apps_registry.json`
- `spaces_registry.json`
- `intents_registry.json`
- `intent_state.json`
- `app_registry.log`
- `spaces.log`
- `intents.log`
- `session_manager_audit.log`
- `launcher_history.log`
- `sandbox_audit.log`
- `shell_mvp.log`
- `updates/`
- `apps/`

Important: install and update flows do not intentionally erase:

- `~/.velyx/apps`
- registries
- logs
- update/recovery state

## Env model

Install writes:

- `~/.config/velyx/velyx.env`

Key env variables:

- `VELYX_INSTALL_PREFIX`
- `VELYX_STATE_DIR`
- `VELYX_MANIFESTS_DIR`
- `VELYX_SESSION_MANAGER_BINARY`
- `VELYX_PERMISSIONS_BINARY`
- `VELYX_LAUNCHER_BINARY`
- `VELYX_SHELL_BINARY`
- `VELYX_APP_REGISTRY`
- `VELYX_SPACES_REGISTRY`
- `VELYX_INTENTS_REGISTRY`

The user units load this env file and therefore stay relocatable inside the selected install prefix.

## User units

Install renders and places user units into:

- `~/.config/systemd/user/`

Main units:

- `velyx-session-manager.service`
- `velyx-session-bootstrap.service`
- `velyx-launcher.service`
- `velyx-permissions.service`
- `velyx-shell.service`
- optional runtime units from the broader stack

## Startup order

1. `velyx-session-manager.service`
2. `velyx-session-bootstrap.service`
3. `session-manager` prepares orchestration/runtime state
4. `velyx-permissions.service`
5. `velyx-launcher.service`
6. `velyx-shell.service`

Shell is part of the normal user session lifecycle and should not require manual startup in the regular path.

## Custom binary locations

If build artifacts are outside `target/release`:

```bash
VELYX_BIN_DIR=/path/to/target/release \
VELYX_SHELL_BINARY=/path/to/velyx-shell \
./scripts/velyx-install.sh
```

## Post-install commands

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

These helpers are linked into:

- `~/.local/bin/`

## Related docs

- [update-recovery.md](./update-recovery.md)
- [runtime-usage.md](./runtime-usage.md)
- [app-model.md](./app-model.md)
- [spaces-system.md](./spaces-system.md)
- [intents-system.md](./intents-system.md)
