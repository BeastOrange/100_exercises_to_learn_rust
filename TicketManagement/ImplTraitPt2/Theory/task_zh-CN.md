## 参数位置的 `impl Trait`

在上一节中，我们看到了如何使用 `impl Trait` 返回一个未指定其名称的类型。\
同样的语法也可以在**参数位置**使用：

```rust
fn print_iter(iter: impl Iterator<Item = i32>) {
    for i in iter {
        println!("{}", i);
    }
}
```

`print_iter` 接收一个 `i32` 类型的迭代器，并打印每个元素。\
当在**参数位置**使用时，`impl Trait` 等价于一个带有特征约束的泛型参数：

```rust
fn print_iter<T>(iter: T) 
where
    T: Iterator<Item = i32>
{
    for i in iter {
        println!("{}", i);
    }
}
```

## 缺点

通常情况下，在参数位置优先使用泛型而不是 `impl Trait`。\
泛型允许调用者通过涡轮鱼语法（`::<>`）显式地指定参数类型，\
这在某些需要消除歧义的场景中非常有用。而 `impl Trait` 则不支持这一点。