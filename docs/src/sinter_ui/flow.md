# 控制流组件 (Flow Control)

Sinter UI 不依赖编译器魔法（如 `rsx!` 宏中的 `if/else` 转换），而是提供了一组高效的原生组件来处理动态内容。

## 1. `Show` (条件渲染)

用于在两个视图之间切换。

```rust
// 基础用法
Show::new(
    move || count.get() > 5, // 条件闭包
    move || div().text("Big!"), // True 分支
    Some(move || div().text("Small...")) // False 分支 (可选)
)

// 语法糖
count.when(move || div().text("Big!"))
     .otherwise(move || div().text("Small..."))
```

**原理**:
`Show` 内部创建一个 Effect 监听条件变化。
*   当条件从 `false` 变为 `true` 时，挂载 True 分支。
*   当条件从 `true` 变为 `false` 时，清空容器并挂载 False 分支。
*   如果条件保持不变（例如连续多次为 `true`），则不会发生任何 DOM 操作。

## 2. `For` (列表渲染)

用于高效渲染动态列表。

```rust
For::new(
    move || list.get(), // 数据源
    |item| item.id,     // Key 函数 (用于 Diff)
    |item| div().text(item.name) // 渲染模板
)
```

**Keyed Diffing 算法**:
Sinter UI 实现了类似 React/Vue 的 Keyed Diffing 算法，但针对细粒度更新进行了优化。
1.  **复用 DOM**: 当列表顺序变化时，它会尽可能移动现有的 DOM 节点 (`insertBefore`) 而不是重新创建。
2.  **复用 Scope**: 每个列表项都有独立的 Scope。当项被移动时，其对应的 Scope 和 Signal 绑定保持不变，无需重新计算。
3.  **高性能**: 时间复杂度接近 O(n)，能轻松处理数千个动态项。

## 3. `Dynamic` (动态组件)

用于根据状态动态渲染任意类型的视图。类似 `<component is="...">`。

```rust
Dynamic::new(move || {
    match route.get() {
        "/home" => Home().into_any(),
        "/about" => About().into_any(),
        _ => NotFound().into_any(),
    }
})
```

## 4. `Suspense` (异步边界)

配合 `Resource` 使用，提供优雅的异步加载体验。

```rust
Suspense::suspense()
    .fallback(move || div().text("Loading..."))
    .children(move || {
        // 这里可以使用 async resource
        let user = resource.get(); 
        UserProfile(user)
    })
```

**原理**:
`Suspense` 并没有使用 React 的抛出异常机制，而是依赖 `SuspenseContext`。
1.  当 `Resource` 开始加载时，它会查找到最近的 `SuspenseContext` 并增加计数器。
2.  `Suspense` 组件监听计数器。当计数 > 0 时，显示 fallback；当计数 == 0 时，显示 children。
3.  这种机制使得多个并行的异步请求可以共享同一个 Loading 指示器。
