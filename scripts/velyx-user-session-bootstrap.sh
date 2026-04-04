#!/usr/bin/env bash
set -euo pipefail

BOOT_USER="${1:-${USER:-velyx}}"
for _ in $(seq 1 30); do
  if busctl --user --list | grep -q '^com\.velyx\.SessionManager'; then
    exec busctl --user call \
      com.velyx.SessionManager \
      /com/velyx/SessionManager \
      com.velyx.SessionManager1 \
      StartUserSession s "${BOOT_USER}"
  fi
  sleep 1
done

echo "session manager D-Bus name did not appear in time for user ${BOOT_USER}" >&2
exit 1
