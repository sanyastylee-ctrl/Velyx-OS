#!/usr/bin/env bash
set -euo pipefail

BOOT_USER="${VELYX_BOOT_USER:-velyx}"
LOG_DIR="/var/log/velyx"
STATE_DIR="/var/lib/velyx"
mkdir -p "${LOG_DIR}" "${STATE_DIR}" "/home/${BOOT_USER}/.velyx"

if [[ -f "/home/${BOOT_USER}/.velyx/install_handoff.json" ]]; then
  echo "$(date -Iseconds) first_boot_marker=present user=${BOOT_USER}" >> "${LOG_DIR}/first_boot.log"
else
  echo "$(date -Iseconds) first_boot_marker=absent user=${BOOT_USER}" >> "${LOG_DIR}/first_boot.log"
fi
