## `thiserror`

这有点像绕了一小圈，对吧？但这是必要的！\
我们现在回到正轨：自定义错误类型和 `thiserror`。

## 自定义错误类型

我们已经了解了如何为自定义错误类型“手动”实现 `Error` trait。\
想象一下，如果你必须为代码库中的大多数错误类型执行此操作。那会有很多样板代码，对吧？

通过使用 [`thiserror`](https://docs.rs/thiserror/latest/thiserror/)，
我们可以减少一些样板代码。`thiserror` 是一个 Rust 包，它提供了一个**过程宏**，用来简化自定义错误类型的创建。

```rust
#[derive(thiserror::Error, Debug)]
enum TicketNewError {
    #[error("{0}")]
    TitleError(String),
    #[error("{0}")]
    DescriptionError(String),
}
```

## 你可以编写自己的宏

到目前为止我们看到的所有 `derive` 宏都由 Rust 标准库提供。\
`thiserror::Error` 是我们遇到的第一个**第三方** `derive` 宏的例子。

`derive` 宏是**过程宏**的一部分，过程宏是在编译时生成 Rust 代码的一种方式。\
本课程不会深入讲解如何编写过程宏，但重要的是你要知道可以自己编写过程宏！\
这是一个适合更高级 Rust 课程讨论的主题。

## 自定义语法

每个过程宏可以定义自己的语法，通常在包的文档中进行解释。\
对于 `thiserror`，我们有以下语法：

- `#[derive(thiserror::Error)]`：这是借助 `thiserror` 为自定义错误类型派生 `Error` trait 的语法。
- `#[error("{0}")]`：这是为自定义错误类型的每个变体定义 `Display` 实现的语法。\
  `{0}` 在显示错误时会被该变体的第零个字段（在此例中为 `String`）替换。