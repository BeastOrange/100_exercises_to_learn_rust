## `From` 和 `Into`

让我们回到字符串旅程的起点：

```rust
let ticket = Ticket::new(
    "A title".into(), 
    "A description".into(), 
    "To-Do".into()
);
```

现在我们已经掌握了足够的知识，可以开始解析这里的 `.into()` 是在做什么。

## 问题

这是 `new` 方法的签名：

```rust
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

我们也已经看到，字符串字面量（如 `"A title"`）的类型是 `&str`。\
但这里存在类型不匹配：方法期望 `String` 类型，而我们提供的是 `&str` 类型。\
这次没有魔法转换来拯救我们了；我们需要**执行一个转换**。

## `From` 和 `Into`

Rust 标准库在 `std::convert` 模块中定义了两个用于**无失败转换**的特性：`From` 和 `Into`。

```rust
pub trait From<T>: Sized {
    fn from(value: T) -> Self;
}

pub trait Into<T>: Sized {
    fn into(self) -> T;
}
```

这些特性的定义展示了一些我们尚未了解的概念：**超特性**和**隐式特性边界**。\
我们先来解析这些内容。

### 超特性 / 子特性

`From: Sized` 语法暗示 `From` 是 `Sized` 的**子特性**：任何实现了 `From` 的类型也必须实现 `Sized`。\
或者可以说，`Sized` 是 `From` 的一个**超特性**。

### 隐式特性边界

每当你使用一个泛型类型参数时，编译器会隐式假设它是 `Sized` 类型。

例如：

```rust
pub struct Foo<T> {
    inner: T,
}
```

实际等效于：

```rust
pub struct Foo<T: Sized> {
    inner: T,
}
```

对于 `From<T>` 而言，特性定义等效于：

```rust
pub trait From<T: Sized>: Sized {
    fn from(value: T) -> Self;
}
```

换句话说，_T_ 和实现了 `From<T>` 的类型都必须是 `Sized`，即便前者的边界是隐式的。

### 否定特性边界

你可以使用**否定特性边界**来取消隐式的 `Sized` 限制：

```rust
pub struct Foo<T: ?Sized> {
    //            ^^^^^^^
    //            这是一个否定特性边界
    inner: T,
}
```

这个语法表示 "`T` 可以是也可以不是 `Sized`"，这样就允许你将 `T` 绑定为一个 DST（例如 `Foo<str>`）。\
但这是一个特例：否定特性边界仅适用于 `Sized`，不能用于其他特性。

## `&str` 到 `String`

在 [`std` 文档](https://doc.rust-lang.org/std/convert/trait.From.html#implementors)中，\
你可以查看 `std` 中哪些类型实现了 `From` 特性。\
你会发现，`String` 实现了 `From<&str> for String`。因此，我们可以写：

```rust
let title = String::from("A title");
```

不过我们一直在使用 `.into()`。\
如果查看 [`Into` 的实现者](https://doc.rust-lang.org/std/convert/trait.Into.html#implementors)，\
你不会找到 `Into<String> for &str`。这是怎么回事？

`From` 和 `Into` 是**对偶特性**。\
具体来说，对任何实现了 `From` 的类型，`Into` 都会通过**通用实现**被自动实现：

```rust
impl<T, U> Into<U> for T
where
    U: From<T>,
{
    fn into(self) -> U {
        U::from(self)
    }
}
```

如果一个类型 `U` 实现了 `From<T>`，那么 `Into<U> for T` 会被自动实现。这就是为什么我们可以写 `let title = "A title".into();`。

## `.into()`

每次看到 `.into()`，实际上就是在观察一种类型之间的转换。\
那么目标类型是什么呢？

在大多数情况下，目标类型会通过以下两种方式之一指定：

- 通过函数/方法的签名指定（例如上面例子中的 `Ticket::new`）
- 在变量声明中通过类型注解指定（例如 `let title: String = "A title".into();`）

只要编译器可以从上下文中无歧义地推断出目标类型，`.into()` 就可以直接使用。