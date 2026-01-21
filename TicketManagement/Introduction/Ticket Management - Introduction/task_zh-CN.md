在上一章中，我们在一个真空环境中建模了 `Ticket`：我们定义了它的字段及其约束，学习了如何在 Rust 中最好地表示它们，但没有考虑 `Ticket` 如何融入更大的系统中。  
我们将在本章中围绕 `Ticket` 构建一个简单的工作流程，引入一个（基础）管理系统来存储和检索票据。

这个任务将给我们提供一个探索 Rust 新概念的机会，例如：

- 栈分配的数组
- `Vec`，一种可增长的数组类型
- `Iterator` 和 `IntoIterator`，用于遍历集合
- 切片（`&[T]`），用于操作集合的部分
- 生命周期，用于描述引用的有效范围
- `HashMap` 和 `BTreeMap`，两种键值对数据结构
- `Eq` 和 `Hash`，用于比较 `HashMap` 中的键
- `Ord` 和 `PartialOrd`，用于操作 `BTreeMap`
- `Index` 和 `IndexMut`，用于访问集合中的元素

<br/>

## 任务  
任务简介是完成 `intro` 函数。  
该函数应返回字符串：***I'm ready to build a ticket management system!***