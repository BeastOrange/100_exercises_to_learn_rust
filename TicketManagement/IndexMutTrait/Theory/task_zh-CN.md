## 可变索引

`Index` 允许只读访问。它不允许你修改你检索到的值。

## `IndexMut`

如果你想允许可变性，你需要实现 `IndexMut` trait。

```rust
// 略作简化
pub trait IndexMut<Idx>: Index<Idx>
{
    // 必须实现的方法
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output;
}
```

`IndexMut` 只能在类型已经实现了 `Index` 的情况下实现，因为它解锁了一个_额外_的功能。