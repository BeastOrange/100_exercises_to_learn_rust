## 循环，第 1 部分：`while`

您之前实现的 `factorial` 函数被迫使用递归。\
如果您来自函数式编程的背景，这可能会让您觉得很自然。\
但如果您习惯于像 C 或 Python 这样的命令式语言，这可能会让您感到有些陌生。

让我们看看如何使用**循环**来实现相同的功能。

## `while` 循环

`while` 循环是一种在**条件**为真时执行代码块的方法。\
它的一般语法如下：

```rust
while <condition> {
    // 要执行的代码
}
```

例如，我们可能想计算从 1 到 5 的数字之和：

```rust
let sum = 0;
let i = 1;
// "当 i 小于或等于 5 时执行"
while i <= 5 {
    // `+=` 是 `sum = sum + i` 的简写
    sum += i;
    i += 1;
}
```

这段代码会不断将 1 添加到 `i` 上，同时将 `i` 添加到 `sum`，\
直到 `i` 不再小于或等于 5。

## `mut` 关键字

以上示例代码无法直接编译。您会收到类似于下面的错误：

```text
error[E0384]: cannot assign twice to immutable variable `sum`
 --> src/main.rs:7:9
  |
2 |     let sum = 0;
  |         ---
  |         |
  |         第一次对 `sum` 进行赋值
  |         提示: 请考虑将此绑定声明为可变：`mut sum`
...
7 |         sum += i;
  |         ^^^^^^^^ 无法对不可变变量进行二次赋值

error[E0384]: cannot assign twice to immutable variable `i`
 --> src/main.rs:8:9
  |
3 |     let i = 1;
  |         -
  |         |
  |         第一次对 `i` 进行赋值
  |         提示: 请考虑将此绑定声明为可变：`mut i`
...
8 |         i += 1;
  |         ^^^^^^ 无法对不可变变量进行二次赋值
```

这是因为 Rust 中的变量默认是**不可变的**。\
一旦为变量赋值，就不能再修改它的值。

如果您想允许对变量进行修改，则必须使用 `mut` 关键字将变量声明为**可变变量**：

```rust
// `sum` 和 `i` 现在是可变的！
let mut sum = 0;
let mut i = 1;

while i <= 5 {
    sum += i;
    i += 1;
}
```

这段代码将可以正常编译并运行。

## 延伸阅读

- [`while` 循环文档](https://doc.rust-lang.org/std/keyword.while.html)