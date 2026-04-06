# AI Layer in Velyx OS

## Principle

Velyx AI is an advisor, planner, and recommender.

It is **not**:
- a raw shell runner
- a direct process launcher
- a bypass around `intents`, `agent`, `launcher`, or `session-manager`

All execution stays inside existing safe system boundaries.

## Runtime files

User state:
- `~/.velyx/ai_config.json`
- `~/.velyx/ai_state.json`
- `~/.velyx/ai.log`
- `~/.velyx/ai_feedback.log`

## Feature modes

Supported modes:
- `off`
- `suggest`
- `auto`

Default:
- `off`

Meaning:
- `off`: AI does not influence runtime state automatically. Manual `summary` and `explain` remain available.
- `suggest`: AI can build summaries and create suggested safe actions. The user must apply them.
- `auto`: AI may auto-dispatch only allowlisted actions, only above confidence threshold, and only through the safe agent layer.

## Adapter architecture

`velyx-ai` is split into the following internal layers:

1. `ContextBuilder`
2. `PromptBuilder`
3. `ModelAdapter`
4. `ResponseParser`
5. `SafeActionTranslator`
6. `FeedbackRecorder`

This keeps UI, system state, model transport, and execution separated.

## Supported adapter modes

Configured via `ai_config.json`:
- `stub`
- `openai-compatible`
- `ollama-compatible`

Provider model fields are configurable:
- `model_family`
- `model_name`
- `endpoint_type`
- `endpoint_url`

Recommended starter local config:
- `model_family = qwen`
- `model_name = qwen3.5-9b`

Example local Ollama-style configuration:

```json
{
  "enabled": true,
  "mode": "suggest",
  "provider": "local",
  "model_family": "qwen",
  "model_name": "qwen3.5-9b",
  "endpoint_type": "ollama-compatible",
  "endpoint_url": "http://127.0.0.1:11434",
  "timeout_ms": 6000,
  "max_tokens": 512,
  "allow_auto_actions": false,
  "allowed_auto_action_types": [],
  "blocked_suggestion_action_types": [],
  "requires_user_confirmation_types": ["enter_recovery", "restart_runtime"],
  "collect_feedback": true,
  "min_confidence_for_suggest": 0.45,
  "min_confidence_for_auto": 0.80
}
```

## Context snapshot

AI receives a compact serialized snapshot built from current runtime state:
- current version
- active space
- active space state
- session state
- session health
- recovery flag
- update state
- last update result
- running apps
- failed apps
- last intent
- last rule
- last agent action
- top intents
- top rules
- current mode

## Suggestion pipeline

Pipeline:

`context -> prompt -> model response -> parser -> safe action translation -> agent -> system`

The model never calls system APIs directly.

Structured output format:

```json
{
  "summary": "...",
  "confidence": 0.0,
  "recommended_action_type": "run_intent",
  "recommended_action_payload": {
    "intent_id": "dev_start"
  },
  "reason": "...",
  "user_message": "...",
  "safe_to_auto_run": false
}
```

If parsing fails:
- Velyx falls back to summary-only mode
- no action is executed

## Safe action translation

Allowed translated action types:
- `run_intent`
- `activate_space`
- `launch_app`
- `restart_app`
- `stop_app`
- `enter_recovery`
- `restart_runtime`
- `none`

Execution always goes through `velyx-agent`.

## Predictive behavior

Predictive trigger points:
- `session_ready`
- `active_space_changed`
- `update_succeeded`
- `update_failed`
- `recovery_needed`
- `no_active_app_in_current_space`
- `idle_context`
- manual ask / manual suggest

In `suggest` mode:
- Velyx shows a suggestion card in shell
- the user chooses `Apply`, `Dismiss`, or `Never`

In `auto` mode:
- Velyx still checks:
  - `allow_auto_actions`
  - action allowlist
  - confidence threshold
  - confirmation-required types
  - `safe_to_auto_run`

## CLI

Supported commands:

```bash
velyx-ai status
velyx-ai mode
velyx-ai set-mode off
velyx-ai set-mode suggest
velyx-ai set-mode auto
velyx-ai summary
velyx-ai explain
velyx-ai suggest
velyx-ai tick
velyx-ai run-last-suggestion
velyx-ai dismiss-last-suggestion
velyx-ai block-last-suggestion
velyx-ai export-training-data
velyx-ai config
velyx-ai ping-model
```

## Feedback and training export

Feedback events are written to:
- `~/.velyx/ai_feedback.log`

Examples:
- `suggestion_shown`
- `suggestion_accepted`
- `suggestion_dismissed`
- `suggestion_blocked`
- `auto_action_executed`

Training-data preparation:

```bash
velyx-ai export-training-data > ai-training.jsonl
```

This is dataset export only, not a training pipeline.

## Safety

Important boundaries:
- AI does not execute arbitrary shell
- AI does not spawn processes directly
- AI does not mutate arbitrary files
- AI does not bypass `velyx-agent`
- AI can be fully disabled with `mode = off`
