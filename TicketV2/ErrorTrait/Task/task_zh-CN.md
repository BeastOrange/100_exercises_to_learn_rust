此任务包含两个主要部分，这两个部分在 `TODO` 注释中有所描述：

- 为 `TicketNewError` 枚举实现 `Debug`、`Display` 和 `Error` 特性。在实现 `Display` 时，可以使用 `write!` 宏。
- 实现 `easy_ticket` 函数。对于无效的标题，应使用 `TicketNewError` 的消息来触发 panic，但对无效的描述则应使用默认描述。