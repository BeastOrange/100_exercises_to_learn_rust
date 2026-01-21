## 模块

你刚定义的 `new` 方法试图对 `Ticket` 的字段值施加一些**约束**。  
但这些不变性真的被强制执行了吗？是什么阻止开发人员绕过 `Ticket::new` 而直接创建一个 `Ticket`？

为了实现正确的**封装**，你需要熟悉两个新概念：**可见性**和**模块**。  
让我们从模块开始。

## 什么是模块？

在 Rust 中，**模块**是一种将相关代码在一个公共命名空间下（即模块的名称）进行分组的方法。  
你已经看到过模块的实际应用：用于验证代码正确性的单元测试就定义在一个不同的模块中，命名为 `tests`。

```rust
#[cfg(test)]
mod tests {
    // [...]
}
```

## 内联模块

上面的 `tests` 模块是一个**内联模块**的例子：模块声明（`mod tests`）和模块内容（`{ ... }` 内的内容）是紧挨在一起的。

## 模块树

模块可以嵌套，形成一个**树形**结构。  
树的根是**crate**本身，它是包含所有其他模块的顶级模块。  
对于一个库 crate，根模块通常是 `src/lib.rs`（除非其位置被自定义）。  
根模块也被称为**crate 根**。

crate 根可以有子模块，而子模块又可以有自己的子模块，以此类推。

## 外部模块与文件系统

内联模块适用于小块代码，但随着项目的增长，你可能希望将代码拆分成多个文件。  
在父模块中，可以使用 `mod` 关键字声明子模块的存在。

```rust
mod dog;
```

Rust 的构建工具 `cargo` 负责找到包含模块实现的文件。  
如果模块声明在 crate 的根部（例如 `src/lib.rs` 或 `src/main.rs`），  
`cargo` 期望文件命名为以下之一：

- `src/<module_name>.rs`
- `src/<module_name>/mod.rs`

如果模块是另一个模块的子模块，文件应命名为：

- `[..]/<parent_module>/<module_name>.rs`
- `[..]/<parent_module>/<module_name>/mod.rs`

例如，如果 `dog` 是 `animals` 的子模块，可以是 `src/animals/dog.rs` 或 `src/animals/dog/mod.rs`。

当你使用 `mod` 关键字声明新模块时，IDE 可能会帮助你自动创建这些文件。

## 项目路径和 `use` 语句

对于同一模块中定义的项目，可以不使用特殊语法直接访问，只需使用它们的名称即可。

```rust
struct Ticket {
    // [...]
}

// 这里不需要任何特别的限定，因为我们在同一模块中
fn mark_ticket_as_done(ticket: Ticket) {
    // [...]
}
```

但是，如果想要访问来自其他模块的实体，就必须使用指向目标实体的**路径**。

你可以通过以下方式构成路径：

- 从当前 crate 的根部开始，例如 `crate::module_1::MyStruct`  
- 从父模块开始，例如 `super::my_function`  
- 从当前模块开始，例如 `sub_module_1::MyStruct`  

`crate` 和 `super` 是**关键字**。  
`crate` 指当前 crate 的根部，而 `super` 指当前模块的父模块。

每次引用一个类型都需要写全路径可能会非常麻烦。  
为了简化，可以使用 `use` 语句将目标实体引入作用域。

```rust
// 将 `MyStruct` 引入作用域
use crate::module_1::module_2::MyStruct;

// 现在可以直接引用 `MyStruct`
fn a_function(s: MyStruct) {
     // [...]
}
```

### 星号导入

你还可以使用单个 `use` 语句导入模块中的所有项目。

```rust
use crate::module_1::module_2::*;
```

这被称为**星号导入**。  
一般不推荐使用，因为它可能污染当前命名空间，导致难以理解每个名称的来源，并可能引发命名冲突。  
不过，在某些情况下它可能很有用，例如编写单元测试时。  
你可能注意到，大多数测试模块以 `use super::*;` 开头，以将父模块（被测试模块）的所有项目引入作用域。

## 可视化模块树

如果你难以想象项目的模块树，可以尝试使用 [`cargo-modules`](https://crates.io/crates/cargo-modules) 来可视化它！

参考其文档获取安装说明和使用示例。