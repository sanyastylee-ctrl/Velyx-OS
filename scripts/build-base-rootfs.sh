#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUTPUT_DIR="${1:-${ROOT_DIR}/build/base-rootfs}"
RELEASE="${VELYX_BASE_RELEASE:-noble}"
MIRROR="${VELYX_BASE_MIRROR:-https://archive.ubuntu.com/ubuntu}"

PACKAGES=(
  systemd-sysv
  dbus
  dbus-user-session
  dbus-x11
  sudo
  libpam-modules
  locales
  ca-certificates
  adduser
  passwd
  kmod
  udev
  iproute2
  netbase
  util-linux
  bash
  coreutils
  python3
  grep
  sed
  findutils
  tar
  procps
  libx11-6
  libx11-xcb1
  libxcb1
  libxcb-cursor0
  libxcb-icccm4
  libxcb-image0
  libxcb-keysyms1
  libxcb-randr0
  libxcb-render-util0
  libxcb-shape0
  libxcb-sync1
  libxcb-xfixes0
  libxcb-xkb1
  libxkbcommon0
  libxkbcommon-x11-0
  libegl1
  libegl-mesa0
  libgl1
  libgl1-mesa-dri
  libglx-mesa0
  libgles2
  libgbm1
  libdrm2
  libopengl0
  libfontconfig1
  libglib2.0-0t64
  libfreetype6
  libdbus-1-3
  x11-utils
  wmctrl
  xdotool
  xauth
  xinit
  xserver-xorg-core
  xserver-xorg-video-all
  xserver-xorg-input-all
  mesa-utils
  mesa-vulkan-drivers
  fonts-dejavu-core
)

sudo rm -rf "${OUTPUT_DIR}"
sudo debootstrap \
  --arch=amd64 \
  --variant=minbase \
  --components=main,universe \
  --include="$(IFS=,; echo "${PACKAGES[*]}")" \
  "${RELEASE}" \
  "${OUTPUT_DIR}" \
  "${MIRROR}"

sudo chown -R "$(id -un)":"$(id -gn)" "${OUTPUT_DIR}"
echo "base rootfs prepared at ${OUTPUT_DIR}"
