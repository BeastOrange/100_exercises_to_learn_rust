## 复制值，第一部分

在上一章中，我们介绍了所有权和借用的概念。\
我们特别提到：

- Rust 中的每个值在任何时候都有唯一的所有者。
- 当一个函数获得一个值的所有权（“它消费了它”）时，调用者将无法继续使用该值。

这些限制可能会有些局限。\
有时我们可能需要调用一个会占用某个值所有权的函数，但随后我们仍需要继续使用该值。

```rust
fn consumer(s: String) { /* */ }

fn example() {
     let mut s = String::from("hello");
     consumer(s);
     s.push_str(", world!"); // 错误：值在移动后被借用
}
```

这时 `Clone` 就派上了用场。

## `Clone`

`Clone` 是一个定义在 Rust 标准库中的 trait：

```rust
pub trait Clone {
    fn clone(&self) -> Self;
}
```

它的方法 `clone` 接受对 `self` 的引用，并返回一个新**拥有所有权**的相同类型实例。

## 实际应用

回到上面的例子，我们可以使用 `clone` 在调用 `consumer` 之前创建一个新的 `String` 实例：

```rust
fn consumer(s: String) { /* */ }

fn example() {
     let mut s = String::from("hello");
     let t = s.clone();
     consumer(t);
     s.push_str(", world!"); // 无错误
}
```

我们并没有将 `s` 的所有权交给 `consumer`，而是通过克隆 `s` 创建了一个新的 `String`，并将它交给 `consumer`。\
调用 `consumer` 后，`s` 依然有效并可继续使用。

## 内存中的变化

让我们看看上例在内存中的变化。\
当执行 `let mut s = String::from("hello");` 时，内存结构如下：

```text
                    s
      +---------+--------+----------+
Stack | pointer | length | capacity | 
      |  |      |   5    |    5     |
      +--|------+--------+----------+
         |
         |
         v
       +---+---+---+---+---+
Heap:  | H | e | l | l | o |
       +---+---+---+---+---+
```

当执行 `let t = s.clone();` 时，将在堆上分配一个全新的区域来存储数据的副本：

```text
                    s                                    t
      +---------+--------+----------+      +---------+--------+----------+
Stack | pointer | length | capacity |      | pointer | length | capacity |
      |  |      |   5    |    5     |      |  |      |   5    |    5     |
      +--|------+--------+----------+      +--|------+--------+----------+
         |                                    |
         |                                    |
         v                                    v
       +---+---+---+---+---+                +---+---+---+---+---+
Heap:  | H | e | l | l | o |                | H | e | l | l | o |
       +---+---+---+---+---+                +---+---+---+---+---+
```

如果你来自像 Java 这样的语言，可以把 `clone` 看作是一种创建对象深拷贝的方法。

## 实现 `Clone`

要让一个类型支持 `Clone`，我们需要为它实现 `Clone` trait。\
我们几乎总是通过派生来实现 `Clone`：

```rust
#[derive(Clone)]
struct MyType {
    // 字段
}
```

编译器会按照预期为 `MyType` 实现 `Clone`：它会逐个克隆 `MyType` 的每个字段，\
然后使用已克隆的字段构造一个新的 `MyType` 实例。\
记住，你可以使用 `cargo expand`（或你的 IDE）来查看由派生宏生成的代码。