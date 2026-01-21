## 可变引用

你的访问器方法现在应该像这样：

```rust
impl Ticket {
    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn status(&self) -> &String {
        &self.status
    }
}
```

这里使用了一些 `&` 符号就解决了问题！  
我们现在可以在不消耗 `Ticket` 实例的情况下访问其字段。  
接下来，让我们看看如何通过**设置器方法**来增强我们的 `Ticket` 结构体。

## 设置器

设置器方法允许用户更改 `Ticket` 的私有字段值，同时确保其不变量得到维护（例如，你不能将 `Ticket` 的标题设置为空字符串）。

在 Rust 中实现设置器有两种常见的方法：

- 将 `self` 作为输入。
- 将 `&mut self` 作为输入。

### 将 `self` 作为输入

第一种方法如下所示：

```rust
impl Ticket {
    pub fn set_title(mut self, new_title: String) -> Self {
        // 验证新标题 [...]
        self.title = new_title;
        self
    }
}
```

它获取 `self` 的所有权，修改标题并返回修改后的 `Ticket` 实例。  
你可以像这样使用它：

```rust
let ticket = Ticket::new(
    "Title".into(), 
    "Description".into(), 
    "To-Do".into()
);
let ticket = ticket.set_title("New title".into());
```

由于 `set_title` 获取了 `self` 的所有权（即**消耗了它**），我们需要将结果重新赋值给一个变量。  
在上面的示例中，我们利用了**变量遮蔽**来重用相同的变量名：当你用相同名字声明一个新变量时，新变量会**遮蔽**旧变量。这是 Rust 代码中常见的模式。

`self`-设置器在需要一次修改多个字段时表现得非常好：你可以将多个调用链在一起！

```rust
let ticket = ticket
    .set_title("New title".into())
    .set_description("New description".into())
    .set_status("In Progress".into());
```

### 将 `&mut self` 作为输入

第二种实现设置器的方法是使用 `&mut self`，如下所示：

```rust
impl Ticket {
    pub fn set_title(&mut self, new_title: String) {
        // 验证新标题 [...]
        
        self.title = new_title;
    }
}
```

这次方法接受一个 `self` 的可变引用作为输入，修改标题，仅此而已。  
没有返回值。

你可以像这样使用它：

```rust
let mut ticket = Ticket::new(
    "Title".into(),
    "Description".into(),
    "To-Do".into()
);
ticket.set_title("New title".into());

// 使用修改后的 ticket
```

所有权仍然属于调用者，因此原始的 `ticket` 变量依然有效。我们不需要重新赋值结果。  
不过，我们需要将 `ticket` 标记为可变的，因为我们传递了一个可变引用。

`&mut`-设置器有一个缺点：你不能将多个调用链在一起。  
由于它们不返回修改后的 `Ticket` 实例，你不能在第一个调用的结果上再调用其他设置器。  
你需要分别调用每个设置器：

```rust
ticket.set_title("New title".into());
ticket.set_description("New description".into());
ticket.set_status("In Progress".into());