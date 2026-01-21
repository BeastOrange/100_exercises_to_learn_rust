## 工单 IDs

让我们再次思考我们的工单管理系统。\
我们的工单模型目前看起来是这样的：

```rust
pub struct Ticket {
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status
}
```

这里缺少了一样东西：一个**标识符**来唯一标识一个工单。\
这个标识符应该对每个工单都是唯一的。可以通过在创建新工单时自动生成它来确保这一点。

## 优化模型

这个 id 应该存储在哪里呢？\
我们可以在 `Ticket` 结构体中添加一个新字段：

```rust
pub struct Ticket {
    pub id: TicketId,
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status
}
```

但是我们在创建工单之前并不知道这个 id。因此，它不能一开始就存在。\
它必须是可选的：

```rust
pub struct Ticket {
    pub id: Option<TicketId>,
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status
}
```

这也不理想——每次我们从存储中检索一个工单时都必须处理 `None` 的情况，\
尽管我们知道一旦工单被创建，id 肯定应该是存在的。

最好的解决方案是使用两个不同的工单**状态**，通过两个单独的类型表示：
一个 `TicketDraft` 和一个 `Ticket`：

```rust
pub struct TicketDraft {
    pub title: TicketTitle,
    pub description: TicketDescription
}

pub struct Ticket {
    pub id: TicketId,
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status
}
```

`TicketDraft` 是尚未创建的工单。它没有 id，也没有状态。\
`Ticket` 是已经创建的工单。它有 id 和状态。\
由于 `TicketDraft` 和 `Ticket` 中的每个字段都包含了自己的约束条件，我们不需要在两个类型之间重复逻辑。