# Velyx OS Update + Recovery

## Update model

Для минимального reproducible update path выбран локальный deploy snapshot из Git checkout:

- source root: checkout с актуальными `scripts/`, `systemd/user/`, `app-manifests/`
- binaries: `target/release/`
- shell binary: указывается отдельно или берётся из `target/release/velyx-shell`

Установка хранит update/recovery state в:

- `~/.velyx/update_state.json`
- `~/.velyx/update.log`

Основные поля state:

- `current_version`
- `staged_version`
- `last_good_version`
- `update_state`
- `last_update_result`
- `rollback_available`
- `recovery_needed`
- `last_failed_version`
- `last_recovery_result`

Version identity на этом этапе:

- `git rev-parse --short HEAD`, если update source — Git checkout
- иначе `VELYX_BUILD_VERSION` / `VELYX_UPDATE_VERSION`

Текущая установленная версия также пишется в:

- `${VELYX_INSTALL_PREFIX}/share/version.txt`

## Staged apply

`velyx-update` работает так:

1. Готовит payload в staging prefix.
2. Валидирует payload:
   - required binaries
   - shell binary
   - `libexec` bootstrap helpers
   - `share/version.txt`
   - наличие `systemd/user` templates
3. Останавливает session target.
4. Сохраняет current prefix как last-known-good payload.
5. Переключает staging prefix в live prefix.
6. Обновляет user units через `scripts/velyx-install.sh --units-only`.
7. Перезапускает runtime.
8. Делает health check.
9. Если всё хорошо:
   - update committed
   - current version становится last known good
10. Если health check падает:
   - выполняется rollback
   - либо система помечается `recovery_needed=true`

## Recovery

`velyx-recovery`:

- восстанавливает last-known-good prefix
- восстанавливает сохранённые user units
- перезапускает session runtime
- обновляет `update_state.json`

`velyx-session-bootstrap.service` теперь запускает preflight:

- `velyx-recovery-bootstrap`

Он смотрит `~/.velyx/update_state.json`, и если `recovery_needed=true`, пытается выполнить auto-recovery до старта обычного session bootstrap.

## User commands

Install:

```bash
./scripts/velyx-install.sh
```

Apply update from current checkout:

```bash
./scripts/velyx-update --source-root "$PWD"
```

Если бинарники лежат в нестандартном каталоге:

```bash
./scripts/velyx-update \
  --source-root "$PWD" \
  --bin-dir /path/to/target/release \
  --shell-binary /path/to/velyx-shell
```

Manual recovery:

```bash
velyx-recovery
```

Status:

```bash
velyx-status
```

Logs:

```bash
velyx-logs.sh
```

## Verification

### Successful update

```bash
cd ~/Velyx-OS
git pull
./scripts/velyx-update --source-root "$PWD"
velyx-status
tail -n 80 ~/.velyx/update.log
cat ~/.velyx/update_state.json
```

### Invalid payload

Например, передать пустой/битый `--bin-dir`:

```bash
./scripts/velyx-update --source-root "$PWD" --bin-dir /tmp/missing
```

Ожидается:

- update blocked before apply
- `current_version` unchanged
- `last_update_result=payload_invalid`

### Recovery after broken runtime

Если новый runtime не проходит health check:

- update marked failed
- rollback starts automatically
- если rollback невозможен:
  - `recovery_needed=true`
  - следующий session bootstrap попробует auto-recovery

Manual path:

```bash
velyx-recovery
```
