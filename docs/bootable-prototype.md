# Velyx OS Bootable Prototype

## Цель

Этот этап переводит Velyx OS из backend-прототипа в первый bootable VM prototype:

- `QEMU`
- `kernel + initrd`
- `rootfs image`
- `systemd` как `PID 1`
- `velyx-boot.target`
- `first boot / session bootstrap`
- `velyx-session.target`
- `velyx-shell`

## Основной boot flow

1. QEMU загружает kernel и rootfs image.
2. `systemd` стартует `multi-user.target`.
3. `multi-user.target` подтягивает `velyx-boot.target`.
4. `velyx-firstboot.service` подготавливает lifecycle markers и state dirs.
5. `velyx-session-bootstrap.service` подготавливает login/session bootstrap.
6. `getty@tty1` выполняет autologin пользователя `velyx`.
7. `systemd --user` поднимает:
   - `velyx-session-manager.service`
   - `velyx-session-bootstrap.service`
8. user bootstrap вызывает `StartUserSession("velyx")`.
9. `session-manager-service` поднимает `velyx-session.target`.
10. target запускает core services и `velyx-shell.service`.

## Почему выбран autologin prototype

Для первого bootable slice это самый простой путь к реальному `systemd --user` lifecycle:

- без полноценного display manager
- без ручного запуска shell
- без обхода session-manager

## Что считается успехом

- kernel и rootfs реально стартуют в QEMU
- `systemd` поднимает system и user units
- `session-manager` оркестрирует сессию
- shell достигается как unit, а не как ручной процесс
