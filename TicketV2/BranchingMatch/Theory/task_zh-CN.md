## `match`

您可能会想——枚举（enum）具体可以用来**做**什么？\
最常见的操作是对它进行**模式匹配**。

```rust
enum Status {
    ToDo,
    InProgress,
    Done
}

impl Status {
    fn is_done(&self) -> bool {
        match self {
            Status::Done => true,
            // 使用 `|` 操作符可以匹配多个模式。
            // 它的意思是“`Status::ToDo` 或者 `Status::InProgress`”。
            Status::InProgress | Status::ToDo => false
        }
    }
}
```

`match` 语句允许您将一个 Rust 值与一系列**模式**进行比较。\
您可以把它看作一种类型级别的 `if` 语句。如果 `status` 是 `Done` 变体，执行第一个代码块；\
如果是 `InProgress` 或 `ToDo` 变体，执行第二个代码块。

## 穷尽性

这里有一个关键细节：`match` 是**穷尽**的。您必须处理所有的枚举变体。\
如果您忘记处理某个变体，Rust 会在**编译时**报错。

例如，如果我们忘记处理 `ToDo` 变体：

```rust
match self {
    Status::Done => true,
    Status::InProgress => false,
}
```

编译器会抱怨：

```text
error[E0004]: non-exhaustive patterns: `ToDo` not covered
 --> src/main.rs:5:9
  |
5 |     match status {
  |     ^^^^^^^^^^^^ pattern `ToDo` not covered
```

这十分重要！\
代码库会随着时间不断演变——比如，您可能会添加一个新的状态，如 `Blocked`。Rust 编译器会为所有缺少新变体处理逻辑的 `match` 语句报错。\
这就是为什么 Rust 开发者经常称赞“由编译器驱动的重构”的原因——编译器会告诉您接下来需要做什么，您只需修复它报告的问题即可。

## 通配模式

如果您不想匹配一个或多个变体，可以使用 `_` 模式作为通配模式：

```rust
match status {
    Status::Done => true,
    _ => false
}
```

`_` 模式匹配任何未被前面模式匹配到的值。

<div class="warning">
使用这种通配模式后，您将**不会**获得由编译器驱动的重构的优势。\
如果添加新的枚举变体，编译器**不会**告知您未处理它们。

如果您注重代码的正确性，避免使用通配模式。借助编译器重新审视所有匹配的代码位置，确定如何处理新的枚举变体。

</div>