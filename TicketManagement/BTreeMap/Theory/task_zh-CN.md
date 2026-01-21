## 排序

通过将存储从 `Vec` 转换为 `HashMap`，我们提升了票务管理系统的性能，同时简化了代码。\
但这并非全是好处。当遍历基于 `Vec` 的存储时，我们可以确信票据的返回顺序与添加顺序一致。\
而对于 `HashMap` 来说情况并非如此：虽然也可以遍历票据，但顺序是随机的。

通过将存储从 `HashMap` 切换为 `BTreeMap`，我们可以恢复一致的顺序。

## `BTreeMap`

`BTreeMap` 保证条目按键排序。\
当你需要按照特定顺序遍历条目，或者需要执行范围查询（例如，“给我 id 在 10 和 20 之间的所有票据”）时，这非常有用。

与 `HashMap` 类似，你不会在 `BTreeMap` 的定义中找到特征约束。\
但你会在它的方法中看到特征约束。让我们看看 `insert` 方法：

```rust
// `K` 和 `V` 分别代表键和值的类型，
// 和 `HashMap` 中一样。
impl<K, V> BTreeMap<K, V> {
    pub fn insert(&mut self, key: K, value: V) -> Option<V>
    where
        K: Ord,
    {
        // 实现
    }
}
```

此时不再需要实现 `Hash`，取而代之的是，键的类型必须实现 `Ord` 特征。

## `Ord`

`Ord` 特征用来比较值的大小关系。\
在 `PartialEq` 用于比较是否相等时，`Ord` 用于比较大小顺序。

它在 `std::cmp` 中定义：

```rust
pub trait Ord: Eq + PartialOrd {
    fn cmp(&self, other: &Self) -> Ordering;
}
```

`cmp` 方法返回一个 `Ordering` 枚举，其值可能是 `Less`、`Equal` 或 `Greater`。\
`Ord` 要求实现两个其他特征：`Eq` 和 `PartialOrd`。

## `PartialOrd`

`PartialOrd` 是 `Ord` 的弱化版本，就像 `PartialEq` 是 `Eq` 的弱化版本一样。\
通过查看它的定义你就能明白为什么：

```rust
pub trait PartialOrd: PartialEq {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>;
}
```

`PartialOrd::partial_cmp` 返回一个 `Option`——并不能保证两个值总是可以比较。\
例如，`f32` 没有实现 `Ord`，因为 `NaN` 值无法比较，这也是 `f32` 没有实现 `Eq` 的原因。

## 实现 `Ord` 和 `PartialOrd`

你可以为你的类型派生 `Ord` 和 `PartialOrd`：

```rust
// 你需要同时添加 `Eq` 和 `PartialEq`，
// 因为 `Ord` 需要它们。
#[derive(Eq, PartialEq, Ord, PartialOrd)]
struct TicketId(u64);
```

如果你选择（或需要）手动实现它们，请务必小心：

- `Ord` 和 `PartialOrd` 必须和 `Eq` 和 `PartialEq` 保持一致。
- `Ord` 和 `PartialOrd` 必须彼此一致。