## `Deref` 特性

在上一节练习中，你几乎没做什么操作，对吧？

将代码从

```rust
impl Ticket {
    pub fn title(&self) -> &String {
        &self.title
    }
}
```

改为

```rust
impl Ticket {
    pub fn title(&self) -> &str {
        &self.title
    }
}
```

就足以使代码编译通过并通过测试了。  
不过，这里应该引起你的警觉。

## 它本不该工作，但它确实有效

让我们回顾一下事实：

- `self.title` 是一个 `String`
- 因此，`&self.title` 是一个 `&String`
- 修改后的 `title` 方法返回值是 `&str`

按理说，你会期待编译器抛出一个错误，对吧？比如“期待 `&String`，但找到 `&str`”之类的错误。  
但实际上，它就这样运行了。**为什么**？

## `Deref` 来解围

`Deref` 特性是支持 [**解引用强制转换**](https://doc.rust-lang.org/std/ops/trait.Deref.html#deref-coercion)这一语言特性的机制。  
此特性定义在标准库的 `std::ops` 模块中：

```rust
// 为简单起见，我稍微简化了定义。
// 在后面我们会看到完整的定义。
pub trait Deref {
    type Target;
    
    fn deref(&self) -> &Self::Target;
}
```

`type Target` 是一个 **关联类型**。  
它是一个占位符，表示在实现该特性时必须指定的具体类型。

## 解引用强制转换

通过为某类型 `T` 实现 `Deref<Target = U>`，你告诉编译器 `&T` 和 `&U` 在某种程度上是可以互换的。  
具体来说，你可以获得如下行为：

- 对 `T` 的引用会被隐式转换为对 `U` 的引用（即 `&T` 被转换为 `&U`）。
- 你可以在 `&T` 上调用所有定义在 `U` 上、以 `&self` 作为输入的方法。

还有一个与解引用操作符 `*` 相关的内容，但目前我们不需要了解它（如有兴趣，请查阅 `std` 的文档）。

## `String` 实现了 `Deref`

`String` 实现了 `Deref`，并将 `Target` 定义为 `str`：

```rust
impl Deref for String {
    type Target = str;
    
    fn deref(&self) -> &str {
        // [...]
    }
}
```

得益于这个实现和解引用强制转换，当需要时，`&String` 会自动转换为 `&str`。

## 不要滥用解引用强制转换

解引用强制转换是一项强大的功能，但它可能会导致混淆。  
类型的自动转换可能会让代码变得难以阅读和理解。如果在 `T` 和 `U` 上都定义了同名方法，那究竟会调用哪一个？

稍后在课程中，我们会探讨解引用强制转换的“最安全”使用场景：智能指针。