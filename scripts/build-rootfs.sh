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

install_binary() {
  local source_name="$1"
  local target_name="$2"
  local source_path=""
  local target_path="${ROOTFS_DIR}/usr/bin/${target_name}"
  if [[ -f "${BIN_DIR}/${source_name}" ]]; then
    source_path="${BIN_DIR}/${source_name}"
  elif [[ -f "${BIN_DIR}/${source_name%.exe}" ]]; then
    source_path="${BIN_DIR}/${source_name%.exe}"
  fi
  if [[ -z "${source_path}" ]]; then
    echo "missing binary: ${BIN_DIR}/${source_name} (or without .exe)" >&2
    exit 1
  fi
  install -Dm755 "${source_path}" "${target_path}"
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
  "${ROOTFS_DIR}/home/${USER_NAME}/.config/velyx" \
  "${ROOTFS_DIR}/home/${USER_NAME}/.local/state/velyx" \
  "${ROOTFS_DIR}/home/${USER_NAME}/.velyx"

install_binary "session-manager-service.exe" "velyx-session-manager"
install_binary "settings-service.exe" "velyx-settings-service"
install_binary "permissions-service.exe" "velyx-permissions-service"
install_binary "launcher-service.exe" "velyx-launcher-service"
install_binary "diagnostics-service.exe" "velyx-diagnostics-service"
install_binary "ai-service.exe" "velyx-ai-service"
install_binary "file-service.exe" "velyx-file-service"
install_binary "update-engine.exe" "velyx-update-engine"
install_binary "recovery-service.exe" "velyx-recovery-service"
install_binary "installer-service.exe" "velyx-installer-service"

if [[ -z "${VELYX_SHELL_BINARY:-}" ]]; then
  echo "VELYX_SHELL_BINARY is required and must point to the shell executable." >&2
  exit 1
fi
install -Dm755 "${VELYX_SHELL_BINARY}" "${ROOTFS_DIR}/usr/bin/velyx-shell"

copy_tree "${ROOT_DIR}/app-manifests" "${ROOTFS_DIR}/usr/share/velyx/app-manifests"
install -Dm755 "${ROOT_DIR}/scripts/velyx-firstboot-dispatch.sh" "${ROOTFS_DIR}/usr/lib/velyx/boot/velyx-firstboot-dispatch"
install -Dm755 "${ROOT_DIR}/scripts/velyx-system-session-bootstrap.sh" "${ROOTFS_DIR}/usr/lib/velyx/boot/velyx-system-session-bootstrap"
install -Dm755 "${ROOT_DIR}/scripts/velyx-user-session-bootstrap.sh" "${ROOTFS_DIR}/usr/lib/velyx/boot/velyx-user-session-bootstrap"

"${ROOT_DIR}/scripts/install-units.sh" "${ROOTFS_DIR}" "${USER_NAME}"

cat > "${ROOTFS_DIR}/etc/velyx/boot-prototype.env" <<EOF
VELYX_BOOT_USER=${USER_NAME}
VELYX_STATE_LAYOUT=compat-home
VELYX_MANIFEST_DIR=/usr/share/velyx/app-manifests
EOF

echo "rootfs prepared at ${ROOTFS_DIR}"
