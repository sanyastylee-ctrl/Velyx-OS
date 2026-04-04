# State Layout Velyx OS

## Принцип

Состояние Velyx OS делится на четыре контура:

- `system state`
- `service state`
- `user state`
- `recovery/update state`

Это разделение является обязательным security и maintainability требованием.

## Предлагаемый layout

### System

- `/etc/velyx/`
  policy defaults, system-wide feature flags, startup manifests
- `/usr/share/velyx/`
  immutable system assets, bundled manifests, first boot resources

### Service State

- `/var/lib/velyx/services/permissions/`
- `/var/lib/velyx/services/settings/`
- `/var/lib/velyx/services/launcher/`
- `/var/lib/velyx/services/session/`
- `/var/lib/velyx/services/installer/`

### Logs

- `/var/log/velyx/system.log`
- `/var/log/velyx/session.log`
- `/var/log/velyx/security.log`
- `/var/log/velyx/installer.log`

### User

- `/home/<user>/.config/velyx/`
- `/home/<user>/.local/state/velyx/`
- `/home/<user>/.cache/velyx/`

## Прототипный dev layout

Пока production paths не реализованы, прототипные сервисы продолжают использовать:

- `/home/<user>/.velyx/`

Но все новые сервисы должны проектироваться так, чтобы позже перейти к системному layout без смены API-контрактов.
