#!/usr/bin/env bash
set -euo pipefail

ENV_FILE="${XDG_CONFIG_HOME:-$HOME/.config}/velyx/velyx.env"
if [[ -f "${ENV_FILE}" ]]; then
  # shellcheck disable=SC1090
  source "${ENV_FILE}"
fi

STATE_DIR="${VELYX_STATE_DIR:-$HOME/.velyx}"
LINES="${VELYX_LOG_LINES:-80}"

echo "== journald: session-manager =="
journalctl --user -u velyx-session-manager.service -n "${LINES}" --no-pager 2>/dev/null || true
echo
echo "== journald: launcher =="
journalctl --user -u velyx-launcher.service -n "${LINES}" --no-pager 2>/dev/null || true
echo
echo "== journald: permissions =="
journalctl --user -u velyx-permissions.service -n "${LINES}" --no-pager 2>/dev/null || true
echo
echo "== journald: update-engine =="
journalctl --user -u velyx-update-engine.service -n "${LINES}" --no-pager 2>/dev/null || true
echo
echo "== journald: recovery =="
journalctl --user -u velyx-recovery.service -n "${LINES}" --no-pager 2>/dev/null || true
echo
echo "== file logs =="
for file in \
  "${STATE_DIR}/session_manager_audit.log" \
  "${STATE_DIR}/launcher_history.log" \
  "${STATE_DIR}/sandbox_audit.log" \
  "${STATE_DIR}/shell_mvp.log" \
  "${STATE_DIR}/update.log" \
  "${STATE_DIR}/update_state.json" \
  "${STATE_DIR}/app_registry.log" \
  "${STATE_DIR}/apps_registry.json" \
  "${STATE_DIR}/spaces.log" \
  "${STATE_DIR}/spaces_registry.json" \
  "${STATE_DIR}/intents.log" \
  "${STATE_DIR}/intents_registry.json" \
  "${STATE_DIR}/intent_state.json" \
  "${STATE_DIR}/rules.log" \
  "${STATE_DIR}/rules_registry.json" \
  "${STATE_DIR}/rules_state.json" \
  "${STATE_DIR}/agent.log" \
  "${STATE_DIR}/agent_state.json" \
  "${STATE_DIR}/ai.log" \
  "${STATE_DIR}/ai_config.json" \
  "${STATE_DIR}/ai_state.json" \
  "${STATE_DIR}/ai_feedback.log" \
  "${STATE_DIR}/model_runtime.log" \
  "${STATE_DIR}/model_config.json" \
  "${STATE_DIR}/model_state.json" \
  "${STATE_DIR}/model_registry.json" \
  "${STATE_DIR}/first_boot.log" \
  "${STATE_DIR}/first_boot_state.json" \
  "${STATE_DIR}/installer.log" \
  "${STATE_DIR}/entry.log" \
  "${STATE_DIR}/live_mode.log" \
  "${STATE_DIR}/diagnostics.log" \
  "${STATE_DIR}/assistant.log" \
  "${STATE_DIR}/assistant_config.json" \
  "${STATE_DIR}/assistant_state.json" \
  "${STATE_DIR}/assistant_feedback.jsonl" \
  "${STATE_DIR}/network.log" \
  "${STATE_DIR}/network_state.json" \
  "${STATE_DIR}/dev_mode.log" \
  "${STATE_DIR}/dev_mode.json" \
  "${STATE_DIR}/dev_mode_config.json" \
  "${STATE_DIR}/dev_history.json" \
  "${STATE_DIR}/dev_agent.log"
do
  if [[ -f "${file}" ]]; then
    echo "-- ${file} --"
    tail -n "${LINES}" "${file}" || true
    echo
  fi
done
