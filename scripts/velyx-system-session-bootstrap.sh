#!/usr/bin/env bash
set -euo pipefail

BOOT_USER="${1:-${VELYX_BOOT_USER:-velyx}}"
LOG_DIR="/var/log/velyx"
mkdir -p "${LOG_DIR}"

if command -v loginctl >/dev/null 2>&1; then
  loginctl enable-linger "${BOOT_USER}" >/dev/null 2>&1 || true
fi

echo "$(date -Iseconds) session_bootstrap_prepared user=${BOOT_USER}" >> "${LOG_DIR}/session_bootstrap.log"
