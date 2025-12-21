# 响应式系统 (Reactivity System)

Sinter UI 的响应式系统是驱动整个应用的核心引擎。它位于 `sinter_ui/src/reactivity` 目录下，采用**拓扑依赖图**的方式管理状态同步。

## 1. 核心原语 (Primitives)

### 1.1 `Signal<T>` (信号)
信号是响应式图中的**源节点 (Source Nodes)**。它们包含具体的值，并在值发生变化时通知订阅者。

*   `create_signal(value)`: 创建一个读写信号对 `(ReadSignal, WriteSignal)`。
*   `ReadSignal` 和 `WriteSignal` 都是实现了 `Copy` 的轻量级句柄（内部只是一个 `NodeId`）。

### 1.2 `Effect` (副作用)
副作用是响应式图中的**观察者节点 (Observer Nodes)**。它们不返回值，但执行某些操作（如更新 DOM、发起网络请求）。

*   `create_effect(f)`: 立即执行闭包 `f`，并在执行过程中自动追踪读取过的 Signal。当这些 Signal 变化时，Effect 会重新执行。

### 1.3 `Memo<T>` (计算缓存)
Memo 是既作为观察者又作为源的**中间节点**。它可以缓存计算结果，只有当其依赖发生实质性变化时，才会通知下游。

*   `create_memo(f)`: 创建一个派生信号。

### 1.4 `Resource<T>` (异步资源)
用于处理异步数据加载（如 `fetch` 请求）。它结合了 `Signal` 和 `Future`，并与 `Suspense` 组件集成，自动管理 loading 状态。

## 2. 运行时架构 (Runtime Architecture)

Sinter UI 的运行时 (`Runtime`) 采用**集中式管理**设计，所有节点数据存储在一个全局的 `SlotMap` 中。

### 2.1 数据结构

```rust
pub(crate) struct Node {
    pub kind: NodeType,               // Signal | Effect | Scope
    pub value: Option<Box<dyn Any>>,  // 存储数据 (仅 Signal)
    pub computation: Option<Rc<dyn Fn()>>, // 存储闭包 (仅 Effect)
    pub subscribers: Vec<NodeId>,     // 谁依赖我 (Signal -> Effects)
    pub dependencies: Vec<NodeId>,    // 我依赖谁 (Effect -> Signals)
    pub children: Vec<NodeId>,        // 拥有权树 (用于清理)
    pub cleanups: Vec<Box<dyn FnOnce()>>,
    // ...
}
```

### 2.2 依赖追踪机制 (Automatic Dependency Tracking)

1.  **全局指针**: Runtime 维护一个 `current_owner` 指针，指向当前正在运行的 Effect 或 Scope。
2.  **追踪 (Track)**: 当调用 `signal.get()` 时，Signal 将 `current_owner` (Effect) 添加到自己的 `subscribers` 列表，同时将自己添加到 Effect 的 `dependencies` 列表。
3.  **触发 (Trigger)**: 当调用 `signal.set(val)` 时，Runtime 遍历该 Signal 的 `subscribers`，并将这些 Effect 标记为需要运行（目前直接同步运行）。

### 2.3 内存管理与生命周期 (Scope System)

由于 Rust 没有垃圾回收 (GC)，Sinter UI 引入了 **Scope (作用域)** 概念来解决响应式节点的生命周期管理问题。

*   **所有权树**: 所有的 Effect、Signal、Memo 都必须归属于某个 Scope。
*   **级联销毁**: 当父 Scope 被销毁时，它会递归销毁所有子节点（Effect, Signal, Sub-Scope）。
*   **`on_cleanup`**: 允许用户注册清理回调（如移除 DOM 事件监听器、清除定时器），这些回调会在 Scope 销毁或 Effect 重新运行时被调用。

这种设计通过 `SlotMap` 的代（Generation）索引机制，优雅地解决了悬垂指针问题 (`dropped value`)，并确保了无内存泄漏。
