#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BUILD_DIR="${ROOT_DIR}/build"
ROOTFS_DIR="${BUILD_DIR}/rootfs"
IMAGE_DIR="${BUILD_DIR}/image"
IMAGE_NAME="${VELYX_IMAGE_NAME:-velyx-rootfs.img}"
IMAGE_SIZE_MB="${VELYX_IMAGE_SIZE_MB:-4096}"
IMAGE_PATH="${IMAGE_DIR}/${IMAGE_NAME}"

require_cmd() {
  command -v "$1" >/dev/null 2>&1 || {
    echo "missing required command: $1" >&2
    exit 1
  }
}

require_cmd truncate
require_cmd mkfs.ext4

"${ROOT_DIR}/scripts/build-rootfs.sh"
"${ROOT_DIR}/scripts/prepare-boot-artifacts.sh"

mkdir -p "${IMAGE_DIR}"
rm -f "${IMAGE_PATH}"
truncate -s "${IMAGE_SIZE_MB}M" "${IMAGE_PATH}"
mkfs.ext4 -F -d "${ROOTFS_DIR}" "${IMAGE_PATH}"

echo "image created at ${IMAGE_PATH}"
