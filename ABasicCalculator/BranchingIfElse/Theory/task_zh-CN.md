## 控制流程

到目前为止，我们的程序都非常简单，指令按照从上到下的顺序依次执行，就是这样。

现在是引入一些**分支**逻辑的时候了。

## `if` 条件语句

`if` 关键字用于在条件为 true 时执行一段代码块。

下面是一个简单的例子：

```rust
let number = 3;
if number < 5 {
    println!("`number` 小于 5");
}
```

这个程序会输出 `number is smaller than 5`，因为条件 `number < 5` 为 true。

### `else` 条件语句

与大多数编程语言一样，Rust 支持可选的 `else` 分支，当 `if` 表达式中的条件为 false 时执行另一段代码块。  
例如：

```rust
let number = 3;

if number < 5 {
    println!("`number` 小于 5");
} else {
    println!("`number` 大于或等于 5");
}
```

### `else if` 条件语句

当你有多个 `if` 表达式时，代码会逐步向右偏移，因为一个 `if` 嵌套在另一个里面。

```rust
let number = 3;

if number < 5 {
    println!("`number` 小于 5");
} else {
    if number >= 3 {
        println!("`number` 大于或等于 3，但小于 5");
    } else {
        println!("`number` 小于 3");
    }
}
```

你可以使用 `else if` 关键字将多个 `if` 表达式组合为一个：

```rust
let number = 3;

if number < 5 {
    println!("`number` 小于 5");
} else if number >= 3 {
    println!("`number` 大于或等于 3，但小于 5");
} else {
    println!("`number` 小于 3");
}
```

## 布尔值

`if` 表达式中的条件必须是类型为 `bool` 的**布尔值**。  
布尔类型与整数一样，是 Rust 中的基础类型之一。

布尔值可以有两个值之一：`true` 或 `false`。

### 没有 truthy 或 falsy 值

如果 `if` 表达式中的条件不是布尔值，程序会出现编译错误。

例如，下面的代码无法编译：

```rust
let number = 3;
if number {
    println!("`number` 不为零");
}
```

你会收到如下的编译错误：

```text
error[E0308]: mismatched types
 --> src/main.rs:3:8
  |
3 |     if number {
  |        ^^^^^^ expected `bool`, found integer
```

这是 Rust 关于类型转换哲学的体现：Rust 不会自动从非布尔类型转换为布尔类型。  
Rust 不像 JavaScript 或 Python 那样存在 **truthy** 或 **falsy** 的值。  
你必须显式地定义你想要检查的条件。

### 比较运算符

在 `if` 表达式中，使用比较运算符构建条件是非常常见的。  
下面是 Rust 在处理整数时可用的比较运算符：

- `==`：等于
- `!=`：不等于
- `<`：小于
- `>`：大于
- `<=`：小于或等于
- `>=`：大于或等于

## `if/else` 是一个表达式

在 Rust 中，`if` 是**表达式**，而不是语句：它会返回一个值。  
该值可以赋给变量或在其他表达式中使用。例如：

```rust
let number = 3;
let message = if number < 5 {
    "小于 5"
} else {
    "大于或等于 5"
};
```

在上面的例子中，每个 `if` 分支都会计算出一个字符串字面值，  
然后将其赋值给变量 `message`。  
唯一的要求是，`if` 的两个分支必须返回相同的类型。