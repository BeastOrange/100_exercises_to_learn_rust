## 更新操作

到目前为止，我们只实现了插入和检索操作。\
接下来让我们看看如何扩展系统以提供更新操作。

## 传统更新

在系统的非多线程版本中，更新操作相对直接：`TicketStore` 提供了一个\
`get_mut` 方法，该方法允许调用者获取一个工单的可变引用，然后对其进行修改。

## 多线程更新

在当前的多线程版本中，同样的策略将不起作用。借用检查器会阻止我们：`SyncSender<&mut Ticket>` 不是 `'static`，因为 `&mut Ticket` 不满足 `'static` 生命周期，\
因此它们无法被捕获到传递给 `std::thread::spawn` 的闭包中。

有几种方法可以绕过这一限制。我们将在接下来的练习中探索其中的一些方法。

### 补丁更新

我们不能通过通道发送 `&mut Ticket`，因此无法在客户端修改它。\
那么，我们可以在服务器端进行修改吗？

如果我们告诉服务器需要进行哪些更改，那我们可以在服务器端修改。换句话说，\
我们可以向服务器发送一个**补丁（patch）**：

```rust
struct TicketPatch {
    id: TicketId,
    title: Option<TicketTitle>,
    description: Option<TicketDescription>,
    status: Option<TicketStatus>,
}
```

`id` 字段是必需的，因为需要用它来标识需要更新的工单。\
所有其他字段是可选的：

- 如果一个字段是 `None`，这意味着该字段不应该被更改。
- 如果一个字段是 `Some(value)`，这意味着该字段应该更改为 `value`。