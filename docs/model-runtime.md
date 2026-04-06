# Velyx Model Runtime

`Velyx Model Runtime` is the local model layer used by `velyx-ai`, `velyx-assistant`, `Velyx First Boot`, and `Velyx Control Center`.

## Supported backends

- `ollama`
- `openai-compatible`
- `stub`

`stub` is a degraded fallback. It keeps the assistant responsive, but it does not mean a real local model is available.

## Model profiles

- `small`: fast system actions and short responses
- `main`: normal assistant work, explanations, summaries, search synthesis
- `heavy`: longer comparison and heavier reasoning

The runtime works with profiles, not one hardcoded model forever.

## Selection modes

- `manual`: user picks the active model
- `auto_hardware`: Velyx recommends based on CPU, RAM, GPU, and VRAM
- `auto_task`: Velyx routes by task type and then applies hardware/fallback constraints

Task routing groups:

- `fast_system_action` -> `small`
- `general_assistant` -> `main`
- `search_synthesis` -> `main`
- `heavy_reasoning` -> `heavy`

## Hardware detection

Use:

```bash
velyx-model detect-hardware
velyx-model recommend
```

The runtime detects:

- CPU model and thread count
- total RAM
- GPU presence
- VRAM when available
- acceleration hints such as CUDA or Vulkan

It then maps the system to a hardware tier:

- `low`
- `medium`
- `high`

## Fallback policy

When the requested model cannot be used:

- `heavy -> main`
- `main -> small`
- `small -> stub fallback`

This keeps the assistant working without pretending that a real local model is available.

Important signals:

- `model_available`: whether a real routed local model is available
- `runtime_ready`: whether the selected backend can answer at all
- `fallback_reason`: why Velyx had to step down
- `resolved_status`: `ready`, `fallback`, or `degraded`

## CLI

List and inspect models:

```bash
velyx-model list
velyx-model status
velyx-model current
```

Hardware and recommendation:

```bash
velyx-model detect-hardware
velyx-model recommend
velyx-model benchmark
```

Manual selection:

```bash
velyx-model use qwen-main-14b
velyx-model set-selection-mode manual
```

Automatic selection:

```bash
velyx-model set-selection-mode auto_hardware
velyx-model set-selection-mode auto_task
```

Backend selection:

```bash
velyx-model set-backend ollama
velyx-model set-backend openai-compatible
velyx-model set-backend stub
```

Install or pull:

```bash
velyx-model pull qwen-main-14b
velyx-model install-profile main
```

## Assistant integration

`velyx-assistant` does not bypass the model layer.

Pipeline:

1. classify the request
2. ask `velyx-model route-task`
3. choose profile + backend
4. call `velyx-ai`
5. use tools if needed
6. return the result

This means:

- simple system requests usually route to `small`
- everyday assistant requests route to `main`
- heavier planning/comparison requests try `heavy`
- when `heavy` is missing, Velyx falls back cleanly

## First Boot and Control Center

Users can manage:

- AI mode
- backend
- selection mode
- current model

From:

- `Velyx First Boot`
- `Velyx Control Center`

## Status and diagnostics

Check:

```bash
velyx-model status
velyx-ai status
velyx-status
tail -n 100 ~/.velyx/model_runtime.log
cat ~/.velyx/model_config.json
cat ~/.velyx/model_state.json
```

## Limits

- no distributed inference
- no training or fine-tuning pipeline
- no cloud routing
- no automatic large model download without explicit user action
