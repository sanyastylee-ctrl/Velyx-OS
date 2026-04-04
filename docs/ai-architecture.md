# AI как системный слой Velyx OS

## Принцип

AI в Velyx OS не является суперпользователем и не имеет прямого доступа к:

- root-действиям
- произвольному `exec`
- прямому чтению файлов
- прямому изменению системного состояния
- обходу `permissions-service`, `policy` и `launcher-service`

AI — это отдельный оркестратор, который:

1. принимает пользовательскую команду
2. определяет намерение
3. выбирает допустимые tools
4. прогоняет план через `policy_guard`
5. вызывает только официальные системные сервисы
6. возвращает объяснимый результат

## Поток исполнения

```text
User
-> Shell AI overlay / command palette
-> ai-service
-> backend intent resolution
-> tool plan
-> policy guard
-> tool executor
-> downstream system service
-> result
-> user-facing explanation
-> AI audit
```

## Границы ответственности

### AI-service

Отвечает за:

- intent parsing
- tool planning
- policy evaluation
- confirmation generation
- execution orchestration
- отдельный AI audit

### LLM backend

Отвечает только за:

- интерпретацию ввода
- предложение intent
- предложение tool plan
- суммаризацию результата

LLM backend не имеет права:

- выполнять инструменты
- звать системные сервисы напрямую
- обходить `policy_guard`

### Tool executor

Отвечает за:

- вызов только зарегистрированных инструментов
- маршрутизацию в системные сервисы
- контроль side effects

### Policy guard

Отвечает за:

- deny неизвестных tools
- confirmation для рискованных действий
- блокировку restricted операций
- оценку session/user context

## Базовые инструменты

- `app.launch`
- `settings.get`
- `settings.set`
- `files.search`
- `permissions.inspect`
- `permissions.update`
- `diagnostics.summary`
- `security.events.list`

## Privacy boundary

AI должен получать только тот контекст, который явно разрешен policy:

- активное приложение
- название текущего окна
- метаданные файлов
- системные события
- security audit

Доступ к содержимому файлов, уведомлениям и облачному backend должен управляться отдельной AI privacy policy.
