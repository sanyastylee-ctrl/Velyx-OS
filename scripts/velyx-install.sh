#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PREFIX="${VELYX_PREFIX:-$HOME/.local/share/velyx}"
STATE_DIR="${VELYX_STATE_DIR:-$HOME/.velyx}"
CONFIG_DIR="${XDG_CONFIG_HOME:-$HOME/.config}/velyx"
UNIT_DIR="${XDG_CONFIG_HOME:-$HOME/.config}/systemd/user"
LOCAL_BIN_DIR="${HOME}/.local/bin"
BIN_SOURCE_DIR="${VELYX_BIN_DIR:-${ROOT_DIR}/target/release}"
MANIFESTS_DIR="${PREFIX}/share/app-manifests"
LIBEXEC_DIR="${PREFIX}/libexec"
BIN_DIR="${PREFIX}/bin"
ENV_FILE="${CONFIG_DIR}/velyx.env"

require_cmd() {
  command -v "$1" >/dev/null 2>&1 || {
    echo "missing required command: $1" >&2
    exit 1
  }
}

warn_optional_cmd() {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo "warning: optional command '$1' not found; window/input features may be limited" >&2
  fi
}

resolve_binary_source() {
  local name="$1"
  if [[ -f "${BIN_SOURCE_DIR}/${name}" ]]; then
    printf '%s\n' "${BIN_SOURCE_DIR}/${name}"
    return 0
  fi
  if [[ -f "${BIN_SOURCE_DIR}/${name}.exe" ]]; then
    printf '%s\n' "${BIN_SOURCE_DIR}/${name}.exe"
    return 0
  fi
  return 1
}

install_binary() {
  local source_name="$1"
  local target_name="$2"
  local source_path
  source_path="$(resolve_binary_source "${source_name}")" || {
    echo "missing binary: ${source_name} in ${BIN_SOURCE_DIR}" >&2
    exit 1
  }
  install -Dm755 "${source_path}" "${BIN_DIR}/${target_name}"
}

install_script_binary() {
  local source_path="$1"
  local target_name="$2"
  install -Dm755 "${source_path}" "${BIN_DIR}/${target_name}"
  ln -snf "${BIN_DIR}/${target_name}" "${LOCAL_BIN_DIR}/${target_name}"
}

render_user_unit() {
  local source_path="$1"
  local target_path="$2"
  sed \
    -e "s|/usr/bin/velyx-session-manager|${BIN_DIR}/velyx-session-manager|g" \
    -e "s|/usr/bin/velyx-launcher-service|${BIN_DIR}/velyx-launcher-service|g" \
    -e "s|/usr/bin/velyx-permissions-service|${BIN_DIR}/velyx-permissions-service|g" \
    -e "s|/usr/bin/velyx-settings-service|${BIN_DIR}/velyx-settings-service|g" \
    -e "s|/usr/bin/velyx-diagnostics-service|${BIN_DIR}/velyx-diagnostics-service|g" \
    -e "s|/usr/bin/velyx-ai-service|${BIN_DIR}/velyx-ai-service|g" \
    -e "s|/usr/bin/velyx-file-service|${BIN_DIR}/velyx-file-service|g" \
    -e "s|/usr/bin/velyx-update-engine|${BIN_DIR}/velyx-update-engine|g" \
    -e "s|/usr/bin/velyx-recovery-service|${BIN_DIR}/velyx-recovery-service|g" \
    -e "s|/usr/bin/velyx-shell|${BIN_DIR}/velyx-shell|g" \
    -e "s|/usr/lib/velyx/boot|${LIBEXEC_DIR}|g" \
    "${source_path}" > "${target_path}"
}

require_cmd install
require_cmd mkdir
require_cmd sed
require_cmd systemctl

warn_optional_cmd busctl
warn_optional_cmd xprop
warn_optional_cmd wmctrl
warn_optional_cmd xdotool

mkdir -p "${BIN_DIR}" "${LIBEXEC_DIR}" "${MANIFESTS_DIR}" "${STATE_DIR}" "${CONFIG_DIR}" "${UNIT_DIR}" "${LOCAL_BIN_DIR}"

install_binary "session-manager-service" "velyx-session-manager"
install_binary "settings-service" "velyx-settings-service"
install_binary "permissions-service" "velyx-permissions-service"
install_binary "launcher-service" "velyx-launcher-service"
install_binary "diagnostics-service" "velyx-diagnostics-service"
install_binary "ai-service" "velyx-ai-service"
install_binary "file-service" "velyx-file-service"
install_binary "update-engine" "velyx-update-engine"
install_binary "recovery-service" "velyx-recovery-service"
install_binary "installer-service" "velyx-installer-service"

if [[ -n "${VELYX_SHELL_BINARY:-}" ]]; then
  install -Dm755 "${VELYX_SHELL_BINARY}" "${BIN_DIR}/velyx-shell"
elif resolve_binary_source "velyx-shell" >/dev/null 2>&1; then
  install -Dm755 "$(resolve_binary_source "velyx-shell")" "${BIN_DIR}/velyx-shell"
else
  echo "missing shell binary: set VELYX_SHELL_BINARY or place velyx-shell in ${BIN_SOURCE_DIR}" >&2
  exit 1
fi

install -Dm755 "${ROOT_DIR}/scripts/velyx-user-session-bootstrap.sh" "${LIBEXEC_DIR}/velyx-user-session-bootstrap"
install -Dm755 "${ROOT_DIR}/scripts/velyx-firstboot-dispatch.sh" "${LIBEXEC_DIR}/velyx-firstboot-dispatch"
install -Dm755 "${ROOT_DIR}/scripts/velyx-system-session-bootstrap.sh" "${LIBEXEC_DIR}/velyx-system-session-bootstrap"

cp -a "${ROOT_DIR}/app-manifests/." "${MANIFESTS_DIR}/"

cat > "${ENV_FILE}" <<EOF
VELYX_SESSION_MANAGER_BINARY=${BIN_DIR}/velyx-session-manager
VELYX_SETTINGS_BINARY=${BIN_DIR}/velyx-settings-service
VELYX_PERMISSIONS_BINARY=${BIN_DIR}/velyx-permissions-service
VELYX_LAUNCHER_BINARY=${BIN_DIR}/velyx-launcher-service
VELYX_DIAGNOSTICS_BINARY=${BIN_DIR}/velyx-diagnostics-service
VELYX_AI_BINARY=${BIN_DIR}/velyx-ai-service
VELYX_FILE_BINARY=${BIN_DIR}/velyx-file-service
VELYX_UPDATE_ENGINE_BINARY=${BIN_DIR}/velyx-update-engine
VELYX_RECOVERY_BINARY=${BIN_DIR}/velyx-recovery-service
VELYX_SHELL_BINARY=${BIN_DIR}/velyx-shell
VELYX_MANIFESTS_DIR=${MANIFESTS_DIR}
VELYX_INSTALL_PREFIX=${PREFIX}
VELYX_STATE_DIR=${STATE_DIR}
EOF

for unit in "${ROOT_DIR}"/systemd/user/*; do
  render_user_unit "${unit}" "${UNIT_DIR}/$(basename "${unit}")"
done

systemctl --user daemon-reload
systemctl --user enable velyx-session-manager.service velyx-session-bootstrap.service
systemctl --user restart velyx-session-manager.service || true
systemctl --user restart velyx-session-bootstrap.service || true

install_script_binary "${ROOT_DIR}/scripts/velyx-status" "velyx-status"
install_script_binary "${ROOT_DIR}/scripts/velyx-restart.sh" "velyx-restart.sh"
install_script_binary "${ROOT_DIR}/scripts/velyx-logs.sh" "velyx-logs.sh"

cat <<EOF
Velyx OS installed.
prefix: ${PREFIX}
state:  ${STATE_DIR}
env:    ${ENV_FILE}
units:  ${UNIT_DIR}

Next steps:
  systemctl --user status velyx-session-manager.service
  systemctl --user status velyx-session-bootstrap.service
  ${LOCAL_BIN_DIR}/velyx-status
EOF
