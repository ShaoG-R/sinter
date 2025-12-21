# DOM 引擎 (DOM Engine)

Sinter UI 的 DOM 引擎旨在提供一层**类型安全**且**极其轻量**的浏览器 API 抽象。它不使用 VDOM，而是直接生成 DOM 操作指令。

## 1. 元素构建器 (Element Builder)

`sinter_ui::dom::element::Element` 是对 `web_sys::Element` 的封装。它提供了一套流式 API 来设置属性和子节点。

```rust
div()
    .id("app")
    .class("container")
    .style("color: red")
    .on_click(|| println!("Clicked"))
```

### 属性多态 (Attribute Polymorphism)
Sinter UI 利用 Rust 的 Trait 系统实现了属性值的多态。`attr()`, `id()`, `class()` 等方法接受任何实现了 `AttributeValue` 的类型：

*   **静态值** (`&str`, `String`, `bool`): 在创建时设置一次属性，零运行时开销。
*   **闭包** (`Fn() -> String`): 被视为响应式属性，会自动包裹在 `create_effect` 中，随依赖变化自动更新 DOM。
*   **信号** (`ReadSignal<T>`): 同样自动建立绑定，是最优化的路径。

## 2. 视图特征 (View Trait)

`View` 特征定义了“可以被挂载到 DOM 树上的东西”。

```rust
pub trait View {
    fn mount(self, parent: &web_sys::Node);
}
```

Sinter UI 为多种类型实现了 `View`:

1.  **基础类型**: `i32`, `f64`, `bool`, `String`, `&str` 等被渲染为 Text Node。
2.  **`Option<V>`**: `Some` 渲染内容，`None` 不渲染。
3.  **`Vec<V>`**: 渲染为节点列表片段。
4.  **元组 `(A, B, C...)`**: 允许在 `child()` 中一次性传入多个子节点。
5.  **闭包 `Fn() -> V`**: 惰性视图，通常用于 `Dynamic` 或文本绑定。
6.  **`Element`**: 元素本身也是 View。

## 3. `AnyView` 与类型擦除

由于 Rust 是强静态类型的，`div().child(a)` 和 `div().child(b)` 会产生不同的类型结构。为了支持动态组件或同一接口返回不同视图（如主题系统），Sinter UI 提供了 `AnyView`。

它内部包装了一个 `Box<dyn Render>`，允许在运行时通过动态分发挂载不同类型的视图。

```rust
pub trait IntoAnyView {
    fn into_any(self) -> AnyView;
}
```

## 4. 事件处理

事件绑定（如 `on_click`）通过 `web_sys::Closure` 实现。
重要的是，Sinter UI 会自动调用 `on_cleanup` 将事件监听器的移除逻辑注册到当前 Scope。这意味着当组件被卸载时，对应的 DOM 事件监听器会被自动移除，无需用户手动管理，防止了常见的内存泄漏。
