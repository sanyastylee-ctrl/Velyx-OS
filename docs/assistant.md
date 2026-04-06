# Velyx Assistant

Velyx Assistant is the user-facing execution layer above the existing Velyx runtime. It translates a natural request into a safe plan, routes each step through the existing system contracts, and only asks for approval when a step changes files or system state.

## Architecture

Assistant flow:

1. user request
2. request parser
3. plan builder
4. tool dispatcher
5. approval gate when needed
6. safe execution through existing system APIs
7. synthesized result

Assistant never bypasses:

- launcher-service
- session-manager
- update / recovery flow
- permissions model
- agent safe action layer

System actions go through `velyx-agent`. Web and file actions are executed inside the assistant backend, but still remain policy-gated and logged.

## Files

Assistant runtime files:

- `~/.velyx/assistant_config.json`
- `~/.velyx/assistant_state.json`
- `~/.velyx/assistant.log`
- `~/.velyx/assistant_feedback.jsonl`

Assistant mode is aligned with the existing AI mode:

- `off`
- `suggest`
- `auto`

Default behavior:

- assistant is available
- predictive AI stays bounded
- file writes and destructive system actions still require approval

## Supported requests

Examples:

- `Open the browser`
- `Find the best Qt IDEs on the internet`
- `Switch me to development and open the browser`
- `Explain the current system state`
- `Create a note in Documents`
- `Show Qt 6.8 release notes`

Supported tool groups:

- system tools
- file tools
- web tools
- content tools

## Approval model

Assistant asks for approval before:

- creating or overwriting files
- app stop / restart
- runtime restart
- system update
- recovery actions

When approval is needed, the shell shows an approval card with:

- `Allow once`
- `Deny`

CLI equivalents:

- `velyx-assistant approve <request_id>`
- `velyx-assistant deny <request_id>`

## Web search

Assistant can perform:

- `web_search`
- `fetch_url`
- `docs_lookup`
- `github_search`
- `release_notes_lookup`

Network policy is configured in `assistant_config.json`:

- `off`
- `ask`
- `on`

For the MVP, benign public information lookups are allowed when the config says they do not require approval.

## File actions

Available file actions:

- read file
- list directory
- find files
- create file
- write file
- append file

File access is limited to configured allowed roots in `assistant_config.json`.

## CLI

Commands:

- `velyx-assistant ask "<query>"`
- `velyx-assistant status`
- `velyx-assistant mode`
- `velyx-assistant set-mode off|suggest|auto`
- `velyx-assistant web-search "<query>"`
- `velyx-assistant explain`
- `velyx-assistant summarize`
- `velyx-assistant approve <request_id>`
- `velyx-assistant deny <request_id>`
- `velyx-assistant export-training-data`

## Shell UX

The shell integrates Assistant as a first-class panel:

- input box
- quick actions
- response history
- approval card
- assistant mode indicator
- current execution status

This keeps the user experience simple:

`Ask Velyx -> Velyx plans -> Velyx asks when needed -> Velyx returns the result`

## Local model readiness

Assistant is compatible with the existing AI layer configuration and is prepared for local model setups such as:

- `model_family = qwen`
- `model_name = qwen3.5-9b`

The assistant does not embed an inference runtime. Model integration remains external and configurable through the AI layer and local endpoint configuration.

## Safety boundaries

Assistant does not allow:

- raw shell execution by the model
- arbitrary code execution
- arbitrary file mutation outside allowed roots
- direct process spawning outside launcher / agent boundaries
- hidden network actions without logs

## Diagnostics

Useful commands:

- `velyx-assistant status`
- `velyx-status`
- `velyx-logs.sh`
- `tail -n 80 ~/.velyx/assistant.log`
- `cat ~/.velyx/assistant_state.json`
