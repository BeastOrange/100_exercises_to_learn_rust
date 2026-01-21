此任务要求您将 `Ticket` 结构体的字段重构为不同的类型：`TicketTitle`、`TicketDescription` 和 `Status`。

您将在每个相应的模块（`title.rs`、`description.rs`、`status.rs` 和 `lib.rs`）中找到 `TODO` 注释。您的主要目标是：

- 为 `TicketTitle`、`TicketDescription` 和 `Status` 实现 `TryFrom<String>` 和 `TryFrom<&str>`，确保在各自的实现中遵守所有指定的验证规则（例如，长度、非空、对 `Status` 的大小写不敏感性）。
- 实现任何额外的特性以使这些新类型的相关测试全部通过。
- 注意 `Ticket` 结构体的字段现在是公开的，并利用新类型提供的封装功能。

此任务强调使用 Rust 的类型系统来确保不变量，并在多个文件中改进代码组织。