# Velyx Dev Visual Loop

`Velyx Dev Visual Loop` extends Dev Mode with screenshot-based feedback.

The workflow is:

1. apply a live UI patch in the overlay
2. reload `Velyx Shell`
3. capture a shell screenshot
4. analyze the result
5. suggest or apply one bounded refinement

## What gets stored

State:

- `~/.velyx/dev_mode.json`

Screenshots:

- `~/.velyx/dev_screenshots/`

Logs:

- `~/.velyx/dev_mode.log`
- `~/.velyx/ai.log`

## Commands

```bash
velyx-dev capture-ui-screenshot
velyx-dev get-last-screenshot
velyx-dev compare-screenshots <before> <after>
velyx-dev analyze-ui-screenshot --request "Make the buttons smaller"
velyx-dev set-auto-refine true
velyx-dev apply-next-refinement
```

## Assistant flow

When Dev Mode is active, a request like:

```text
Make the buttons smaller
```

now goes through:

1. capture before screenshot
2. preview diff
3. approval for the live patch
4. apply patch
5. restart shell
6. capture after screenshot
7. analyze result
8. optionally prepare or auto-apply one refinement

## Auto refine

`auto_refine` is off by default.

When enabled:

- the visual loop may apply one extra refinement automatically
- the loop stays bounded
- it stops when:
  - `max_visual_iterations` is reached
  - no safe refinement is available
  - the visual delta is too weak

## Current behavior

This is a practical preview feature, not a full vision design engine.

Current analysis uses:

- screenshot capture
- before/after comparison
- bounded AI-style visual heuristics
- safe refinement requests inside the existing Dev Mode scope

If screenshot capture fails, Dev Mode falls back to the original non-visual workflow.

## Safety

- visual loop works only when Dev Mode is enabled
- edits still stay inside the shell/design overlay scope
- no install/update/runtime ownership files are changed
- rollback still restores the previous overlay files
