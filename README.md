# OCLive Kernel Base

一个独立的 Rust 公共库：六槽 Base、请求与失败表达，以及最小逻辑角色定义和校验。
配套一个很小的终端 Host，展示作者如何绑定自己的实现并调用这些接口。

这是从 OCLive 已落地公共 API 提取的本地交付候选。原参考工程尚未切换到本仓依赖，package 发布关闭；不是新增的回合执行引擎。
来源和兼容迁移状态见 [extraction](docs/extraction.md)，责任和未覆盖范围见 [boundary](docs/boundary.md)。

## 运行

在本仓目录执行；首次运行需要 Cargo 能取得锁文件中的依赖，缓存完备时可以添加 `--offline`：

```sh
cargo test --locked --workspace
cargo run --locked -p oclive_minimal_host
```

应看到 Base 与实现分开列出，Memory/Prompt/LLM 各调用一次，Memory 选中两条材料：

```text
MemoryBase -> LiteralMemory (Host implementation)
PromptBase -> LiteralPrompt (Host implementation)
LlmBase -> DemoLlm (deterministic substitute, no model inference)
EmotionBase / EventBase / AgentBase: not bound, not called
selected=2 calls: memory=1 prompt=1 llm=1
```

随后输出含所选材料的确定性文本，**没有模型推理、网络请求或真实外部操作**。
`host-owned:portrait` 是合成的视觉引用，只验证逻辑内容，不读取或展示图片。

## 改哪里

| 位置 | 用途 |
|---|---|
| [公共库](crates/oclive_kernel_base/src/lib.rs) | 六槽接口、请求/失败、最小逻辑角色；没有默认槽实现 |
| [Host 接线](examples/minimal-host/src/main.rs) | 本示例选择 Memory → Prompt → LLM；这是此 Host 的流程 |
| [Host 模块](examples/minimal-host/src/modules.rs) | 字面 Memory、Prompt 和确定性 LLM 替身，可由作者替换 |
| [Host 测试](examples/minimal-host/src/tests.rs) | 实际调用与材料、替换效果、非法角色及失败后不启动依赖调用 |
| [Base fixture](crates/oclive_kernel_base/tests/base_only_fixture.rs) | 六槽分别可独立调用，检查本地借用 future、dyn 与失败表达 |

开发练习：在 `main.rs` 中把 Memory 的 `limit: 2` 改成 `limit: 1`，再运行 Host。
输出应变成 `selected=1`，第二条 river 材料不再进入最终文本。
测试中的 `replacing_the_memory_binding_changes_the_actual_result` 已实际比较两个绑定。
作者改变自己的功能承诺时，应同时调整相应实现与断言；字面匹配和条数限制不是所有 Memory 的义务。

六槽可独立使用，不要求每轮全跑。最简 Host 只依赖本库和自己的 `futures-lite` 执行支持，
不经过旧 runtime、参考 Host、SQL、HTTP、Tauri 或模型设施。可选共享消费者仍属于原工程的内核外接入支持。

## 冻结检查

```sh
cargo test --locked --workspace
cargo test --locked --workspace --doc
cargo clippy --locked --workspace --all-targets -- -D warnings
cargo fmt --all -- --check
cargo tree --locked -p oclive_kernel_base --edges normal
```

通过分别说明本库及示例的接口/行为成立；不替代目标 Host 产品场景、真实模型、权限隔离、真实 TTS 或平台实机验收。
核心源码、文档、测试、Host 与依赖分别计量；没有用整个参考工程的行数或示例 binary 大小冒充内核大小。

English: [scope and responsibilities](docs/boundary.md), [source provenance](docs/extraction.md).
All source code documentation is in English. Apache-2.0; see [LICENSE](LICENSE) and [NOTICE](NOTICE).
