# Sinter CLI 编译器设计

`sinter_cli` 是 Sinter 系统的构建工具和编译器。它负责将原始的 Markdown 内容和主题资源转化为 Sinter UI 引擎可消费的优化数据格式。

## 1. 架构概览

Sinter CLI 采用**管线式 (Pipeline)** 架构，整个构建过程是单向的数据流。核心逻辑位于 `src/compiler.rs` 中。

### 1.1 核心构建流程

1.  **配置加载 (Configuration Loading)**: 
    *   读取 `sinter.toml` 获取站点基础配置（如标题、每页文章数）。
    *   读取 `sinter_themes/themes.toml` 获取主题构建指令。
    
2.  **主题构建 (Theme Building)**:
    *   Sinter 将主题视为独立子工程。
    *   CLI 调用 `themes::process_themes`，并行执行每个主题的 `pre_build_cmd` 和 `build_cmd`。
    *   构建完成后，自动将生成的 CSS 等静态资源复制到 `sinter_web/themes/{theme_name}/` 目录。

3.  **内容扫描与并行解析 (Parallel Scanning & Parsing)**:
    *   扫描 `posts` 和 `archives` 目录下的 Markdown 文件。
    *   使用 `rayon` 线程池将解析任务分发到每个 CPU 核心。
    *   解析过程包括：Frontmatter 提取（YAML）和 正文 AST 转换。

4.  **数据分片生成 (Data Sharding)**: 
    *   **Post Chunks**: 每篇文章生成独立的 `posts/{slug}.json`。
    *   **Pagination Chunks**: 根据 `posts_per_page` 配置，将文章摘要聚合生成 `page_{n}.json`。
    *   **Site Metadata**: 生成全局站点元数据 `site_data.json`。

5.  **原子化部署 (Atomic Deployment)**: 
    *   所有构建首先在 `tempfile` 创建的临时目录中进行。
    *   只有当所有步骤顺利完成后，才会将最终产物递归复制到目标输出目录 (`sinter_web/sinter_data`)。

## 2. 关键组件详解

### 2.1 主入口 (`main.rs`)

使用 `clap` 库处理命令行参数。
*   `cargo run -p sinter_cli -- build`: 触发构建流程。
*   初始化 `tracing` 日志系统，根据 `--verbose` 标记决定日志级别。

### 2.2 编译器核心 (`compiler.rs`)

编译器是整个 CLI 的大脑，协调各个子系统的运作。

*   **并行遍历**: 使用 `WalkDir` 收集文件后，通过 `par_iter()` 转换为并行迭代器。
*   **无锁设计**: 每个文件的解析任务是独立的，不共享可变状态，极大提高了多核 CPU 利用率。
*   **错误处理**: 使用 `anyhow` 库提供上下文丰富的错误报告。任何一个文件的解析失败都会被捕获并记录，但不会导致整个构建进程立即崩溃（取决于具体实现策略，目前倾向于 fail-fast）。

### 2.3 自研 Markdown 解析器 (`compiler/markdown_parser.rs`)

Sinter 并没有简单的将 Markdown 转换为 HTML 字符串。为了让前端能够进行细粒度的 DOM 控制（这对 Sinter UI 的高性能至关重要），CLI 必须生成一个结构化的、类型安全的**抽象语法树 (AST)**。

这通过一个自定义的**下推自动机 (Pushdown Automaton)** 实现，它消费 `pulldown-cmark` 产生的事件流。

#### 2.3.1 核心数据结构

解析器由 `AstStateMachine` 驱动，它内部维护了一个状态栈：

```rust
pub struct AstStateMachine {
    // 栈结构，栈底总是根节点（Root），栈顶是当前正在构建的节点
    stack: Vec<Frame>,
}

struct Frame {
    // 节点元数据（FrameType）
    // 包含了如 Heading(level), Link(url), CodeBlock(lang) 等信息
    tag: Option<FrameType>, 
    // 当前节点已收集到的子节点列表
    children: Vec<ContentNode>,
}

enum FrameType {
    // 容器型：如 Paragraph, BlockQuote，只负责包裹子节点
    Container(fn(Vec<ContentNode>) -> ContentNode),
    // 属性型：携带特定属性，如 Heading(level), Link(url)
    Heading(u8, Option<String>, Vec<String>),
    Link(String, Option<String>),
    // ...
}
```

#### 2.3.2 解析循环 (The Parsing Cycle)

对于每一个 Markdown 事件 (`Event`)，状态机执行以下操作：

1.  **`Event::Start(Tag)` -> 入栈 (`enter_node`)**:
    *   根据 Tag 类型创建一个新的 `Frame`。
    *   如果是 `Heading`，会提取 `level`, `id`, `classes` 等属性。
    *   将 `Frame` 压入 `stack`。后续的内容将被收集到这个新 Frame 的 `children` 中。

2.  **`Event::Text/Code/Math` -> 追加内容**:
    *   获取当前栈顶的 Frame。
    *   将内容封装为叶子节点（如 `ContentNode::Text`, `ContentNode::Math`）。
    *   追加到 Frame 的 `children` 列表中。

3.  **`Event::End` -> 出栈 (`exit_node`)**:
    *   当前节点构建完成。从栈顶弹出 `Frame`。
    *   利用 `FrameType` 的构建逻辑将 `children` 组装成完整的 `ContentNode`。
    *   将生成的 `ContentNode` 追加到**新栈顶**（即父节点）的 `children` 列表中。

#### 2.3.3 支持的高级特性

解析器不仅支持标准 CommonMark，还集成了 GitHub Flavored Markdown (GFM) 和扩展特性：

*   **数学公式 (Math)**: 
    *   Inline: `$E=mc^2$` -> `ContentNode::Math { display: false }`
    *   Display: `$$...$$` -> `ContentNode::Math { display: true }`
*   **任务列表 (Task Lists)**: 解析 `- [x]` 为 `ContentNode::TaskListMarker { checked: true }`。
*   **扩展属性 (Attributes)**: 支持类似 `{#id .class}` 的标题属性语法，允许用户自定义锚点和样式类，这对生成目录 (TOC) 非常有用。
*   **元数据注入**: 通过 Frontmatter 解析，文章的元数据作为独立字段存在，不混入 AST。

### 2.4 主题构建器 (`themes.rs`)

CLI 通过 `themes.toml` 读取主题配置，主要解决以下问题：

*   **独立构建**: 允许主题使用自己的构建链（如 Tailwind, Sass, Webpack）。CLI 只负责调用命令行命令。
*   **跨平台兼容**: 自动检测操作系统 (Windows `cmd /C` vs Unix `sh -c`) 来执行构建脚本。
*   **资源同步**: 构建完成后，将指定的文件（通常是 CSS）从主题源码目录同步到 `sinter_web` 的输出目录。

## 3. 性能优化总结

*   **Rayon 并行化**: 解析 1000+ 篇文章的时间在现代多核 CPU 上仅需毫秒级。
*   **IO/CPU 分离**: 通过 `WalkDir` 快速扫描文件路径（IO），然后交给 `par_iter` 进行 CPU 密集型的解析，最大化吞吐量。
*   **AST 预计算**: 将 Markdown 解析提前到编译期，显著减轻了客户端（浏览器）的 JavaScript 主线程负担。这是 Sinter 无论是首屏可见还是交互响应都极快的重要原因。
