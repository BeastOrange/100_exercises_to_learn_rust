## 析构函数

在介绍堆时，我们提到您需要负责释放您分配的内存。\
在介绍借用检查器时，我们也提到在 Rust 中，您很少需要直接管理内存。

这两种说法乍看之下似乎是矛盾的。\
通过介绍**作用域**和**析构函数**，我们来看看它们如何协同工作。

## 作用域

变量的**作用域**指的是在 Rust 代码中该变量有效或者说**存活**的区域。

变量的作用域从它的声明开始。\
它会在以下任意一种情况发生时结束：

1. 声明该变量的代码块（即 `{}` 之间的代码）结束
   ```rust
   fn main() {
      // 在这之前 `x` 尚未进入作用域
      let y = "Hello".to_string();
      let x = "World".to_string(); // <-- x 的作用域从这里开始...
      let h = "!".to_string(); //   |
   } //  <---------------- ...并在这里结束
   ```
2. 变量的所有权被转移给其他地方（例如函数或另一个变量）
   ```rust
   fn compute(t: String) {
      // 执行某些操作 [...]
   }

   fn main() {
       let s = "Hello".to_string(); // <-- s 的作用域从这里开始...
       compute(s); // <------------------- ...并在这里结束
                   // 因为 `s` 被移动到了 `compute`
   }
   ```

## 析构函数

当一个值的所有者超出其作用域时，Rust 会调用它的**析构函数**。\
析构函数会尝试清理该值使用的资源，特别是它分配的内存。

您可以通过将值传递给 `std::mem::drop` 来手动调用其析构函数。\
这也是为什么您会经常听到 Rust 开发者说“该值已经被**丢弃（dropped）**”，意思是该值已超出作用域，并且其析构函数已被调用。

### 可视化 drop 点

我们可以插入显式的 `drop` 调用，以“明确”编译器为我们所做的工作。回顾前面的示例：

```rust
fn main() {
   let y = "Hello".to_string();
   let x = "World".to_string();
   let h = "!".to_string();
}
```

等效于：

```rust
fn main() {
   let y = "Hello".to_string();
   let x = "World".to_string();
   let h = "!".to_string();
   // 变量按与声明相反的顺序丢弃
   drop(h);
   drop(x);
   drop(y);
}
```

接下来看看第二个例子，其中 `s` 的所有权被转移到了 `compute`：

```rust
fn compute(s: String) {
   // 执行某些操作 [...]
}

fn main() {
   let s = "Hello".to_string();
   compute(s);
}
```

等效于：

```rust
fn compute(t: String) {
    // 执行某些操作 [...]
    drop(t); // <-- 假设 `t` 在此之前未被丢弃或移动，
             //     则编译器会在超出作用域时调用 `drop`
}

fn main() {
    let s = "Hello".to_string();
    compute(s);
}
```

注意区别：即使在 `main` 中调用了 `compute` 后 `s` 不再有效，但不会有 `drop(s)` 出现在 `main` 中。\
当您把一个值的所有权转移给函数时，您同时也**转移了清理该值的责任**。

这确保了一个值的析构函数**最多[^leak]被调用一次**，从设计上防止了[双重释放漏洞](https://owasp.org/www-community/vulnerabilities/Doubly_freeing_memory)。

### 丢弃后使用

如果您尝试在一个值被丢弃之后使用它，会发生什么？

```rust
let x = "Hello".to_string();
drop(x);
println!("{}", x);
```

如果尝试编译这段代码，您将收到一个错误提示：

```rust
error[E0382]: use of moved value: `x`
 --> src/main.rs:4:20
  |
3 |     drop(x);
  |          - value moved here
4 |     println!("{}", x);
  |                    ^ value used here after move
```

`drop` **消费**了被调用的值，这意味着在调用之后该值将不再有效。\
因此，编译器会阻止您使用它，从而避免了[释放后使用漏洞](https://owasp.org/www-community/vulnerabilities/Using_freed_memory)。

### 丢弃引用

如果一个变量包含的是引用，该怎么办？\
例如：

```rust
let x = 42i32;
let y = &x;
drop(y);
```

当您调用 `drop(y)` 时……什么都不会发生。\
如果您真的尝试编译这段代码，您会收到一个警告：

```text
warning: calls to `std::mem::drop` with a reference 
         instead of an owned value does nothing
 --> src/main.rs:4:5
  |
4 |     drop(y);
  |     ^^^^^-^
  |          |
  |          argument has type `&i32`
  |
```

这可以追溯到我们之前提到的：我们只希望析构函数被调用一次。\
您可以拥有对同一个值的多个引用——如果当其中一个引用超出作用域时就调用那个值的析构函数，其他引用会怎样？\
它们会指向一个不再有效的内存位置：即所谓的[**悬空指针**](https://en.wikipedia.org/wiki/Dangling_pointer)——这是[**释放后使用漏洞**](https://owasp.org/www-community/vulnerabilities/Using_freed_memory)的近亲。\
Rust 的所有权系统通过设计杜绝了这些类型的漏洞。

[^leak]: Rust 不保证析构函数一定会运行，例如如果您明确选择[内存泄漏](../../../Threads/Leaking%20memory/Theory/task.md)。