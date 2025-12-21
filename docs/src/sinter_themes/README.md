# 主题系统 (Theme System)

Sinter 拥有一个强大且灵活的主题系统，采用**插件化**架构设计。它允许开发者使用 Rust 编写高性能的、类型安全的主题，同时通过 WASM 动态加载机制支持运行时的无缝主题切换。

## 1. 核心契约 (Theme Contract)

所有的主题必须实现 `sinter_theme_sdk::Theme` 特征。这个特征定义了一组标准化的渲染接口，确保了主题与主程序之间的解耦。

```rust
pub trait Theme: Send + Sync + std::fmt::Debug {
    // 渲染首页
    fn render_home(&self) -> AnyView;
    // 渲染归档页
    fn render_archive(&self) -> AnyView;
    // 渲染文章详情页
    fn render_post(&self, post: Post) -> AnyView;
    // 渲染全局布局（包裹所有页面）
    fn render_layout(
        &self,
        children: Children,
        site_meta: ReadSignal<Option<SiteMetaData>>,
    ) -> AnyView;
    
    // 状态反馈视图
    fn render_loading(&self) -> AnyView;
    fn render_post_loading(&self) -> AnyView;
    fn render_post_not_found(&self) -> AnyView;
    fn render_error(&self, message: String) -> AnyView;
}
```

### 1.1 `AnyView` (类型擦除)

由于不同主题可能返回完全不同的 DOM 结构（即不同的 Rust 类型），`Theme` 接口必须统一返回 `AnyView`。这是一种类型擦除wrapper（基于 `Box<dyn View>`），允许运行时多态。

## 2. 主题管理器 (Theme Manager)

`ThemeManager` 是主题系统的中枢，负责注册、存储和切换主题。

*   **注册**: 在应用启动时（`sinter_themes::init_manager`），所有编译进来的主题会被注册到一个 `HashMap` 中。
*   **切换**: `switch_theme(name)` 方法执行以下操作：
    1.  查找目标主题实例。
    2.  动态加载该主题对应的 CSS 文件（实现无刷新换肤）。
    3.  返回新的主题实例供应用层渲染。

### 2.1 CSS 动态加载与双缓冲

为了防止切换主题时的样式闪烁（FOUC），`ThemeManager` 采用了**CSS 双缓冲**策略：
1.  创建一个新的 `<link>` 标签指向新主题的 CSS。
2.  等待新 CSS 加载完成 (`onload` 事件)。
3.  移除旧主题的 `<link>` 标签。
4.  最后更新应用状态，触发重新渲染。

## 3. 开发新主题

### 3.1 目录结构
主题作为独立的 Cargo Package 位于 `sinter_themes/packages/` 下。

```text
my_theme/
├── Cargo.toml      # 依赖 sinter_ui, sinter_core, sinter_theme_sdk
├── theme.toml      # 主题元数据与构建配置
├── src/
│   └── lib.rs      # 实现 Theme Trait
└── style/
    └── main.css    # 样式文件
```

### 3.2 注册流程
1.  在 `sinter_themes/Cargo.toml` 中添加新主题依赖。
2.  修改 `sinter_themes/src/lib.rs` 的 `init_manager` 函数进行注册。
3.  在 `sinter_themes/themes.toml` 中配置构建路径。

## 4. 上下文与数据获取

主题不应直接发起网络请求，而是应该消费由主程序提供的上下文（Context）。

*   `use_site_meta()`: 获取全局站点信息。
*   `use_page_data()`: 获取当前页的文章列表。
*   `use_current_page()`: 获取当前页码。

这种设计使得主题专注于**视图呈现**，而将数据获取和状态管理的复杂性留给主程序处理。
