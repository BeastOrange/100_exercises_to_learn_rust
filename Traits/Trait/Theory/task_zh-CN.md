## 特性 (Traits)

我们再来看一下我们的 `Ticket` 类型：

```rust
pub struct Ticket {
    title: String,
    description: String,
    status: String,
}
```

到目前为止，所有的测试都是通过使用 `Ticket` 的字段进行断言。

```rust
assert_eq!(ticket.title(), "A new title");
```

如果我们希望直接比较两个 `Ticket` 实例呢？

```rust
let ticket1 = Ticket::new(/* ... */);
let ticket2 = Ticket::new(/* ... */);
ticket1 == ticket2
```

编译器会阻止我们这样做：

```text
error[E0369]: binary operation `==` cannot be applied to type `Ticket`
  --> src/main.rs:18:13
   |
18 |     ticket1 == ticket2
   |     ------- ^^ ------- Ticket
   |     |
   |     Ticket
   |
note: an implementation of `PartialEq` might be missing for `Ticket`
```

`Ticket` 是一个自定义类型。默认情况下，它**没有任何行为附加到类型上**。\
Rust 并不会因为它包含 `String` 字段而自动推断出如何比较两个 `Ticket` 实例。

不过，Rust 编译器已经给出了正确的指引：提示我们可能缺少对 `PartialEq` 的实现。\
`PartialEq` 是一个**特性 (trait)**！

## 什么是特性 (Traits)？

特性 (Traits) 是 Rust 定义**接口**的方式。\
特性定义了一组方法，类型必须实现这些方法才能满足特性的约定。

### 定义特性

定义特性的语法如下：

```rust
trait <TraitName> {
    fn <method_name>(<parameters>) -> <return_type>;
}
```

举个例子，我们可以定义一个名为 `MaybeZero` 的特性，要求实现者定义一个 `is_zero` 方法：

```rust
trait MaybeZero {
    fn is_zero(self) -> bool;
}
```

### 实现特性

要为某个类型实现特性，我们使用 `impl` 关键字，就像定义普通[^inherent]方法一样，
但语法稍有不同：

```rust
impl <TraitName> for <TypeName> {
    fn <method_name>(<parameters>) -> <return_type> {
        // 方法体
    }
}
```

例如，为自定义数字类型 `WrappingU32` 实现 `MaybeZero` 特性：

```rust
pub struct WrappingU32 {
    inner: u32,
}

impl MaybeZero for WrappingU32 {
    fn is_zero(self) -> bool {
        self.inner == 0
    }
}
```

### 调用特性方法

调用特性方法时，我们使用 `.` 操作符，与调用普通方法一样：

```rust
let x = WrappingU32 { inner: 5 };
assert!(!x.is_zero());
```

调用特性方法时，需要满足两个条件：

- 类型必须实现该特性。
- 特性必须在作用域内。

为满足第二条件，可能需要为该特性添加一个 `use` 语句：

```rust
use crate::MaybeZero;
```

以下情况则不需要：

- 特性在调用发生的同一模块中定义。
- 特性定义在标准库的**预导入模块 (prelude)** 中。\
  预导入模块是一个默认自动导入到每个 Rust 程序中的特性和类型集合。\
  它相当于在每个 Rust 模块的开头自动添加了 `use std::prelude::*;`。

您可以在 [Rust 文档](https://doc.rust-lang.org/std/prelude/index.html) 中找到预导入模块中包含的特性和类型列表。

[^inherent]: 直接在类型上定义的方法，不需要使用特性，也称为**固有方法 (inherent method)**。