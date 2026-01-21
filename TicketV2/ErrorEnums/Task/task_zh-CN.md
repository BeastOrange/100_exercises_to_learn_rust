此任务要求您重构`Ticket`创建的错误处理。您的主要目标是：

- 定义`TicketNewError` `enum`，为标题和描述问题提供特定的变体。
- 更新`Ticket::new`以使用此新的`enum`作为错误返回`Result`。
- 实现`easy_ticket`，在遇到标题错误时使用`TicketNewError`触发恐慌，但对于描述错误则使用默认描述。

所有详细说明都在代码中的`TODO`注释中。