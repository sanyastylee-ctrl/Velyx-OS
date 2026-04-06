# Velyx Dev Agent

`Velyx Dev Agent` is the controlled system-development layer inside `Velyx Dev Mode`.

It is designed for:
- live shell UI iteration
- bounded shell behavior changes
- staged system-development changes
- explicit approval, validation, history, and rollback

It is not:
- an unrestricted shell bot
- a full-repo autonomous coding agent
- an auto-push or auto-commit system

## Modes

- `disabled`
  - Dev Agent is off.
- `ui_live_only`
  - allows only shell/UI-scoped live changes
  - uses the live overlay and shell reload path
- `full_dev`
  - allows staged system-development changes in approved scopes
  - may mark changes as reboot-required

State is stored in:
- `~/.velyx/dev_mode_config.json`
- `~/.velyx/dev_mode.json`

## Pipeline

Every dev request goes through:

1. `RequestParser`
2. `ChangeClassifier`
3. `ScopeResolver`
4. `FileLocator`
5. `PatchPlanner`
6. `ApplyStrategyResolver`
7. `ApprovalBuilder`
8. `Executor`
9. `Validator`
10. `RollbackManager`

The user-facing experience stays simple:

`request -> plan -> diff summary -> approval -> apply -> validate -> rollback if needed`

## Change Classes

- `ui_live`
  - layout, spacing, card tweaks, QML visual edits
  - apply strategy: `live_apply`
- `shell_logic`
  - shell panel behavior and non-critical shell interaction logic
  - apply strategy: `live_apply` or `staged_update`
- `runtime_noncritical`
  - intents, assistant tooling, model routing heuristics, diagnostics formatting
  - apply strategy: `staged_update`
- `runtime_critical`
  - update/recovery/install/bootstrap/permissions/session-manager-adjacent changes
  - apply strategy: `reboot_required`
- `forbidden`
  - out-of-scope or unsafe requests
  - apply strategy: `deny`

## Allowed Scopes

Safe live scopes:
- `apps/shell/qml/`
- `apps/shell/src/`
- `packages/design-system/qml/Velyx/DesignSystem/Theme.qml`

Staged update scopes:
- `services/`
- `scripts/velyx-assistant`
- `scripts/velyx-ai`
- `scripts/velyx-agent`
- `scripts/velyx-intent`
- `scripts/velyx-rule`
- `scripts/velyx-model`
- `scripts/velyx-firstboot`
- `scripts/velyx-installer`
- `scripts/velyx-diagnostics`
- `docs/`

Forbidden:
- arbitrary system paths
- unrelated user files
- secrets and credentials
- unrestricted execution paths

## Commands

Core:
- `velyx-dev enable`
- `velyx-dev disable`
- `velyx-dev set-mode disabled|ui_live_only|full_dev`
- `velyx-dev status`

Planning:
- `velyx-dev dev-agent-plan "<request>"`
- `velyx-dev dev-agent-apply "<request>"`

Inspection:
- `velyx-dev list-ui-files`
- `velyx-dev list-system-modules`
- `velyx-dev list-qml-components`
- `velyx-dev search-ui-component "<query>"`
- `velyx-dev search-code "<query>"`
- `velyx-dev read-ui-file <path>`
- `velyx-dev read-dev-file <path>`

Validation and export:
- `velyx-dev validate-code-scope <path>`
- `velyx-dev validate-qml-file <path>`
- `velyx-dev run-dev-validation [files...]`
- `velyx-dev export-patch-bundle [change_id]`
- `velyx-dev commit-message [change_id]`

History and rollback:
- `velyx-dev history`
- `velyx-dev rollback`
- `velyx-dev restore <change_id>`

## Live vs Staged

Live apply:
- writes into `~/.velyx/dev_overlay/`
- reloads `Velyx Shell`
- can use screenshot analysis and refinement

Staged update:
- writes bounded source changes into the working tree
- records a patch bundle in `~/.velyx/dev_patches/`
- marks the change for `Velyx Update`

## History and Logs

Files:
- `~/.velyx/dev_history.json`
- `~/.velyx/dev_agent.log`
- `~/.velyx/dev_patches/`

Each change records:
- request text
- change class
- apply mode
- affected files
- validation result
- rollback availability

## Examples

- `Make the buttons smaller`
  - classified as `ui_live`
  - applied live through the overlay

- `Add a new intent "Focus Session"`
  - classified as `runtime_noncritical`
  - staged for update

- `Improve recovery bootstrap logic`
  - classified as `runtime_critical`
  - marked `reboot_required` in `full_dev`
