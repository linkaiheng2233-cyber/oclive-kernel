# OCLive Kernel Base

一个独立的 Rust 公共库：六槽 Base、请求与失败表达，以及默认开启的角色一致性剖面（character conformance profile）。
剖面提供人设文本与视觉引用的最低内容契约：**推荐 7 张，最低 1 张**，兼容已有单图定义。
宿主负责解析资源并实现渲染；内核提供数据与校验，不自动加载图片或执行角色。
不需要角色内容的宿主可以关闭 `character-profile`，只使用六槽 Base。

两个很小的终端 Host 分别演示角色内容接入与非角色文本处理，展示作者如何绑定自己的实现。

这是从 OCLive 已落地公共 API 提取的独立源码工程，package 发布关闭；不是新增的回合执行引擎。
原参考工程的兼容迁移单独验收，本仓测试不代表它已合入或通过发行版验收。
来源和兼容迁移状态见 [extraction](docs/extraction.md)，责任和未覆盖范围见 [boundary](docs/boundary.md)。

## 运行

取得源码后，在本仓目录执行；首次运行需要 Cargo 能取得锁文件中的依赖，缓存完备时可以添加 `--offline`：

```sh
git clone https://github.com/linkaiheng2233-cyber/oclive-kernel.git
cd oclive-kernel
cargo test --locked --workspace
cargo run --locked -p oclive_minimal_host
cargo run --locked -p oclive_non_role_host
```

作为另一个工程的依赖时，使用 `oclive_kernel_base`，并将 Git `rev` 固定为审阅过的完整提交 SHA。
旧 OCLive 的兼容入口仍由原工程维护；不要将本机绝对路径当作交付依赖。

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
非角色 Host 只绑定 Memory、Prompt、LLM，选中一条诊断材料；不创建或引用剖面类型。

## 剖面与兼容

默认功能是 `default = ["character-profile"]`。规范名称为：

```rust
use oclive_kernel_base::{
    CharacterProfileDefinition, parse_character_profile_definition,
    validate_character_profile_definition,
};
```

`character_profile` 是规范模块；旧 `minimal_role` 模块、`MinimalRoleDefinition`、
`parse_minimal_role_definition`、`validate_minimal_role_definition` 保留为同一实现的别名。
JSON 字段、人设与引用的原样保留规则、错误文本及最低一张的校验保持兼容。
直接反序列化或构造结构体仍需调用校验；七张是推荐数量，不是七个强制情绪槽。

在使用包含本次变更的源码修订时，可以单独检查关闭剖面的版本：

```sh
cargo test --locked -p oclive_kernel_base --no-default-features
cargo test --locked -p oclive_kernel_base --no-default-features --doc
cargo run --locked -p oclive_non_role_host
```

下游在自己已固定 Git `rev` 的依赖声明中设置 `default-features = false`；需要时可用
`features = ["character-profile"]` 显式开启。关闭后，新旧剖面导入均不可用，库的正常依赖不再包含
`serde`/`serde_json`。六槽的签名、请求与失败表达不变，仍可独立选择。

Cargo 会合并依赖图中对同一包的功能请求。其他选中的依赖若开启剖面，单条依赖上的
`default-features = false` 无法将其否决；整个 workspace 一起测试时，角色示例也会开启剖面。
因此 CI 另行选择纯库和非角色示例测试。详见 [功能与宿主边界](docs/boundary.md#cargo-feature-boundary)。

## 改哪里

| 位置 | 用途 |
|---|---|
| [公共库](crates/oclive_kernel_base/src/lib.rs) | 六槽接口、请求/失败，以及可关闭的角色剖面；没有默认槽实现 |
| [角色剖面](crates/oclive_kernel_base/src/character_profile.rs) | 人设与至少一条视觉引用的校验、JSON 投影和旧名别名 |
| [Host 接线](examples/minimal-host/src/main.rs) | 本示例选择 Memory → Prompt → LLM；这是此 Host 的流程 |
| [非角色 Host](examples/non-role-host/src/main.rs) | 关闭默认功能，只处理显式诊断材料，不引用角色类型 |
| [示例私有实现](examples/common/demo_modules.rs) | 两个 Host 复用的字面 Memory、Prompt 和确定性 LLM 替身；在内核之外 |
| [Host 测试](examples/minimal-host/src/tests.rs) | 实际调用与材料、替换效果、非法角色及失败后不启动依赖调用 |
| [Base fixture](crates/oclive_kernel_base/tests/base_only_fixture.rs) | 六槽分别可独立调用，检查本地借用 future、dyn 与失败表达 |
| [剖面兼容测试](crates/oclive_kernel_base/tests/character_profile_compatibility.rs) | 新旧导入的类型同一性、单图兼容及错误保持 |

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
cargo test --locked -p oclive_kernel_base --no-default-features
cargo test --locked -p oclive_kernel_base --no-default-features --doc
cargo test --locked -p oclive_kernel_base --no-default-features --features character-profile
cargo test --locked -p oclive_non_role_host
cargo clippy --locked --workspace --all-targets -- -D warnings
cargo clippy --locked -p oclive_kernel_base --no-default-features --all-targets -- -D warnings
cargo clippy --locked -p oclive_non_role_host --all-targets -- -D warnings
cargo fmt --all -- --check
cargo tree --locked -p oclive_kernel_base --edges normal
cargo tree --locked -p oclive_kernel_base --no-default-features --edges normal,build
cargo tree --locked -p oclive_non_role_host --edges normal,build,features
```

通过分别说明本库及示例的接口/行为成立；不替代目标 Host 产品场景、真实模型、权限隔离、真实 TTS 或平台实机验收。
核心源码、文档、测试、Host 与依赖分别计量；没有用整个参考工程的行数或示例 binary 大小冒充内核大小。

English: [scope and responsibilities](docs/boundary.md), [source provenance](docs/extraction.md).
All source code documentation is in English. Apache-2.0; see [LICENSE](LICENSE) and [NOTICE](NOTICE).
