## 所有权

如果您使用本课程到目前为止教授的内容解决了上一道练习，那么您的访问器方法可能看起来像这样：

```rust
impl Ticket {
    pub fn title(self) -> String {
        self.title
    }

    pub fn description(self) -> String {
        self.description
    }

    pub fn status(self) -> String {
        self.status
    }
}
```

这些方法可以成功编译并通过测试，但在实际场景中，它们的作用并不大。考虑以下代码片段：

```rust
if ticket.status() == "To-Do" {
    // 我们尚未学习 `println!` 宏，
    // 但目前您只需知道它会将一条（模板化的）消息打印到控制台
    println!("Your next task is: {}", ticket.title());
}
```

如果您尝试编译它，会出现如下错误：

```text
error[E0382]: use of moved value: `ticket`
  --> src/main.rs:30:43
   |
25 |     let ticket = Ticket::new(/* */);
   |         ------ move occurs because `ticket` has type `Ticket`, 
   |                which does not implement the `Copy` trait
26 |     if ticket.status() == "To-Do" {
   |               -------- `ticket` moved due to this method call
...
30 |         println!("Your next task is: {}", ticket.title());
   |                                           ^^^^^^ 
   |                                value used here after move
   |
note: `Ticket::status` takes ownership of the receiver `self`, 
      which moves `ticket`
  --> src/main.rs:12:23
   |
12 |         pub fn status(self) -> String {
   |                       ^^^^
```

恭喜，这是您的第一个借用检查器错误！

## Rust 所有权系统的优势

Rust 的所有权系统旨在确保：

- 数据在被读取时不会被修改
- 数据在被修改时不会被读取
- 数据在销毁后不会被访问

这些限制由 **借用检查器** (borrow checker) 强制执行，这是 Rust 编译器的一个子系统，也是 Rust 社区中笑话和表情包的常见主题。

所有权是 Rust 的核心概念，使该语言与众不同。所有权使 Rust 能够提供 **内存安全性而不牺牲性能**。以下是 Rust 同时满足的一些特点：

1. 没有运行时垃圾回收器
2. 作为开发者，您很少需要直接管理内存
3. 您不会导致野指针、重复释放或其他与内存相关的错误

像 Python、JavaScript 和 Java 这样的语言能提供第 2 和第 3 点，但无法实现第 1 点。而像 C 或 C++ 这样的语言可以实现第 1 点，但无法实现第 2 和第 3 点。

根据您的背景知识，第 3 点可能听起来有点晦涩：什么是“野指针”？什么是“重复释放”？它们为什么危险？\
别担心：我们将在课程中更详细地解释这些概念。

现在，让我们专注于学习如何在 Rust 的所有权系统中工作。

## 所有者

在 Rust 中，每个值都有一个 **所有者**，该所有者在编译时静态确定。在任何给定时间，每个值只有一个所有者。

## 移动语义

所有权是可以转移的。

如果您拥有一个值，例如，您可以将所有权转移给另一个变量：

```rust
let a = "hello, world".to_string(); // <- `a` 是这个字符串的所有者
let b = a;  // <- 现在 `b` 是这个字符串的所有者
```

Rust 的所有权系统嵌入在类型系统中：每个函数必须在其签名中声明 _如何_ 与其参数交互。

到目前为止，我们的方法和函数都是 **消费** 它们的参数：它们接管了参数的所有权。例如：

```rust
impl Ticket {
    pub fn description(self) -> String {
        self.description
    }
}
```

`Ticket::description` 接管了被调用的 `Ticket` 实例的所有权。\
这被称为 **移动语义**：值（`self`）的所有权从调用方 **转移** 到被调用方，调用方失去了对它的使用权。

这与我们之前在错误信息中看到的编译器使用的语言完全一致：

```text
error[E0382]: use of moved value: `ticket`
  --> src/main.rs:30:43
   |
25 |     let ticket = Ticket::new(/* */);
   |         ------ move occurs because `ticket` has type `Ticket`, 
   |                which does not implement the `Copy` trait
26 |     if ticket.status() == "To-Do" {
   |               -------- `ticket` moved due to this method call
...
30 |         println!("Your next task is: {}", ticket.title());
   |                                           ^^^^^^ 
   |                                 value used here after move
   |
note: `Ticket::status` takes ownership of the receiver `self`, 
      which moves `ticket`
  --> src/main.rs:12:23
   |
12 |         pub fn status(self) -> String {
   |                       ^^^^
```

特别是，当我们调用 `ticket.status()` 时，会按以下顺序发生这些事件：

- `Ticket::status` 接管 `Ticket` 实例的所有权
- `Ticket::status` 从 `self` 中提取 `status`，并将 `status` 的所有权传回给调用方
- 剩余的 `Ticket` 实例被丢弃（包括 `title` 和 `description`）

当我们试图通过 `ticket.title()` 再次使用 `ticket` 时，编译器会报错：`ticket` 的值已经消失，我们不再拥有它，因此不能再使用它。

要构建 _有用的_ 访问器方法，我们需要开始使用 **引用**。

## 借用

理想情况下，我们希望有一些方法可以读取变量的值，而无需获取其所有权。\
否则，编程将受到很大的限制。在 Rust 中，这通过 **借用** 实现。

每当借用一个值时，您会得到该值的一个 **引用**。\
引用包含一些权限标签[^refine]：

- 不可变引用（`&`）允许您读取值，但不能修改它
- 可变引用（`&mut`）允许您读取并修改值

回到 Rust 所有权系统的目标：

- 数据在被读取时不会被修改
- 数据在被修改时不会被读取

为了确保这两条规则，Rust 对引用引入了一些限制：

- 同一时间，不能同时存在一个值的可变引用和不可变引用
- 同一时间，一个值不能有多个可变引用
- 在被借用时，所有者不能修改值
- 您可以有任意数量的不可变引用，只要不存在可变引用

某种程度上，您可以将不可变引用视为值的“只读”锁，而可变引用则类似于“读写”锁。

所有这些限制都由借用检查器在编译时强制执行。

### 语法

在实践中，如何借用一个值？\
通过在变量 **前面加 `&` 或 `&mut`** 来借用其值。但请注意！相同的符号（`&` 和 `&mut`）在类型 **前面** 有不同的含义：它们表示对原始类型的引用。

例如：

```rust
struct Configuration {
    version: u32,
    active: bool,
}

fn main() {
    let config = Configuration {
        version: 1,
        active: true,
    };
    // `b` 是对 `config` 的 `version` 字段的引用。
    // `b` 的类型是 `&u32`，因为它存储了对 `u32` 值的引用。
    // 我们通过使用 `&` 操作符借用 `config.version` 来创建引用。
    // 相同的符号（`&`），具体含义取决于上下文！
    let b: &u32 = &config.version;
    //     ^ 类型标注不是必须的，
    //       仅用于便于理解
}
```

这同样适用于函数的参数和返回类型：

```rust
// `f` 接受一个可变引用到一个 `u32` 作为参数，并绑定到名称 `number`
fn f(number: &mut u32) -> &u32 {
    // [...]
}
```

## 深呼吸

Rust 的所有权系统刚开始可能令人难以适应。\
但别担心：通过实践，它会变得自然而然。\
在本章以及整门课程的余下部分，您将有大量的实践机会！我们会多次回顾每个概念，确保您熟悉并真正理解它们的工作原理。

到本章最后，我们会解释为什么 Rust 的所有权系统是这样设计的。\
目前，请专注于理解 _如何_ 使用它。将每个编译器错误视为一个学习的机会！

[^refine]: 这是一个很好的入门心智模型，但它并未完全覆盖所有细节。后续章节中我们会[进一步完善](../../../Threads/Interior%20mutability/Theory/task.md)对引用的理解。