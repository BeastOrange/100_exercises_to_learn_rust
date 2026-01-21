## `Error::source`

我们还需要讨论一个内容，才能完整覆盖 `Error` 特性：`source` 方法。

```rust
// 这次是完整定义！
pub trait Error: Debug + Display {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
```

`source` 方法是一种访问**错误原因**的方式（如果存在）。\
错误通常是层层相连的，这意味着一个错误是另一个错误的原因：你可能有一个高级别错误（例如，无法连接到数据库），而其原因是较低级别的错误（例如，无法解析数据库主机名）。\
`source` 方法允许你“遍历”完整的错误链，这在捕获日志中的错误上下文时非常有用。

## 实现 `source`

`Error` 特性提供了一个默认实现，该实现始终返回 `None`（即没有底层原因）。这就是为什么你在之前的练习中无需考虑 `source`。\
你可以覆盖此默认实现，以为你的错误类型提供一个原因。

```rust
use std::error::Error;

#[derive(Debug)]
struct DatabaseError {
    source: std::io::Error
}

impl std::fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Failed to connect to the database")
    }
}

impl std::error::Error for DatabaseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}
```

在此例中，`DatabaseError` 包装了一个 `std::io::Error` 作为其来源。\
然后我们覆盖了 `source` 方法，当调用时返回此来源。

## `&(dyn Error + 'static)`

`&(dyn Error + 'static)` 是什么类型？\
我们来解析一下：

- `dyn Error` 是一个**特性对象**。它用于引用实现了 `Error` 特性的任何类型。
- `'static` 是一个特殊的**生命周期标注**。\
  `'static` 意味着该引用在“我们需要的时间段内”始终有效，即整个程序的运行期间。

组合起来：`&(dyn Error + 'static)` 是一个指向实现了 `Error` 特性并在整个程序执行期间有效的特性对象的引用。

目前不必太担心这些概念。我们将在后续章节中更详细地讨论它们。

## 使用 `thiserror` 实现 `source`

`thiserror` 为你的错误类型提供了三种自动实现 `source` 的方式：

- 名为 `source` 的字段会自动用作错误的来源。
  ```rust
  use thiserror::Error;

  #[derive(Error, Debug)]
  pub enum MyError {
      #[error("Failed to connect to the database")]
      DatabaseError {
          source: std::io::Error
      }
  }
  ```
- 使用 `#[source]` 注释的字段会自动用作错误的来源。
  ```rust
  use thiserror::Error;

  #[derive(Error, Debug)]
  pub enum MyError {
      #[error("Failed to connect to the database")]
      DatabaseError {
          #[source]
          inner: std::io::Error
      }
  }
  ```
- 使用 `#[from]` 注释的字段会自动用作错误的来源，**并且**\
  `thiserror` 会自动生成一个 `From` 实现，用于将注释的类型转换为你的错误类型。
  ```rust
  use thiserror::Error;

  #[derive(Error, Debug)]
  pub enum MyError {
      #[error("Failed to connect to the database")]
      DatabaseError {
          #[from]
          inner: std::io::Error
      }
  }
  ```

## `?` 运算符

`?` 运算符是用于传播错误的简写方式。\
当在返回 `Result` 的函数中使用时，如果 `Result` 是 `Err`，它会提前以错误返回。

例如：

```rust
use std::fs::File;

fn read_file() -> Result<String, std::io::Error> {
    let mut file = File::open("file.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
```

等价于：

```rust
use std::fs::File;

fn read_file() -> Result<String, std::io::Error> {
    let mut file = match File::open("file.txt") {
        Ok(file) => file,
        Err(e) => {
            return Err(e);
        }
    };
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => (),
        Err(e) => {
            return Err(e);
        }
    }
    Ok(contents)
}
```

你可以使用 `?` 运算符显著简化错误处理代码。\
特别是，`?` 运算符会自动将可能失败操作的错误类型转换为函数的错误类型，如果可能进行转换（即存在适当的 `From` 实现）。