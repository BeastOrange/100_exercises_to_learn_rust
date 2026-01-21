## 生命周期

让我们尝试通过为 `&TicketStore` 添加 `IntoIterator` 的实现来完成上一个练习，这样可以在 `for` 循环中提供最大的便利。

让我们先填写实现中最“显而易见”的部分：

```rust
impl IntoIterator for &TicketStore {
    type Item = &Ticket;
    type IntoIter = // 这里应该填写什么？

    fn into_iter(self) -> Self::IntoIter {
        self.tickets.iter()
    }
}
```

`type IntoIter` 应该设置为什么？\
直觉上，它应该是 `self.tickets.iter()` 返回的类型，也就是 `Vec::iter()` 返回的类型。\
如果你查看标准库文档，就会发现 `Vec::iter()` 返回的是一个 `std::slice::Iter`。\
`Iter` 的定义是：

```rust
pub struct Iter<'a, T> { /* fields omitted */ }
```

`'a` 是一个 **生命周期参数**。

## 生命周期参数

生命周期是 Rust 编译器用来跟踪一个引用（无论是可变的还是不可变的）有效时长的 **标签**。\
一个引用的生命周期受它所引用的值的作用域限制。Rust 会在编译期间确保引用不会在其所引用的值被释放后使用，以避免悬空指针和使用已释放内存的错误。

这应该听起来很熟悉：我们在讨论所有权和借用时已经见过这些概念。生命周期只是用来 **命名** 一个特定引用的有效时间。

命名变得重要是因为当你有多个引用时，你需要明确它们 **如何相互关联**。让我们看一下 `Vec::iter()` 的签名：

```rust
impl <T> Vec<T> {
    // 略微简化版本
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        // [...]
    }
}
```

`Vec::iter()` 针对一个名为 `'a` 的生命周期参数是泛型的。\
`'a` 用来 **绑定** `Vec` 的生命周期和 `iter()` 返回的 `Iter` 的生命周期。\
用大白话来说：`iter()` 返回的 `Iter` 不能超过创建它的 `Vec` 引用（`&self`）的生命周期。

这点非常重要，因为我们讨论过 `Vec::iter` 返回的是对 `Vec` 元素的**引用**的迭代器。\
如果 `Vec` 被释放，迭代器返回的引用将变得无效。Rust 必须确保这种情况不会发生，而生命周期就是它用来执行这一规则的工具。

## 生命周期省略

Rust 有一套规则，称为 **生命周期省略规则**，在许多情况下允许你省略显式的生命周期标注。\
例如，`Vec::iter` 的定义在标准库源码中看起来是这样的：

```rust
impl <T> Vec<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        // [...]
    }
}
```

`Vec::iter()` 的签名中没有显式的生命周期参数。\
省略规则隐含了 `iter()` 返回的 `Iter` 的生命周期与 `&self` 引用的生命周期绑定。\
你可以将 `'_` 视为 `&self` 引用生命周期的 **占位符**。

参见 [引用](#references) 部分以获得关于生命周期省略的官方文档链接。\
在大多数情况下，你可以依赖编译器来告诉你何时需要添加显式的生命周期标注。

## 参考

- [std::vec::Vec::iter](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.iter)
- [std::slice::Iter](https://doc.rust-lang.org/std/slice/struct.Iter.html)
- [生命周期省略规则](https://doc.rust-lang.org/reference/lifetime-elision.html)