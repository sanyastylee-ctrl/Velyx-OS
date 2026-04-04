# File Service Foundation Velyx OS

## Цель

`file-service` является системным backend для безопасной работы с файловой metadata.

На первом этапе сервис:

- не читает содержимое файлов
- не дает raw content access
- не обходит будущую portal и permissions model
- предоставляет только metadata search, recent files и metadata lookup

## Базовый принцип

Все клиенты Velyx OS должны двигаться к модели:

`shell / files app / AI -> file-service -> access policy -> metadata layer`

а не к прямому обходу в raw filesystem.

## Что разрешено на первом этапе

- `name`
- `path`
- `extension`
- `modified_time`
- `size`
- `is_dir`

## Что запрещено на первом этапе

- чтение file contents
- прямой экспорт произвольных чувствительных путей без policy
- обход audit trail

## Future hooks

- file picker portal
- scoped content read
- device portal
- permissions-aware content open
