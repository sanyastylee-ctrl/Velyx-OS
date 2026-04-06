# Velyx Network

`Velyx Network` makes connectivity a first-class part of `Velyx OS`.

It provides a single runtime view for:

- `offline`
- `online`
- `limited`

The network layer is exposed through:

- `velyx-network`
- `~/.velyx/network_state.json`
- `~/.velyx/network.log`

## What Velyx Network Checks

### Internet connectivity

Best-effort check that the system can reach the public internet.

### Update reachability

Checks whether `Velyx Update` can reach its remote source when network update is requested.

Typical states:

- `reachable`
- `local_only`
- `unavailable`

### AI backend reachability

Checks whether the configured local AI backend is available.

Typical states:

- `reachable`
- `not_required`
- `unavailable`

If AI is disabled or uses `stub`, backend reachability is reported as `not_required`.

## CLI

```bash
velyx-network status
velyx-network check
velyx-network state
velyx-network update-reachable
velyx-network ai-reachable
velyx-network last-error
```

## State Model

`offline`
- public internet is not reachable

`online`
- internet works
- update source is reachable
- AI backend is reachable or not required

`limited`
- internet works
- but update reachability or AI backend availability is degraded

## System Integration

### Assistant

`Velyx Assistant` reads the network state before web actions.

Effects:

- offline network blocks web search and URL fetch
- assistant status shows current network state
- web search remains allowed only when both network policy and live network state permit it

### Update

`Velyx Update` now checks network reachability before remote update fetch.

If a remote source is not reachable:

- the condition is logged
- update warns clearly
- remote fetch is attempted only as a bounded path
- local fallback remains available

### Control Center

`Velyx Control Center` shows:

- network state
- update reachability
- AI backend reachability
- last network error

This gives the user a single place to see whether:

- internet is available
- updates can use the network
- assistant web search is likely to work
- the local AI endpoint is reachable

## Verification

1. Run:
```bash
velyx-network check
```

2. Disconnect network and run:
```bash
velyx-network check
velyx-status
```

Expected:
- network state becomes `offline`
- assistant web search should fail gracefully

3. Reconnect network and run:
```bash
velyx-network check
velyx-status
```

Expected:
- network state becomes `online` or `limited`

4. Try network update:
```bash
velyx-update --source github
```

5. Try assistant web flow:
```bash
velyx-assistant ask "Find the best Qt IDEs on the internet"
```

## Notes

- `Velyx Network` is a runtime service layer, not a full network manager.
- It is designed to provide readiness, reachability, and product-visible state for the rest of Velyx.
