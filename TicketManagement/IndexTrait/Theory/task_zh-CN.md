## 索引

`TicketStore::get` 方法会根据给定的 `TicketId` 返回一个 `Option<&Ticket>`。  
我们之前已经了解了如何使用 Rust 的索引语法访问数组和向量中的元素：

```rust
let v = vec![0, 1, 2];
assert_eq!(v[0], 0);
```

我们如何为 `TicketStore` 提供相同的体验呢？  
你猜对了：我们需要实现一个特质（trait），`Index`！

## `Index`

`Index` 特质是由 Rust 的标准库定义的：

```rust
// 略微简化版
pub trait Index<Idx>
{
    type Output;

    // 必须实现的方法
    fn index(&self, index: Idx) -> &Self::Output;
}
```

它包含：

- 一个泛型参数 `Idx`，用于表示索引的类型  
- 一个关联类型 `Output`，表示我们通过索引获取的类型  

请注意，`index` 方法并不会返回一个 `Option`。它假设如果你尝试访问不存在的元素时，`index` 会触发 panic，就像数组和向量的索引操作一样。