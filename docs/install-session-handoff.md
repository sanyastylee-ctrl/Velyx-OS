# Install to Session Handoff Velyx OS

## Контур

`installer-service` завершает install lifecycle и оставляет систему не просто в состоянии `installed`, а в состоянии явного handoff:

- `install_handoff.json`
- `first_boot_state.json`

Дальше `session-manager-service` обнаруживает pending handoff и выполняет first boot pipeline до старта пользовательской сессии.

## Handoff state

`install_handoff.json`

- `install_id`
- `target_id`
- `profile_id`
- `encryption_enabled`
- `requested_username`
- `requested_locale`
- `first_boot_pending`
- `baseline_settings_pending`
- `session_start_pending`
- `created_at`

## Success path

1. `PrepareInstall`
2. `CommitInstall`
3. installer пишет handoff state
4. first boot marker = `Pending`
5. `session-manager` запускает first boot pipeline
6. baseline config применяется через `settings-service`
7. выполняется `StartUserSession`
8. shell стартует
9. first boot marker -> `Completed`
10. handoff pending flags сбрасываются

## Failure path

Если baseline config или session handoff падает:

- `first_boot_state = Failed`
- причина фиксируется в persistent state
- audit остается источником диагностики
