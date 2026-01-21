## 线程

在我们开始编写多线程代码之前，让我们先退一步，讨论一下什么是线程，以及为什么我们可能需要使用它们。

## 什么是线程？

**线程**是由底层操作系统管理的一个执行上下文。\
每个线程都有其自己的堆栈和指令指针。

一个**进程**可以管理多个线程。\
这些线程共享同一块内存空间，这意味着它们可以访问相同的数据。

线程是一个**逻辑**概念。最终，在一个 CPU 核心（**物理**执行单元）上一次只能运行一段指令。\
由于线程的数量可以远多于 CPU 核心的数量，操作系统的**调度器**负责决定在任意时间运行哪个线程，\
通过在线程之间分配 CPU 时间来最大化吞吐量和响应速度。

## `main`

当一个 Rust 程序启动时，它运行在一个单线程上，称为**主线程**。\
此线程由操作系统创建，负责运行 `main` 函数。

```rust
use std::thread;
use std::time::Duration;

fn main() {
    loop {
        thread::sleep(Duration::from_secs(2));
        println!("Hello from the main thread!");
    }
}
```

## `std::thread`

Rust 的标准库提供了一个模块 `std::thread`，可以用来创建和管理线程。

### `spawn`

你可以使用 `std::thread::spawn` 创建新线程并在其中执行代码。

例如：

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        loop {
            thread::sleep(Duration::from_secs(1));
            println!("Hello from a thread!");
        }
    });
    
    loop {
        thread::sleep(Duration::from_secs(2));
        println!("Hello from the main thread!");
    }
}
```

如果你在 [Rust playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=afedf7062298ca8f5a248bc551062eaa) 上运行这个程序，\
你会看到主线程与创建的线程并发运行。\
每个线程都会独立于其他线程取得进展。

### 进程终止

当主线程结束时，整体进程将退出。\
已创建的线程会继续运行，直到自身结束或主线程结束。

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        loop {
            thread::sleep(Duration::from_secs(1));
            println!("Hello from a thread!");
        }
    });

    thread::sleep(Duration::from_secs(5));
}
```

在上述例子中，你可以预期会看到大约五次打印 "Hello from a thread!" 的信息。\
然后主线程会结束（当 `sleep` 调用返回时），由于整体进程退出，已创建的线程也会被终止。

### `join`

你还可以通过对 `spawn` 返回的 `JoinHandle` 调用 `join` 方法来等待已创建线程结束。

```rust
use std::thread;
fn main() {
    let handle = thread::spawn(|| {
        println!("Hello from a thread!");
    });

    handle.join().unwrap();
}
```

在这个例子中，主线程将等待已创建的线程结束后再退出。\
这引入了一种线程之间的**同步**形式：程序退出前，你可以确保看到消息 \
"Hello from a thread!" 被打印在屏幕上，因为主线程会等到已创建线程完成后才会结束。