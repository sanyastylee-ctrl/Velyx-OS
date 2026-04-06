# Velyx Dev Mode UX

## Overview

`Dev Mode` turns the installed Velyx system into a fast UI iteration environment.
When it is enabled, `Velyx Assistant` can plan shell changes, preview them, apply them in the live overlay, reload the shell, capture screenshots, and offer the next refinement step.

This flow is intentionally bounded:

- UI-only work can use `live apply`
- broader development work can require `staged update`
- deeper runtime work can require restart or reboot
- forbidden scopes are denied instead of being applied live

## How To Enable

You can enable Dev Mode in two places:

- `Control Center -> Dev`
- `velyx-dev enable`

You can also choose the current mode:

- `ui_live_only`
- `full_dev`

Shell UI shows the active mode, current apply strategy, validation status, visual feedback status, and recent history.

## Dev Flow

The normal shell flow is:

1. Ask Velyx for a visual or shell change.
2. Velyx classifies the request.
3. Velyx shows the planned change and affected files.
4. You approve the action if required.
5. Velyx applies the patch.
6. The shell reloads.
7. A screenshot is captured after the change.
8. Velyx summarizes the visual result and can suggest a refinement.
9. You can apply the next refinement or roll back.

Examples:

- `Make the buttons smaller`
- `Move the panel right`
- `Reduce card spacing`
- `Add a button near Assistant`

## Live Vs Staged

Velyx now states the apply strategy directly in the shell:

- `live_apply`: safe shell or theme change, applied in the overlay
- `staged_update`: change should be staged and applied through the update path
- `reboot_required`: deeper runtime change, requires restart or reboot cycle
- `deny`: blocked by scope or policy

This keeps Dev Mode honest. If a request is not safe to apply live, the assistant says so instead of pretending it is live-editable.

## Visual Feedback

When visual feedback is active, the shell shows:

- before screenshot
- after screenshot
- short change summary
- next refinement suggestion
- bounded iteration loop

`Auto refine` can be toggled from the shell. Iterations stay limited for safety.

## Rollback

Rollback is available directly from the shell:

- `Rollback` for the latest change
- `Rollback selected` from the dev history list

Every recent change shows:

- change summary
- change type
- apply mode
- affected files

After rollback, Velyx requests a shell reload so the restored UI is visible immediately.

## Limits

This is not a full unrestricted coding agent.

It is a controlled development layer for:

- shell UI edits
- bounded shell logic changes
- preview-first visual iteration
- safe rollback and history

It does not bypass update, recovery, permissions, or broader runtime boundaries.
