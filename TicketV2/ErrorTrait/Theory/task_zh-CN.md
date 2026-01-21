## 错误特质（Error trait）

### 错误报告

在上一个练习中，你需要解构 `TitleError` 变体以提取错误信息并将其传递给 `panic!` 宏。\
这是一个（初步）**错误报告**的示例：将错误类型转换为一种可以展示给用户、服务操作员或开发者的形式。

让每个 Rust 开发者自行设计错误报告策略是不实际的：这既浪费时间，也不利于多个项目之间的协作。
这正是 Rust 提供 `std::error::Error` 特质的原因。

### `Error` 特质

对于 `Result` 中的 `Err` 变体，其类型没有特定限制，但良好的实践是使用实现了 `Error` 特质的类型。
`Error` 是 Rust 错误处理机制的核心：

```rust
// 略微简化版的 `Error` 特质定义
pub trait Error: Debug + Display {}
```

你可能还记得 `:` 语法来自于 [关于 `From` 特质](../../../Traits/From%20trait/Theory/task.md#supertrait--subtrait) 的讲解——它用于指定**超特质（supertraits）**。
对于 `Error`，它有两个超特质：`Debug` 和 `Display`。如果一个类型想实现 `Error`，它必须同时实现 `Debug` 和 `Display`。

### `Display` 和 `Debug`

我们已经在 [之前的一课](../../../Traits/Derive%20macros/Theory/task.md) 中遇到过 `Debug` 特质——这是 `assert_eq!` 用来显示变量值（当断言失败时）的特质。

从“功能”角度来看，`Display` 和 `Debug` 是相同的——它们都定义了一个类型应如何被转换为类似字符串的表示形式：

```rust
// `Debug`
pub trait Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>;
}

// `Display`
pub trait Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>;
}
```

二者的区别在于它们的**用途**：`Display` 返回的表示是面向“终端用户”的，而 `Debug` 提供的是更适合开发者和服务操作员的低级表示。\
因此，`Debug` 可以通过 `#[derive(Debug)]` 属性自动实现，而 `Display` **需要**手动实现。