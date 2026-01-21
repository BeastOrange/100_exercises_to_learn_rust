## 可变切片

每当我们讨论切片类型（例如 `str` 和 `[T]`）时，我们使用的都是它们的不可变借用形式（`&str` 和 `&[T]`）。  
但是，切片也可以是可变的！

以下是创建可变切片的方法：

```rust
let mut numbers = vec![1, 2, 3];
let slice: &mut [i32] = &mut numbers;
```

然后你可以修改切片中的元素：

```rust
slice[0] = 42;
```

这会将 `Vec` 的第一个元素修改为 `42`。

## 限制

在处理不可变借用时，建议是明确的：优先使用切片引用而不是引用拥有类型（例如 `&[T]` 而不是 `&Vec<T>`）。  
但是，这对可变借用并不适用。

考虑以下场景：

```rust
let mut numbers = Vec::with_capacity(2);
let mut slice: &mut [i32] = &mut numbers;
slice.push(1);
```

这段代码无法编译！  
`push` 是 `Vec` 的方法，而不是切片的方法。这体现了一条更普遍的原则：Rust 不允许你向切片添加或移除元素。你只能修改/替换切片中已有的元素。

在这一点上，`&mut Vec` 或 `&mut String` 的功能显然比 `&mut [T]` 或 `&mut str` 更强大。  
根据你需要执行的操作选择最合适的类型。