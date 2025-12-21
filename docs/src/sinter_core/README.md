# Sinter Core 模块设计

`sinter_core` 是 Sinter 系统的核心契约库，主要负责定义数据结构（Schema）和序列化/反序列化逻辑。它充当了 Rust 编译器（`sinter_cli`）与 WebAssembly 运行时（`sinter_ui`）之间的协议层。

## 1. 设计目标

1.  **类型共享**：确保编译端生成的 JSON 数据结构与前端读取的数据结构严格一致，消除类型不匹配导致的运行时错误。
2.  **极致轻量**：仅包含必要的结构体定义和 Serde 派生宏，作为公共依赖引入时不会显著增加 WASM 体积。
3.  **抽象语法树 (AST) 定义**：定义了通用且易于渲染的内容 AST，使前端渲染逻辑与具体的 Markdown 解析实现解耦。

## 2. 核心数据结构

### 2.1 基础元数据

#### `LiteDate`
自定义的轻量级日期结构，用于替代庞大的 `chrono::NaiveDate` 在某些场景下的序列化开销，并提供定制的 `YYYY-MM-DD` 格式化输出。

```rust
pub struct LiteDate {
    pub year: i32,
    pub month: u8,
    pub day: u8,
}
```

#### `PostMetadata`
文章的元数据信息，通常来源于 Markdown 文件的 Frontmatter。

*   `id`: 唯一标识符。
*   `title`: 文章标题。
*   `slug`:URL 友好的别名。
*   `date`: 发布日期。
*   `tags`: 标签列表。
*   `summary`: 文章摘要。

### 2.2 内容 AST (`ContentNode`)

Sinter 不直接发送 HTML 字符串给前端，而是发送一个经过简化的、类型安全的 AST。这允许前端框架（Sinter UI）使用细粒度的 DOM构建器来渲染内容，而不是危险的 `innerHTML`。

`ContentNode` 是一个递归枚举（Recursive Enum），主要包含以下节点类型：

*   **容器节点 (Container Nodes)**: `Paragraph`, `Heading`, `List`, `BlockQuote` 等，包含子节点列表。
*   **叶子节点 (Leaf Nodes)**: `Text`, `CodeBlock`, `Html` (用于保留的 HTML), `Math` (LaTeX 公式)。
*   **内联格式 (Inline Formatting)**: `Emphasis` (斜体), `Strong` (粗体), `Link` (链接), `Image` (图片)。

```rust
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ContentNode {
    Heading {
        level: u8,
        children: Vec<ContentNode>,
        // ...
    },
    Text {
        value: String,
    },
    // ...
}
```

### 2.3 数据交换格式

Sinter 采用了**分片加载**策略，因此定义了多种层级的数据结构：

1.  **`SiteMetaData` (`site_data.json`)**:
    包含全局站点配置信息，如标题、描述、总页数。这是前端应用启动时首先加载的文件。

2.  **`PageData` (`page_{n}.json`)**:
    分页数据，包含当前页的文章列表摘要 (`SitePostMetadata`) 和标签索引。这实现了首屏加载的 O(1) 复杂度，不随文章总数增加而变慢。

3.  **`Post` (`posts/{slug}.json`)**:
    包含完整的 `PostMetadata` 和 `content_ast` (AST)。只有当用户点击进入具体的文章页时，才会请求此文件。

## 3. 跨端通讯

在构建阶段，`sinter_cli` 将这些结构体序列化为 JSON 文件。
在运行阶段，`sinter_web` (WASM) 使用相同的结构体定义将 JSON 反序列化。

这种设计使得我们可以在编译时利用 Rust 的类型系统保证数据完整性，同时在运行时享受 JSON 的通用性和调试便利性。
