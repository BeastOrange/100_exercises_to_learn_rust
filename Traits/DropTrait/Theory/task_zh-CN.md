## `Drop` 特性

当我们介绍[析构函数](../../../Ticket%20v1/Destructors/Theory/task.md)时，提到过 `drop` 函数的作用：

1. 回收被类型占用的内存（即 `std::mem::size_of` 字节）
2. 清理值可能管理的任何额外资源（例如 `String` 的堆缓冲区）

步骤2. 就是 `Drop` 特性派上用场的地方。

```rust
pub trait Drop {
    fn drop(&mut self);
}
```

`Drop` 特性是一种机制，允许你为你的类型定义_额外的_清理逻辑，\
超出编译器自动为你执行的部分。\
无论你在 `drop` 方法中放入什么逻辑，都会在值超出作用域时被执行。

## `Drop` 和 `Copy`

在谈到 `Copy` 特性时，我们提到如果一个类型管理超出其占用内存的 `std::mem::size_of` 字节的额外资源，那么它就不能实现 `Copy`。

你可能会好奇：编译器怎么知道一个类型是否管理了额外资源呢？\
没错：通过 `Drop` 特性的实现！\
如果你的类型有一个显式的 `Drop` 实现，编译器会假设你的类型附带了额外资源，因此不会允许你实现 `Copy`。

```rust
// 这是一个单元结构体，即没有字段的结构体。
#[derive(Clone, Copy)]
struct MyType;

impl Drop for MyType {
    fn drop(&mut self) {
       // 这里我们不需要做任何事情，
       // 拥有一个“空”的 Drop 实现就足够了
    }
}
```

编译器会报如下错误信息：

```text
error[E0184]: the trait `Copy` cannot be implemented for this type; 
              the type has a destructor
 --> src/lib.rs:2:17
  |
2 | #[derive(Clone, Copy)]
  |                 ^^^^ `Copy` not allowed on types with destructors