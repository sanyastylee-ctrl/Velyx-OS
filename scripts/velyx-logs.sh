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
echo "== file logs =="
for file in \
  "${STATE_DIR}/session_manager_audit.log" \
  "${STATE_DIR}/launcher_history.log" \
  "${STATE_DIR}/sandbox_audit.log" \
  "${STATE_DIR}/shell_mvp.log"
do
  if [[ -f "${file}" ]]; then
    echo "-- ${file} --"
    tail -n "${LINES}" "${file}" || true
    echo
  fi
done
