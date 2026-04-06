# Local Model Runtime

Velyx OS uses a local model runtime layer so the assistant can adapt to different hardware and different task types without hardcoding a single model.

## Architecture

The local model runtime is split into six pieces:

- `ModelDiscovery`
  Finds supported models and checks which ones are locally installed.
- `HardwareProfiler`
  Detects CPU, threads, RAM, GPU, VRAM, and acceleration hints.
- `ModelProfileRegistry`
  Stores supported model profiles and hardware requirements in `~/.velyx/model_registry.json`.
- `ModelRuntimeAdapter`
  Talks to a real backend:
  - `ollama`
  - `openai-compatible local HTTP`
  - `stub`
- `ModelRouter`
  Chooses a model from `manual`, `auto_hardware`, or `auto_task` selection modes.
- `ModelHealth`
  Tracks backend status, benchmark results, last route, and fallback events.

The assistant never talks to a model directly without routing. The path is:

`assistant request -> task classification -> model router -> backend adapter -> AI response -> planner/tools`

## Model profiles

Velyx works with profiles instead of one fixed model name:

- `small`
  Fast actions, short summaries, lightweight suggestions.
- `main`
  Everyday assistant work, explanations, search synthesis, tool planning.
- `heavy`
  Longer reasoning, richer comparisons, advanced planning.

Default Qwen-ready catalog:

- `qwen-small-8b`
  Local name: `qwen2.5:7b-instruct`
- `qwen-main-14b`
  Local name: `qwen2.5:14b-instruct`
- `qwen-heavy-32b`
  Local name: `qwen2.5:32b-instruct`

These defaults are configurable and only represent the initial recommended lineup.

## Files

Persistent model runtime files live in `~/.velyx/`:

- `model_config.json`
- `model_state.json`
- `model_registry.json`
- `model_runtime.log`

### `model_config.json`

Main fields:

- `selection_mode`
  `manual | auto_hardware | auto_task`
- `default_small_model`
- `default_main_model`
- `default_heavy_model`
- `current_active_model`
- `allow_fallback`
- `backend_preferences`
- `routing_thresholds`

### `model_state.json`

Main fields:

- `active_model`
- `active_profile`
- `last_used_model`
- `last_used_profile`
- `selection_mode`
- `fallback_count`
- `last_benchmark`
- `backend_status`
- `model_available`
- `last_route`
- `hardware_profile`

### `model_registry.json`

Each model entry stores:

- `model_id`
- `display_name`
- `family`
- `size_class`
- `runtime_backend`
- `endpoint_type`
- `endpoint_url`
- `local_name`
- `minimum_ram_gb`
- `minimum_vram_gb`
- `recommended_for`
- `installed`
- `enabled`

## Hardware detection

Use:

```bash
velyx-model detect-hardware
```

The profiler tries to detect:

- CPU model
- thread count
- total RAM
- GPU availability
- GPU VRAM
- acceleration hints such as CUDA or Vulkan

If GPU information is missing, Velyx falls back to CPU and RAM based recommendations.

Recommendation output includes:

- hardware tier: `low | medium | high`
- recommended profile: `small | main | heavy`

## Selection modes

### Manual

You explicitly pick the model:

```bash
velyx-model use qwen-main-14b
velyx-model set-selection-mode manual
```

### Auto by hardware

Velyx chooses the strongest suitable default model for the detected machine:

```bash
velyx-model detect-hardware
velyx-model set-selection-mode auto_hardware
```

### Auto by task

Velyx chooses the model based on request type while still respecting hardware limits:

```bash
velyx-model set-selection-mode auto_task
```

Task groups:

- `fast_system_action`
  Usually routes to `small`
- `general_assistant`
  Usually routes to `main`
- `search_synthesis`
  Usually routes to `main`, sometimes `heavy`
- `heavy_reasoning`
  Prefers `heavy`, falls back if unavailable

## Fallback policy

Fallback is explicit and recorded.

Examples:

- `heavy` requested but unavailable -> fallback to `main`
- `main` unavailable -> fallback to `small`
- nothing usable available -> degraded local AI mode

State and logs keep:

- selected profile
- actual used model
- fallback reason
- routing explanation

The shell and assistant surface this as human-readable context, for example:

- `Using fast model for quick system action.`
- `Heavy model unavailable, falling back to main.`

## Backends

Supported backends in this stage:

- `ollama-compatible`
- `openai-compatible local HTTP`
- `stub`

Practical first path is Ollama.

## Commands

List models:

```bash
velyx-model list
```

Show status:

```bash
velyx-model status
velyx-model current
```

Detect hardware and recommendation:

```bash
velyx-model detect-hardware
velyx-model recommend
```

Install or pull a model:

```bash
velyx-model pull qwen-main-14b
velyx-model install-profile main
```

Choose defaults:

```bash
velyx-model set-default main qwen-main-14b
velyx-model set-default small qwen-small-8b
velyx-model set-default heavy qwen-heavy-32b
```

Choose runtime mode:

```bash
velyx-model set-selection-mode manual
velyx-model set-selection-mode auto_hardware
velyx-model set-selection-mode auto_task
```

Benchmark:

```bash
velyx-model benchmark
velyx-model benchmark qwen-main-14b
```

Inspect routing:

```bash
velyx-model route-task fast_system_action
velyx-model route-task search_synthesis
velyx-model route-task heavy_reasoning
```

## Connecting a local Qwen model

With Ollama installed, the typical path is:

```bash
velyx-model pull qwen-main-14b
velyx-model set-default main qwen-main-14b
velyx-model set-selection-mode auto_task
```

If you prefer a custom local endpoint, update the backend information in the model registry or AI config to point at:

- `local_http`
- `ollama-compatible`
- `openai-compatible`

The assistant and AI layer then route through the configured backend rather than hardcoding one engine.

## Shell visibility

Shell surfaces:

- current model
- selected profile
- selection mode
- backend
- model availability
- routing reason
- fallback reason

This keeps model routing explainable instead of invisible.

## Interpreting model status

- `ready`
  Local backend is reachable and the selected model is usable.
- `degraded`
  A backend or selected model is missing, but fallback is available.
- `unavailable`
  No usable local model could be selected.

If the model runtime is degraded, the assistant should still remain operational with explicit fallback messaging.
