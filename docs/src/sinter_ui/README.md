# Sinter UI 引擎

Sinter UI 是 Sinter 系统的核心渲染引擎，它是一个**高性能、细粒度响应式**的 Rust Web 框架。它的设计深受 SolidJS 的启发，致力于在保持 Rust 强类型和内存安全特性的同时，提供极致的运行时性能。

## 1. 核心特性

*   **无虚拟 DOM (No VDOM)**: Sinter UI 不使用虚拟 DOM Diff 算法。它通过编译器或构建器直接生成针对具体 DOM 节点的更新指令。当状态变化时，只有受影响的那个文本节点或属性会被更新，开销降至最低。
*   **细粒度响应式 (Fine-Grained Reactivity)**: 状态管理基于 Signal（信号）。依赖追踪是自动的、精确的。
*   **流式构建者 API (Fluent Builder API)**: 不同于 JSX 宏，Sinter UI 采用符合 Rust 习惯的 Builder 模式（`div().class("...")`），对 IDE 的自动补全和重构非常友好。
*   **极致产物体积**: 得益于无 VDOM 运行时和 Rust 的 LTO 优化，Sinter UI 的 WASM 产物非常小（Hello World < 10KB, 完整应用 < 100KB gzip）。

## 2. 模块结构

Sinter UI 主要由三个子模块构成：

1.  [**Reactivity (响应式系统)**](./reactivity.md): 定义了 Signal, Effect, Memo, Resource 等原语，以及底层的运行时（Runtime）。
2.  [**DOM Engine (视图引擎)**](./dom.md): 封装了 `web-sys`，提供了强类型的 HTML 元素构建器、多态属性绑定和 View 特征。
3.  [**Flow Control (控制流)**](./flow.md): 提供了 `Show`, `For`, `Suspense` 等内置组件，用于高效地处理条件渲染和列表渲染。

## 3. 快速上手

```rust
use sinter_ui::prelude::*;

fn main() {
    mount_to_body(|| {
        let (count, set_count) = create_signal(0);

        div()
            .class("counter-app")
            .child((
                h1().text("Hello Sinter"),
                p().text(move || format!("Count: {}", count.get().unwrap())),
                button()
                    .text("Increment")
                    .on_click(move || set_count.update(|n| *n += 1))
            ))
    });
}
```
