# Velyx First Boot

After installation, Velyx Shell is the primary session surface, but the first entry now opens `Velyx First Boot` as an onboarding overlay instead of dropping the user into a raw runtime dashboard.

## State

First boot state lives in:

- `~/.velyx/first_boot_state.json`
- `~/.velyx/first_boot.log`

## Flow

The first boot experience covers:

1. Welcome to Velyx
2. System ready check
3. AI and model setup
4. Network visibility
5. Default space selection
6. Predictive feature toggle
7. Enter Velyx

## Commands

Helper CLI:

```bash
velyx-firstboot status
velyx-firstboot rerun-checks
velyx-firstboot set-ai-mode suggest
velyx-firstboot set-model-selection auto_task
velyx-firstboot set-default-space development
velyx-firstboot set-predictive suggest
velyx-firstboot complete
```

## Repair path

First boot also exposes:

- repair / recovery entry
- diagnostics export
- model readiness
- update / recovery state

This keeps recovery inside the Velyx UX instead of pushing the user into a substrate-first workflow.
