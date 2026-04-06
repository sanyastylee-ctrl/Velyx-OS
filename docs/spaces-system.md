# Velyx OS Spaces System

`Spaces` are registry-backed work contexts for the user session. A space defines:

- `space_id`
- `display_name`
- `description`
- `apps`
- `autostart_apps`
- `required_apps`
- `preferred_active_app`
- `security_mode`
- `permissions_profile`
- `focus_policy`
- `ui_layout`
- `status`
- `source`

## Registry

Primary registry:

- `~/.velyx/spaces_registry.json`

Runtime and audit log:

- `~/.velyx/spaces.log`

The registry stores:

- `active_space_id`
- `spaces[]`

If the registry is missing or broken, Velyx seeds default system spaces.

## Default Spaces

Default seeded spaces:

- `general`
- `development`
- `safe-web`
- `recovery`

## Activation Policy

`velyx-space activate <space_id>` or shell UI activation does:

1. Validate the target space exists.
2. Persist `active_space_id`.
3. Ask `session-manager` to activate the space.
4. `session-manager` orchestration loop tries to launch `autostart_apps`.
5. Required apps determine `space runtime state`:
   - `ready`
   - `degraded`
   - `failed`

Current MVP policy for apps outside the active space:

- apps are **not** stopped automatically
- shell marks them as `outside-space`

## CLI

- `velyx-space list`
- `velyx-space info <space_id>`
- `velyx-space current`
- `velyx-space activate <space_id>`

## Session Restore

Velyx restores the last valid `active_space_id` from the registry.
If it is missing or invalid, fallback is `general`.
