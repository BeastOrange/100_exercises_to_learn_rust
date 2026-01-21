## 枚举

根据您在[之前部分](../../../Ticket%20v1/Validation/Theory/task.md)中编写的验证逻辑，门票只有几个合法的状态：`To-Do`、`InProgress` 和 `Done`。  
如果我们查看 `Ticket` 结构体中的 `status` 字段或 `new` 方法中 `status` 参数的类型，这一点并不明显：

```rust
#[derive(Debug, PartialEq)]
pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    pub fn new(
        title: String, 
        description: String, 
        status: String
    ) -> Self {
        // [...]
    }
}
```

在以上两种情况下，我们都使用了 `String` 来表示 `status` 字段。  
`String` 是一个非常通用的类型——它并无法直接表明 `status` 字段有一组固定的可能值。更糟的是，`Ticket::new` 的调用者只能在**运行时**才会发现他们提供的状态是否合法。

通过使用 **枚举**，我们可以改进这一点。

## `enum`

枚举是一种可以具有固定值集合（称为**变体**）的类型。  
在 Rust 中，可以使用 `enum` 关键字定义枚举：

```rust
enum Status {
    ToDo,
    InProgress,
    Done,
}
```

`enum` 就像 `struct` 一样，定义了**一个新的 Rust 类型**。