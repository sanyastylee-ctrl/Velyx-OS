# Velyx Control Center

`Velyx Control Center` is the unified state and controls panel for `Velyx OS`.

It is available directly inside `Velyx Shell` and brings together the operational controls that were previously spread across system panels, AI settings, developer toggles, and recovery utilities.

## What It Shows

The Control Center is organized into five sections:

### System

- `version`
- `uptime`
- `install state`
- `session state`
- `update state`
- `last update result`

### AI

- `AI mode`
- `current model`
- `backend`
- `model selection mode`

Available actions:

- switch AI mode
- switch model selection mode
- re-detect hardware
- set current model manually

### Network

- `online / offline`
- `update reachability`

Available actions:

- check update source
- prepare a local or remote update source

### Dev

- `dev mode status`
- `dev mode toggle`
- `auto refine toggle`
- `rollback last change`

This section integrates with `velyx-dev` and only exposes developer controls when they are relevant to the active preview/runtime state.

### Recovery

- `recovery state`
- `diagnostics availability`

Available actions:

- export diagnostics
- enter recovery
- apply update

## Shell Integration

The Control Center lives in the right-side operational rail of `Velyx Shell`.

It is designed to work as a persistent system panel rather than a separate debug window. The right rail is scrollable so the Control Center can coexist with:

- `Velyx Assistant`
- `Dev Mode`
- `AI Suggestions`
- `Details`
- `Automation`
- `Diagnostics`

## Connected Backends

The panel is a UI layer over existing Velyx commands and state:

- `velyx-version`
- `velyx-update`
- `velyx-model`
- `velyx-dev`
- `velyx-recovery`
- runtime state surfaced by `PermissionClient`

It does not replace those tools. It gives the user a system-native operational surface for them.

## Verification

From a running Velyx preview:

1. Open `Velyx Shell`.
2. Open the right-side `Control Center`.
3. Verify `System` shows version, uptime, install state, and update state.
4. In `AI`, switch between `manual`, `auto_hardware`, and `auto_task`.
5. In `AI`, enter a model id and press `Use model`.
6. In `Dev`, enable `Dev Mode`.
7. Toggle `Auto refine`.
8. Trigger `Rollback`.
9. In `Recovery`, run `Export diagnostics`.
10. Press `Apply update` and verify `Velyx Update` is launched.

## Notes

- The Control Center is part of `Velyx OS`, not a host-desktop settings panel.
- It is intended to be the main operational entry point for system state, model control, developer toggles, and recovery actions.
