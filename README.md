# Velyx OS

Velyx OS — настольная операционная система на базе современного Linux-стека, спроектированная как продуктовый desktop platform с тремя равноприоритетными целями:

- практичность и широта сценариев уровня Windows
- цельность, предсказуемость и визуальная аккуратность уровня современной premium desktop-системы
- безопасность по умолчанию как обязательный критерий архитектуры

## Базовая позиция проекта

Velyx OS проектируется не как тема поверх готового DE и не как экспериментальная hobby-distro, а как управляемая платформа со своими сервисами, UX-контрактами, моделями обновления, восстановления и контроля разрешений.

Ключевое правило этого репозитория:

> Любое архитектурное решение считается неприемлемым, если оно ухудшает security posture системы без явно зафиксированной и обоснованной причины.

## Допущения

- Первая целевая архитектура: `x86_64`.
- Базовый стек: Linux LTS kernel, Wayland, PipeWire, systemd, Btrfs.
- GUI и shell: `Qt 6 + QML`.
- Новые системные и security-critical сервисы: `Rust`.
- Межпроцессное взаимодействие: `D-Bus`.
- Базовая модель приложений: sandbox-first.
- Обновления системы: атомарные или полуатомарные, с rollback.
- Совместимость с Windows-приложениями: только через управляемый compatibility manager.

## Первый milestone

`M1 — Security-First Shell Bootstrap`

Цель milestone:

- зафиксировать модульную архитектуру Velyx OS
- заложить monorepo-структуру
- собрать buildable shell prototype
- оформить design-system и shared-ui
- подготовить каркас Settings и Files
- подготовить каркас security-critical сервисов:
  - update-engine
  - recovery-service
  - permissions/privacy-service
  - compatibility-manager

Критерии приемки milestone:

- кодовая база собирается локально
- shell, settings и files запускаются как отдельные прототипы
- security-critical сервисы имеют выделенные модули и documented boundaries
- безопасность явно зафиксирована как acceptance criterion архитектурных решений
- CI с первого дня проверяет сборку и базовые quality gates

## Структура monorepo

```text
apps/        UI-приложения и оболочка сессии
packages/    Общие UI-модули, design tokens, shared components
services/    Системные и session-сервисы на Rust
docs/        Архитектура, угрозы, milestones, ADR и контракты
scripts/     Локальные скрипты разработки и сборки
.github/     CI/CD пайплайны
```

## Рекомендуемый стек сборки

- `CMake + Ninja` для Qt/C++ части
- `Cargo` для Rust-сервисов
- `qmllint`, `qmlformat`, `clang-format`, `cargo fmt`, `cargo clippy`
- позже: `CTest`, контрактные тесты IPC, smoke UI tests

## Локальный запуск

```powershell
cmake -S . -B build -G Ninja
cmake --build build
ctest --test-dir build --output-on-failure
```

## Что здесь принципиально запрещено

- писать новое ядро с нуля
- строить продукт как набор тем для GNOME/KDE
- откладывать модель безопасности на “после MVP”
- выводить Windows-совместимость напрямую через raw Wine UX
- смешивать пользовательский и системный контуры без явной модели привилегий

## Ближайший следующий шаг

После bootstrap-фазы нужно перейти к `M2 — Session Runtime Foundation`:

- shell-session-service
- notification-service
- app-registry-service
- permissions ledger
- settings search index
- compatibility execution pipeline
