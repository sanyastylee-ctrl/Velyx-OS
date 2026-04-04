# AI Contracts

Пакет фиксирует контракты данных для AI-слоя Velyx OS.

Здесь не должно быть произвольной бизнес-логики. Только:

- схемы intent
- схемы tool definitions
- схемы execution request/result
- схемы confirmation
- схемы audit entries

Контракты должны быть стабильной точкой между:

- shell UI
- ai-service
- backend adapter
- tool executor
- settings pages
