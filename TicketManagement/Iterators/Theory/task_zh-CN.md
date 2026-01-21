## 迭代

在最初的练习中，您学到 Rust 允许使用 `for` 循环遍历集合。  
当时我们查看了范围（例如 `0..5`），但对于诸如数组和向量这样的集合也同样适用。

```rust
// 对 `Vec` 有效
let v = vec![1, 2, 3];
for n in v {
    println!("{}", n);
}

// 对数组也有效
let a: [u32; 3] = [1, 2, 3];
for n in a {
    println!("{}", n);
}
```

是时候了解这一机制背后的原理了。

## `for` 的语法糖解析

每次您在 Rust 中写一个 `for` 循环，编译器会将其转换为如下代码：

```rust
let mut iter = IntoIterator::into_iter(v);
loop {
    match iter.next() {
        Some(n) => {
            println!("{}", n);
        }
        None => break,
    }
}
```

`loop` 是一种循环结构，除了 `for` 和 `while` 之外，它也是一种循环方式。  
除非您显式使用 `break` 跳出，它的代码块会无限运行。

## `Iterator` 特征

前面代码片段中的 `next` 方法来源于 `Iterator` 特征。  
`Iterator` 特征定义在 Rust 标准库中，提供一个通用接口，供能生成一系列值的类型使用：

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

`Item` 关联类型指定了由迭代器生成的值的类型。

`next` 返回序列中的下一个值。  
如果有值返回，它会返回 `Some(value)`，如果没有值，则返回 `None`。

请注意：迭代器返回 `None` 时并不保证它已被耗尽。这只有当迭代器实现了（更严格的）  
[`FusedIterator`](https://doc.rust-lang.org/std/iter/trait.FusedIterator.html) 特征时才有保证。

## `IntoIterator` 特征

并非所有类型都实现了 `Iterator`，但很多类型可以通过转换成为一个实现了 `Iterator` 的类型。  
这就是 `IntoIterator` 特征的作用：

```rust
trait IntoIterator {
    type Item;
    type IntoIter: Iterator<Item = Self::Item>;
    fn into_iter(self) -> Self::IntoIter;
}
```

`into_iter` 方法会消费原始值，并返回一个基于其元素的迭代器。  
每种类型只能实现一次 `IntoIterator`，以确保 `for` 的语法糖解析不产生歧义。

需注意：每个实现了 `Iterator` 的类型都会自动实现 `IntoIterator` 特征。  
它们只需通过 `into_iter` 返回自身！

## 边界检查

通过迭代器迭代有一个很好的副作用：设计上不可能越界。  
因此 Rust 能移除生成的机器代码中的边界检查，从而加速迭代过程。

换句话说，

```rust
let v = vec![1, 2, 3];
for n in v {
    println!("{}", n);
}
```

通常比

```rust
let v = vec![1, 2, 3];
for i in 0..v.len() {
    println!("{}", v[i]);
}
```

更快。

不过也存在例外：即使使用手动索引，编译器有时也可以证明您没有越界，因此仍然会移除边界检查。但一般来说，优先使用迭代而非索引。