## 总结
##

Rust 的异步模型非常强大，但它确实引入了额外的复杂性。花时间了解你的工具：深入阅读 `tokio` 的文档，熟悉其原语，以最大限度地发挥它的作用。

同时，请记住，在语言和 `std` 层面上，有一些工作正在进行中，以简化和“完善” Rust 的异步生态。由于某些缺失的部分，在日常工作中，你可能会遇到一些棘手的问题。

以下是一些让你的异步开发相对轻松的建议：

- **选择一个运行时并坚持使用它。**\
  一些原语（例如定时器、I/O）在不同运行时之间不可移植。尝试混合使用多个运行时可能会让你头痛。而尝试编写与运行时无关的代码，会显著增加代码库的复杂性。尽量避免这种情况。

- **目前还没有稳定的 `Stream`/`AsyncIterator` 接口。**\
  从概念上讲，`AsyncIterator` 是一种能异步返回新项的迭代器。这方面的设计工作仍在进行中，但尚未达成共识。\
  如果你使用的是 `tokio`，请参考 [`tokio_stream`](https://docs.rs/tokio-stream/latest/tokio_stream/) 作为你的默认接口。

- **小心缓冲处理。**\
  缓冲处理往往是引发细微 bug 的原因之一。你可以查阅 ["Barbara battles buffered streams"](https://rust-lang.github.io/wg-async/vision/submitted_stories/status_quo/barbara_battles_buffered_streams.html) 了解更多细节。

- **异步任务目前没有类似作用域线程的功能。**\
  你可以阅读 ["The scoped task trilemma"](https://without.boats/blog/the-scoped-task-trilemma/) 了解更多信息。

不要因这些限制而感到畏惧：异步 Rust 已经在 _大规模_ 应用中被证明是高效的（例如 AWS、Meta），并驱动了基础服务。\
如果你计划用 Rust 构建网络应用程序，你需要精通异步编程。