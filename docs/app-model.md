# Velyx OS App Model / Registry

## App entity

Приложение теперь хранится в registry как отдельная сущность со следующими полями:

- `app_id`
- `display_name`
- `version`
- `executable_path`
- `sandbox_profile`
- `requested_permissions`
- `category`
- `trust_level`
- `install_source`
- `install_time`
- `status`
- `manifest_path`
- `payload_root`

`install_source`:

- `system`
- `user`

`status`:

- `installed`
- `removed`
- `broken`

## Registry layout

Registry хранится в:

- `~/.velyx/apps_registry.json`

User apps payload хранится в:

- `~/.velyx/apps/<app_id>/`

System manifests по-прежнему живут в install prefix:

- `${VELYX_INSTALL_PREFIX}/share/app-manifests/`

При первом запуске или после install/update выполняется seed/sync system apps в registry.

## Launcher integration

`launcher-service` теперь читает apps из registry.

Поведение:

- `ListApps()` и `GetAppInfo()` работают от registry
- `Launch()` использует registry entry как source of truth
- `removed` apps не считаются launchable
- `broken` apps видны в launcher info/list и блокируются на launch

## CLI

Доступны команды:

```bash
velyx-app list
velyx-app info <app_id>
velyx-app install <path>
velyx-app remove <app_id>
velyx-app remove --force-system <app_id>
velyx-app update <app_id> <path>
velyx-app sync-system
```

## Install / update behavior

### Install

`velyx-app install <path>`

- читает manifest из файла или каталога
- валидирует manifest/security shape
- копирует payload в `~/.velyx/apps/<app_id>/`
- пишет normalized manifest
- добавляет app в registry
- если executable пока невалиден, app ставится со `status=broken`

### Remove

`velyx-app remove <app_id>`

- для user app удаляет payload
- через launcher пытается остановить running app
- помечает registry entry как `removed`

System apps:

- требуют `--force-system`

### Update

`velyx-app update <app_id> <path>`

- делает staged replace payload внутри `~/.velyx/apps/<app_id>`
- обновляет version и metadata в registry
- если app был running, пытается вызвать launcher restart

## Logging

App registry audit:

- `~/.velyx/app_registry.log`

События:

- `app_install_begin/ok/failed`
- `app_remove_begin/ok/failed`
- `app_update_begin/ok/failed`
- `app_registry_sync`
