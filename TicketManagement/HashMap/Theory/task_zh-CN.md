## `HashMap`

我们对 `Index`/`IndexMut` 的实现并不理想：我们需要遍历整个 `Vec` 才能通过 id 检索到一张票据；其算法复杂度是 `O(n)`，其中 `n` 是存储中票据的数量。

通过使用一种不同的数据结构来存储票据，我们可以做得更好：`HashMap<K, V>`。

```rust
use std::collections::HashMap;

// 类型推断使我们可以省略显式的类型签名
// （在这个例子中是 `HashMap<String, String>`）。
let mut book_reviews = HashMap::new();

book_reviews.insert(
    "Adventures of Huckleberry Finn".to_string(),
    "My favorite book.".to_string(),
);
```

`HashMap` 工作原理是通过键值对。它对两者都是泛型的：`K` 是键类型的泛型参数，而 `V` 是值类型的泛型参数。

插入、检索和移除的预期成本是**常数时间**，`O(1)`。这对我们的用例来说是不是很完美呢？

## 键的要求

在 `HashMap` 的结构定义上没有任何 trait 限制，但你会发现其方法上有一些限制。例如，让我们看一下 `insert`：

```rust
// 稍加简化
impl<K, V> HashMap<K, V>
where
    K: Eq + Hash,
{
    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        // [...]
    }
}
```

键类型必须实现 `Eq` 和 `Hash` 这两个 trait。\
接下来我们深入了解这两个 trait。

## `Hash`

哈希函数（或哈希器）将可能无限的值集合（例如所有可能的字符串）映射到一个有限范围（例如一个 `u64` 值）。\
有很多不同的哈希函数，每一种都有不同的特性（速度、碰撞风险、可逆性等）。

正如其名字所示，`HashMap` 在背后使用了一个哈希函数。\
它对你的键进行哈希处理，然后使用该哈希值来存储/检索相关的值。\
这种策略要求键类型必须是可哈希的，因此 `K` 上有一个 `Hash` trait 限制。

你可以在 `std::hash` 模块中找到 `Hash` trait：

```rust
pub trait Hash {
    // 必须实现的方法
    fn hash<H>(&self, state: &mut H)
       where H: Hasher;
}
```

你很少需要手动实现 `Hash`。大多数情况下，你会派生它：

```rust
#[derive(Hash)]
struct Person {
    id: u32,
    name: String,
}
```

## `Eq`

`HashMap` 必须能够比较键是否相等。在处理哈希碰撞时，这一点尤其重要——例如，当两个不同的键哈希到相同的值。

你可能会问：这不是 `PartialEq` trait 的功能吗？差一点！\
`PartialEq` 对于 `HashMap` 来说还不够，因为它不保证自反性，即 `a == a` 总是 `true`。\
例如，浮点数（`f32` 和 `f64`）实现了 `PartialEq`，但它们不满足自反性：`f32::NAN == f32::NAN` 是 `false`。\
自反性对于 `HashMap` 正确工作至关重要：如果没有它，你将无法使用相同的键从映射中检索到你插入的值。

`Eq` trait 扩展了 `PartialEq`，增加了自反性属性：

```rust
pub trait Eq: PartialEq {
    // 没有额外的方法
}
```

这是一个标记 trait：它不添加任何新方法，仅仅是一种向编译器声明的方式，\
表明在 `PartialEq`中实现的相等逻辑是自反的。

当你派生 `PartialEq` 时，可以自动派生 `Eq`：

```rust
#[derive(PartialEq, Eq)]
struct Person {
    id: u32,
    name: String,
}
```

## `Eq` 和 `Hash` 的关系

在 `Eq` 和 `Hash` 之间存在一个隐含的约定：如果两个键是相等的，它们的哈希值也必须相等。\
这对于 `HashMap` 的正确运行至关重要。如果违反这个约定，在使用 `HashMap` 时你会得到不合逻辑的结果。