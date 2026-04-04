# Velyx OS Systemd User Session Runtime

## Цель

`session-manager-service` больше не должен запускать shell напрямую как дочерний процесс.
Он должен:

- устанавливать user units
- выполнять `systemctl --user daemon-reload`
- запускать `velyx-session.target`
- проверять readiness через `systemd --user` и D-Bus

## Основные units

- `velyx-session.target`
- `velyx-settings.service`
- `velyx-permissions.service`
- `velyx-launcher.service`
- `velyx-diagnostics.service`
- `velyx-ai.service`
- `velyx-shell.service`

## Runtime flow

1. `session-manager` убеждается, что доступен `systemd --user`
2. ставит unit files в `~/.config/systemd/user/`
3. выполняет `daemon-reload`
4. вызывает `systemctl --user start velyx-session.target`
5. проверяет required services
6. проверяет `velyx-shell.service`
7. фиксирует `Ready`, `Degraded` или `Failed`

## Принцип

Shell и runtime services принадлежат lifecycle systemd, а не прямому `spawn()` внутри orchestrator.
