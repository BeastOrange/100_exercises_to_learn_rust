## 总结

在领域建模中，细节决定成败。  
Rust 提供了多种工具，帮助你在类型系统中直接表示领域的约束，但要掌握这些工具并编写出符合惯用风格的代码需要一些练习。

让我们用对 `Ticket` 模型的最终优化来结束本章。  
我们将为 `Ticket` 的每个字段引入一种新类型，以封装各自的约束。  
每次有人访问 `Ticket` 的字段时，都会返回一个保证有效的值——例如，一个 `TicketTitle` 而不是一个 `String`。他们不需要担心标题在代码的其他地方会为空：只要他们拥有一个 `TicketTitle`，他们就知道它在**构造时**是有效的。

这只是一个例子，展示了如何利用 Rust 的类型系统让代码更加安全和具有更强的表达力。

## 拓展阅读

- [解析，而非验证](https://lexi-lambda.github.io/blog/2019/11/05/parse-don-t-validate/)  
- [使用类型保证领域不变量](https://www.lpalmieri.com/posts/2020-12-11-zero-to-production-6-domain-modelling/)