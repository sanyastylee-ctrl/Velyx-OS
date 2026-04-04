#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
IMAGE_PATH="${1:-${ROOT_DIR}/build/image/velyx-rootfs.img}"
MODE="${VELYX_QEMU_MODE:-windowed}"
KERNEL_PATH="${VELYX_KERNEL_PATH:-${ROOT_DIR}/build/qemu/vmlinuz}"
INITRD_PATH="${VELYX_INITRD_PATH:-${ROOT_DIR}/build/qemu/initrd.img}"
MEMORY_MB="${VELYX_QEMU_MEMORY_MB:-4096}"
CPUS="${VELYX_QEMU_CPUS:-2}"

if [[ ! -f "${IMAGE_PATH}" ]]; then
  echo "image not found: ${IMAGE_PATH}" >&2
  exit 1
fi

if [[ ! -f "${KERNEL_PATH}" ]]; then
  echo "kernel not found: ${KERNEL_PATH}" >&2
  exit 1
fi

QEMU_ARGS=(
  -m "${MEMORY_MB}"
  -smp "${CPUS}"
  -drive "file=${IMAGE_PATH},format=raw,if=virtio"
  -kernel "${KERNEL_PATH}"
  -append "root=/dev/vda rw console=ttyS0 systemd.unit=multi-user.target"
)

if [[ -f "${INITRD_PATH}" ]]; then
  QEMU_ARGS+=(-initrd "${INITRD_PATH}")
fi

if [[ "${MODE}" == "headless" ]]; then
  QEMU_ARGS+=(-nographic -serial mon:stdio)
else
  QEMU_ARGS+=(-serial stdio)
fi

exec qemu-system-x86_64 "${QEMU_ARGS[@]}"
