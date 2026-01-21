## 依赖项

一个包可以通过在其 `Cargo.toml` 文件的 `[dependencies]` 部分中列出其他包来依赖它们。\
指定依赖项的最常见方法是提供其名称和版本：

```toml
[dependencies]
thiserror = "1"
```

这将把 `thiserror` 添加为你的包的依赖项，版本最低是 `1.0.0`。\
`thiserror` 将从 [crates.io](https://crates.io)，Rust 的官方包注册库中下载。\
当你运行 `cargo build` 时，`cargo` 会经历以下几个阶段：

- 依赖解析
- 下载依赖项
- 编译你的项目（包括你自己的代码和依赖项）

如果你的项目有一个 `Cargo.lock` 文件并且你的清单文件没有被修改，依赖解析的过程会被跳过。\
`Cargo.lock` 文件是 `cargo` 在成功完成依赖解析后自动生成的：它包含了项目中使用的所有依赖项的确切版本，并确保在不同的构建环境（如 CI）中始终一致地使用相同的版本。\
如果你正在和多个开发者一起协作开发项目，你应该将 `Cargo.lock` 提交到版本控制系统中。

你可以使用 `cargo update` 来更新 `Cargo.lock` 文件，使其包含所有依赖项的最新（兼容的）版本。

### 路径依赖

你还可以通过 **路径** 指定依赖项。\
这在处理多个本地包时非常有用。

```toml
[dependencies]
my-library = { path = "../my-library" }
```

路径是相对于声明该依赖项的包的 `Cargo.toml` 文件的。

### 其他来源

查阅 [Cargo 文档](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html)，了解更多关于从哪里获取依赖项以及如何在 `Cargo.toml` 文件中指定它们的详细信息。

## 开发依赖

你还可以指定仅在开发时需要的依赖项——即它们仅会在运行 `cargo test` 时被引入。\
这些依赖项应该放在你的 `Cargo.toml` 文件中的 `[dev-dependencies]` 部分：

```toml
[dev-dependencies]
static_assertions = "1.1.0"
```

在本书中，我们使用过一些这样的依赖项，以简化测试代码。