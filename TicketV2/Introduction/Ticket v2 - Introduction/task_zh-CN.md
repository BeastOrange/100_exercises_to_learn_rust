## 建模票据，第2部分

我们在前几章中处理的 `Ticket` 结构体是一个不错的开始，  
但它仍然让人感觉像是在说：“我是一个初学者 Rustacean！”。

我们将利用本章来改进我们的 Rust 领域建模技能。  
在此过程中，我们需要引入一些新的概念：

- `enum`，Rust 最强大的数据建模特性之一  
- `Option` 类型，用于建模可空值  
- `Result` 类型，用于建模可恢复的错误  
- `Debug` 和 `Display` 特性，用于打印  
- `Error` 特性，用于标记错误类型  
- `TryFrom` 和 `TryInto` 特性，用于可失败的转换  
- Rust 的包管理系统，解释什么是库，什么是二进制文件，以及如何使用第三方 crate

<br/>

## 任务  
引导任务是完成 `intro` 函数。  
该函数应返回字符串：***I'm ready to refine the `Ticket` type!***