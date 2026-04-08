#!/usr/bin/env bash
set -euo pipefail

HOME_DIR="${HOME:-/home/velyx}"
USER_ID="$(id -u)"
QT_PREFIX="${VELYX_QT_PREFIX:-${HOME_DIR}/Qt/6.8.0/gcc_64}"
ENV_FILE="${HOME_DIR}/.config/velyx/velyx.env"
STATE_DIR="${VELYX_STATE_DIR:-${HOME_DIR}/.velyx}"
INSTALL_PREFIX="${VELYX_INSTALL_PREFIX:-${HOME_DIR}/.local/share/velyx}"
BIN_PREFIX="${INSTALL_PREFIX}/bin"
LOG_FILE="${STATE_DIR}/shell-session-launch.log"

log() {
  local message="$1"
  local timestamp
  timestamp="$(date -Is)"
  mkdir -p "${STATE_DIR}"
  printf '%s %s\n' "${timestamp}" "${message}" | tee -a "${LOG_FILE}" >&2
}

if [[ -f "${ENV_FILE}" ]]; then
  set -a
  # shellcheck disable=SC1090
  source "${ENV_FILE}"
  set +a
fi

export HOME="${HOME_DIR}"
export USER="${USER:-$(id -un)}"
export SHELL="${SHELL:-/bin/bash}"
export PATH="${BIN_PREFIX}:${PATH}"
export XDG_RUNTIME_DIR="${XDG_RUNTIME_DIR:-/run/user/${USER_ID}}"
export XDG_SESSION_CLASS="${XDG_SESSION_CLASS:-user}"
export XDG_SESSION_TYPE="${XDG_SESSION_TYPE:-tty}"
export XDG_CURRENT_DESKTOP="${XDG_CURRENT_DESKTOP:-Velyx}"
export XDG_SESSION_DESKTOP="${XDG_SESSION_DESKTOP:-Velyx}"
export DESKTOP_SESSION="${DESKTOP_SESSION:-velyx}"
export VELYX_SESSION_BACKEND="${VELYX_SESSION_BACKEND:-auto}"
export VELYX_GRAPHICS_MODE="${VELYX_GRAPHICS_MODE:-auto}"
export VELYX_GRAPHICS_MODE_REQUESTED="${VELYX_GRAPHICS_MODE}"
export VELYX_GRAPHICS_MODE_ACTIVE="${VELYX_GRAPHICS_MODE_ACTIVE:-unset}"
export VELYX_GRAPHICS_FALLBACK_OCCURRED="${VELYX_GRAPHICS_FALLBACK_OCCURRED:-0}"
export VELYX_GRAPHICS_FALLBACK_REASON="${VELYX_GRAPHICS_FALLBACK_REASON:-}"
export VELYX_ENVIRONMENT_MODE="${VELYX_ENVIRONMENT_MODE:-auto}"
export VELYX_CURSOR_SIZE="${VELYX_CURSOR_SIZE:-24}"
export QT_ENABLE_HIGHDPI_SCALING="${QT_ENABLE_HIGHDPI_SCALING:-1}"
export QT_SCALE_FACTOR_ROUNDING_POLICY="${QT_SCALE_FACTOR_ROUNDING_POLICY:-PassThrough}"

if [[ -S "${XDG_RUNTIME_DIR}/bus" ]]; then
  export DBUS_SESSION_BUS_ADDRESS="${DBUS_SESSION_BUS_ADDRESS:-unix:path=${XDG_RUNTIME_DIR}/bus}"
fi

if [[ -d "${QT_PREFIX}" ]]; then
  export QT_PLUGIN_PATH="${QT_PLUGIN_PATH:-${QT_PREFIX}/plugins}"
  export QML2_IMPORT_PATH="${QML2_IMPORT_PATH:-${QT_PREFIX}/qml}"
  export QML_IMPORT_PATH="${QML_IMPORT_PATH:-${QT_PREFIX}/qml}"
  export LD_LIBRARY_PATH="${QT_PREFIX}/lib${LD_LIBRARY_PATH:+:${LD_LIBRARY_PATH}}"
fi

if [[ "${VELYX_SHELL_DEBUG:-0}" == "1" ]]; then
  export QT_DEBUG_PLUGINS="${QT_DEBUG_PLUGINS:-1}"
  export QML_IMPORT_TRACE="${QML_IMPORT_TRACE:-1}"
  export QSG_INFO="${QSG_INFO:-1}"
  export QT_LOGGING_RULES="${QT_LOGGING_RULES:-qt.qml.binding.removal.info=true;qt.qml.imports=true;qevdevmouseplugin.debug=true;qevdevkeyboardplugin.debug=true;qt.scenegraph.general=true}"
fi

validate_graphics_mode() {
  case "${VELYX_GRAPHICS_MODE}" in
    auto|gpu|software)
      ;;
    *)
      log "shell-session-launch invalid graphics mode '${VELYX_GRAPHICS_MODE}', using auto"
      export VELYX_GRAPHICS_MODE="auto"
      export VELYX_GRAPHICS_MODE_REQUESTED="auto"
      ;;
  esac
}

detect_environment_mode() {
  if [[ "${VELYX_ENVIRONMENT_MODE}" == "auto" ]]; then
    if command -v systemd-detect-virt >/dev/null 2>&1 && systemd-detect-virt --quiet; then
      export VELYX_ENVIRONMENT_MODE="vm"
    else
      export VELYX_ENVIRONMENT_MODE="bare-metal"
    fi
  fi
}

prepare_linuxfb_env() {
  export XDG_SESSION_TYPE="tty"
  export VELYX_GRAPHICS_MODE_ACTIVE="software"
  export QT_QPA_PLATFORM="${QT_QPA_PLATFORM:-linuxfb}"
  export QT_QUICK_BACKEND="${QT_QUICK_BACKEND:-software}"
  export QSG_RHI_BACKEND="${QSG_RHI_BACKEND:-software}"
  export QT_OPENGL="${QT_OPENGL:-software}"
  export QT_QPA_FB_FORCE_FULLSCREEN="${QT_QPA_FB_FORCE_FULLSCREEN:-1}"
  export QT_QPA_FB_HIDECURSOR="${QT_QPA_FB_HIDECURSOR:-0}"
  export QT_QPA_GENERIC_PLUGINS="${QT_QPA_GENERIC_PLUGINS:-evdevmouse,evdevkeyboard}"
}

prepare_x11_env_common() {
  export XDG_SESSION_TYPE="x11"
  export QT_QPA_PLATFORM="xcb"
  unset QT_QPA_FB_FORCE_FULLSCREEN
  unset QT_QPA_FB_HIDECURSOR
  unset QT_QPA_GENERIC_PLUGINS
}

apply_x11_gpu_env() {
  export VELYX_GRAPHICS_MODE_ACTIVE="gpu"
  export QSG_RHI_BACKEND="${QSG_RHI_BACKEND:-opengl}"
  export QT_XCB_GL_INTEGRATION="${QT_XCB_GL_INTEGRATION:-glx}"
  unset QT_QUICK_BACKEND
  unset LIBGL_ALWAYS_SOFTWARE
  unset QT_OPENGL
}

apply_x11_software_env() {
  export VELYX_GRAPHICS_MODE_ACTIVE="software"
  export QT_QUICK_BACKEND="software"
  export QSG_RHI_BACKEND="software"
  export LIBGL_ALWAYS_SOFTWARE="1"
  export QT_XCB_GL_INTEGRATION="none"
  export QT_OPENGL="software"
}

create_x11_client_script() {
  local client_script="$1"
  local entry_binary="$2"

  cat > "${client_script}" <<EOF
#!/usr/bin/env bash
set -euo pipefail
export HOME='${HOME}'
export USER='${USER}'
export SHELL='${SHELL}'
export XDG_RUNTIME_DIR='${XDG_RUNTIME_DIR}'
export XDG_SESSION_CLASS='user'
export XDG_SESSION_TYPE='${XDG_SESSION_TYPE}'
export XDG_CURRENT_DESKTOP='${XDG_CURRENT_DESKTOP}'
export XDG_SESSION_DESKTOP='${XDG_SESSION_DESKTOP}'
export DESKTOP_SESSION='${DESKTOP_SESSION}'
export QT_PLUGIN_PATH='${QT_PLUGIN_PATH:-}'
export QML2_IMPORT_PATH='${QML2_IMPORT_PATH:-}'
export QML_IMPORT_PATH='${QML_IMPORT_PATH:-}'
export LD_LIBRARY_PATH='${LD_LIBRARY_PATH:-}'
export QT_QPA_PLATFORM='${QT_QPA_PLATFORM:-}'
export QT_QUICK_BACKEND='${QT_QUICK_BACKEND:-}'
export QSG_RHI_BACKEND='${QSG_RHI_BACKEND:-}'
export QT_XCB_GL_INTEGRATION='${QT_XCB_GL_INTEGRATION:-}'
export LIBGL_ALWAYS_SOFTWARE='${LIBGL_ALWAYS_SOFTWARE:-}'
export QT_OPENGL='${QT_OPENGL:-}'
export VELYX_GRAPHICS_MODE='${VELYX_GRAPHICS_MODE}'
export VELYX_GRAPHICS_MODE_REQUESTED='${VELYX_GRAPHICS_MODE_REQUESTED}'
export VELYX_GRAPHICS_MODE_ACTIVE='${VELYX_GRAPHICS_MODE_ACTIVE}'
export VELYX_GRAPHICS_FALLBACK_OCCURRED='${VELYX_GRAPHICS_FALLBACK_OCCURRED}'
export VELYX_GRAPHICS_FALLBACK_REASON='${VELYX_GRAPHICS_FALLBACK_REASON}'
export VELYX_ENVIRONMENT_MODE='${VELYX_ENVIRONMENT_MODE}'
export VELYX_CURSOR_SIZE='${VELYX_CURSOR_SIZE}'
export QT_ENABLE_HIGHDPI_SCALING='${QT_ENABLE_HIGHDPI_SCALING}'
export QT_SCALE_FACTOR_ROUNDING_POLICY='${QT_SCALE_FACTOR_ROUNDING_POLICY}'
if [[ -n '${DBUS_SESSION_BUS_ADDRESS:-}' ]]; then
  export DBUS_SESSION_BUS_ADDRESS='${DBUS_SESSION_BUS_ADDRESS}'
elif [[ -S '${XDG_RUNTIME_DIR}/bus' ]]; then
  export DBUS_SESSION_BUS_ADDRESS='unix:path=${XDG_RUNTIME_DIR}/bus'
fi
exec '${entry_binary}' --session
EOF
  chmod 0755 "${client_script}"
}

run_linuxfb_session() {
  local entry_binary="$1"
  prepare_linuxfb_env
  log "shell-session-launch backend=linuxfb graphics=${VELYX_GRAPHICS_MODE_ACTIVE} QT_QPA_PLATFORM=${QT_QPA_PLATFORM} QT_QUICK_BACKEND=${QT_QUICK_BACKEND} QSG_RHI_BACKEND=${QSG_RHI_BACKEND} QT_QPA_GENERIC_PLUGINS=${QT_QPA_GENERIC_PLUGINS}"
  exec "${entry_binary}" --session
}

run_x11_session() {
  local graphics_mode="$1"
  local entry_binary="$2"
  local xinit_bin
  local xorg_bin
  local client_script
  local started_at
  local rc
  local elapsed

  xinit_bin="$(command -v xinit || true)"
  xorg_bin="$(command -v Xorg || true)"
  if [[ -z "${xinit_bin}" || -z "${xorg_bin}" ]]; then
    return 1
  fi

  prepare_x11_env_common
  if [[ "${graphics_mode}" == "gpu" ]]; then
    apply_x11_gpu_env
  else
    apply_x11_software_env
  fi

  client_script="${STATE_DIR}/xsession-client-${graphics_mode}.sh"
  create_x11_client_script "${client_script}" "${entry_binary}"

  log "shell-session-launch backend=x11 graphics=${VELYX_GRAPHICS_MODE_ACTIVE} requested=${VELYX_GRAPHICS_MODE_REQUESTED} xinit=${xinit_bin} xorg=${xorg_bin} QT_QPA_PLATFORM=${QT_QPA_PLATFORM} QT_QUICK_BACKEND=${QT_QUICK_BACKEND:-unset} QSG_RHI_BACKEND=${QSG_RHI_BACKEND:-unset} QT_XCB_GL_INTEGRATION=${QT_XCB_GL_INTEGRATION:-unset} LIBGL_ALWAYS_SOFTWARE=${LIBGL_ALWAYS_SOFTWARE:-unset} QT_OPENGL=${QT_OPENGL:-unset}"
  started_at="$(date +%s)"
  set +e
  "${xinit_bin}" "${client_script}" -- "${xorg_bin}" :0 vt1 -keeptty -nolisten tcp -verbose 2
  rc="$?"
  set -e
  elapsed="$(( $(date +%s) - started_at ))"
  log "shell-session-launch x11 exit graphics=${graphics_mode} rc=${rc} elapsed=${elapsed}s"
  return "${rc}"
}

validate_graphics_mode
detect_environment_mode

log "shell-session-launch start uid=${USER_ID} user=${USER} tty=$(tty 2>/dev/null || echo unknown)"
log "shell-session-launch env HOME=${HOME} SHELL=${SHELL} XDG_RUNTIME_DIR=${XDG_RUNTIME_DIR} XDG_SESSION_TYPE=${XDG_SESSION_TYPE} DBUS_SESSION_BUS_ADDRESS=${DBUS_SESSION_BUS_ADDRESS:-unset}"
log "shell-session-launch qt QT_PREFIX=${QT_PREFIX} session_backend=${VELYX_SESSION_BACKEND} env_mode=${VELYX_ENVIRONMENT_MODE} graphics_requested=${VELYX_GRAPHICS_MODE_REQUESTED} cursor_size=${VELYX_CURSOR_SIZE} QT_QPA_PLATFORM=${QT_QPA_PLATFORM:-unset} QT_QPA_GENERIC_PLUGINS=${QT_QPA_GENERIC_PLUGINS:-unset} QT_QPA_FB_HIDECURSOR=${QT_QPA_FB_HIDECURSOR:-unset} QT_PLUGIN_PATH=${QT_PLUGIN_PATH:-unset} QML2_IMPORT_PATH=${QML2_IMPORT_PATH:-unset}"
ENTRY_BINARY="${VELYX_ENTRY_BINARY:-$(command -v velyx-entry || true)}"
[[ -n "${ENTRY_BINARY}" ]] || fail "velyx-entry not found in PATH or VELYX_ENTRY_BINARY"
log "shell-session-launch paths shell=$(command -v velyx-shell || echo missing) entry=${ENTRY_BINARY} fb0=$([[ -e /dev/fb0 ]] && echo yes || echo no) tty1=$([[ -e /dev/tty1 ]] && echo yes || echo no) dri_render=$([[ -e /dev/dri/renderD128 ]] && echo yes || echo no)"

if [[ -z "${DISPLAY:-}" && "${VELYX_SESSION_BACKEND}" != "linuxfb" ]]; then
  case "${VELYX_GRAPHICS_MODE}" in
    gpu)
      run_x11_session gpu "${ENTRY_BINARY}"
      ;;
    software)
      run_x11_session software "${ENTRY_BINARY}" || log "shell-session-launch x11 software startup unavailable; falling back to linuxfb software path"
      ;;
    auto)
      if run_x11_session gpu "${ENTRY_BINARY}"; then
        exit 0
      fi
      export VELYX_GRAPHICS_FALLBACK_OCCURRED="1"
      export VELYX_GRAPHICS_FALLBACK_REASON="gpu_start_failed"
      log "shell-session-launch auto graphics fallback reason=${VELYX_GRAPHICS_FALLBACK_REASON}"
      run_x11_session software "${ENTRY_BINARY}" || log "shell-session-launch x11 software fallback unavailable; falling back to linuxfb software path"
      ;;
  esac
fi

run_linuxfb_session "${ENTRY_BINARY}"
