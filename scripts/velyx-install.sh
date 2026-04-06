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
PROFILES_DIR="${PREFIX}/share/profiles"
LIBEXEC_DIR="${PREFIX}/libexec"
BIN_DIR="${PREFIX}/bin"
ENV_FILE="${CONFIG_DIR}/velyx.env"
MODE="full"

if [[ "${1:-}" == "--payload-only" ]]; then
  MODE="payload-only"
  shift
elif [[ "${1:-}" == "--units-only" ]]; then
  MODE="units-only"
  shift
fi

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

install_helper_script() {
  local source_path="$1"
  local target_name="$2"
  install -Dm755 "${source_path}" "${BIN_DIR}/${target_name}"
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

determine_build_version() {
  if [[ -n "${VELYX_BUILD_VERSION:-}" ]]; then
    printf '%s\n' "${VELYX_BUILD_VERSION}"
    return 0
  fi
  if command -v git >/dev/null 2>&1 && git -C "${ROOT_DIR}" rev-parse --is-inside-work-tree >/dev/null 2>&1; then
    printf '%s\n' "$(git -C "${ROOT_DIR}" rev-parse --short HEAD)"
    return 0
  fi
  printf 'unknown\n'
}

write_version_metadata() {
  local version
  version="$(determine_build_version)"
  mkdir -p "${PREFIX}/share"
  cat > "${PREFIX}/share/version.txt" <<EOF
product=Velyx OS Preview
channel=preview
version=${version}
build_id=${version}
installed_at=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
source_root=${ROOT_DIR}
EOF
}

write_update_state() {
  local version
  version="$(determine_build_version)"
  cat > "${STATE_DIR}/update_state.json" <<EOF
{
  "current_version": "${version}",
  "staged_version": "",
  "last_good_version": "${version}",
  "update_state": "installed",
  "last_update_result": "install_ok",
  "rollback_available": false,
  "recovery_needed": false,
  "last_failed_version": "",
  "last_recovery_result": "none",
  "current_prefix": "${PREFIX}",
  "staging_root": "${STATE_DIR}/updates/staged",
  "rollback_prefix": "${STATE_DIR}/updates/last-known-good-prefix",
  "last_update_at": "$(date -u +"%Y-%m-%dT%H:%M:%SZ")"
}
EOF
}

require_cmd install
require_cmd mkdir
require_cmd sed

if [[ "${MODE}" != "payload-only" ]]; then
  require_cmd systemctl
fi

warn_optional_cmd busctl
warn_optional_cmd xprop
warn_optional_cmd wmctrl
warn_optional_cmd xdotool
warn_optional_cmd python3

mkdir -p "${BIN_DIR}" "${LIBEXEC_DIR}" "${MANIFESTS_DIR}" "${PROFILES_DIR}" "${STATE_DIR}" "${CONFIG_DIR}" "${UNIT_DIR}" "${LOCAL_BIN_DIR}"

if [[ "${MODE}" != "units-only" ]]; then
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
  install -Dm755 "${ROOT_DIR}/scripts/velyx-recovery-bootstrap.sh" "${LIBEXEC_DIR}/velyx-recovery-bootstrap"
  install_helper_script "${ROOT_DIR}/scripts/velyx-status" "velyx-status"
  install_helper_script "${ROOT_DIR}/scripts/velyx-restart.sh" "velyx-restart.sh"
  install_helper_script "${ROOT_DIR}/scripts/velyx-logs.sh" "velyx-logs.sh"
  install_helper_script "${ROOT_DIR}/scripts/velyx-update" "velyx-update"
  install_helper_script "${ROOT_DIR}/scripts/velyx-recovery" "velyx-recovery"
  install_helper_script "${ROOT_DIR}/scripts/velyx-version" "velyx-version"
  install_helper_script "${ROOT_DIR}/scripts/velyx-app" "velyx-app"
  install_helper_script "${ROOT_DIR}/scripts/velyx-space" "velyx-space"
  install_helper_script "${ROOT_DIR}/scripts/velyx-intent" "velyx-intent"
  install_helper_script "${ROOT_DIR}/scripts/velyx-rule" "velyx-rule"
  install_helper_script "${ROOT_DIR}/scripts/velyx-agent" "velyx-agent"
  install_helper_script "${ROOT_DIR}/scripts/velyx-ai" "velyx-ai"
  install_helper_script "${ROOT_DIR}/scripts/velyx-model" "velyx-model"
  install_helper_script "${ROOT_DIR}/scripts/velyx-assistant" "velyx-assistant"
  install_helper_script "${ROOT_DIR}/scripts/velyx-firstboot" "velyx-firstboot"
  install_helper_script "${ROOT_DIR}/scripts/velyx-installer" "velyx-installer"
  install_helper_script "${ROOT_DIR}/scripts/velyx-diagnostics" "velyx-diagnostics"
  install_helper_script "${ROOT_DIR}/scripts/velyx-vm-preview" "velyx-vm-preview"

  cp -a "${ROOT_DIR}/app-manifests/." "${MANIFESTS_DIR}/"
  if [[ -d "${ROOT_DIR}/profiles" ]]; then
    cp -a "${ROOT_DIR}/profiles/." "${PROFILES_DIR}/"
  fi
  write_version_metadata
  if [[ "${MODE}" == "full" ]]; then
    mkdir -p "${STATE_DIR}/updates/staged" "${STATE_DIR}/updates/failed" "${STATE_DIR}/apps"
    write_update_state
  fi
fi

if [[ "${MODE}" != "payload-only" ]]; then
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
VELYX_APP_REGISTRY=${STATE_DIR}/apps_registry.json
VELYX_USER_APPS_DIR=${STATE_DIR}/apps
VELYX_SPACES_REGISTRY=${STATE_DIR}/spaces_registry.json
VELYX_INTENTS_REGISTRY=${STATE_DIR}/intents_registry.json
VELYX_RULES_REGISTRY=${STATE_DIR}/rules_registry.json
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
  install_script_binary "${ROOT_DIR}/scripts/velyx-update" "velyx-update"
  install_script_binary "${ROOT_DIR}/scripts/velyx-recovery" "velyx-recovery"
  install_script_binary "${ROOT_DIR}/scripts/velyx-version" "velyx-version"
  install_script_binary "${ROOT_DIR}/scripts/velyx-app" "velyx-app"
  install_script_binary "${ROOT_DIR}/scripts/velyx-space" "velyx-space"
  install_script_binary "${ROOT_DIR}/scripts/velyx-intent" "velyx-intent"
  install_script_binary "${ROOT_DIR}/scripts/velyx-rule" "velyx-rule"
  install_script_binary "${ROOT_DIR}/scripts/velyx-agent" "velyx-agent"
  install_script_binary "${ROOT_DIR}/scripts/velyx-ai" "velyx-ai"
  install_script_binary "${ROOT_DIR}/scripts/velyx-model" "velyx-model"
  install_script_binary "${ROOT_DIR}/scripts/velyx-assistant" "velyx-assistant"
  install_script_binary "${ROOT_DIR}/scripts/velyx-firstboot" "velyx-firstboot"
  install_script_binary "${ROOT_DIR}/scripts/velyx-installer" "velyx-installer"
  install_script_binary "${ROOT_DIR}/scripts/velyx-diagnostics" "velyx-diagnostics"
  install_script_binary "${ROOT_DIR}/scripts/velyx-vm-preview" "velyx-vm-preview"
  if command -v python3 >/dev/null 2>&1; then
    "${BIN_DIR}/velyx-app" sync-system >/dev/null || true
    "${BIN_DIR}/velyx-space" seed-defaults >/dev/null || true
    "${BIN_DIR}/velyx-intent" seed-defaults >/dev/null || true
    "${BIN_DIR}/velyx-rule" seed-defaults >/dev/null || true
    "${BIN_DIR}/velyx-ai" status >/dev/null || true
    "${BIN_DIR}/velyx-model" status >/dev/null || true
    "${BIN_DIR}/velyx-assistant" status >/dev/null || true
    "${BIN_DIR}/velyx-firstboot" prepare --force --action install --install-mode standard_preview --ai-mode off --model-selection auto_hardware --backend stub --default-space general --predictive-mode off >/dev/null || true
    "${BIN_DIR}/velyx-diagnostics" status >/dev/null || true
  fi
fi

if [[ "${MODE}" == "payload-only" ]]; then
  cat <<EOF
Velyx OS payload prepared.
prefix: ${PREFIX}
state:  ${STATE_DIR}
EOF
  exit 0
fi

if [[ "${MODE}" == "units-only" ]]; then
  cat <<EOF
Velyx OS units refreshed.
prefix: ${PREFIX}
state:  ${STATE_DIR}
env:    ${ENV_FILE}
units:  ${UNIT_DIR}
EOF
  exit 0
fi

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
