# Lifecycle Pipeline Velyx OS

## Цель

Этот документ фиксирует минимальный, но архитектурно правильный lifecycle Velyx OS:

`boot -> installer -> first boot -> initial setup -> session startup -> shell ready`

Безопасность является обязательным критерием приемки каждого перехода состояния.

## 1. Installer Architecture

На первом этапе Velyx OS получает backend `installer-service`, а UI-инсталлятор остается отдельным клиентом, который позже может жить в `apps/installer`.

### Границы ответственности installer-service

- обнаружение целевых дисков и install target abstraction
- выбор install profile
- фиксация параметров шифрования
- подготовка user bootstrap для первого запуска
- запись post-install config
- подготовка handoff в bootloader и first boot

### Что installer-service не должен делать

- не должен напрямую запускать shell session
- не должен обходить recovery/update контур
- не должен писать произвольные файлы вне известных каталогов состояния
- не должен смешивать live-installer state и installed system state

### Install Profiles

Минимальные профили:

- `desktop-default`
- `developer`
- `gaming-ready`
- `recovery-safe`

Каждый профиль определяет:

- базовый набор системных пакетов
- нужен ли Steam target
- нужен ли dev preset
- baseline security posture
- baseline AI mode

### Disk Target Abstraction

`DiskTarget` описывает не конкретные shell-команды разметки, а намерение:

- `target_id`
- `device_path`
- `capacity_gb`
- `scheme`
- `supports_encryption`
- `supports_rollback_layout`

На первом этапе это backend abstraction и contracts, а не production disk provisioning engine.

## 2. First Boot State Machine

После установки система должна входить в `first boot mode`, пока не завершены обязательные шаги.

Состояния:

1. `Installed`
2. `AwaitingInitialSetup`
3. `AwaitingUserCreation`
4. `ApplyingBaselineConfig`
5. `AwaitingSessionBootstrap`
6. `Completed`

### Переходы

- `Installed -> AwaitingInitialSetup`
  запускается после первого старта установленной системы
- `AwaitingInitialSetup -> AwaitingUserCreation`
  после выбора locale, keyboard, privacy baseline, network baseline
- `AwaitingUserCreation -> ApplyingBaselineConfig`
  после создания первого пользователя
- `ApplyingBaselineConfig -> AwaitingSessionBootstrap`
  после записи базовых настроек и session manifest
- `AwaitingSessionBootstrap -> Completed`
  после успешного старта core services и shell session

Если шаг неуспешен:

- состояние не продвигается вперед
- причина фиксируется в audit
- recovery hook получает точку возврата

## 3. Session Startup Sequence

Session startup должен управляться отдельным backend:

`session-manager-service`

Последовательность:

1. Проверка first boot state
2. Запуск `settings-service`
3. Запуск `permissions-service`
4. Запуск `launcher-service`
5. Запуск `diagnostics-service`
6. Запуск `ai-service`
7. Health checks по D-Bus
8. Старт shell session
9. Переход в `Ready`

Если часть сервисов не поднялась:

- сессия переходит в `Degraded`, если shell еще может работать
- сессия переходит в `Failed`, если не стартуют критичные security/core services

## 4. Basic State Layout

### System State

- `/etc/velyx/`
  системные конфиги и policy defaults
- `/var/lib/velyx/`
  service state, installer state, first-boot state, recovery markers
- `/var/log/velyx/`
  системные агрегированные логи

### User State

- `/home/<user>/`
  пользовательские данные
- `/home/<user>/.config/velyx/`
  UI preferences и user-level configuration
- `/home/<user>/.local/state/velyx/`
  user session state, local caches, AI local history
- `/home/<user>/.velyx/`
  текущий dev-oriented sandbox для сервисных прототипов и audit logs

### Recovery / Update State

- `/var/lib/velyx/recovery/`
  recovery markers, rollback metadata
- `/var/lib/velyx/update/`
  staged update state
- `/var/lib/velyx/boot/`
  boot intent, first boot markers, pending session bootstrap state

## 5. Recovery-Aware Hooks

Точки интеграции закладываются сразу:

- после `PrepareInstall`
  создается install plan snapshot
- перед `CommitInstall`
  фиксируется target layout intent
- на первом boot
  пишется `first_boot_pending`
- перед session startup
  пишется `session_start_pending`
- после успешного startup
  pending markers очищаются

Если startup сорвался:

- recovery-service должен видеть незавершенный bootstrap
- update-engine и recovery-service должны иметь право предложить rollback или повтор bootstrap

## 6. Acceptance Rule

Архитектурное решение для installer, first boot и session startup считается неприемлемым, если:

- оно требует ручного shell-скриптинга как основного пользовательского пути
- оно размывает границу между system state и user state
- оно не оставляет recovery markers
- оно не фиксирует audit trail критичных переходов
