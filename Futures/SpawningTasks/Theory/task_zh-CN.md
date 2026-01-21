## 生成任务

您对前一个练习的解决方案应该看起来像这样：

```rust
pub async fn echo(listener: TcpListener) -> Result<(), anyhow::Error> {
    loop {
        let (mut socket, _) = listener.accept().await?;
        let (mut reader, mut writer) = socket.split();
        tokio::io::copy(&mut reader, &mut writer).await?;
    }
}
```

这已经不错了！\
如果两次传入的连接之间的时间间隔很长，`echo` 函数将会处于空闲状态（因为 `TcpListener::accept` 是一个异步函数），从而允许执行器在此期间运行其他任务。

但我们怎样才能真正地让多个任务同时运行呢？\
如果我们总是运行异步函数直到完成（通过使用 `.await`），那么我们一次只会有一个任务在运行。

这就是 `tokio::spawn` 函数的作用所在。

## `tokio::spawn`

`tokio::spawn` 允许您将任务交给执行器，而**无需等待其完成**。\
每次调用 `tokio::spawn`，您实际上是在告诉 `tokio` 在后台**并发**运行被生成的任务，同时让生成它的任务继续执行其他工作。

下面是一个使用它来同时处理多个连接的示例：

```rust
use tokio::net::TcpListener;

pub async fn echo(listener: TcpListener) -> Result<(), anyhow::Error> {
    loop {
        let (mut socket, _) = listener.accept().await?;
        // 生成一个后台任务来处理这个连接
        // 从而允许主任务立即开始接受新的连接
        tokio::spawn(async move {
            let (mut reader, mut writer) = socket.split();
            tokio::io::copy(&mut reader, &mut writer).await?;
        });
    }
}
```

### 异步代码块

在这个示例中，我们将一个**异步代码块**传递给了 `tokio::spawn`：`async move { /* */ }`\
异步代码块是一种快速将一段代码标记为异步的方法，无需定义一个单独的异步函数。

### `JoinHandle`

`tokio::spawn` 会返回一个 `JoinHandle`。\
您可以像对生成线程使用 `join` 一样，通过 `.await` 来等待后台任务完成。

```rust
pub async fn run() {
    // 生成一个后台任务，将遥测数据发送到远程服务器
    let handle = tokio::spawn(emit_telemetry());
    // 同时，执行其他有用的工作
    do_work().await;
    // 但在返回调用者之前，
    // 确保遥测数据已经成功发送完成
    handle.await;
}

pub async fn emit_telemetry() {
    // [...]
}

pub async fn do_work() {
    // [...]
}
```

### 崩溃边界

如果通过 `tokio::spawn` 生成的任务发生崩溃，崩溃将被执行器捕获。\
如果您不对相应的 `JoinHandle` 调用 `.await`，崩溃将不会传播到生成者。\
即使您对 `JoinHandle` 调用了 `.await`，崩溃也不会自动传播。\
等待 `JoinHandle` 会返回一个 `Result`，错误类型为 [`JoinError`](https://docs.rs/tokio/latest/tokio/task/struct.JoinError.html)。\
您可以通过调用 `JoinError::is_panic` 检查任务是否发生崩溃，并选择如何处理崩溃——记录、忽略或传播它。

```rust
use tokio::task::JoinError;

pub async fn run() {
    let handle = tokio::spawn(work());
    if let Err(e) = handle.await {
        if let Ok(reason) = e.try_into_panic() {
            // 任务发生了崩溃
            // 继续展开崩溃，从而将其传播到当前任务
            panic::resume_unwind(reason);
        }
    }
}

pub async fn work() {
    // [...]
}
```

### `std::thread::spawn` 与 `tokio::spawn`

可以将 `tokio::spawn` 视为 `std::thread::spawn` 的异步版本。

注意一个关键区别：对于 `std::thread::spawn`，您将调度控制交给了操作系统的调度器。\
您无法控制线程的调度方式。

对于 `tokio::spawn`，您将控制权交给了完全运行在用户空间内的异步执行器。\
底层的操作系统调度器不涉及决定要运行哪个任务。\
我们现在掌控了这个决定权，通过我们选择使用的执行器来实现。