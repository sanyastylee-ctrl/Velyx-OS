#!/usr/bin/env bash
set -euo pipefail

ROOTFS_DIR="${1:-}"
USER_NAME="${2:-velyx}"

if [[ -z "${ROOTFS_DIR}" ]]; then
  echo "usage: install-units.sh <rootfs-dir> [user]" >&2
  exit 1
fi

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

install -d \
  "${ROOTFS_DIR}/etc/systemd/system" \
  "${ROOTFS_DIR}/etc/systemd/system/multi-user.target.wants" \
  "${ROOTFS_DIR}/etc/systemd/system/getty@tty1.service.d" \
  "${ROOTFS_DIR}/usr/lib/systemd/user" \
  "${ROOTFS_DIR}/usr/lib/sysusers.d" \
  "${ROOTFS_DIR}/usr/lib/tmpfiles.d" \
  "${ROOTFS_DIR}/home/${USER_NAME}/.config/systemd/user/default.target.wants"

cp "${ROOT_DIR}"/systemd/system/* "${ROOTFS_DIR}/etc/systemd/system/"
cp "${ROOT_DIR}"/systemd/user/* "${ROOTFS_DIR}/usr/lib/systemd/user/"
cp "${ROOT_DIR}/systemd/sysusers/velyx.conf" "${ROOTFS_DIR}/usr/lib/sysusers.d/velyx.conf"
cp "${ROOT_DIR}/systemd/tmpfiles/velyx.conf" "${ROOTFS_DIR}/usr/lib/tmpfiles.d/velyx.conf"
cp "${ROOT_DIR}/systemd/getty/tty1-autologin.conf" "${ROOTFS_DIR}/etc/systemd/system/getty@tty1.service.d/velyx-autologin.conf"

ln -snf ../velyx-boot.target "${ROOTFS_DIR}/etc/systemd/system/multi-user.target.wants/velyx-boot.target"
ln -snf /usr/lib/systemd/user/velyx-session-manager.service "${ROOTFS_DIR}/home/${USER_NAME}/.config/systemd/user/default.target.wants/velyx-session-manager.service"
ln -snf /usr/lib/systemd/user/velyx-session-bootstrap.service "${ROOTFS_DIR}/home/${USER_NAME}/.config/systemd/user/default.target.wants/velyx-session-bootstrap.service"
