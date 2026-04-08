#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BUILD_DIR="${ROOT_DIR}/build"
ROOTFS_DIR="${BUILD_DIR}/rootfs"
BASE_ROOTFS="${VELYX_BASE_ROOTFS:-}"
BIN_DIR="${VELYX_BIN_DIR:-${ROOT_DIR}/target/release}"
USER_NAME="${VELYX_BOOT_USER:-velyx}"
QT_RUNTIME_DEST="/home/${USER_NAME}/Qt"

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

copy_file_with_parents() {
  local source_path="$1"
  local target_path="${ROOTFS_DIR}${source_path}"
  mkdir -p "$(dirname "${target_path}")"
  cp -a "${source_path}" "${target_path}"
}

detect_qt_prefix() {
  local shell_path="$1"
  local core_path=""
  core_path="$(ldd "${shell_path}" 2>/dev/null | awk '/libQt6Core\.so\.6/ {print $3; exit}')"
  if [[ -n "${core_path}" ]]; then
    dirname "${core_path}"
    return 0
  fi

  return 1
}

copy_binary_dependencies() {
  local binary_path="$1"
  ldd "${binary_path}" 2>/dev/null | awk '/=> \// {print $3} /^\/lib/ {print $1}' | while read -r library; do
    [[ -n "${library}" && -f "${library}" ]] || continue
    copy_file_with_parents "${library}"
  done
}

install_qt_runtime() {
  local shell_path="$1"
  local qt_lib_dir=""
  qt_lib_dir="$(detect_qt_prefix "${shell_path}")" || return 0

  local qt_prefix
  qt_prefix="$(dirname "${qt_lib_dir}")"
  local qt_rootfs_prefix="${ROOTFS_DIR}${qt_prefix}"
  mkdir -p "$(dirname "${qt_rootfs_prefix}")"
  rm -rf "${qt_rootfs_prefix}"
  mkdir -p "${qt_rootfs_prefix}"
  copy_tree "${qt_prefix}" "${qt_rootfs_prefix}"
}

detect_qml_build_root() {
  local shell_path="$1"
  local shell_dir=""
  shell_dir="$(cd "$(dirname "${shell_path}")" && pwd)"
  local candidate="${shell_dir%/apps/shell}"
  if [[ -d "${candidate}/packages/design-system" ]]; then
    printf '%s\n' "${candidate}"
    return 0
  fi
  return 1
}

install_project_qml_module() {
  local source_dir="$1"
  local module_rel_path="$2"
  local qmltypes_name="$3"
  local target_dir="${ROOTFS_DIR}${QT_RUNTIME_DEST}/6.8.0/gcc_64/qml/${module_rel_path}"

  mkdir -p "${target_dir}"
  cp -a "${source_dir}/qml/${module_rel_path}/." "${target_dir}/"
  if [[ -n "${qmltypes_name}" && -f "${source_dir}/${qmltypes_name}" ]]; then
    cp -a "${source_dir}/${qmltypes_name}" "${target_dir}/${qmltypes_name}"
  fi
}

install_project_qml_modules() {
  local shell_path="$1"
  local qml_build_root=""
  qml_build_root="$(detect_qml_build_root "${shell_path}")" || return 0

  install_project_qml_module "${qml_build_root}/packages/design-system" "Velyx/DesignSystem" "VelyxDesignSystem.qmltypes"
  install_project_qml_module "${qml_build_root}/packages/core-ui" "Velyx/UI" "VelyxCoreUi.qmltypes"
}

write_user_env() {
  cat > "${ROOTFS_DIR}/home/${USER_NAME}/.config/velyx/velyx.env" <<EOF
VELYX_INSTALL_PREFIX=/usr
VELYX_STATE_DIR=/home/${USER_NAME}/.velyx
VELYX_MANIFESTS_DIR=/usr/share/velyx/app-manifests
VELYX_QT_PREFIX=${QT_RUNTIME_DEST}/6.8.0/gcc_64
VELYX_SESSION_BACKEND=auto
QT_PLUGIN_PATH=${QT_RUNTIME_DEST}/6.8.0/gcc_64/plugins
QML2_IMPORT_PATH=${QT_RUNTIME_DEST}/6.8.0/gcc_64/qml
QML_IMPORT_PATH=${QT_RUNTIME_DEST}/6.8.0/gcc_64/qml
VELYX_SHELL_DEBUG=1
EOF
}

fix_runtime_ownership() {
  chown -R 0:0 \
    "${ROOTFS_DIR}/etc" \
    "${ROOTFS_DIR}/usr" \
    "${ROOTFS_DIR}/var"

  if [[ -f "${ROOTFS_DIR}/etc/sudo.conf" ]]; then
    chown 0:0 "${ROOTFS_DIR}/etc/sudo.conf"
    chmod 0644 "${ROOTFS_DIR}/etc/sudo.conf"
  fi
  if [[ -f "${ROOTFS_DIR}/usr/bin/sudo" ]]; then
    chown 0:0 "${ROOTFS_DIR}/usr/bin/sudo"
    chmod 4755 "${ROOTFS_DIR}/usr/bin/sudo"
  fi
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

SHELL_BINARY_PATH="${VELYX_SHELL_BINARY:-$(resolve_binary_source "velyx-shell")}"
copy_binary_dependencies "${SHELL_BINARY_PATH}"
install_qt_runtime "${SHELL_BINARY_PATH}"
install_project_qml_modules "${SHELL_BINARY_PATH}"

install -Dm755 "${ROOT_DIR}/scripts/velyx-user-session-bootstrap.sh" "${ROOTFS_DIR}/usr/lib/velyx/boot/velyx-user-session-bootstrap"
install -Dm755 "${ROOT_DIR}/scripts/velyx-firstboot-dispatch.sh" "${ROOTFS_DIR}/usr/lib/velyx/boot/velyx-firstboot-dispatch"
install -Dm755 "${ROOT_DIR}/scripts/velyx-system-session-bootstrap.sh" "${ROOTFS_DIR}/usr/lib/velyx/boot/velyx-system-session-bootstrap"
install -Dm755 "${ROOT_DIR}/scripts/velyx-recovery-bootstrap.sh" "${ROOTFS_DIR}/usr/lib/velyx/boot/velyx-recovery-bootstrap"
install -Dm755 "${ROOT_DIR}/scripts/velyx-shell-session-launch.sh" "${ROOTFS_DIR}/usr/lib/velyx/boot/velyx-shell-session-launch"
install -Dm755 "${ROOT_DIR}/scripts/velyx-primary-session-launch.sh" "${ROOTFS_DIR}/usr/lib/velyx/boot/velyx-primary-session-launch"

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

if [[ -f "${ROOTFS_DIR}/etc/pam.d/login" ]]; then
  sed -i '/pam_lastlog\.so/d' "${ROOTFS_DIR}/etc/pam.d/login"
fi
write_version_metadata
write_user_env
fix_runtime_ownership

cat > "${ROOTFS_DIR}/etc/velyx/boot-prototype.env" <<EOF
VELYX_BOOT_USER=${USER_NAME}
VELYX_STATE_LAYOUT=compat-home
VELYX_MANIFEST_DIR=/usr/share/velyx/app-manifests
VELYX_PROFILE_DIR=/usr/share/velyx/profiles
VELYX_PRODUCT=Velyx OS Preview
EOF

echo "rootfs prepared at ${ROOTFS_DIR}"
