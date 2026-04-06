# Velyx First Boot

`Velyx First Boot` is the first experience after installation and reboot. Instead of dropping straight into a raw shell session, Velyx now routes startup through `velyx-entry`, checks first-boot state, and shows a short onboarding flow before normal use.

## State file

Canonical first boot state:

- `~/.velyx/first_boot.json`

Compatibility mirror:

- `~/.velyx/first_boot_state.json`

Base shape:

```json
{
  "completed": false,
  "step": "welcome",
  "ai_mode": null,
  "username": null
}
```

After completion:

```json
{
  "completed": true
}
```

## Screens

The UI is intentionally short and capped at three screens:

1. `Welcome`
2. `Setup`
3. `Ready`

This keeps the setup path fast enough to finish in under a minute.

## CLI

```bash
velyx-firstboot status
velyx-firstboot start
velyx-firstboot complete --username "Sasha" --ai-mode local
velyx-firstboot reset
```

## Session flow

Startup now follows:

`Velyx Entry -> Velyx First Boot -> Velyx Shell`

If `first_boot.json` says `completed=false`, `velyx-entry --session` launches the shell with the first-boot overlay visible.

If `completed=true`, `velyx-entry --session` opens normal `Velyx Shell`.

## Integration

- chosen AI mode is written into Velyx AI / assistant config
- `Hybrid` keeps network-assisted assistant behavior available
- `Local` keeps the assistant local-first
- Dev Mode remains inaccessible until the user has entered the shell

## Reset behavior

Running:

```bash
velyx-firstboot reset
```

marks the setup as incomplete again, so the next session returns to `Velyx First Boot`.
