# First Boot Lifecycle Velyx OS

## Цель

`First boot` в Velyx OS является отдельным контролируемым lifecycle-этапом, а не неявным UI-сценарием.

Pipeline:

`install handoff -> first boot marker -> user bootstrap -> baseline config -> service bootstrap -> session handoff -> completed`

## State machine

- `None`
- `Pending`
- `InitialSetupStarted`
- `UserCreationPending`
- `BaselineConfigPending`
- `ServiceBootstrapPending`
- `HandoffToSessionPending`
- `Completed`
- `Failed`

## Критические правила

- first boot не зависит от shell как источника истины
- baseline settings применяются только через `settings-service`
- user bootstrap фиксируется в отдельном record
- handoff в session-manager должен быть явным и audit-friendly
- markers очищаются только после успешного lifecycle completion
