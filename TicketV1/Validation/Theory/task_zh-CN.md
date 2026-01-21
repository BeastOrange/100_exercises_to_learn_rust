## 验证

让我们回到我们的 ticket 定义：

```rust
struct Ticket {
    title: String,
    description: String,
    status: String,
}
```

我们为 `Ticket` 结构体的字段使用了“原始”类型。  
这意味着用户可以创建一个标题为空、描述超级长或者状态不合理（例如 "Funny"）的 ticket。\
我们可以做得更好！

## 延伸阅读

- 查看 [`String` 的文档](https://doc.rust-lang.org/std/string/struct.String.html)，  
  获取其提供的各种方法的详尽概述。这在练习中会用到！