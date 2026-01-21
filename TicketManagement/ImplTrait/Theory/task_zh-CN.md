## `impl Trait`

`TicketStore::to_dos` 返回一个 `Vec<&Ticket>`。\
这个函数签名每次调用 `to_dos` 时都会引入一次新的堆分配，而这可能是多余的，具体取决于调用者如何处理返回结果。\
如果 `to_dos` 返回的是一个迭代器而不是 `Vec`，会更好一些，这样可以让调用者自行决定是将结果收集到 `Vec` 中还是直接遍历它。

不过这有点棘手！\
以下实现中，`to_dos` 的返回类型是什么？

```rust
impl TicketStore {
    pub fn to_dos(&self) -> ??? {
        self.tickets.iter().filter(|t| t.status == Status::ToDo)
    }
}
```

## 不可命名的类型

`filter` 方法返回一个 `std::iter::Filter` 实例，其定义如下：

```rust
pub struct Filter<I, P> { /* fields omitted */ }
```

其中，`I` 是正在被过滤的迭代器的类型，`P` 是用于过滤元素的谓词。\
在这个例子中，我们知道 `I` 是 `std::slice::Iter<'_, Ticket>`，但 `P` 是什么呢？\
`P` 是一个闭包，即 **匿名函数**。顾名思义，闭包是没有名字的，所以我们无法在代码中直接写出它的类型。

Rust 为此提供了一个解决方案：**impl Trait**。

## `impl Trait`

`impl Trait` 是一种功能，可以让你在返回类型时不必指定具体的类型名称。\
你只需声明该类型实现了什么 trait，剩下的由 Rust 自动推断。

在本例中，我们希望返回一个指向 `Ticket` 的引用的迭代器：

```rust
impl TicketStore {
    pub fn to_dos(&self) -> impl Iterator<Item = &Ticket> {
        self.tickets.iter().filter(|t| t.status == Status::ToDo)
    }
}
```

就这样！

## 泛型？

返回位置的 `impl Trait` **不是**一个泛型参数。

泛型是类型占位符，会在函数调用时由调用者提供具体的类型。\
带有泛型参数的函数是 **多态的**：它可以通过不同类型调用，并且编译器会为每种类型生成不同的实现。

而 `impl Trait` 并不是这样。\
一个带有 `impl Trait` 的函数的返回类型在编译时是 **固定的**，编译器仅会为它生成一个实现。\
这也是为什么 `impl Trait` 被称为 **不透明返回类型**：调用者无法知道返回值的确切类型，只知道它实现了指定的 trait。但编译器知道确切的类型，且不涉及多态性。

## RPIT

如果你阅读关于 Rust 的 RFC 或深度解析文章，可能会看到缩写 **RPIT**。\
它代表 **"Return Position Impl Trait"（返回位置的 Impl Trait）**，指代在返回位置上使用 `impl Trait` 的场景。