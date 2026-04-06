# Velyx System Entry

`Velyx Entry` is the main front door for `Velyx OS Preview`.

It gives the user one clear starting point instead of separate runtime commands:

- `Install Velyx`
- `Try Velyx (Live / Preview)`
- `Repair / Diagnostics`

## Main entry

Command:

```bash
velyx-entry
```

This opens the primary preview flow and routes into one of three paths:

1. `Velyx Installer`
2. `Velyx Live`
3. `Velyx Repair / Diagnostics`

## Install path

`velyx-entry -> velyx-installer -> velyx-firstboot -> Velyx Shell`

The installer is responsible for:
- system check
- network readiness
- model recommendation
- install confirmation
- preparing first-boot state

The installer remains backed by the existing install pipeline, but the user-facing story is now `Velyx Installer`, not a manual bash walkthrough.

## Live path

`velyx-entry -> velyx-live -> Velyx Shell`

`velyx-live`:
- does not install Velyx
- creates a temporary live session state
- prepares first-boot style handoff for preview mode
- launches `Velyx Shell` directly when available

Live session state is stored under:
- `~/.velyx/live_sessions/`

This path is meant for:
- trying Velyx without install
- VM preview checks
- diagnostics of the shell/session flow

## Repair and diagnostics

`velyx-entry` can also route into:
- repair
- diagnostics

These paths reuse:
- `velyx-installer --action repair`
- `velyx-installer --action diagnostics`

## Product identity

User-facing naming should stay consistent:

- `Velyx OS`
- `Velyx Installer`
- `Velyx First Boot`
- `Velyx Shell`
- `Velyx Recovery`
- `Velyx Update`

`velyx-version` exposes:
- `product`
- `channel`
- `version`
- `build_id`

## Intended session flow

The target user journey is:

`Velyx Entry -> Install or Live -> Velyx First Boot -> Velyx Shell`

The goal is to keep the user inside the `Velyx` experience instead of dropping them into a generic desktop path.
