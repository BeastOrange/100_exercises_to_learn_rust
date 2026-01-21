## `Future` 特性

### 局部 `Rc` 问题

让我们回到 `tokio::spawn` 的函数签名：

```rust
pub fn spawn<F>(future: F) -> JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
{ /* */ }
```

`F` 被要求是 `Send` 到底是什么意思？\
正如我们在上一节看到的，这意味着，它从生成环境中捕获的任何值都必须是 `Send` 的。但它不仅仅是这样。

任何**跨越 `.await` 点**保留的值也必须是 `Send` 的。\
让我们来看一个例子：

```rust
use std::rc::Rc;
use tokio::task::yield_now;

fn spawner() {
    tokio::spawn(example());
}

async fn example() {
    // 一个在 async 函数中创建的非 `Send` 值
    let non_send = Rc::new(1);
    
    // 一个什么都不做的 `.await` 点
    yield_now().await;

    // `.await` 之后仍需要该局部非 `Send` 值
    println!("{}", non_send);
}
```

编译器会拒绝这段代码：

```text
error: future cannot be sent between threads safely
    |
5   |     tokio::spawn(example());
    |                  ^^^^^^^^^ 
    | example 返回的 future 不是 `Send`
    |
note: future is not `Send` as this value is used across an await
    |
11  |     let non_send = Rc::new(1);
    |         -------- 类型为 `Rc<i32>`，不是 `Send`
12  |     // 一个 `.await` 点
13  |     yield_now().await;
    |                 ^^^^^ 
    |   这里发生了 await，后续可能会用到 `non_send`
note: required by a bound in `tokio::spawn`
    |
164 |     pub fn spawn<F>(future: F) -> JoinHandle<F::Output>
    |            ----- 该函数需要符合此约束
165 |     where
166 |         F: Future + Send + 'static,
    |                     ^^^^ 由此约束要求 `Send`
```

为了理解这是为什么，我们需要更深入地了解 Rust 的异步模型。

## `Future` 特性

我们之前提到过，`async` 函数会返回**future**——一个实现了 `Future` 特性的类型。可以将 future 想象成一个**状态机**，它可以处于以下两种状态之一：

- **pending**：计算尚未完成。
- **ready**：计算已经完成，并可以获取其输出。

这一点体现在特性的定义中：

```rust
trait Future {
    type Output;
    
    // 暂时忽略 `Pin` 和 `Context`
    fn poll(
      self: Pin<&mut Self>, 
      cx: &mut Context<'_>
    ) -> Poll<Self::Output>;
}
```

### `poll`

`poll` 方法是 `Future` 特性的核心。\
单独一个 future 什么都不会做，它需要被**轮询（polled）**才能推进进程。\
调用 `poll` 时，就是在请求 future 执行一些操作。`poll` 会尝试推进，并返回以下之一：

- `Poll::Pending`：future 尚未准备好，需要稍后再次调用 `poll`。
- `Poll::Ready(value)`：future 已完成，`value` 是计算结果，其类型为 `Self::Output`。

一旦 `Future::poll` 返回 `Poll::Ready`，它就不应该再被轮询：future 已经完成，没有剩余工作可做。

### 运行时的角色

你几乎不会直接调用 `poll`。\
这是异步运行时的任务：它拥有所有需要的信息（`poll` 签名中的 `Context`），以确保你的 future 在能够推进时取得进展。

## `async fn` 和 future

我们学习了高层 API，即异步函数。\
现在，我们了解了底层原语，即 `Future` 特性。

它们之间是什么关系？

每次你将一个函数标记为异步时，该函数会返回一个 future。\
编译器会将异步函数的函数体转换为一个**状态机**：每个 `.await` 点对应一个状态。

回到 `Rc` 的例子：

```rust
use std::rc::Rc;
use tokio::task::yield_now;

async fn example() {
    let non_send = Rc::new(1);
    yield_now().await;
    println!("{}", non_send);
}
```

编译器会将其转换为类似如下的枚举：

```rust
pub enum ExampleFuture {
    NotStarted,
    YieldNow(Rc<i32>),
    Terminated,
}
```

调用 `example` 时，它会返回 `ExampleFuture::NotStarted`。此时，future 尚未被轮询，因此没有任何操作发生。\
运行时首次轮询它时，`ExampleFuture` 会推进到下一个 `.await` 点的状态：状态机会停留在 `ExampleFuture::YieldNow(Rc<i32>)` 阶段，并返回 `Poll::Pending`。\
再次轮询时，它会执行剩下的代码（`println!`）并返回 `Poll::Ready(())`。

从状态机表示 `ExampleFuture` 的角度来看，现在可以清楚地理解为何 `example` 不是 `Send`：它持有一个 `Rc`，因此不能是 `Send`。

## 挂起点

正如你在 `example` 中所见，每个 `.await` 点都会在 future 的生命周期中引入一个新的中间状态。\
因此，`.await` 点也被称为**挂起点（yield points）**：你的 future **将控制权让渡**回了轮询它的运行时，允许运行时暂停它，并在必要时调度另一个任务执行，从而同时推进多个任务的进程。

我们将在后续章节讨论挂起的重要性。