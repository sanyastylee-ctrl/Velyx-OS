# Velyx OS Intent System

`Intent` is a higher-level action layer above `Spaces`.

Intent entity:

- `intent_id`
- `display_name`
- `description`
- `target_space`
- `actions`
- `auto_focus`
- `ensure_apps`
- `restart_policy`
- `status`
- `created_at`
- `updated_at`
- `source`

## Storage

Registry:

- `~/.velyx/intents_registry.json`

Runtime state:

- `~/.velyx/intent_state.json`

Audit:

- `~/.velyx/intents.log`

## Default intents

- `dev_start`
- `safe_browse`
- `general_use`
- `recovery_mode`

## Execution flow

`velyx-intent run <intent_id>`:

1. Validates the registry entry.
2. Validates `target_space`.
3. Switches space through `SessionManager.ActivateSpace`.
4. Falls back to `velyx-space activate` if needed.
5. Records result into `intent_state.json`.
6. Logs execution to `intents.log`.

## MVP behavior

- Intent does not own runtime orchestration.
- Runtime ownership stays with:
  - `session-manager`
  - `launcher-service`
- Intent is a scenario entrypoint on top of the existing space/runtime graph.
