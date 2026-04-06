#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BUILD_DIR="${ROOT_DIR}/build"
ROOTFS_DIR="${BUILD_DIR}/rootfs"
BASE_ROOTFS="${VELYX_BASE_ROOTFS:-}"
BIN_DIR="${VELYX_BIN_DIR:-${ROOT_DIR}/target/release}"
USER_NAME="${VELYX_BOOT_USER:-velyx}"

require_cmd() {
  command -v "$1" >/dev/null 2>&1 || {
    echo "missing required command: $1" >&2
    exit 1
  }
}

copy_tree() {
  local src="$1"
  local dst="$2"
  if command -v rsync >/dev/null 2>&1; then
    rsync -a "$src"/ "$dst"/
  else
    cp -a "$src"/. "$dst"/
  fi
}

resolve_binary_source() {
  local name="$1"
  if [[ -f "${BIN_DIR}/${name}" ]]; then
    printf '%s\n' "${BIN_DIR}/${name}"
    return 0
  fi
  if [[ -f "${BIN_DIR}/${name}.exe" ]]; then
    printf '%s\n' "${BIN_DIR}/${name}.exe"
    return 0
  fi
  if [[ "${name}" == *.exe ]]; then
    local without_exe="${name%.exe}"
    if [[ -f "${BIN_DIR}/${without_exe}" ]]; then
      printf '%s\n' "${BIN_DIR}/${without_exe}"
      return 0
    fi
  fi
  return 1
}

install_binary() {
  local source_name="$1"
  local target_name="$2"
  local source_path=""
  source_path="$(resolve_binary_source "${source_name}")" || {
    echo "missing binary: ${source_name} in ${BIN_DIR}" >&2
    exit 1
  }
  install -Dm755 "${source_path}" "${ROOTFS_DIR}/usr/bin/${target_name}"
}

install_helper_script() {
  local source_path="$1"
  local target_name="$2"
  install -Dm755 "${source_path}" "${ROOTFS_DIR}/usr/bin/${target_name}"
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
  mkdir -p "${ROOTFS_DIR}/usr/share/velyx"
  cat > "${ROOTFS_DIR}/usr/share/velyx/version.txt" <<EOF
product=Velyx OS Preview
channel=preview
version=${version}
build_id=${version}
installed_at=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
source_root=${ROOT_DIR}
EOF
}

require_cmd install
require_cmd cp
require_cmd mkdir

if [[ -z "${BASE_ROOTFS}" ]]; then
  echo "VELYX_BASE_ROOTFS is required and must point to a minimal Linux rootfs with systemd." >&2
  exit 1
fi

if [[ ! -d "${BASE_ROOTFS}" ]]; then
  echo "VELYX_BASE_ROOTFS does not exist or is not a directory: ${BASE_ROOTFS}" >&2
  exit 1
fi

rm -rf "${ROOTFS_DIR}"
mkdir -p "${ROOTFS_DIR}"
copy_tree "${BASE_ROOTFS}" "${ROOTFS_DIR}"

mkdir -p \
  "${ROOTFS_DIR}/etc/velyx" \
  "${ROOTFS_DIR}/var/lib/velyx" \
  "${ROOTFS_DIR}/var/log/velyx" \
  "${ROOTFS_DIR}/usr/lib/velyx/boot" \
  "${ROOTFS_DIR}/usr/share/velyx/app-manifests" \
  "${ROOTFS_DIR}/usr/share/velyx/profiles" \
  "${ROOTFS_DIR}/home/${USER_NAME}/.config/velyx" \
  "${ROOTFS_DIR}/home/${USER_NAME}/.config/systemd/user" \
  "${ROOTFS_DIR}/home/${USER_NAME}/.local/state/velyx" \
  "${ROOTFS_DIR}/home/${USER_NAME}/.local/bin" \
  "${ROOTFS_DIR}/home/${USER_NAME}/.velyx"

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
  install -Dm755 "${VELYX_SHELL_BINARY}" "${ROOTFS_DIR}/usr/bin/velyx-shell"
elif resolve_binary_source "velyx-shell" >/dev/null 2>&1; then
  install -Dm755 "$(resolve_binary_source "velyx-shell")" "${ROOTFS_DIR}/usr/bin/velyx-shell"
else
  echo "missing shell binary: set VELYX_SHELL_BINARY or place velyx-shell in ${BIN_DIR}" >&2
  exit 1
fi

install -Dm755 "${ROOT_DIR}/scripts/velyx-user-session-bootstrap.sh" "${ROOTFS_DIR}/usr/lib/velyx/boot/velyx-user-session-bootstrap"
install -Dm755 "${ROOT_DIR}/scripts/velyx-firstboot-dispatch.sh" "${ROOTFS_DIR}/usr/lib/velyx/boot/velyx-firstboot-dispatch"
install -Dm755 "${ROOT_DIR}/scripts/velyx-system-session-bootstrap.sh" "${ROOTFS_DIR}/usr/lib/velyx/boot/velyx-system-session-bootstrap"
install -Dm755 "${ROOT_DIR}/scripts/velyx-recovery-bootstrap.sh" "${ROOTFS_DIR}/usr/lib/velyx/boot/velyx-recovery-bootstrap"

install_helper_script "${ROOT_DIR}/scripts/velyx-status" "velyx-status"
install_helper_script "${ROOT_DIR}/scripts/velyx-restart.sh" "velyx-restart.sh"
install_helper_script "${ROOT_DIR}/scripts/velyx-logs.sh" "velyx-logs.sh"
install_helper_script "${ROOT_DIR}/scripts/velyx-update" "velyx-update"
install_helper_script "${ROOT_DIR}/scripts/velyx-recovery" "velyx-recovery"
install_helper_script "${ROOT_DIR}/scripts/velyx-version" "velyx-version"
install_helper_script "${ROOT_DIR}/scripts/velyx-shell-watchdog" "velyx-shell-watchdog"
install_helper_script "${ROOT_DIR}/scripts/velyx-app" "velyx-app"
install_helper_script "${ROOT_DIR}/scripts/velyx-space" "velyx-space"
install_helper_script "${ROOT_DIR}/scripts/velyx-intent" "velyx-intent"
install_helper_script "${ROOT_DIR}/scripts/velyx-rule" "velyx-rule"
install_helper_script "${ROOT_DIR}/scripts/velyx-agent" "velyx-agent"
install_helper_script "${ROOT_DIR}/scripts/velyx-ai" "velyx-ai"
install_helper_script "${ROOT_DIR}/scripts/velyx-network" "velyx-network"
install_helper_script "${ROOT_DIR}/scripts/velyx-model" "velyx-model"
install_helper_script "${ROOT_DIR}/scripts/velyx-assistant" "velyx-assistant"
install_helper_script "${ROOT_DIR}/scripts/velyx-dev" "velyx-dev"
install_helper_script "${ROOT_DIR}/scripts/velyx-live" "velyx-live"
install_helper_script "${ROOT_DIR}/scripts/velyx-entry" "velyx-entry"
install_helper_script "${ROOT_DIR}/scripts/velyx-firstboot" "velyx-firstboot"
install_helper_script "${ROOT_DIR}/scripts/velyx-installer" "velyx-installer"
install_helper_script "${ROOT_DIR}/scripts/velyx-diagnostics" "velyx-diagnostics"
install_helper_script "${ROOT_DIR}/scripts/velyx-vm-profile" "velyx-vm-profile"
install_helper_script "${ROOT_DIR}/scripts/velyx-vm-preview" "velyx-vm-preview"

if [[ -f "${ROOT_DIR}/scripts/velyx-release-preview" ]]; then
  install_helper_script "${ROOT_DIR}/scripts/velyx-release-preview" "velyx-release-preview"
fi
if [[ -f "${ROOT_DIR}/scripts/velyx-validate-preview" ]]; then
  install_helper_script "${ROOT_DIR}/scripts/velyx-validate-preview" "velyx-validate-preview"
fi

copy_tree "${ROOT_DIR}/app-manifests" "${ROOTFS_DIR}/usr/share/velyx/app-manifests"
if [[ -d "${ROOT_DIR}/profiles" ]]; then
  copy_tree "${ROOT_DIR}/profiles" "${ROOTFS_DIR}/usr/share/velyx/profiles"
fi

"${ROOT_DIR}/scripts/install-units.sh" "${ROOTFS_DIR}" "${USER_NAME}"
write_version_metadata

cat > "${ROOTFS_DIR}/etc/velyx/boot-prototype.env" <<EOF
VELYX_BOOT_USER=${USER_NAME}
VELYX_STATE_LAYOUT=compat-home
VELYX_MANIFEST_DIR=/usr/share/velyx/app-manifests
VELYX_PROFILE_DIR=/usr/share/velyx/profiles
VELYX_PRODUCT=Velyx OS Preview
EOF

echo "rootfs prepared at ${ROOTFS_DIR}"
