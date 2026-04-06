#!/usr/bin/env bash
set -euo pipefail

ENV_FILE="${XDG_CONFIG_HOME:-$HOME/.config}/velyx/velyx.env"
if [[ -f "${ENV_FILE}" ]]; then
  # shellcheck disable=SC1090
  source "${ENV_FILE}"
fi

STATE_DIR="${VELYX_STATE_DIR:-$HOME/.velyx}"
STATE_FILE="${STATE_DIR}/update_state.json"
RECOVERY_CMD="${VELYX_INSTALL_PREFIX:-$HOME/.local/share/velyx}/bin/velyx-recovery"
LOG_FILE="${STATE_DIR}/update.log"

mkdir -p "${STATE_DIR}"

if [[ ! -f "${STATE_FILE}" ]]; then
  exit 0
fi

recovery_needed="$(grep -oE '"recovery_needed"[[:space:]]*:[[:space:]]*(true|false)' "${STATE_FILE}" | head -n1 | sed -E 's/.*:[[:space:]]*(true|false)/\1/' || printf 'false')"
if [[ "${recovery_needed}" != "true" ]]; then
  exit 0
fi

printf '%s event=recovery_bootstrap status=started auto=true\n' "$(date -u +"%Y-%m-%dT%H:%M:%SZ")" >> "${LOG_FILE}"
if [[ -x "${RECOVERY_CMD}" ]]; then
  "${RECOVERY_CMD}" --auto || true
else
  printf '%s event=recovery_bootstrap status=failed detail=missing_recovery_command\n' "$(date -u +"%Y-%m-%dT%H:%M:%SZ")" >> "${LOG_FILE}"
fi
