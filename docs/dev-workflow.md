# Локальный workflow

## Рекомендуемый режим разработки

1. Собирать UI-часть через `CMake + Ninja`.
2. Собирать системные сервисы через `Cargo workspace`.
3. Проверять форматирование и линтеры до коммита.
4. Любые изменения в security-critical модулях сопровождать обновлением документации или ADR.

## Базовые команды

```powershell
cmake -S . -B build -G Ninja
cmake --build build
ctest --test-dir build --output-on-failure
```

```powershell
cd services
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

## Quality gates

- сборка обязана проходить локально
- для Rust-сервисов запрещены предупреждения `clippy`
- QML должен проходить `qmllint`
- security-critical модули требуют review с точки зрения границ привилегий и rollback impact
