## 组合器

迭代器能做的远不止 `for` 循环！  
如果你查看 `Iterator` 特性的文档，你会发现有一个**丰富**的方法集合，这些方法可以用来以多种方式转换、过滤和组合迭代器。

以下是最常见的方法：

- `map` 对迭代器中的每个元素应用一个函数。
- `filter` 仅保留满足条件的元素。
- `filter_map` 将 `filter` 和 `map` 合并为一步操作。
- `cloned` 将引用的迭代器转换为值的迭代器，并对每个元素进行克隆。
- `enumerate` 返回一个新的迭代器，该迭代器生成 `(索引, 值)` 的对。
- `skip` 跳过迭代器开头的前 `n` 个元素。
- `take` 在迭代器中的前 `n` 个元素后停止。
- `chain` 将两个迭代器合并为一个。

这些方法被称为**组合器**。  
它们通常会被**链式调用**，从而以简洁且可读的方式创建复杂的转换：

```rust
let numbers = vec![1, 2, 3, 4, 5];
// 偶数平方的总和
let outcome: u32 = numbers.iter()
    .filter(|&n| n % 2 == 0)
    .map(|&n| n * n)
    .sum();
```

## 闭包

上面 `filter` 和 `map` 方法是怎么回事？  
它们接受**闭包**作为参数。

闭包是**匿名函数**，也就是说，这些函数并不是使用我们熟悉的 `fn` 语法定义的。  
它们使用 `|参数| 函数体` 的语法定义，其中 `参数` 是传入的参数，`函数体`是函数的主体。  
`函数体`可以是一个代码块或者单一表达式。例如：

```rust
// 一个为参数加 1 的匿名函数
let add_one = |x| x + 1;
// 也可以用代码块写：
let add_one = |x| { x + 1 };
```

闭包可以接受多个参数：

```rust
let add = |x, y| x + y;
let sum = add(1, 2);
```

它们还可以捕获其环境中的变量：

```rust
let x = 42;
let add_x = |y| x + y;
let sum = add_x(1);
```

如果需要，可以显式指定参数类型和/或返回类型：

```rust
// 仅指定输入类型
let add_one = |x: i32| x + 1;
// 或同时指定输入和返回类型，使用 `fn` 语法
let add_one: fn(i32) -> i32 = |x| x + 1;
```

## `collect`

当你使用组合器完成对迭代器的转换后会发生什么？  
你可以通过 `for` 循环遍历转换后的值，或者将它们收集到一个集合中。

后者可以通过 `collect` 方法完成。  
`collect` 会消耗迭代器，并将其元素收集到你选择的集合中。

例如，可以将偶数的平方收集到一个 `Vec` 中：

```rust
let numbers = vec![1, 2, 3, 4, 5];
let squares_of_evens: Vec<u32> = numbers.iter()
    .filter(|&n| n % 2 == 0)
    .map(|&n| n * n)
    .collect();
```

`collect` 是泛型的**返回类型**。  
因此，通常需要提供一个类型提示以帮助编译器推导出正确的类型。  
在上面的例子中，我们注释了 `squares_of_evens` 的类型为 `Vec<u32>`。  
或者，可以使用**涡轮鱼语法**指定类型：

```rust
let squares_of_evens = numbers.iter()
    .filter(|&n| n % 2 == 0)
    .map(|&n| n * n)
    // 涡轮鱼语法：`<方法名>::<类型>()`
    // 因为 `::<>` 看起来像一条鱼，所以称为涡轮鱼
    .collect::<Vec<u32>>();
```

## 延伸阅读

- [`Iterator` 的文档](https://doc.rust-lang.org/std/iter/trait.Iterator.html) 为你概述了 `std` 中迭代器可用的方法。
- [`itertools` crate](https://docs.rs/itertools/) 为迭代器定义了更多的**组合器**。