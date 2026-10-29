# OCLive Kernel Base

[![CI](https://github.com/linkaiheng2233-cyber/oclive-kernel/actions/workflows/ci.yml/badge.svg)](https://github.com/linkaiheng2233-cyber/oclive-kernel/actions/workflows/ci.yml)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache--2.0-blue.svg)](LICENSE)
[![Rust: tested 1.97.1](https://img.shields.io/badge/Rust-tested%201.97.1-blue.svg)](rust-toolchain.toml)

**六个可独立使用的 Rust Base 接口，组合与执行顺序由 Host 决定。**

[English introduction](README.en.md) · [边界与契约](docs/boundary.md) · [贡献](CONTRIBUTING.md) · [安全报告](SECURITY.md) · [变更记录](CHANGELOG.md)

这是从 OCLive 已落地公共 API 提取的独立源码库，提供借用请求、显式失败和默认开启的角色一致性剖面。
Host 选择能力实现、组织材料并驱动调用；内核不携带 Host 状态、调度器、数据库、网络或模型适配器。
package 发布保持关闭，来源及主仓兼容记录见 [extraction](docs/extraction.md)。

## 先运行两个 Host

贡献工具链固定为 **Rust 1.97.1**，由 [rust-toolchain.toml](rust-toolchain.toml) 选择。
这是已验证工具链，尚未声明更早版本的 MSRV。首次运行需要下载工具链和锁定依赖；缓存完备时可添加 `--offline`。

```sh
git clone https://github.com/linkaiheng2233-cyber/oclive-kernel.git
cd oclive-kernel
cargo run --locked -p oclive_minimal_host
cargo run --locked -p oclive_non_role_host
```

角色 Host 接入人设与一条合成视觉引用，非角色 Host 只处理诊断材料。二者都选择 Memory → Prompt → LLM，
Emotion、Event、Agent 不绑定也不调用。这个顺序属于示例，六槽不要求每轮全跑。

角色 Host 应显示：

```text
MemoryBase -> LiteralMemory (Host implementation)
PromptBase -> LiteralPrompt (Host implementation)
LlmBase -> DemoLlm (deterministic substitute, no model inference)
EmotionBase / EventBase / AgentBase: not bound, not called
selected=2 calls: memory=1 prompt=1 llm=1
```

非角色 Host 则显示 `selected=1 calls: memory=1 prompt=1 llm=1`。最终文本来自确定性替身，
没有真实模型推理、网络或外部操作；`host-owned:portrait` 不会读取或展示图片。

开发练习：将角色 Host 的 Memory `limit: 2` 改为 `limit: 1`，输出会只保留一条 river 材料。
现有测试已比较这两种绑定；字面匹配与条数限制是示例实现的选择。

## 六槽分别做什么

| Base | 本次调用的职责 | 正常返回类型 |
|---|---|---|
| Memory | 从显式候选材料中检索相关内容 | `Vec<String>` |
| Emotion | 分析材料表达的情绪 | `Option<String>` |
| Event | 分析所述事件的含义或可能影响 | `Option<String>` |
| Prompt | 按本次要求组织材料 | `String` |
| LLM | 从调用者已准备的输入生成文本 | `String` |
| Agent | 处理委托并用文本报告结果 | `String` |

所有方法返回包含 `Result` 的局部 boxed future，可通过 `dyn` 选择实现。正常结果可以为空；
失败由 `BaseCallError` 表达。能力返回值不等于整个回合结束，也不授予外部操作权限。
精确语义以 [Base 文档](crates/oclive_kernel_base/src/slot_base.rs) 与 [请求/失败定义](crates/oclive_kernel_base/src/requests.rs) 为准。

## 角色剖面：推荐七张，最低一张

`character-profile` 默认开启，提供 `CharacterProfileDefinition`、
`parse_character_profile_definition` 和 `validate_character_profile_definition`。
内容最低要求是非空人设文本、至少一条非空视觉引用；**推荐 7 张，兼容单图**，不规定七种情绪或文件名。
宿主负责资源解析与渲染；直接构造或反序列化结构体后仍需校验。

旧 `minimal_role` 模块、`MinimalRoleDefinition` 及旧解析/校验函数保留为同一实现的别名。
JSON 字段、错误文本、原文和引用顺序保持兼容；规范 Rust 类型名会反映在 Debug/type-name 输出中。

```sh
cargo test --locked -p oclive_kernel_base --no-default-features
cargo test --locked -p oclive_kernel_base --no-default-features --doc
```

关闭后新旧剖面导入均不可用，库的正常/构建依赖不再包含 `serde`/`serde_json`。
Cargo 会合并功能请求：其他选中依赖若开启剖面，单条 `default-features = false` 无法否决。
所以必须单独选择纯库或非角色 Host 检查，不能用整个 workspace 的 `--no-default-features` 代替。
详见 [功能边界](docs/boundary.md#cargo-feature-boundary)。

## 接入自己的程序

源码依赖应固定为审阅过、已能从远端取得的完整提交 SHA。下例中的 `rev` 是必须替换的占位符：

```toml
[dependencies]
oclive_kernel_base = { git = "https://github.com/linkaiheng2233-cyber/oclive-kernel", rev = "<reviewed-40-character-commit-sha>", default-features = false }
```

需要角色剖面时，删除 `default-features = false`，或添加 `features = ["character-profile"]`。
旧 OCLive 的兼容重导出由原工程维护，不要把本机绝对路径当作交付依赖。

在现有异步任务内可以直接 `.await`；需要 Tokio 局部调度时，运行这个仅属于示例 Host 的程序：

```sh
cargo run --locked -p oclive_non_role_host --example local-tokio
```

它验证 `Rc<Cell<_>>` 局部状态与栈上材料跨一次让出执行仍可使用，Tokio 仅为该 Host 的开发依赖。
线程、借用与 `spawn_local` 的条件见 [局部 future 接入说明](docs/local-futures.md)。

## 已知限制

| 范围 | 使用者需要知道的边界 |
|---|---|
| 线程与借用 | Base 不要求 `Send`/`Sync`/`'static`；擦除后的 future 没有 `Send` 保证，不能直接交给要求它的线程任务 |
| 分配与调用 | boxed future/dyn 存在一般情况下的分配与间接调用成本，没有零分配保证 |
| 取消与重试 | 丢弃 future 不证明物理取消或回滚；内核不提供自动重试协议 |
| 权限与流程 | Host 负责资源与调用组织；返回文本不等于授权、状态提交或产品成功 |
| 错误显示 | 剖面解析诊断不回显输入值；`BaseCallError.detail` 由调用者提供，显示前的脱敏由 Host 负责 |
| 角色与媒体 | 剖面验证内容，不验证图片存在、解码或渲染，不是完整角色包加载器 |
| 兼容与支持 | 不承诺稳定二进制 ABI；工具链、源码兼容与发布政策见 CONTRIBUTING |

完整责任边界只在 [boundary](docs/boundary.md) 维护。

## 仓库入口与验证

| 入口 | 用途 |
|---|---|
| [公共库](crates/oclive_kernel_base/src/lib.rs) / [角色剖面](crates/oclive_kernel_base/src/character_profile.rs) | 契约、数据与内容校验 |
| [角色 Host](examples/minimal-host/src/main.rs) / [非角色 Host](examples/non-role-host/src/main.rs) | 作者自己的绑定、执行器与调用顺序 |
| [示例私有实现](examples/common/demo_modules.rs) | 可替换的字面 Memory、Prompt 和 LLM 替身 |
| [Host 测试](examples/minimal-host/src/tests.rs) / [Base fixture](crates/oclive_kernel_base/tests/base_only_fixture.rs) / [剖面兼容测试](crates/oclive_kernel_base/tests/character_profile_compatibility.rs) | 实际调用、失败传播、借用与新旧类型同一性 |
| [贡献与完整检查](CONTRIBUTING.md) | 工作方式、兼容规则、默认/关闭/重新开启的检查命令 |

```sh
cargo test --locked --workspace
cargo test --locked --workspace --doc
cargo clippy --locked --workspace --all-targets -- -D warnings
cargo fmt --all -- --check
```

CI 徽章反映 GitHub 已推送源码的运行结果；本地验证不能替代目标提交的远端 CI。
源码和示例测试不证明真实模型、平台媒体、权限隔离、硬件或完整产品已验收。
库源码、测试、示例及依赖应分别计量，示例 binary 的大小不等于内核固有大小。

本仓提交 `Cargo.lock`，用于重现源码示例与 CI；下游程序仍维护自己的锁文件。
原理与来源以 [提取记录](docs/extraction.md) 和其中固定的原工程责任说明为准。
代码文档为英文，中文为主页主语言。Apache-2.0，见 [LICENSE](LICENSE) 与 [NOTICE](NOTICE)。
