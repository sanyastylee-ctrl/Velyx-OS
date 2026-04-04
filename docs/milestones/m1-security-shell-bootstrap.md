# M1 — Security-First Shell Bootstrap

## Цель

Создать инженерную основу Velyx OS, в которой:

- shell прототипирует визуальное направление и UX-каркас
- security-first принципы уже зафиксированы в структуре системы
- критичные сервисы выделены в отдельные модули
- репозиторий готов к дальнейшему переходу в usable alpha

## Входит в milestone

- monorepo-структура
- design-system
- shared-ui
- `velyx-shell`
- `velyx-settings`
- `velyx-files`
- каркасы сервисов:
  - `update-engine`
  - `recovery-service`
  - `permissions-service`
  - `compatibility-manager`
- CI baseline
- первичная архитектурная документация

## Не входит в milestone

- готовый инсталлятор
- полноценный compositor
- production-ready app store
- реальный backend Wi-Fi/Bluetooth
- production-grade Windows compatibility layer

## Критерии приемки

- архитектурные границы модулей задокументированы
- UI-модули переиспользуются между приложениями
- security-critical подсистемы выделены отдельно
- есть явный список рисков и ограничений
- решения не противоречат immutable/semi-immutable модели ядра системы
