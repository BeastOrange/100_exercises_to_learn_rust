## 简洁的分支结构

您对前一练习的解决方案可能看起来是这样的：

```rust
impl Ticket {
    pub fn assigned_to(&self) -> &str {
        match &self.status {
            Status::InProgress { assigned_to } => assigned_to,
            Status::Done | Status::ToDo => {
                panic!(
                    "只有 `In-Progress` 类型的票据可以分配给某人"
                )
            }
        }
    }
}
```

您只关心 `Status::InProgress` 这一种情况。  
真的需要匹配所有其他的情况吗？

新的结构来解救！

## `if let`

`if let` 结构允许您只匹配枚举中的单一变体，  
无需处理所有其他的变体。

以下是如何使用 `if let` 简化 `assigned_to` 方法的示例：

```rust
impl Ticket {
    pub fn assigned_to(&self) -> &str {
        if let Status::InProgress { assigned_to } = &self.status {
            assigned_to
        } else {
            panic!(
                "只有 `In-Progress` 类型的票据可以分配给某人"
            );
        }
    }
}
```

## `let/else`

如果 `else` 分支需要提前返回（例如 `panic!` 也被视为提前返回！），  
您可以使用 `let/else` 结构：

```rust
impl Ticket {
    pub fn assigned_to(&self) -> &str {
        let Status::InProgress { assigned_to } = &self.status else {
            panic!(
                "只有 `In-Progress` 类型的票据可以分配给某人"
            );
        };
        assigned_to
    }
}
```

它允许您在不引入所谓的“向右偏移”（right drift）的情况下解构变量，  
也就是说，变量的赋值与之前的代码保持相同的缩进级别。

## 风格

`if let` 和 `let/else` 都是惯用的 Rust 结构。  
根据需要使用它们来提升代码的可读性，  
但不要过度使用：当需要时，`match` 仍然始终可用。