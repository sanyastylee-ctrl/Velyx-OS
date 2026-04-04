# Rootfs Layout

## System state

- `/etc/velyx`
- `/var/lib/velyx`
- `/var/log/velyx`
- `/usr/lib/velyx/boot`
- `/usr/share/velyx/app-manifests`

## User state

- `/home/velyx/.config/velyx`
- `/home/velyx/.local/state/velyx`
- `/home/velyx/.velyx`

`~/.velyx` сохраняется как compatibility state для текущих сервисов, потому что многие из них пока читают состояние именно оттуда. Это осознанный transitional layer для первого bootable prototype.

## Binary deployment

Бинарники раскладываются единообразно:

- `/usr/bin/velyx-shell`
- `/usr/bin/velyx-session-manager`
- `/usr/bin/velyx-settings-service`
- `/usr/bin/velyx-permissions-service`
- `/usr/bin/velyx-launcher-service`
- `/usr/bin/velyx-diagnostics-service`
- `/usr/bin/velyx-ai-service`
- `/usr/bin/velyx-file-service`
- `/usr/bin/velyx-installer-service`
- `/usr/bin/velyx-update-engine`
- `/usr/bin/velyx-recovery-service`

## systemd layout

- system units: `/etc/systemd/system`
- user units: `/usr/lib/systemd/user`
- sysusers: `/usr/lib/sysusers.d/velyx.conf`
- tmpfiles: `/usr/lib/tmpfiles.d/velyx.conf`

## Boot helper scripts

- `/usr/lib/velyx/boot/velyx-firstboot-dispatch`
- `/usr/lib/velyx/boot/velyx-system-session-bootstrap`
- `/usr/lib/velyx/boot/velyx-user-session-bootstrap`
