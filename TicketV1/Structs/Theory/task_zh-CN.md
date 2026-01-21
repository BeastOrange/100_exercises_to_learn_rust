## 结构体

我们需要为每个任务跟踪以下三部分信息：

- 标题
- 描述
- 状态

我们可以通过使用 [`String`](https://doc.rust-lang.org/std/string/struct.String.html)
来表示它们。`String` 是 Rust 标准库中定义的类型，用于表示
[UTF-8 编码](https://en.wikipedia.org/wiki/UTF-8) 的文本。

但是，如何将这三部分信息**组合**成一个实体呢？

## 定义一个 `struct`

`struct` 用于定义一个**新的 Rust 类型**。

```rust
struct Ticket {
    title: String,
    description: String,
    status: String
}
```

结构体类似于你在其他编程语言中所称的类或对象。

## 定义字段

新类型是通过将其他类型**作为字段**组合而成的。\
每个字段必须有一个名称和一个类型，中间用冒号 `:` 分隔。如果有多个字段，用逗号 `,` 分隔。

字段不必是相同的类型，如下例中的 `Configuration` 结构体所示：

```rust
struct Configuration {
   version: u32,
   active: bool
}
```

## 初始化

你可以通过为每个字段指定值来创建结构体的一个实例：

```rust
// 语法：<StructName> { <field_name>: <value>, ... }
let ticket = Ticket {
    title: "Build a ticket system".into(),
    description: "A Kanban board".into(),
    status: "Open".into()
};
```

## 访问字段

你可以使用 `.` 运算符访问结构体的字段：

```rust
// 字段访问
let x = ticket.description;
```

## 方法

我们可以通过定义**方法**为结构体添加行为功能。\
以 `Ticket` 结构体为例：

```rust
impl Ticket {
    fn is_open(self) -> bool {
        self.status == "Open"
    }
}

// 语法：
// impl <StructName> {
//    fn <method_name>(<parameters>) -> <return_type> {
//        // 方法体
//    }
// }
```

方法类似于函数，但有两个关键的区别：

1. 方法必须定义在 **`impl` 块**中。
2. 方法的第一个参数可以是 `self`。
   `self` 是一个关键词，表示调用该方法的结构体实例。

### `self`

如果方法的第一个参数是 `self`，你可以使用**方法调用语法**来调用它：

```rust
// 方法调用语法：<instance>.<method_name>(<parameters>)
let is_open = ticket.is_open();
```

这与在[上一章](../02_basic_calculator/09_saturating.md)中对 `u32` 值执行饱和算术运算时所使用的调用语法是相同的。

### 静态方法

如果方法的第一个参数不是 `self`，它就是一个**静态方法**。

```rust
struct Configuration {
    version: u32,
    active: bool
}

impl Configuration {
    // `default` 是 `Configuration` 的一个静态方法
    fn default() -> Configuration {
        Configuration { version: 0, active: false }
    }
}
```

调用静态方法的唯一方式是使用**函数调用语法**：

```rust
// 函数调用语法：<StructName>::<method_name>(<parameters>)
let default_config = Configuration::default();
```

### 等价性

对于那些以 `self` 为第一个参数的方法，你也可以使用函数调用语法：

```rust
// 函数调用语法：
//   <StructName>::<method_name>(<instance>, <parameters>)
let is_open = Ticket::is_open(ticket);
```

函数调用语法明确表明了 `ticket` 被用作方法的第一个参数 `self`，但它的写法更加冗长。在可能的情况下，建议优先使用方法调用语法。