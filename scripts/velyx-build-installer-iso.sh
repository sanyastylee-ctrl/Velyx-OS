#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DIST_DIR="${ROOT_DIR}/dist"
BUILD_DIR="${ROOT_DIR}/build/installer-iso"
ARTIFACT_DIR=""
ISO_LABEL="${VELYX_ISO_LABEL:-VELYX_INSTALLER}"
OUTPUT_ISO="${DIST_DIR}/velyx-os-installer.iso"
LIVE_CMDLINE_EXTRA="${VELYX_LIVE_CMDLINE_EXTRA:-}"

usage() {
  cat <<'EOF'
Usage:
  scripts/velyx-build-installer-iso.sh [--artifact-dir DIR] [--output ISO]

Builds a bootable UEFI Velyx installer ISO using the latest preview artifact.
EOF
}

fail() {
  printf 'error: %s\n' "$*" >&2
  exit 1
}

require_cmd() {
  command -v "$1" >/dev/null 2>&1 || fail "missing required command: $1"
}

latest_artifact_dir() {
  find "${DIST_DIR}" -maxdepth 1 -mindepth 1 -type d -name 'velyx-os-preview-*' | sort | tail -n1
}

artifact_img() {
  local dir="$1"
  find "${dir}" -maxdepth 1 -type f -name 'velyx-os-preview-*.img' | sort | head -n1
}

cleanup() {
  if [[ -n "${WORK_DIR:-}" && -d "${WORK_DIR}" ]]; then
    rm -rf "${WORK_DIR}"
  fi
}

trap cleanup EXIT

while [[ $# -gt 0 ]]; do
  case "$1" in
    --artifact-dir)
      ARTIFACT_DIR="$2"
      shift 2
      ;;
    --output)
      OUTPUT_ISO="$2"
      shift 2
      ;;
    --help)
      usage
      exit 0
      ;;
    *)
      fail "unknown argument: $1"
      ;;
  esac
done

require_cmd grub-mkrescue
require_cmd xorriso
require_cmd unmkinitramfs
require_cmd mksquashfs
require_cmd cpio
require_cmd gzip
require_cmd python3
require_cmd zstd

[[ -n "${ARTIFACT_DIR}" ]] || ARTIFACT_DIR="$(latest_artifact_dir)"
[[ -n "${ARTIFACT_DIR}" ]] || fail "no preview artifact dir found under ${DIST_DIR}"
ARTIFACT_DIR="$(readlink -f "${ARTIFACT_DIR}")"
[[ -d "${ARTIFACT_DIR}" ]] || fail "artifact dir not found: ${ARTIFACT_DIR}"

SOURCE_IMAGE="$(artifact_img "${ARTIFACT_DIR}")"
[[ -f "${SOURCE_IMAGE}" ]] || fail "preview root image not found in ${ARTIFACT_DIR}"
KERNEL_IMAGE="${ARTIFACT_DIR}/vmlinuz"
INITRD_IMAGE="${ARTIFACT_DIR}/initrd.img"
[[ -f "${KERNEL_IMAGE}" ]] || fail "missing kernel image: ${KERNEL_IMAGE}"
[[ -f "${INITRD_IMAGE}" ]] || fail "missing initrd image: ${INITRD_IMAGE}"

mkdir -p "${BUILD_DIR}" "$(dirname "${OUTPUT_ISO}")"
WORK_DIR="$(mktemp -d "${BUILD_DIR}/work.XXXXXX")"
ISO_ROOT="${WORK_DIR}/iso-root"
INITRD_UNPACK="${WORK_DIR}/initrd-unpacked"
INITRD_STAGING="${WORK_DIR}/initrd-staging"
mkdir -p "${ISO_ROOT}/boot/grub" "${ISO_ROOT}/live" "${INITRD_STAGING}"

cp -a "${SOURCE_IMAGE}" "${ISO_ROOT}/live/filesystem.img"
cp -a "${KERNEL_IMAGE}" "${ISO_ROOT}/live/vmlinuz"

unmkinitramfs "${INITRD_IMAGE}" "${INITRD_UNPACK}" >/dev/null

for part_dir in "${INITRD_UNPACK}"/*; do
  [[ -d "${part_dir}" ]] || continue
  cp -a "${part_dir}/." "${INITRD_STAGING}/"
done

MODULES_DIR="$(find "${INITRD_STAGING}/usr/lib/modules" -mindepth 1 -maxdepth 1 -type d | head -n1 || true)"
if [[ -n "${MODULES_DIR}" ]]; then
  KERNEL_RELEASE="$(basename "${MODULES_DIR}")"
  OVERLAY_MODULE_SRC="/lib/modules/${KERNEL_RELEASE}/kernel/fs/overlayfs/overlay.ko.zst"
  OVERLAY_MODULE_DST="${MODULES_DIR}/kernel/fs/overlayfs/overlay.ko"
  if [[ -f "${OVERLAY_MODULE_SRC}" && ! -f "${OVERLAY_MODULE_DST}" ]]; then
    mkdir -p "$(dirname "${OVERLAY_MODULE_DST}")"
    zstd -d -q -c "${OVERLAY_MODULE_SRC}" > "${OVERLAY_MODULE_DST}"
  fi
fi

python3 - <<'PY' "${INITRD_STAGING}/scripts/local" "${ISO_LABEL}"
from pathlib import Path
import sys

path = Path(sys.argv[1])
iso_label = sys.argv[2]
text = path.read_text()
needle = "local_mount_root()\n{"
if needle not in text:
    raise SystemExit("local_mount_root() not found in initrd scripts/local")
replacement = """local_mount_root()\n{\n\tif grep -Eq '(^|[[:space:]])velyx_live=1($|[[:space:]])' /proc/cmdline 2>/dev/null; then\n\t\tmount_live_root || panic \"Unable to mount Velyx live root\"\n\t\treturn 0\n\tfi"""
text = text.replace(needle, replacement, 1)
append = f"""

mount_live_root()
{{
\tmodprobe overlay >/dev/null 2>&1 || insmod /usr/lib/modules/*/kernel/fs/overlayfs/overlay.ko >/dev/null 2>&1 || true
\tmodprobe isofs >/dev/null 2>&1 || true

\tlocal_device_setup "LABEL={iso_label}" "Velyx installer media"
\tlocal medium_dev="${{DEV}}"
\tlocal medium_mnt="/run/velyx-iso"
\tlocal lower_mnt="/run/velyx-lower"
\tlocal rw_mnt="/run/velyx-rw"

\tmkdir -p "${{medium_mnt}}" "${{lower_mnt}}" "${{rw_mnt}}" "${{rootmnt}}"
\tmount -o ro "${{medium_dev}}" "${{medium_mnt}}" || mount -t iso9660 -o ro "${{medium_dev}}" "${{medium_mnt}}"
\t[ -f "${{medium_mnt}}/live/filesystem.img" ] || return 1
\tmount -t ext4 -o loop,ro "${{medium_mnt}}/live/filesystem.img" "${{lower_mnt}}"
\tmount -t tmpfs tmpfs "${{rw_mnt}}"
\tmkdir -p "${{rw_mnt}}/upper" "${{rw_mnt}}/work"
\tmount -t overlay overlay -o lowerdir="${{lower_mnt}}",upperdir="${{rw_mnt}}/upper",workdir="${{rw_mnt}}/work" "${{rootmnt}}"

\tmkdir -p "${{rootmnt}}/run/live/medium" "${{rootmnt}}/usr/local/bin" "${{rootmnt}}/etc/profile.d" "${{rootmnt}}/etc/sudoers.d" "${{rootmnt}}/etc/systemd/system/serial-getty@ttyS0.service.d"
\tmount --bind "${{medium_mnt}}" "${{rootmnt}}/run/live/medium"
\tcat > "${{rootmnt}}/usr/local/bin/velyx-install" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
ARTIFACT_DIR="/run/live/medium/live"
if [[ "${{EUID}}" -ne 0 ]]; then
\texec sudo /usr/bin/velyx-baremetal-install.sh --artifact-dir "${{ARTIFACT_DIR}}" "$@"
fi
exec /usr/bin/velyx-baremetal-install.sh --artifact-dir "${{ARTIFACT_DIR}}" "$@"
EOF
\tchmod 0755 "${{rootmnt}}/usr/local/bin/velyx-install"
\tcat > "${{rootmnt}}/etc/sudoers.d/90-velyx-live" <<'EOF'
velyx ALL=(ALL) NOPASSWD:ALL
EOF
\tchmod 0440 "${{rootmnt}}/etc/sudoers.d/90-velyx-live"
\tcat > "${{rootmnt}}/etc/systemd/system/serial-getty@ttyS0.service.d/autologin.conf" <<'EOF'
[Service]
ExecStart=
ExecStart=-/sbin/agetty --autologin velyx --keep-baud 115200,38400,9600 %I $TERM
EOF
\tcat > "${{rootmnt}}/etc/profile.d/velyx-live.sh" <<'EOF'
export VELYX_LIVE_MODE=1
export VELYX_INSTALLER_ARTIFACT_DIR=/run/live/medium/live
echo "Velyx Live environment ready."
echo "Install command: velyx-install --target-disk /dev/sdX --yes-wipe"
EOF
\treturn 0
}}
"""
path.write_text(text + append)
PY

chmod 0755 "${INITRD_STAGING}/scripts/local"

(
  cd "${INITRD_STAGING}"
  find . -print0 | cpio --null -o -H newc 2>/dev/null | gzip -9 > "${ISO_ROOT}/live/initrd.img"
)

cat > "${ISO_ROOT}/boot/grub/grub.cfg" <<EOF
set default=0
set timeout=1

menuentry 'Velyx OS Installer (Live)' {
    linux /live/vmlinuz velyx_live=1 noapic console=tty1 console=ttyS0 systemd.journald.forward_to_console=1 systemd.log_target=console ${LIVE_CMDLINE_EXTRA}
    initrd /live/initrd.img
}
EOF

grub-mkrescue \
  -o "${OUTPUT_ISO}" \
  "${ISO_ROOT}" \
  -volid "${ISO_LABEL}" \
  -iso-level 3 \
  >/dev/null

python3 - <<'PY' "${OUTPUT_ISO}" "${ARTIFACT_DIR}"
from pathlib import Path
import json
import sys

iso_path = Path(sys.argv[1])
artifact_dir = Path(sys.argv[2])
manifest = {
    "product": "Velyx OS Installer ISO",
    "channel": "preview",
    "iso": iso_path.name,
    "source_artifact": artifact_dir.name,
    "uefi_boot": True,
    "live_mode": True,
}
Path(str(iso_path) + ".json").write_text(json.dumps(manifest, indent=2) + "\n")
PY

printf 'Built installer ISO:\n%s\n' "${OUTPUT_ISO}"
