#!/usr/bin/env bash
set -euo pipefail

HOME_DIR="${HOME:-/home/velyx}"
USER_ID="$(id -u)"
STATE_DIR="${VELYX_STATE_DIR:-${HOME_DIR}/.velyx}"
ENV_FILE="${HOME_DIR}/.config/velyx/velyx.env"
PID_FILE="${STATE_DIR}/primary-shell.pid"
LOG_FILE="${STATE_DIR}/primary-shell.log"

log() {
  local message="$1"
  local timestamp
  timestamp="$(date -Is)"
  printf '%s %s\n' "${timestamp}" "${message}" | tee -a "${LOG_FILE}" >&2
}

if [[ -f "${ENV_FILE}" ]]; then
  set -a
  # shellcheck disable=SC1090
  source "${ENV_FILE}"
  set +a
fi

export HOME="${HOME_DIR}"
export XDG_RUNTIME_DIR="${XDG_RUNTIME_DIR:-/run/user/${USER_ID}}"
if [[ -S "${XDG_RUNTIME_DIR}/bus" ]]; then
  export DBUS_SESSION_BUS_ADDRESS="${DBUS_SESSION_BUS_ADDRESS:-unix:path=${XDG_RUNTIME_DIR}/bus}"
fi

mkdir -p "${STATE_DIR}"
printf '%s\n' "$$" > "${PID_FILE}"
touch "${LOG_FILE}"

log "primary-session start pid=$$ uid=${USER_ID} user=$(id -un) tty=$(tty 2>/dev/null || echo unknown)"
log "primary-session env HOME=${HOME} XDG_RUNTIME_DIR=${XDG_RUNTIME_DIR:-} DBUS_SESSION_BUS_ADDRESS=${DBUS_SESSION_BUS_ADDRESS:-unset}"
systemctl --user start --no-block velyx-session.target >> "${LOG_FILE}" 2>&1 || true
/usr/lib/velyx/boot/velyx-shell-session-launch
status=$?
log "primary-session exit pid=$$ status=${status}"
exit "${status}"
