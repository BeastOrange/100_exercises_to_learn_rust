## 取消

当一个挂起的 future 被丢弃时会发生什么？\
运行时将不再轮询它，因此它不会再有任何进展。换句话说，它的执行已被**取消**。

在实际应用中，这种情况经常发生在处理超时时。\
例如：

```rust
use tokio::time::timeout;
use tokio::sync::oneshot;
use std::time::Duration;

async fn http_call() {
    // [...]
}

async fn run() {
    // 将 future 包装为 `Timeout`，设置超时时间为 10 毫秒。
    let duration = Duration::from_millis(10);
    if let Err(_) = timeout(duration, http_call()).await {
        println!("在 10 毫秒内没有收到值");
    }
}
```

当超时发生时，由 `http_call` 返回的 future 将被取消。\
让我们假设 `http_call` 的内容如下：

```rust
use std::net::TcpStream;

async fn http_call() {
    let (stream, _) = TcpStream::connect(/* */).await.unwrap();
    let request: Vec<u8> = /* */;
    stream.write_all(&request).await.unwrap();
}
```

每个 yield 点都会成为一个**取消点**。\
`http_call` 无法被运行时抢占，因此它只能在通过 `.await` 将控制权交还给执行器后被丢弃。\
这一点是递归适用的——例如，`stream.write_all(&request)` 的实现中可能会包含多个 yield 点。\
完全有可能在 `http_call` 被取消之前，它已部分发送了请求，从而断开连接并无法完成主体的传输。

## 清理

Rust 的取消机制非常强大——允许调用者在无需任务自身任何配合的情况下取消正在进行的任务。\
同时，这也可能带来一定风险。\
在某些情况下，可能希望进行**优雅的取消**，以确保在操作中断之前执行一些清理任务。

例如，考虑一个用于处理 SQL 事务的虚构 API：

```rust
async fn transfer_money(
    connection: SqlConnection,
    payer_id: u64,
    payee_id: u64,
    amount: u64
) -> Result<(), anyhow::Error> {
    let transaction = connection.begin_transaction().await?;
    update_balance(payer_id, amount, &transaction).await?;
    decrease_balance(payee_id, amount, &transaction).await?;
    transaction.commit().await?;
}
```

在取消时，理想的情况是显式中止未完成的事务，而不是让它悬而未决。\
不幸的是，Rust 并未提供针对这种**异步**清理操作的完美机制。

最常见的策略是依赖 `Drop` trait 来安排所需的清理工作。这可以通过以下方式实现：

- 在运行时中启动一个新任务
- 在通道中排队一条消息
- 启动一个后台线程

最佳选择需要根据上下文确定。

## 取消已启动的任务

当你通过 `tokio::spawn` 启动一个任务时，你将无法再丢弃它；\
它将归属于运行时。\
尽管如此，你可以使用它的 `JoinHandle` 在需要时对其进行取消：

```rust
async fn run() {
    let handle = tokio::spawn(/* 某个异步任务 */);
    // 取消启动的任务
    handle.abort();
}
```

## 延伸阅读

- 使用 `tokio` 的 `select!` 宏对两个不同的 future 进行“竞争”时需极其小心。\
  在循环中重试同一个任务是危险的，除非你能够确保**取消安全性**。\
  查看 [`select!` 文档](https://tokio.rs/tokio/tutorial/select) 了解更多详情。\
  如果需要交错处理两个异步数据流（例如一个 socket 和一个 channel），优先使用
  [`StreamExt::merge`](https://docs.rs/tokio-stream/latest/tokio_stream/trait.StreamExt.html#method.merge)。
- 在某些情况下，[`CancellationToken`](https://docs.rs/tokio-util/latest/tokio_util/sync/struct.CancellationToken.html) 可能比 `JoinHandle::abort` 更合适。