## 运算符重载

现在我们对特性（trait）有了一个基本的理解，让我们回到**运算符重载**。
运算符重载是为类似 `+`、`-`、`*`、`/`、`==`、`!=` 等运算符定义自定义行为的能力。

## 运算符是特性

在 Rust 中，运算符是特性。\
对于每个运算符，都有一个对应的特性定义了该运算符的行为。\
通过为你的类型实现该特性，你可以**解锁**相应运算符的使用。

例如，[`PartialEq` 特性](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html)定义了 `==` 和 `!=` 运算符的行为：

```rust
// Rust 标准库中的 `PartialEq` 特性定义
// （这里稍微进行了简化）
pub trait PartialEq {
    // 必需方法
    //
    // `Self` 是一个 Rust 关键字，代表
    // “实现该特性的类型”
    fn eq(&self, other: &Self) -> bool;

    // 提供的方法
    fn ne(&self, other: &Self) -> bool { ... }
}
```

当你写 `x == y` 时，编译器会查找 `x` 和 `y` 类型的 `PartialEq` 特性实现，并将 `x == y` 替换为 `x.eq(y)`。这是一种语法糖！

以下是主要运算符与特性的对应关系：

| 运算符                  | 特性                                                                  |
| ------------------------ | --------------------------------------------------------------------- |
| `+`                      | [`Add`](https://doc.rust-lang.org/std/ops/trait.Add.html)             |
| `-`                      | [`Sub`](https://doc.rust-lang.org/std/ops/trait.Sub.html)             |
| `*`                      | [`Mul`](https://doc.rust-lang.org/std/ops/trait.Mul.html)             |
| `/`                      | [`Div`](https://doc.rust-lang.org/std/ops/trait.Div.html)             |
| `%`                      | [`Rem`](https://doc.rust-lang.org/std/ops/trait.Rem.html)             |
| `==` 和 `!=`             | [`PartialEq`](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html) |
| `<`、`>`、`<=` 和 `>=`   | [`PartialOrd`](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html) |

算术运算符位于 [`std::ops`](https://doc.rust-lang.org/std/ops/index.html) 模块中，
而比较运算符位于 [`std::cmp`](https://doc.rust-lang.org/std/cmp/index.html) 模块中。

## 默认实现

关于 `PartialEq::ne` 的注释指出，"`ne` 是一个提供的方法（provided method）"。\
这意味着 `PartialEq` 在特性定义中为 `ne` 提供了一个**默认实现**——定义代码片段中省略的 `{ ... }` 块。\
如果展开省略的块，它看起来像这样：

```rust
pub trait PartialEq {
    fn eq(&self, other: &Self) -> bool;

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}
```

这正是你所期望的：`ne` 是 `eq` 的取反。\
由于提供了默认实现，当你为你的类型实现 `PartialEq` 时可以跳过实现 `ne`。只需实现 `eq` 即可：

```rust
struct WrappingU8 {
    inner: u8,
}

impl PartialEq for WrappingU8 {
    fn eq(&self, other: &WrappingU8) -> bool {
        self.inner == other.inner
    }
    
    // 这里没有实现 `ne`
}
```

不过，你并不是必须使用默认实现。\
在实现特性时，你可以选择重写它：

```rust
struct MyType;

impl PartialEq for MyType {
    fn eq(&self, other: &MyType) -> bool {
        // 自定义实现
    }

    fn ne(&self, other: &MyType) -> bool {
        // 自定义实现
    }
}