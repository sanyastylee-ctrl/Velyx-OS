#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BOOT_DIR="${ROOT_DIR}/build/qemu"
KERNEL_SOURCE="${VELYX_KERNEL:-}"
INITRD_SOURCE="${VELYX_INITRD:-}"

mkdir -p "${BOOT_DIR}"

if [[ -z "${KERNEL_SOURCE}" ]]; then
  KERNEL_SOURCE="$(find /boot -maxdepth 1 -type f \( -name 'vmlinuz*' -o -name 'bzImage*' \) | head -n1 || true)"
fi

if [[ -z "${INITRD_SOURCE}" ]]; then
  INITRD_SOURCE="$(find /boot -maxdepth 1 -type f \( -name 'initramfs*' -o -name 'initrd*' \) | head -n1 || true)"
fi

if [[ -z "${KERNEL_SOURCE}" || ! -f "${KERNEL_SOURCE}" ]]; then
  echo "kernel artifact not found; set VELYX_KERNEL=/path/to/vmlinuz" >&2
  exit 1
fi

cp "${KERNEL_SOURCE}" "${BOOT_DIR}/vmlinuz"

if [[ -n "${INITRD_SOURCE}" && -f "${INITRD_SOURCE}" ]]; then
  cp "${INITRD_SOURCE}" "${BOOT_DIR}/initrd.img"
fi

echo "boot artifacts prepared in ${BOOT_DIR}"
