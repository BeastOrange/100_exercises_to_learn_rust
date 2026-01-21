## 向量（Vectors）

数组的优点也是其缺点：它们的大小必须在编译时预先确定。  
如果尝试创建一个大小只能在运行时确定的数组，就会遇到编译错误：

```rust
let n = 10;
let numbers: [u32; n];
```

```text
error[E0435]: attempt to use a non-constant value in a constant
 --> src/main.rs:3:20
  |
2 | let n = 10;
3 | let numbers: [u32; n];
  |                    ^ 非常量值
```

对于我们的票务管理系统，数组是不适用的——编译时我们并不知道需要储存多少张票。  
这正是 `Vec` 的用武之地。

## `Vec`

`Vec` 是标准库中提供的一种可动态增长的数组类型。  
你可以使用 `Vec::new` 函数创建一个空数组：

```rust
let mut numbers: Vec<u32> = Vec::new();
```

然后可以通过 `push` 方法向向量中添加元素：

```rust
numbers.push(1);
numbers.push(2);
numbers.push(3);
```

新值会被添加到向量的末尾。  
如果在创建时已经知道初始值，也可以使用 `vec!` 宏创建一个初始化的向量：

```rust
let numbers = vec![1, 2, 3];
```

## 访问元素

访问元素的语法与数组相同：

```rust
let numbers = vec![1, 2, 3];
let first = numbers[0];
let second = numbers[1];
let third = numbers[2];
```

索引必须是 `usize` 类型。  
也可以使用 `get` 方法，它返回一个 `Option<&T>`：

```rust
let numbers = vec![1, 2, 3];
assert_eq!(numbers.get(0), Some(&1));
// 如果尝试访问超出范围的索引，
// 而不是 panic，你会得到一个 `None`。
assert_eq!(numbers.get(3), None);
```

访问是边界检查的，和数组元素访问一样。其复杂度为 O(1)。

## 内存布局

`Vec` 是一个堆分配的数据结构。  
当创建一个 `Vec` 时，它会在堆上分配内存来存储元素。

如果运行以下代码：

```rust
let mut numbers = Vec::with_capacity(3);
numbers.push(1);
numbers.push(2);
```

你会得到如下内存布局：

```text
      +---------+--------+----------+
栈区  | pointer | length | capacity | 
      |  |      |   2    |    3     |
      +--|------+--------+----------+
         |
         |
         v
       +---+---+---+
堆区:  | 1 | 2 | ? |
       +---+---+---+
```

`Vec` 维护了以下三个信息：

- 指向你在堆上保留内存区域的 **指针**。
- 向量的 **长度**，即向量中有多少元素。
- 向量的 **容量**，即堆上预留空间可以容纳的最大元素数量。

这个布局应该看起来很熟悉：它和 `String` 完全相同！  
这不是巧合：在底层实现中，`String` 就是一个字节向量 `Vec<u8>`：

```rust
pub struct String {
    vec: Vec<u8>,
}