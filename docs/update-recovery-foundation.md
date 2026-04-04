# Update & Recovery Foundation Velyx OS

## Цель

Velyx OS должна считать update и recovery частью базовой security model, а не эксплуатационной надстройкой.

Базовый pipeline:

`check -> verify -> snapshot -> apply -> verify -> commit | rollback`

## 1. Update Engine Architecture

`update-engine` является единственной точкой применения системных обновлений.

### Ответственность update-engine

- обнаружение доступных update packages
- фиксация источника обновления
- signature verification hook
- orchestration pre-update snapshot
- apply attempt tracking
- post-apply verification
- explicit commit или explicit rollback handoff

### Что update-engine не должен делать

- не должен применять unsigned update
- не должен применять update без snapshot
- не должен silently continue при verify failure
- не должен напрямую менять recovery state без recovery-service

## 2. Snapshot Model

Модель ориентирована на Btrfs.

Каждая попытка обновления получает:

- `attempt_id`
- `update_id`
- `snapshot_id`
- `created_at`
- `state`
- `verification_state`

### Правило

Перед любым apply:

1. создается snapshot restore point
2. snapshot связывается с update attempt metadata
3. только после этого update может перейти в apply

## 3. Recovery Architecture

`recovery-service` отвечает за:

- список restore points
- rollback orchestration
- recovery mode hooks
- explicit recovery status

### Restore Point

Restore point описывает:

- `snapshot_id`
- `kind`
- `created_at`
- `source_update_id`
- `bootable`
- `reason`

## 4. State Transitions

### Update State

- `Idle`
- `Checking`
- `Ready`
- `VerifyingSignature`
- `CreatingSnapshot`
- `Applying`
- `VerifyingPostApply`
- `Committed`
- `RollbackRequired`
- `RolledBack`
- `Failed`

### Recovery State

- `Idle`
- `ListingRestorePoints`
- `RollbackPending`
- `RollbackInProgress`
- `RollbackCompleted`
- `RecoveryModeReady`
- `Failed`

## 5. Recovery Integration Points

- `pre_update_snapshot_created`
- `post_apply_verification_failed`
- `rollback_required`
- `recovery_mode_requested`
- `boot_recovery_target_available`

Эти точки должны быть видимы:

- `update-engine`
- `recovery-service`
- будущему boot/session pipeline

## 6. Fail-Safe Rules

- unsigned update => deny
- snapshot creation failed => deny update
- verify failed => rollback required
- rollback failed => recovery mode hook must stay visible
- audit должен фиксировать не только success, но и источник отказа
