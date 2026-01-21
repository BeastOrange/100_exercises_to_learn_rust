## 崩溃

让我们回到["变量"课程](../../Variables/Theory/task.md)中您编写的`speed`函数。  
它可能看起来像这样：

```rust
fn speed(start: u32, end: u32, time_elapsed: u32) -> u32 {
    let distance = end - start;
    distance / time_elapsed
}
```

如果您有敏锐的眼光，可能已经注意到一个问题[^one]：如果`time_elapsed`为零会发生什么？

您可以在[Rust playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=36e5ddbe3b3f741dfa9f74c956622bac)上试一试！\
程序将会因以下错误消息退出：

```text
thread 'main' panicked at src/main.rs:3:5:
attempt to divide by zero
```

这就是所谓的**崩溃**。\
崩溃是Rust用来表示发生了严重错误，以致程序无法继续执行的方式，它是一种**无法恢复的错误**[^catching]。除以零就属于这样的错误。

## `panic!` 宏

您可以通过调用`panic!`宏[^macro]来有意触发一个崩溃：

```rust
fn main() {
    panic!("This is a panic!");
    // 下面的代码将永远不会被执行
    let x = 1 + 2;
}
```

在Rust中还有其他用于处理可恢复错误的机制，我们[稍后会讲到](../../../Ticket%20v2/Fallibility/Theory/task.md)。  
目前，我们将先使用崩溃作为一种简单但严厉的临时解决方案。

## 扩展阅读

- [`panic!` 宏的文档](https://doc.rust-lang.org/std/macro.panic.html)

[^one]: `speed`函数中还有另一个问题，我们很快就会讨论。您能发现吗？

[^catching]: 您可以尝试捕获一次崩溃，但这应该是一种特定情况下的最后手段。

[^macro]: 如果后面跟着`!`，它就是一个宏调用。暂时可以将宏看作是“增强版的函数”。我们会在课程的后面详细探讨它们。