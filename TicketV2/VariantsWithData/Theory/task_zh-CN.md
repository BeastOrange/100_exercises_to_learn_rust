## 枚举的变体可以持有数据

```rust
enum Status {
    ToDo,
    InProgress,
    Done,
}
```

我们的 `Status` 枚举通常被称为 **C风格枚举**。\
每个变体都是一个简单的标识，有点像一个命名常量。这种类型的枚举可以在许多编程语言中找到，例如 C、C++、Java、C#、Python 等。

不过，Rust 的枚举功能更加强大。我们可以为**每个变体附加数据**。

## 变体

假设我们想存储当前正在处理一个任务的人员的名字。\
这类信息只有在任务**正在进行中**时才会存在，而对于待办或已完成的任务则不会有。\
我们可以通过在 `InProgress` 变体中附加一个 `String` 字段来建模这一情况：

```rust
enum Status {
    ToDo,
    InProgress {
        assigned_to: String,
    },
    Done,
}
```

现在的 `InProgress` 是一个 **类结构体的变体**。\
实际上，这种语法借鉴了我们定义结构体的方式，只是它被“内联”到了枚举中作为一个变体。

## 访问变体数据

如果我们试图访问 `Status` 实例中的 `assigned_to`，

```rust
let status: Status = /* */;

// 这段代码不能编译通过
println!("Assigned to: {}", status.assigned_to);
```

编译器会阻止我们：

```text
error[E0609]: no field `assigned_to` on type `Status`
 --> src/main.rs:5:40
  |
5 |     println!("Assigned to: {}", status.assigned_to);
  |                                        ^^^^^^^^^^^ unknown field
```

`assigned_to` 是**特定于某个变体**的字段，它并不适用于所有 `Status` 实例。\
要访问 `assigned_to`，我们需要使用 **模式匹配**：

```rust
match status {
    Status::InProgress { assigned_to } => {
        println!("Assigned to: {}", assigned_to);
    },
    Status::ToDo | Status::Done => {
        println!("ToDo or Done");
    }
}
```

## 绑定

在匹配模式 `Status::InProgress { assigned_to }` 中，`assigned_to` 是一个**绑定**。\
我们正在**解构** `Status::InProgress` 变体，并将其 `assigned_to` 字段绑定到一个新的变量，也叫 `assigned_to`。\
如果我们愿意，也可以将该字段绑定到一个不同的变量名字：

```rust
match status {
    Status::InProgress { assigned_to: person } => {
        println!("Assigned to: {}", person);
    },
    Status::ToDo | Status::Done => {
        println!("ToDo or Done");
    }
}