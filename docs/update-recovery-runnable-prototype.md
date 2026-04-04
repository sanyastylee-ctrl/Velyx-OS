# Update/Recovery Runnable Prototype Velyx OS

## Цель

Перевести `update-engine` и `recovery-service` из архитектурного foundation в живой backend flow:

`check -> attempt -> snapshot -> restore point -> apply -> verify -> commit | rollback_required -> rollback`

## Что теперь является исполняемым

- `CheckForUpdates()` возвращает реестр update packages
- `ApplyUpdate(update_id)` создает реальный `UpdateAttempt`
- создается `SnapshotRecord`
- `recovery-service` регистрирует restore point
- simulated apply пишет apply marker
- post-apply verification реально проверяет marker/linkage
- rollback path реально меняет `recovery_status`, `restore_points` и snapshot state

## Основные state transitions

### Update success

`Idle -> Checking -> Ready -> VerifyingSignature -> CreatingSnapshot -> Applying -> VerifyingPostApply -> Committed`

### Update unsigned

`Idle -> VerifyingSignature -> Failed`

### Update rollback required

`Idle -> VerifyingSignature -> CreatingSnapshot -> Applying -> VerifyingPostApply -> RollbackRequired`

### Recovery rollback

`RollbackRequired -> recovery.Rollback(snapshot_id) -> RolledBack backend linkage`

## Persisted state

- `~/.velyx/update_status.json`
- `~/.velyx/update_attempts.json`
- `~/.velyx/update_snapshots.json`
- `~/.velyx/update_apply_marker.json`
- `~/.velyx/recovery_status.json`
- `~/.velyx/restore_points.json`

## Security rules

- unsigned update runtime-denied
- update без snapshot не commit-ится
- post-apply verification обязателен
- rollback path явный и audit-friendly
- simulated apply не мутирует произвольный system state
