#!/usr/bin/env bash
set -euo pipefail

ENV_FILE="${XDG_CONFIG_HOME:-$HOME/.config}/velyx/velyx.env"
if [[ -f "${ENV_FILE}" ]]; then
  # shellcheck disable=SC1090
  source "${ENV_FILE}"
fi

STATE_DIR="${VELYX_STATE_DIR:-$HOME/.velyx}"
CLEAN_STATE="${1:-}"

if [[ "${CLEAN_STATE}" == "--clean-state" ]]; then
  rm -f "${STATE_DIR}/session_state.json"
fi

systemctl --user daemon-reload
systemctl --user restart velyx-session-manager.service
systemctl --user restart velyx-session-bootstrap.service
systemctl --user try-restart velyx-permissions.service velyx-launcher.service velyx-update-engine.service velyx-recovery.service velyx-shell.service || true

echo "Velyx user services restarted."
