# Velyx OS Update + Recovery

## Update model

Velyx OS uses a staged user-level runtime update model.

Supported sources:

- local working tree via `--source-root`
- remote Git repository via `--source <git-url>`
- remote archive via `--source <https://...tar.gz>` or `--source <https://...zip>`
- convenience GitHub shortcut:
  - `velyx-update --source github`

Persisted state:

- `~/.velyx/update_state.json`
- `~/.velyx/update.log`

Installed version metadata:

- `${VELYX_INSTALL_PREFIX}/share/version.txt`

Main state fields:

- `current_version`
- `staged_version`
- `last_good_version`
- `update_state`
- `last_update_result`
- `rollback_available`
- `recovery_needed`
- `last_failed_version`
- `last_recovery_result`

Version identity:

- local Git source: `git rev-parse --short HEAD`
- remote Git source: clone HEAD short hash
- archive source: `VELYX_UPDATE_VERSION`, `VELYX_BUILD_VERSION`, or fallback timestamp snapshot

## What update does not touch

The runtime update path is intended to preserve user-level mutable state:

- `~/.velyx/apps`
- app/space/intent registries
- logs in `~/.velyx/*.log`
- update/recovery state files

The update primarily swaps the install prefix and refreshes user units.

## Staged apply lifecycle

`velyx-update` performs:

1. Resolve source.
   - local path or fetched remote source
2. Prepare staging payload.
   - new runtime is installed into `~/.velyx/updates/staged/<version>`
3. Validate payload.
   - required binaries
   - helper scripts
   - bootstrap helpers
   - `share/version.txt`
   - `systemd/user` templates
4. Backup current runtime.
   - current install prefix becomes last-known-good payload
   - current user units/env are backed up
5. Apply update.
   - staging prefix becomes live prefix
6. Refresh units.
   - `scripts/velyx-install.sh --units-only`
7. Restart runtime.
8. Run health check.
   - `launcher`, `permissions`, `session-manager`, `shell`
   - D-Bus names where available
9. Commit or rollback.
   - success => new version becomes current and last known good
   - failure => rollback is attempted

## Network update support

Examples:

Update from current checkout after `git pull`:

```bash
cd ~/Velyx-OS
git pull
velyx-update
```

Explicit equivalent:

```bash
cd ~/Velyx-OS
git pull
velyx-update --source-root "$PWD"
```

Update directly from GitHub repository:

```bash
velyx-update --source https://github.com/sanyastylee-ctrl/Velyx-OS.git
```

Shortcut to default upstream:

```bash
velyx-update --source github
```

Update from archive:

```bash
velyx-update --source https://example.com/Velyx-OS-release.tar.gz
```

Network behavior:

- fetch into a temporary directory under `~/.velyx/updates/`
- use fetched tree as `source_root`
- then continue through the normal validation/apply path
- if remote fetch fails and a local working tree is available, `velyx-update` falls back to the local `source_root`

If network fetch fails:

- remote fetch failure is logged to `~/.velyx/update.log`
- if local `source_root` exists, update falls back to it
- if no usable local source exists, update aborts before apply

## Recovery

`velyx-recovery`:

- restores the last-known-good prefix
- restores saved user units and env
- seeds app/space/intent registries if needed
- restarts user runtime
- updates `update_state.json`

`velyx-session-bootstrap.service` invokes `velyx-recovery-bootstrap` before normal bootstrap. If `recovery_needed=true`, the system tries recovery before continuing normal startup.

## Commands

Install:

```bash
./scripts/velyx-install.sh
```

Version:

```bash
velyx-version
```

Local/dev update:

```bash
velyx-update
```

Explicit local/dev update:

```bash
velyx-update --source-root "$PWD"
```

Remote update:

```bash
velyx-update --source github
```

Custom binaries:

```bash
velyx-update \
  --source-root "$PWD" \
  --bin-dir /path/to/target/release \
  --shell-binary /path/to/velyx-shell
```

Recovery:

```bash
velyx-recovery
```

Status and logs:

```bash
velyx-status
velyx-logs.sh
```

## Verification

### Successful update

```bash
cd ~/Velyx-OS
git pull
velyx-update --source-root "$PWD"
velyx-version
velyx-status
tail -n 80 ~/.velyx/update.log
cat ~/.velyx/update_state.json
```

### Invalid payload

```bash
velyx-update --source-root "$PWD" --bin-dir /tmp/missing
```

Expected:

- update blocked before apply
- `current_version` unchanged
- `last_update_result=payload_invalid`

### Broken runtime after apply

If health check fails after apply:

- update marked failed
- rollback starts automatically
- if rollback fails:
  - `recovery_needed=true`
  - next session bootstrap tries auto-recovery

Manual path:

```bash
velyx-recovery
```

### Remote/network update

```bash
velyx-update --source github
```

Expected:

- remote source cloned or downloaded into a temporary staging source
- payload validated
- update applied through the same staged flow
