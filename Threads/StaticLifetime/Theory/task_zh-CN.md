## `'static`

如果你尝试在上一个练习中从向量中借用一个切片，你可能会遇到一个类似以下的编译错误：

```text
error[E0597]: `v` does not live long enough
   |
11 | pub fn sum(v: Vec<i32>) -> i32 {
   |            - binding `v` declared here
...
15 |     let right = &v[split_point..];
   |                  ^ borrowed value does not live long enough
16 |     let left_handle = spawn(move || left.iter().sum::<i32>());
   |                             -------------------------------- 
                     argument requires that `v` is borrowed for `'static`
19 | }
   |  - `v` dropped here while still borrowed
```

`argument requires that v is borrowed for 'static`，这是什么意思？

`'static` 生命周期是 Rust 中的一个特殊生命周期。\
它表示值在程序的整个生命周期中都是有效的。

## 独立线程

通过 `thread::spawn` 启动的线程可以 **比启动它的线程存活更久**。\
例如：

```rust
use std::thread;

fn f() {
    thread::spawn(|| {
        thread::spawn(|| {
            loop {
                thread::sleep(std::time::Duration::from_secs(1));
                println!("Hello from the detached thread!");
            }
        });
    });
}
```

在这个例子中，第一个被启动的线程会进一步启动一个子线程，
该子线程每秒打印一次消息。\
然后，第一个线程会结束并退出。当第一个线程退出时，
它的子线程将 **继续运行**，只要整个程序还在运行。\
用 Rust 的术语来说，我们称子线程 **比它的父线程存活得更久**。

## `'static` 生命周期

由于启动的线程可以：

- 比启动它的线程（即父线程）存活更久
- 一直运行到程序退出

因此，它不能借用任何在程序退出前可能被释放的值；\
如果违反这个约束，我们可能会遇到使用已释放数据的错误。\
这就是为什么 `std::thread::spawn` 的函数签名要求传递给它的闭包必须具有 `'static` 生命周期：

```rust
pub fn spawn<F, T>(f: F) -> JoinHandle<T> 
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static
{
    // [..]
}
```

## `'static` 并不（仅仅）与引用相关

Rust 中的所有值都有生命周期，不仅仅是引用。

特别是，一个拥有其数据的类型（如 `Vec` 或 `String`）\
满足 `'static` 约束：如果你拥有它，那么即使创建它的函数已经返回，\
你也可以继续使用它。

因此，你可以将 `'static` 解释为一种表述：

- 给我一个拥有的值
- 给我一个在程序整个生命周期内都有效的引用

第一种方式是你在上一个练习中解决问题的方法：\
通过分配新的向量来保存原向量的左右部分，然后将它们移动到启动的线程中。

## `'static` 引用

现在来说说第二种情况，即在程序整个生命周期内有效的引用。

### 静态数据

最常见的情况是对 **静态数据** 的引用，比如字符串字面量：

```rust
let s: &'static str = "Hello world!";
```

由于字符串字面量在编译时是已知的，\
Rust 将它们存储在你的可执行文件的一个区域中，\
这个区域被称为 **只读数据段**。  
所有指向该区域的引用在程序运行时都是有效的；它们满足 `'static` 合约。

## 延伸阅读

- [数据段](https://en.wikipedia.org/wiki/Data_segment)