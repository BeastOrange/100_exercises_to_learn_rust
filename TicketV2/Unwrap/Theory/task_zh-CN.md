## 解包

`Ticket::new`现在返回一个`Result`，而不是在输入无效时直接触发程序崩溃。\
这对调用者来说意味着什么？

## 错误不能被（隐式）忽略

与异常不同，Rust 的`Result`强制你**在调用点处理错误**。\
如果你调用一个返回`Result`的函数，Rust 不允许你隐式忽略错误情况。

```rust
fn parse_int(s: &str) -> Result<i32, ParseIntError> {
    // ...
}

// 这段代码无法通过编译：我们没有处理错误情况。
// 我们必须使用`match`或者`Result`提供的某个组合器，
// 来“解包”成功值或处理错误。
let number = parse_int("42") + 2;
```

## 你得到了一个`Result`。接下来怎么办？

当你调用一个返回`Result`的函数时，你有两个主要选择：

- 如果操作失败，则触发崩溃。  
  这是通过使用`unwrap`或`expect`方法完成的。
  ```rust
  // 如果`parse_int`返回`Err`，则触发崩溃
  let number = parse_int("42").unwrap();
  // `expect`允许你提供一个自定义的崩溃信息。
  let number = parse_int("42").expect("解析整数失败");
  ```
- 使用`match`表达式解构`Result`，显式处理错误情况。
  ```rust
  match parse_int("42") {
      Ok(number) => println!("解析成功的数字: {}", number),
      Err(err) => eprintln!("错误: {}", err),
  }