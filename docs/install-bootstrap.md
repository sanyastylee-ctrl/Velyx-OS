# Velyx OS Install + Bootstrap

## Install layout

Минимальная воспроизводимая установка теперь строится вокруг двух слоёв:

- install prefix: `${VELYX_PREFIX:-$HOME/.local/share/velyx}`
- user state: `${VELYX_STATE_DIR:-$HOME/.velyx}`

### Prefix layout

- `bin/`
- `libexec/`
- `share/app-manifests/`

### User layout

- `~/.velyx/` — runtime state и file-based audit logs
- `~/.config/velyx/velyx.env` — install env
- `~/.config/systemd/user/` — user units
- `~/.local/bin/` — helper commands

## Install command

```bash
cd ~/Velyx-OS
./scripts/velyx-install.sh
```

Если бинарники собраны в нестандартный каталог:

```bash
VELYX_BIN_DIR=/path/to/target/release \
VELYX_SHELL_BINARY=/path/to/velyx-shell \
./scripts/velyx-install.sh
```

## Startup order

1. `velyx-session-manager.service`
2. `velyx-session-bootstrap.service`
3. `session-manager` пишет runtime units и стартует `velyx-session.target`
4. `velyx-permissions.service`
5. `velyx-launcher.service`
6. `velyx-shell.service`

Shell стартует не вручную, а как часть user session lifecycle.

## Health / recovery

После установки доступны команды:

- `velyx-status`
- `velyx-restart.sh`
- `velyx-logs.sh`
- `velyx-update`
- `velyx-recovery`
- `velyx-app`

Они ставятся в `~/.local/bin/`.

## Update / recovery

После установки доступен staged update path:

```bash
./scripts/velyx-update --source-root "$PWD"
```

Manual recovery:

```bash
velyx-recovery
```

Подробности:

- [update-recovery.md](./update-recovery.md)
- [app-model.md](./app-model.md)
