## 泛型和关联类型

让我们重新审视之前学习过的两个 trait，`From` 和 `Deref` 的定义：

```rust
pub trait From<T> {
    fn from(value: T) -> Self;
}

pub trait Deref {
    type Target;
    
    fn deref(&self) -> &Self::Target;
}
```

它们都包含类型参数。\
对于 `From`，这是一个泛型参数 `T`。\
对于 `Deref`，这是一个关联类型 `Target`。

二者有什么区别呢？为什么要使用其中一种而不是另一种呢？

## 最多只能有一个实现

由于 deref 类型强制转换的工作原理，对于一个给定的类型只能有一个“目标”类型。例如，`String`\
只能 deref 为 `str`。\
这是为了避免歧义：如果对一个类型可以多次实现 `Deref`，当你调用 `&self` 方法时，编译器该选择哪个 `Target` 类型？

这就是 `Deref` 使用关联类型 `Target` 的原因。\
关联类型是由**trait 实现**唯一确定的。\
由于 `Deref` 只能实现一次，你只能为给定类型指定一个 `Target`，这样就不会出现歧义。

## 泛型 trait

另一方面，你可以对一个类型多次实现 `From`，**只要输入类型 `T` 不同**。\
例如，你可以为 `WrappingU32` 实现 `From`，并分别使用 `u32` 和 `u16` 作为输入类型：

```rust
impl From<u32> for WrappingU32 {
    fn from(value: u32) -> Self {
        WrappingU32 { inner: value }
    }
}

impl From<u16> for WrappingU32 {
    fn from(value: u16) -> Self {
        WrappingU32 { inner: value.into() }
    }
}
```

这是可行的，因为 `From<u16>` 和 `From<u32>` 被视为**不同的 trait**。\
这里没有歧义：编译器可以根据待转换值的类型来确定使用哪种实现。

## 案例研究：`Add`

最后的一个例子，我们来看来自标准库的 `Add` trait：

```rust
pub trait Add<RHS = Self> {
    type Output;
    
    fn add(self, rhs: RHS) -> Self::Output;
}
```

它同时使用了两种机制：

- 它有一个泛型参数 `RHS`（右侧操作数），默认为 `Self`
- 它有一个关联类型 `Output`，表示加法操作结果的类型

### `RHS`

`RHS` 是一个泛型参数，用于允许不同类型之间进行加法操作。\
例如，你会在标准库中发现以下两种实现：

```rust
impl Add<u32> for u32 {
    type Output = u32;
    
    fn add(self, rhs: u32) -> u32 {
      //                      ^^^
      // 这里也可以写成 `Self::Output`。
      // 编译器对此并不在意，只要你在这里指定的类型
      // 和上面分配给 `Output` 的类型匹配即可。
      // [...]
    }
}

impl Add<&u32> for u32 {
    type Output = u32;
    
    fn add(self, rhs: &u32) -> u32 {
        // [...]
    }
}
```

这允许以下代码正常编译：

```rust
let x = 5u32 + &5u32 + 6u32;
```

因为 `u32` 同时实现了 `Add<&u32>` 和 `Add<u32>`。

### `Output`

`Output` 表示加法操作结果的类型。

为什么我们需要 `Output` 呢？我们不能直接使用实现 `Add` 的类型 `Self` 作为输出类型吗？\
我们可以，但这会限制 trait 的灵活性。例如，在标准库中，你会找到以下实现：

```rust
impl Add<&u32> for &u32 {
    type Output = u32;

    fn add(self, rhs: &u32) -> u32 {
        // [...]
    }
}
```

在这里实现 trait 的类型是 `&u32`，但加法操作的结果是 `u32`。\
如果 `add` 必须返回 `Self`（即 `&u32`），那么这种实现将不可能完成[^flexible]。\
`Output` 让标准库得以将实现者和返回类型分离，从而支持这种情况。

另一方面，`Output` 不能是一个泛型参数。\
一旦操作数的类型确定，操作的输出类型**必须**能够唯一确定。\
这就是它是一个关联类型的原因：对于给定的实现者和泛型参数组合，只有一个 `Output` 类型。

## 结论

总结：

- 如果对于给定的 trait 实现类型必须是唯一确定的，那就使用**关联类型**。
- 如果你希望允许为同一类型多次实现 trait，但有不同的输入类型，请使用**泛型参数**。

[^flexible]: 灵活性并非没有代价：由于引入了 `Output`，trait 的定义变得更加复杂，开发者也需要仔细考虑返回的类型。\
这个权衡只有在真正需要这种灵活性时才有意义。在设计自己的 trait 时要牢记这一点。