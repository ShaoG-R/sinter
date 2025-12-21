# Web 应用架构 (Web App Architecture)

`sinter_web` 是 Sinter 系统的前端运行时应用。它是一个纯粹的 Single Page Application (SPA)，编译为 WebAssembly 运行在浏览器中。

## 1. 应用入口 (`app.rs`)

`app()` 函数是整个应用的根组件。它的职责非常清晰：

1.  **初始化基础设施**: 启动主题管理器 (`ThemeManager`)，创建全局状态 (`GlobalState`) 并通过 Context API 注入到组件树的顶层。
2.  **路由分发**: 使用 `use_router()` 获取当前路由状态，根据 URL 模式匹配对应的页面组件（Home, Archives, Post 等）。
3.  **布局包裹**: 将匹配到的页面内容传递给 `layout` 组件进行包裹。

```rust
// 伪代码示例
let (route, _) = use_router();
let content = move || match route.get() {
    Route::Home => home().into_any(),
    Route::Post(slug) => post_view(slug).into_any(),
    // ...
};
layout(content)
```

## 2. 路由系统 (`router.rs`)

Sinter 实现了一个轻量级的、针对内容站点优化的客户端路由系统。

*   **状态驱动**: `Route` 是一个枚举类型，被包装在 Signal 中。
*   **事件拦截**: 
    *   全局监听 `click` 事件，拦截所有同源的 `<a>` 标签点击，改为调用 `history.pushState` 并更新 Route Signal，从而实现**无刷新跳转**。
    *   监听 `popstate` 事件，以响应浏览器的前进/后退按钮。
*   **智能参数解析**: 自动解析 URL 中的 Path 和 Query String（如 `?page=2`），并将其转化为类型安全的 Signal。

## 3. 页面与数据流 (`pages.rs`)

每个页面组件（如 `home`, `post_view`）遵循相同的**Resource-Suspense** 模式：

1.  **Resource 创建**: 根据路由参数（页码或文章 Slug）创建一个 `Resource`。这个 Resource 会自动触发异步请求去获取对应的 JSON 数据（`fetch_page_data` 或 `fetch_json`）。
2.  **Context 注入**: 将 Resource 包装在 `PageDataContext` 中注入，供下层的主题组件消费。
3.  **Suspense 边界**: 使用 `Suspense` 组件包裹主题渲染逻辑。
    *   当 Resource 正在加载时，显示主题提供的 `render_loading`。
    *   当加载完成时，触发主题的 `render_home` 或 `render_post`，此时主题可以通过 Hook 直接拿到已就绪的数据。

这种设计实现了**数据获取与 UI 渲染的解耦**，同时保证了优雅的加载体验。

## 4. 全局状态 (`GlobalState`)

`GlobalState` 是穿越整个组件树的数据总线，包含：

*   `site_meta`: 站点的元数据（标题、描述等）。
*   `theme`: 当前选中的主题实例（`RwSignal<Arc<dyn Theme>>`）。
*   `manager`: 主题管理器实例，用于执行切主题操作。

由于它是 `Clone` 的（内部字段都是 `Rc` 或 `Arc`），任何组件都可以低成本地获取并订阅全局状态的变化。
