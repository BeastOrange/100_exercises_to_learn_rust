## 实现 trait

当一个类型定义在另一个 crate 中（例如，`u32` 来自 Rust 的标准库），你无法直接为它定义新方法。如果你尝试这样做：

```rust
impl u32 {
    fn is_even(&self) -> bool {
        self % 2 == 0
    }
}
```

编译器会报错：

```text
error[E0390]: cannot define inherent `impl` for primitive types
  |
1 | impl u32 {
  | ^^^^^^^^
  |
  = help: consider using an extension trait instead
```

## 扩展 trait

**扩展 trait** 是一种 trait，其主要目的是为外部类型（例如 `u32`）附加新方法。
这正是你在前一个练习中采用的模式，通过定义 `IsEven` trait 并为 `i32` 和 `u32` 实现它。只要 `IsEven` 在作用域中，你就可以自由地在这些类型上调用 `is_even`。

```rust
// 引入 trait 到作用域中
use my_library::IsEven;

fn main() {
    // 在支持该 trait 的类型上调用它的方法
    if 4.is_even() {
        // [...]
    }
}
```

## 单一实现

在编写 trait 实现时会有一些限制。\
最简单明了的限制是：你不能在一个 crate 中为同一类型实现同一个 trait 两次。

例如：

```rust
trait IsEven {
    fn is_even(&self) -> bool;
}

impl IsEven for u32 {
    fn is_even(&self) -> bool {
        true
    }
}

impl IsEven for u32 {
    fn is_even(&self) -> bool {
        false
    }
}
```

编译器会拒绝这种情况：

```text
error[E0119]: conflicting implementations of trait `IsEven` for type `u32`
   |
5  | impl IsEven for u32 {
   | ------------------- first implementation here
...
11 | impl IsEven for u32 {
   | ^^^^^^^^^^^^^^^^^^^ conflicting implementation for `u32`
```

当在一个 `u32` 值上调用 `IsEven::is_even` 时，不能出现模糊不清的情况，因此一个类型只能有一个 trait 实现。

## 孤儿规则

当涉及多个 crate 时，情况会变得更复杂。
特别是，以下条件中至少有一个必须成立：

- 该 trait 是在当前 crate 中定义的
- 该实现类型是当前 crate 中定义的

这被称为 Rust 的**孤儿规则**（orphan rule）。它的目的是使方法解析过程不产生歧义。

想象以下情形：

- Crate `A` 定义了 `IsEven` trait
- Crate `B` 为 `u32` 实现了 `IsEven`
- Crate `C` 为 `u32` 提供了另一个（不同的）`IsEven` 实现
- Crate `D` 同时依赖于 `B` 和 `C`，并调用 `1.is_even()`

应该使用哪个实现呢？`B` 中定义的那个？还是 `C` 中定义的那个？\
这个问题没有明确的答案，因此孤儿规则被定义出来以避免这种情况。\
由于孤儿规则，`B` 和 `C` 中的 crate 都无法编译。

## 延伸阅读

- 关于孤儿规则还有一些细微差别和例外情况，如需了解更多，请参考 [官方文档](https://doc.rust-lang.org/reference/items/implementations.html#trait-implementation-coherence)。