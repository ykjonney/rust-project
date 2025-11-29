<!--
为 AI 编码代理准备的仓库特定说明。
请保持简洁、可操作，重点在于让代理能快速定位构建、运行和代码模式。
-->

# Copilot 指南（仓库特定）

此仓库是一个 Rust workspace（根目录 `Cargo.toml` 的 `members` 列表），包含若干小型示例/练习 crate。

- **关键路径**:
  - 根 `Cargo.toml`：工作区成员定义（`async-io`, `hello_macro`, `minigrep`, `algorithm`, `hello_world`, `lua`）
  - `algorithm/`：算法实现（`src/lib.rs`, `src/sort.rs`, `src/kmp.rs`，`examples/sort.rs`）
  - `async-io/`：异步示例（`src/main.rs`）
  - `hello_macro/` 与 `hello_macro_derive/`：展示 proc-macro（宏派生）实现（注意：分为 lib 与 derive crate）
  - `minigrep/`：小型 grep 示例（有 `poem.txt` 测试数据）
  - `lua/`：一个小型 Lua 解释器（`lex.rs`, `parse.rs`, `vm.rs`, `bytecode.rs`）

## 大局与设计意图

- 这是一个教学/练习型仓库，采用 Rust workspace 把若干互相独立但演示不同概念的小 crate 组织在一起。
- 大多数 crate 都是单一二进制或库，旨在展示：算法实现、示例二进制、proc-macro 用法与一个小型解释器。
- 变更通常局部（修改某个 crate），但若修改共享库或 proc-macro，请在整个 workspace 上编译/测试。

## 常用命令（在仓库根目录执行）

- 全量构建：`cargo build --workspace` 或 `cargo build`
- 全量测试：`cargo test --workspace`
- 构建/运行单个 crate：`cargo build -p <crate>` / `cargo run -p <crate> -- [args...]`
- 运行 crate 中的示例（例如 `algorithm/examples/sort.rs`）：
  - `cargo run -p algorithm --example sort`
- 运行可执行文件（直接从 target）：`target/debug/<crate>`（构建后）

## 常见开发工作流与注意事项

- 编辑某个 crate（例如 `algorithm`）时，可只构建该 crate：`cargo build -p algorithm`。但若改动是库 API 或 proc-macro，使用 `cargo build --workspace`。
- 修改 proc-macro（`hello_macro_derive/`）会影响依赖它的 crate，构建时可能触发重新编译多个 crate。
- 当需要示例数据时（`minigrep/poem.txt`），放在 crate 根，下游代码直接以相对路径读取或作为运行参数传入。

## 代码风格与约定（可在本仓库中发现的模式）

- 每个 crate 保持简单、以单一职责为主：库代码放在 `src/lib.rs`，可执行程序在 `src/main.rs`，示例在 `examples/`。
- 如果你要改进接口，要同时更新相应 crate 的 `src/lib.rs` 和依赖该库的示例/二进制。
- proc-macro 实现放在独立的 `*_derive` crate，引用时通过正常的 `Cargo.toml` 依赖声明。

## 典型文件参考（在修改建议或自动补全时优先查看）

- `algorithm/src/lib.rs`：算法集合的导出点。
- `algorithm/examples/sort.rs`：如何调用 `algorithm` 中的排序函数的示例。
- `hello_macro_derive/src/builder.rs`：proc-macro 的实现示例。
- `lua/src/{lex.rs,parse.rs,vm.rs}`：跨模块实现解释器的示例，展示如何把功能拆分到多个源文件。
- `minigrep/src/main.rs`：一个小型 CLI 的参数与 IO 处理示例。

## 测试与格式化

- 在提交前运行：`cargo fmt --all` 与 `cargo clippy --all-targets -- -D warnings`（如启用 clippy）。
- 运行单个 crate 的测试：`cargo test -p minigrep`。

## CI / 发布 注意点

- 当前仓库没有检测到 CI 配置文件（`.github/workflows`），如需要自动化，请把 `cargo test --workspace` 作为主要步骤。
- 新增 crate 时，记得在根 `Cargo.toml` 的 `members` 中注册新成员。

## 当你卡住时（诊断提示）

- 构建错误：先尝试 `cargo clean` 后 `cargo build --workspace`，查看完整错误输出。
- 依赖/版本冲突：检查各 crate 的 `Cargo.toml` 是否对同一依赖使用不兼容的版本。
- proc-macro 问题：proc-macro 的变化常常引发长期缓存或重复编译，使用 `cargo clean` 并重试。

---

如果你希望我把指南扩展为英文版本、添加 CI 示例或把常用 run/test 命令写进仓库 `Makefile`/`scripts/`，请告诉我你想要的格式。现在我会把此初稿提交到仓库，然后请你审阅哪些部分不够具体或遗漏重要流程。
