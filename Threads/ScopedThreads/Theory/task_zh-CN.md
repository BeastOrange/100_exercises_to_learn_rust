## 有范围的线程

我们之前讨论的所有生命周期问题都有一个共同的根源：  
被创建的线程可能会比它的父线程存活得更久。  
我们可以通过使用**有范围的线程（scoped threads）**来规避这个问题。

```rust
let v = vec![1, 2, 3];
let midpoint = v.len() / 2;

std::thread::scope(|scope| {
    scope.spawn(|| {
        let first = &v[..midpoint];
        println!("这是 v 的前半部分：{first:?}");
    });
    scope.spawn(|| {
        let second = &v[midpoint..];
        println!("这是 v 的后半部分：{second:?}");
    });
});

println!("这是 v：{v:?}");
```

让我们来解析一下发生了什么。

## `scope`

`std::thread::scope` 函数创建了一个新的**范围（scope）**。  
`std::thread::scope` 接受一个闭包作为输入，该闭包包含一个参数：一个 `Scope` 实例。

## 有范围的线程创建

`Scope` 提供了一个 `spawn` 方法。  
与 `std::thread::spawn` 不同的是，所有使用 `Scope` 创建的线程在范围结束时会被**自动加入（joined）**。

如果我们将之前的示例“翻译”成 `std::thread::spawn`，会是这样：

```rust
let v = vec![1, 2, 3];
let midpoint = v.len() / 2;

let handle1 = std::thread::spawn(|| {
    let first = &v[..midpoint];
    println!("这是 v 的前半部分：{first:?}");
});
let handle2 = std::thread::spawn(|| {
    let second = &v[midpoint..];
    println!("这是 v 的后半部分：{second:?}");
});

handle1.join().unwrap();
handle2.join().unwrap();

println!("这是 v：{v:?}");
```

## 从环境中借用

然而，转换后的示例无法编译：编译器会提示  
由于 `&v` 的生命周期不是 `'static`，所以它不能在我们创建的线程中使用。

但使用 `std::thread::scope` 时不会出现这个问题——你可以**安全地从环境中借用**。

在我们的示例中，`v` 是在线程创建点之前生成的。  
它会在 `scope` 返回之后才被销毁。同时，所有在 `scope` 中创建的线程都**保证在** `scope` 返回之前完成，  
因此不会有悬空引用的风险。

编译器不会报错！