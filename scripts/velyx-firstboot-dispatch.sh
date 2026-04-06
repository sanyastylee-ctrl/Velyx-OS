#!/usr/bin/env bash
set -euo pipefail

ENV_FILE="${XDG_CONFIG_HOME:-$HOME/.config}/velyx/velyx.env"
if [[ -f "${ENV_FILE}" ]]; then
  # shellcheck disable=SC1090
  source "${ENV_FILE}"
fi

STATE_DIR="${VELYX_STATE_DIR:-$HOME/.velyx}"
PREFIX="${VELYX_INSTALL_PREFIX:-$HOME/.local/share/velyx}"
LOG_FILE="${STATE_DIR}/first_boot.log"
mkdir -p "${STATE_DIR}"

if command -v python3 >/dev/null 2>&1 && [[ -x "${PREFIX}/bin/velyx-firstboot" ]]; then
  "${PREFIX}/bin/velyx-firstboot" status >/dev/null || true
fi

if [[ -f "${STATE_DIR}/first_boot_state.json" ]]; then
  echo "$(date -Iseconds) first_boot_state=present user=${USER:-unknown}" >> "${LOG_FILE}"
else
  echo "$(date -Iseconds) first_boot_state=missing user=${USER:-unknown}" >> "${LOG_FILE}"
fi
