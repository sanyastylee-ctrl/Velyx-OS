# Velyx Dev Mode

`Velyx Dev Mode` is a hidden developer feature for fast shell UI iteration.

It is disabled by default and only affects the shell UI overlay layer. It does not edit install, update, recovery, systemd, or core runtime files.

## What it does

- enables a live overlay at `~/.velyx/dev_overlay/`
- lets the assistant preview and apply limited shell UI changes
- restarts `Velyx Shell` without reinstalling the runtime
- keeps rollback history for the last live edits

## Commands

```bash
velyx-dev enable
velyx-dev disable
velyx-dev status
velyx-dev list-ui-files
velyx-dev search-ui-component button
velyx-dev read-ui-file apps/shell/qml/AssistantPanel.qml
velyx-dev snapshot-ui-state
velyx-dev preview-ui-diff "Make the buttons smaller"
velyx-dev patch-ui-file "Make the buttons smaller"
velyx-dev restart-shell-dev
velyx-dev rollback
```

## Editable scope

Allowed live-edit scope:

- `apps/shell/qml/`
- `apps/shell/src/` for read/search only
- `packages/design-system/qml/Velyx/DesignSystem/Theme.qml` as an advanced token target

Blocked scope:

- install/update/recovery scripts
- systemd units
- service backends
- runtime ownership state

## Overlay model

When Dev Mode is enabled:

1. Velyx seeds `~/.velyx/dev_overlay/apps/shell/qml/`
2. `velyx-shell` prefers `~/.velyx/dev_overlay/apps/shell/qml/Main.qml`
3. live changes are written only into the overlay
4. the installed base remains untouched

This makes experimentation safer and keeps rollback simple.

## Assistant workflow

In Dev Mode the assistant can handle requests like:

- `Make the buttons smaller`
- `Move the panel right`
- `Add button "Debug view" next to Create Note`

Current flow:

1. assistant detects a UI-edit request
2. creates a snapshot
3. previews the UI diff
4. asks for approval before applying the patch
5. applies the overlay change
6. restarts the shell through the dev-safe path

## Rollback

Every live patch stores a backup in:

- `~/.velyx/dev_overlay_history/`

Use:

```bash
velyx-dev rollback
```

Rollback restores the previous overlay files and requests a shell restart.

## Notes

- Dev Mode is for shell UI iteration, not general repository editing.
- If shell restart is unavailable in the current environment, the helper reports a degraded result and tells you to restart manually.
- Live apply is currently focused on QML surface changes. C++ shell code remains outside the live-reload guarantee.
