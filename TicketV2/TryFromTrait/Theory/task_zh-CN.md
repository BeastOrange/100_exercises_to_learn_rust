## `TryFrom` 和 `TryInto`

在上一章中，我们学习了 Rust 用于**无错误**类型转换的惯用接口 [`From` 和 `Into` traits](../../../Traits/From%20trait/Theory/task.md)。\
但如果转换操作不能保证成功会怎么样呢？

现在我们已经对错误有了一定的了解，可以来讨论 `From` 和 `Into` 的**可能失败**的对应版本：`TryFrom` 和 `TryInto`。

### `TryFrom` 和 `TryInto`

与 `From` 和 `Into` 一样，`TryFrom` 和 `TryInto` 都定义在 `std::convert` 模块中。

```rust
pub trait TryFrom<T>: Sized {
    type Error;
    fn try_from(value: T) -> Result<Self, Self::Error>;
}

pub trait TryInto<T>: Sized {
    type Error;
    fn try_into(self) -> Result<T, Self::Error>;
}
```

`From`/`Into` 和 `TryFrom`/`TryInto` 的主要区别在于后者返回 `Result` 类型。\
这使得转换操作可以失败，返回错误而不是触发 panic。

## `Self::Error`

`TryFrom` 和 `TryInto` 都有一个关联的 `Error` 类型。\
这允许每个实现指定其自己的错误类型，理想情况下是最适合当前尝试转换的类型。

`Self::Error` 是一种引用 trait 中定义的关联类型 `Error` 的方式。

## 对偶性

与 `From` 和 `Into` 一样，`TryFrom` 和 `TryInto` 是对偶的 traits。\
如果你为某一类型实现了 `TryFrom`，那么你会自动获得 `TryInto` 的实现。