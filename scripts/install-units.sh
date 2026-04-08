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
  "${ROOTFS_DIR}/etc/systemd/system/graphical.target.wants" \
  "${ROOTFS_DIR}/usr/lib/systemd/user" \
  "${ROOTFS_DIR}/usr/lib/sysusers.d" \
  "${ROOTFS_DIR}/usr/lib/tmpfiles.d" \
  "${ROOTFS_DIR}/home/${USER_NAME}/.config/systemd/user/default.target.wants"

cp "${ROOT_DIR}"/systemd/system/* "${ROOTFS_DIR}/etc/systemd/system/"
cp "${ROOT_DIR}"/systemd/user/* "${ROOTFS_DIR}/usr/lib/systemd/user/"
cp "${ROOT_DIR}/systemd/sysusers/velyx.conf" "${ROOTFS_DIR}/usr/lib/sysusers.d/velyx.conf"
cp "${ROOT_DIR}/systemd/tmpfiles/velyx.conf" "${ROOTFS_DIR}/usr/lib/tmpfiles.d/velyx.conf"

ln -snf ../velyx-boot.target "${ROOTFS_DIR}/etc/systemd/system/multi-user.target.wants/velyx-boot.target"
ln -snf ../velyx-primary-shell.service "${ROOTFS_DIR}/etc/systemd/system/graphical.target.wants/velyx-primary-shell.service"
ln -snf /dev/null "${ROOTFS_DIR}/etc/systemd/system/getty@tty1.service"
ln -snf /usr/lib/systemd/user/velyx-session.target "${ROOTFS_DIR}/home/${USER_NAME}/.config/systemd/user/default.target.wants/velyx-session.target"
