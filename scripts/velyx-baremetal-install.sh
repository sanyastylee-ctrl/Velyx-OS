#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
STATE_DIR="${VELYX_STATE_DIR:-$HOME/.velyx}"
LOG_FILE="${STATE_DIR}/baremetal-install.log"

TARGET_DISK=""
ARTIFACT_DIR=""
SOURCE_IMAGE=""
KERNEL_IMAGE=""
INITRD_IMAGE=""
MOUNT_ROOT="${VELYX_INSTALL_MOUNT_ROOT:-/mnt/velyx-root}"
SOURCE_MOUNT=""
YES_WIPE=0

log() {
  local timestamp
  timestamp="$(date -u +"%Y-%m-%dT%H:%M:%SZ")"
  mkdir -p "${STATE_DIR}"
  printf '%s %s\n' "${timestamp}" "$*" | tee -a "${LOG_FILE}" >&2
}

fail() {
  log "error: $*"
  exit 1
}

usage() {
  cat <<'EOF'
Usage:
  velyx-baremetal-install.sh --target-disk /dev/sdX --yes-wipe [options]

Options:
  --target-disk PATH     Required target block device, for example /dev/sda or /dev/nvme0n1.
  --artifact-dir DIR     Preview artifact directory containing .img, vmlinuz, initrd.img.
  --source-image PATH    Rootfs ext4 image to copy to the target root partition.
  --kernel PATH          Kernel image to install into /boot/vmlinuz.
  --initrd PATH          Initrd image to install into /boot/initrd.img.
  --mount-root DIR       Temporary root mount path. Default: /mnt/velyx-root
  --yes-wipe             Required confirmation flag for destructive partitioning.
  --help                 Show this help.
EOF
}

require_cmd() {
  command -v "$1" >/dev/null 2>&1 || fail "missing required command: $1"
}

cleanup() {
  set +e
  if [[ -n "${MOUNT_ROOT:-}" ]]; then
    if findmnt -rno TARGET --target "${MOUNT_ROOT}/boot/efi" >/dev/null 2>&1; then
      umount "${MOUNT_ROOT}/boot/efi"
    fi
    if findmnt -rno TARGET --target "${MOUNT_ROOT}" >/dev/null 2>&1; then
      umount "${MOUNT_ROOT}"
    fi
  fi
  if [[ -n "${SOURCE_MOUNT:-}" ]] && findmnt -rno TARGET --target "${SOURCE_MOUNT}" >/dev/null 2>&1; then
    umount "${SOURCE_MOUNT}"
  fi
}

trap cleanup EXIT

resolve_latest_artifact_dir() {
  find "${ROOT_DIR}/dist" -maxdepth 1 -mindepth 1 -type d -name 'velyx-os-preview-*' | sort | tail -n 1
}

resolve_source_image_from_artifact() {
  local artifact_dir="$1"
  local candidate

  candidate="$(find "${artifact_dir}" -maxdepth 1 -type f -name 'velyx-os-preview-*.img' | sort | head -n1)"
  if [[ -n "${candidate}" ]]; then
    printf '%s\n' "${candidate}"
    return 0
  fi

  candidate="$(find "${artifact_dir}" -maxdepth 1 -type f -name '*.img' ! -name 'initrd.img' | sort | head -n1)"
  if [[ -n "${candidate}" ]]; then
    printf '%s\n' "${candidate}"
    return 0
  fi

  return 1
}

partition_path() {
  local disk="$1"
  local number="$2"
  if [[ "${disk}" =~ [0-9]$ ]]; then
    printf '%sp%s\n' "${disk}" "${number}"
  else
    printf '%s%s\n' "${disk}" "${number}"
  fi
}

canonical_device() {
  readlink -f "$1"
}

ensure_block_device() {
  local path="$1"
  [[ -b "${path}" ]] || fail "target disk is not a block device: ${path}"
}

ensure_supported_target_type() {
  local target_type
  target_type="$(lsblk -ndo TYPE "${TARGET_DISK}" | head -n1)"
  case "${target_type}" in
    disk|loop)
      ;;
    *)
      fail "unsupported TARGET_DISK type '${target_type}' for ${TARGET_DISK}"
      ;;
  esac
}

ensure_target_not_mounted() {
  local mounted
  mounted="$(lsblk -nrpo NAME,MOUNTPOINT "${TARGET_DISK}" | awk 'NF >= 2 && $2 != "" { print $0 }')"
  [[ -z "${mounted}" ]] || fail "target disk has mounted filesystems, refusing to continue: ${mounted}"
}

wait_for_partition() {
  local part="$1"
  local attempt
  for attempt in $(seq 1 20); do
    if [[ -b "${part}" ]]; then
      return 0
    fi
    sleep 0.5
  done
  fail "partition node did not appear: ${part}"
}

ensure_mount_source_matches() {
  local expected
  local mountpoint
  local actual
  expected="$(canonical_device "$1")"
  mountpoint="$2"
  actual="$(findmnt -n -o SOURCE --target "${mountpoint}" 2>/dev/null || true)"
  [[ -n "${actual}" ]] || fail "mountpoint is not mounted: ${mountpoint}"
  actual="$(canonical_device "${actual}")"
  [[ "${actual}" == "${expected}" ]] || fail "mountpoint ${mountpoint} is backed by ${actual}, expected ${expected}"
}

write_fstab() {
  local root_uuid
  local efi_uuid
  root_uuid="$(blkid -s UUID -o value "${ROOT_PARTITION}")"
  efi_uuid="$(blkid -s UUID -o value "${EFI_PARTITION}")"
  [[ -n "${root_uuid}" ]] || fail "could not resolve UUID for ${ROOT_PARTITION}"
  [[ -n "${efi_uuid}" ]] || fail "could not resolve UUID for ${EFI_PARTITION}"

  cat > "${MOUNT_ROOT}/etc/fstab" <<EOF
UUID=${root_uuid} / ext4 defaults 0 1
UUID=${efi_uuid} /boot/efi vfat noauto,nofail,x-systemd.automount,x-systemd.idle-timeout=60,umask=0077,utf8,x-systemd.device-timeout=3s 0 0
EOF
}

write_grub_config() {
  local root_uuid
  root_uuid="$(blkid -s UUID -o value "${ROOT_PARTITION}")"
  mkdir -p "${MOUNT_ROOT}/boot/grub"
  cat > "${MOUNT_ROOT}/boot/grub/grub.cfg" <<EOF
set default=0
set timeout=1

search --no-floppy --fs-uuid --set=root ${root_uuid}

menuentry 'Velyx OS Preview' {
    search --no-floppy --fs-uuid --set=root ${root_uuid}
    linux /boot/vmlinuz root=UUID=${root_uuid} rw rootwait quiet splash console=tty1 console=ttyS0
    initrd /boot/initrd.img
}
EOF
}

write_efi_chain_config() {
  local root_uuid
  root_uuid="$(blkid -s UUID -o value "${ROOT_PARTITION}")"
  mkdir -p "${MOUNT_ROOT}/boot/efi/EFI/velyx" "${MOUNT_ROOT}/boot/efi/EFI/BOOT"
  local efi_cfg_content
  efi_cfg_content="$(cat <<EOF
insmod part_gpt
insmod fat
insmod ext2
search --no-floppy --fs-uuid --set=root ${root_uuid}
set prefix=(\$root)/boot/grub
configfile /boot/grub/grub.cfg
EOF
)"
  printf '%s\n' "${efi_cfg_content}" > "${MOUNT_ROOT}/boot/efi/EFI/velyx/grub.cfg"
  printf '%s\n' "${efi_cfg_content}" > "${MOUNT_ROOT}/boot/efi/EFI/BOOT/grub.cfg"
}

install_bootloader() {
  log "bootloader install target=${TARGET_DISK} efi_directory=${MOUNT_ROOT}/boot/efi"
  grub-install \
    --target=x86_64-efi \
    --efi-directory="${MOUNT_ROOT}/boot/efi" \
    --boot-directory="${MOUNT_ROOT}/boot" \
    --bootloader-id=velyx \
    --no-nvram \
    --recheck \
    "${TARGET_DISK}"

  grub-install \
    --target=x86_64-efi \
    --efi-directory="${MOUNT_ROOT}/boot/efi" \
    --boot-directory="${MOUNT_ROOT}/boot" \
    --bootloader-id=velyx \
    --no-nvram \
    --removable \
    --recheck \
    "${TARGET_DISK}"

  write_efi_chain_config
  build_standalone_efi_loaders

  [[ -f "${MOUNT_ROOT}/boot/efi/EFI/velyx/grubx64.efi" ]] \
    || fail "missing bootloader file after install: ${MOUNT_ROOT}/boot/efi/EFI/velyx/grubx64.efi"
}

build_standalone_efi_loaders() {
  local tmp_cfg
  tmp_cfg="$(mktemp /tmp/velyx-grub-standalone.XXXXXX.cfg)"
  cp "${MOUNT_ROOT}/boot/efi/EFI/velyx/grub.cfg" "${tmp_cfg}"

  grub-mkstandalone \
    -O x86_64-efi \
    -o "${MOUNT_ROOT}/boot/efi/EFI/velyx/grubx64.efi" \
    "boot/grub/grub.cfg=${tmp_cfg}"

  grub-mkstandalone \
    -O x86_64-efi \
    -o "${MOUNT_ROOT}/boot/efi/EFI/BOOT/BOOTX64.EFI" \
    "boot/grub/grub.cfg=${tmp_cfg}"

  rm -f "${tmp_cfg}"
}

copy_payload() {
  SOURCE_MOUNT="$(mktemp -d /tmp/velyx-baremetal-src.XXXXXX)"
  mount -o loop,ro "${SOURCE_IMAGE}" "${SOURCE_MOUNT}"

  mkdir -p "${MOUNT_ROOT}"
  mount "${ROOT_PARTITION}" "${MOUNT_ROOT}"
  mkdir -p "${MOUNT_ROOT}/boot/efi"
  mount "${EFI_PARTITION}" "${MOUNT_ROOT}/boot/efi"

  ensure_mount_source_matches "${ROOT_PARTITION}" "${MOUNT_ROOT}"
  ensure_mount_source_matches "${EFI_PARTITION}" "${MOUNT_ROOT}/boot/efi"

  log "copying rootfs source_image=${SOURCE_IMAGE} target_root=${ROOT_PARTITION}"
  rsync -aHAX --numeric-ids \
    --exclude='/dev/*' \
    --exclude='/proc/*' \
    --exclude='/run/*' \
    --exclude='/sys/*' \
    --exclude='/tmp/*' \
    "${SOURCE_MOUNT}/" "${MOUNT_ROOT}/"

  install -Dm644 "${KERNEL_IMAGE}" "${MOUNT_ROOT}/boot/vmlinuz"
  install -Dm644 "${INITRD_IMAGE}" "${MOUNT_ROOT}/boot/initrd.img"
  write_fstab
  write_grub_config
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --target-disk)
      TARGET_DISK="$2"
      shift 2
      ;;
    --artifact-dir)
      ARTIFACT_DIR="$2"
      shift 2
      ;;
    --source-image)
      SOURCE_IMAGE="$2"
      shift 2
      ;;
    --kernel)
      KERNEL_IMAGE="$2"
      shift 2
      ;;
    --initrd)
      INITRD_IMAGE="$2"
      shift 2
      ;;
    --mount-root)
      MOUNT_ROOT="$2"
      shift 2
      ;;
    --yes-wipe)
      YES_WIPE=1
      shift
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

[[ "${EUID}" -eq 0 ]] || fail "this installer must run as root"
[[ -n "${TARGET_DISK}" ]] || fail "--target-disk is required"
[[ "${YES_WIPE}" -eq 1 ]] || fail "--yes-wipe is required for destructive disk install"

require_cmd lsblk
require_cmd blkid
require_cmd findmnt
require_cmd sgdisk
require_cmd mkfs.fat
require_cmd mkfs.ext4
require_cmd grub-install
require_cmd grub-mkstandalone
require_cmd mount
require_cmd umount
require_cmd rsync
require_cmd readlink

TARGET_DISK="$(canonical_device "${TARGET_DISK}")"
ensure_block_device "${TARGET_DISK}"
ensure_supported_target_type
ensure_target_not_mounted

if [[ -z "${ARTIFACT_DIR}" && ( -z "${SOURCE_IMAGE}" || -z "${KERNEL_IMAGE}" || -z "${INITRD_IMAGE}" ) ]]; then
  ARTIFACT_DIR="$(resolve_latest_artifact_dir)"
fi

if [[ -n "${ARTIFACT_DIR}" ]]; then
  ARTIFACT_DIR="$(readlink -f "${ARTIFACT_DIR}")"
  [[ -d "${ARTIFACT_DIR}" ]] || fail "artifact dir not found: ${ARTIFACT_DIR}"
  [[ -n "${SOURCE_IMAGE}" ]] || SOURCE_IMAGE="$(resolve_source_image_from_artifact "${ARTIFACT_DIR}")"
  [[ -n "${KERNEL_IMAGE}" ]] || KERNEL_IMAGE="${ARTIFACT_DIR}/vmlinuz"
  [[ -n "${INITRD_IMAGE}" ]] || INITRD_IMAGE="${ARTIFACT_DIR}/initrd.img"
fi

[[ -f "${SOURCE_IMAGE}" ]] || fail "source image not found: ${SOURCE_IMAGE}"
[[ -f "${KERNEL_IMAGE}" ]] || fail "kernel image not found: ${KERNEL_IMAGE}"
[[ -f "${INITRD_IMAGE}" ]] || fail "initrd image not found: ${INITRD_IMAGE}"

SOURCE_IMAGE="$(readlink -f "${SOURCE_IMAGE}")"
KERNEL_IMAGE="$(readlink -f "${KERNEL_IMAGE}")"
INITRD_IMAGE="$(readlink -f "${INITRD_IMAGE}")"

log "TARGET_DISK=${TARGET_DISK}"
log "SOURCE_IMAGE=${SOURCE_IMAGE}"
log "KERNEL_IMAGE=${KERNEL_IMAGE}"
log "INITRD_IMAGE=${INITRD_IMAGE}"
log "disk inventory follows"
lsblk -o NAME,PATH,SIZE,TYPE,FSTYPE,MOUNTPOINT "${TARGET_DISK}" | tee -a "${LOG_FILE}" >&2

sgdisk --zap-all "${TARGET_DISK}"
sgdisk -o "${TARGET_DISK}"
sgdisk -n 1:2048:+512M -t 1:ef00 -c 1:"EFI System" "${TARGET_DISK}"
sgdisk -n 2:0:0 -t 2:8300 -c 2:"Velyx Root" "${TARGET_DISK}"
partprobe "${TARGET_DISK}" >/dev/null 2>&1 || true
sleep 1

EFI_PARTITION="$(partition_path "${TARGET_DISK}" 1)"
ROOT_PARTITION="$(partition_path "${TARGET_DISK}" 2)"
wait_for_partition "${EFI_PARTITION}"
wait_for_partition "${ROOT_PARTITION}"

log "EFI_PARTITION=${EFI_PARTITION}"
log "ROOT_PARTITION=${ROOT_PARTITION}"

mkfs.fat -F 32 -n VELYX_EFI "${EFI_PARTITION}"
mkfs.ext4 -F -L VELYX_ROOT "${ROOT_PARTITION}"

copy_payload
sync
install_bootloader
sync

ensure_mount_source_matches "${ROOT_PARTITION}" "${MOUNT_ROOT}"
ensure_mount_source_matches "${EFI_PARTITION}" "${MOUNT_ROOT}/boot/efi"

log "install complete TARGET_DISK=${TARGET_DISK} EFI_PARTITION=${EFI_PARTITION} ROOT_PARTITION=${ROOT_PARTITION}"
cat <<EOF
Velyx bare-metal install complete.
TARGET_DISK=${TARGET_DISK}
EFI_PARTITION=${EFI_PARTITION}
ROOT_PARTITION=${ROOT_PARTITION}
MOUNT_ROOT=${MOUNT_ROOT}
BOOTLOADER_EFI=${MOUNT_ROOT}/boot/efi/EFI/velyx/grubx64.efi
EOF
