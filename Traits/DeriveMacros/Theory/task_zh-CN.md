## 派生宏

为 `Ticket` 实现 `PartialEq` 有点繁琐，对吧？  
你必须手动比较结构体的每个字段。

## 解构语法

此外，这种实现方式不够健壮：如果结构体定义发生变化（例如添加了一个新字段），你必须记得更新 `PartialEq` 的实现。

你可以通过将结构体**解构**成其字段来降低这种风险：

```rust
impl PartialEq for Ticket {
    fn eq(&self, other: &Self) -> bool {
        let Ticket {
            title,
            description,
            status,
        } = self;
        // [...]
    }
}
```

如果 `Ticket` 的定义发生改变，编译器会报错，提示你的解构操作不再完整。  
你还可以对结构体字段重命名，以避免变量遮蔽：

```rust
impl PartialEq for Ticket {
    fn eq(&self, other: &Self) -> bool {
        let Ticket {
            title,
            description,
            status,
        } = self;
        let Ticket {
            title: other_title,
            description: other_description,
            status: other_status,
        } = other;
        // [...]
    }
}
```

解构是一种非常实用的模式，但这里还有一种更加便捷的方式：**派生宏**。

## 宏

在之前的练习中，你已经遇到过一些宏：

- 测试用例中的 `assert_eq!` 和 `assert!`
- 用于打印到控制台的 `println!`

Rust 宏是**代码生成器**。  
它基于你提供的输入生成新的 Rust 代码，这些生成的代码会与程序的其他部分一起编译。一些宏是内置于 Rust 标准库中的，同时你也可以自行编写宏。尽管我们在本课程中不会创建自定义宏，但你可以在[“进一步阅读”部分](#further-reading)找到一些有用的参考资料。

### 检查

有些集成开发环境（IDE）允许你展开宏以检查生成的代码。如果无法直接在 IDE 中完成，也可以使用 [`cargo-expand`](https://github.com/dtolnay/cargo-expand)。

### 派生宏

**派生宏**是 Rust 宏的一种特殊形式。它以**属性**的形式，添加在结构体的顶部。

```rust
#[derive(PartialEq)]
struct Ticket {
    title: String,
    description: String,
    status: String
}
```

派生宏用于为自定义类型自动实现一些常见（且“显而易见”）的 trait。  
在上面的例子