# Velyx Auto Boot

`Velyx OS` now starts into `Velyx Shell` automatically after user session startup without requiring a manual terminal command.

## Startup order

The user session path is:

1. `velyx-permissions.service`
2. `velyx-launcher.service`
3. `velyx-session-manager.service`
4. `velyx-shell.service`

`velyx-shell.service` does not launch the shell binary directly. It runs:

```bash
velyx-entry --session
```

That entrypoint checks `~/.velyx/first_boot.json`:

- `completed=false` -> the shell opens with `Velyx First Boot`
- `completed=true` -> normal `Velyx Shell`

This keeps first boot and normal shell startup inside one primary session path.

## Enabled units

The install pipeline enables:

- `velyx-session.target`
- `velyx-settings.service`
- `velyx-permissions.service`
- `velyx-launcher.service`
- `velyx-diagnostics.service`
- `velyx-ai.service`
- `velyx-file.service`
- `velyx-update-engine.service`
- `velyx-recovery.service`
- `velyx-session-manager.service`
- `velyx-session-bootstrap.service`
- `velyx-shell.service`

`velyx-firstboot.service` exists as an optional entry unit for diagnostics or future session variants, but the main boot path is owned by `velyx-shell.service -> velyx-entry --session`.

## Shell watchdog

`velyx-shell.service` now has restart protection and a lightweight watchdog:

- `Restart=always`
- `StartLimitIntervalSec=120`
- `StartLimitBurst=5`
- `ExecStartPre=/usr/bin/velyx-shell-watchdog prestart`
- `ExecStopPost=/usr/bin/velyx-shell-watchdog poststop`

Watchdog files:

- `~/.velyx/shell_watchdog.log`
- `~/.velyx/shell_state.json`

If the shell fails repeatedly within a short window, the watchdog marks:

- `recovery_suggested=true` in `shell_state.json`
- `recovery_needed=true` in `update_state.json`

This creates a visible diagnostics / recovery signal instead of silently looping forever.

## Manual checks

```bash
velyx-status
velyx-logs.sh
journalctl --user -u velyx-shell.service -b
systemctl --user status velyx-shell.service
systemctl --user show velyx-shell.service -p NRestarts -p Result -p SubState
```

## VM validation

Expected flow after install:

1. install Velyx
2. complete first boot
3. reboot the VM
4. Velyx Shell starts automatically
5. first boot no longer appears

Crash validation:

1. terminate the shell process
2. confirm `velyx-shell.service` restarts it
3. inspect `~/.velyx/shell_state.json`
4. inspect `journalctl --user -u velyx-shell.service -b`
