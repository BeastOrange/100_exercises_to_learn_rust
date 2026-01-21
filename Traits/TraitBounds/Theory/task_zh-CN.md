## 特征约束

到目前为止，我们已经了解了特征的两种用法：

- 解锁“内置”的行为（例如操作符重载）
- 为现有类型添加新行为（即扩展特征）

还有第三种用法：**泛型编程**。

## 问题

到目前为止，我们的所有函数和方法都在使用**具体类型**。\
针对具体类型编写的代码通常易于编写和理解。但这也限制了其可复用性。\
想象一下，我们需要编写一个函数用于判断一个整数是否为偶数。\
在使用具体类型的情况下，我们需要为想要支持的每种整数类型分别编写一个单独的函数：

```rust
fn is_even_i32(n: i32) -> bool {
    n % 2 == 0
}

fn is_even_i64(n: i64) -> bool {
    n % 2 == 0
}

// 等等。
```

或者，我们也可以编写一个扩展特征，然后为每种整数类型分别实现它：

```rust
trait IsEven {
    fn is_even(&self) -> bool;
}

impl IsEven for i32 {
    fn is_even(&self) -> bool {
        self % 2 == 0
    }
}

impl IsEven for i64 {
    fn is_even(&self) -> bool {
        self % 2 == 0
    }
}

// 等等。
```

但重复的代码仍然存在。

## 泛型编程

通过使用**泛型**，我们可以做得更好。\
泛型允许我们编写适用于**类型参数**而不是具体类型的代码：

```rust
fn print_if_even<T>(n: T)
where
    T: IsEven + Debug
{
    if n.is_even() {
        println!("{n:?} is even");
    }
}
```

`print_if_even` 是一个**泛型函数**。\
它并不限于特定的输入类型，而是适用于任何实现了以下条件的类型 `T`：

- 实现了 `IsEven` 特征。
- 实现了 `Debug` 特征。

这个约定通过**特征约束**来表达：`T: IsEven + Debug`。\
`+` 符号用于要求 `T` 实现多个特征。`T: IsEven + Debug` 等价于“`T` 必须同时实现 `IsEven` **和** `Debug`”。

## 特征约束

特征约束在 `print_if_even` 中有什么作用？\
为了找出答案，我们来尝试移除它们：

```rust
fn print_if_even<T>(n: T) {
    if n.is_even() {
        println!("{n:?} is even");
    }
}
```

这段代码无法编译：

```text
error[E0599]: no method named `is_even` found for type parameter `T` 
              in the current scope
 --> src/lib.rs:2:10
  |
1 | fn print_if_even<T>(n: T) {
  |                  - method `is_even` not found 
  |                    for this type parameter
2 |     if n.is_even() {
  |          ^^^^^^^ method not found in `T`

error[E0277]: `T` doesn't implement `Debug`
 --> src/lib.rs:3:19
  |
3 |         println!("{n:?} is even");
  |                   ^^^^^ 
  |   `T` cannot be formatted using `{:?}` because 
  |         it doesn't implement `Debug`
  |
help: consider restricting type parameter `T`
  |
1 | fn print_if_even<T: std::fmt::Debug>(n: T) {
  |                   +++++++++++++++++
```

没有特征约束，编译器并不知道 `T` **能做什么**。\
编译器不知道 `T` 有一个 `is_even` 方法，也不知道如何格式化 `T` 以进行打印。\
在编译器看来，一个裸露的 `T` 完全没有行为。\
特征约束通过确保函数体需要的行为存在来限制可以使用的类型集合。

## 语法：内联特征约束

上面的所有例子都使用了一个 **`where` 子句** 来指定特征约束：

```rust
fn print_if_even<T>(n: T)
where
    T: IsEven + Debug
//  ^^^^^^^^^^^^^^^^^
//  这是一个 `where` 子句
{
    // [...]
}
```

如果特征约束很简单，可以直接将它们**内联**到类型参数旁边：

```rust
fn print_if_even<T: IsEven + Debug>(n: T) {
    //           ^^^^^^^^^^^^^^^^^
    //           这是一个内联特征约束
    // [...]
}
```

## 语法：有意义的名称

在以上示例中，我们使用 `T` 作为类型参数名。当一个函数只有一个类型参数时，这是一个常见的约定。\
但是，你也可以使用更有意义的名称：

```rust
fn print_if_even<Number: IsEven + Debug>(n: Number) {
    // [...]
}
```

当涉及多个类型参数或当 `T` 无法充分传达该类型在函数中扮演的角色时，使用有意义的名称实际上是**可取的**。\
就像为变量或函数参数命名时一样，尽量最大限度地提高类型参数的清晰度和可读性。\
不过需要遵循 Rust 的约定：对类型参数名称使用[大驼峰命名法](https://rust-lang.github.io/api-guidelines/naming.html#casing-conforms-to-rfc-430-c-case)。

## 函数签名为王

你可能会问，为什么我们需要特征约束？编译器就不能从函数体中推断出所需的特征吗？\
其实可以，但它不会这么做。\
其原理与[函数参数上的显式类型注释](../../../A%20Basic%20Calculator/Variables/Theory/task.md#function-arguments-are-variables)是相同的：\
每个函数签名都是调用者和被调用者之间的契约，其条款必须被显式地声明。\
这样才能提供更好的错误信息、更好的文档、减少跨版本的意外破坏，并提高编译速度。