# OCLive Kernel Base

[中文主页](README.md) · [Scope and responsibilities](docs/boundary.md) · [Contributing](CONTRIBUTING.md) · [Security](SECURITY.md) · [Changelog](CHANGELOG.md)

Six independently usable Rust Base contracts, with composition owned by your Host.
Extracted from OCLive's implemented public surface, this source library provides borrowed
requests, explicit call failures and a default-on optional character conformance profile.
It owns no Host state, scheduler, storage, transport or model adapter. Package publication is disabled.

Current source preview: **0.1.0-preview.1**. Included changes are listed in the [changelog](CHANGELOG.md).

## Try the two Hosts

The contributor toolchain is pinned to **Rust 1.97.1**. This is a verified toolchain, not a claim
that earlier Rust versions have been tested or that a lower MSRV has been established.

```sh
git clone https://github.com/linkaiheng2233-cyber/oclive-kernel.git
cd oclive-kernel
cargo run --locked -p oclive_minimal_host
cargo run --locked -p oclive_non_role_host
```

Both Hosts bind Memory, Prompt and LLM in a Host-chosen sequence; Emotion, Event and Agent remain
unbound. The character Host selects two materials, and the non-character Host selects one.
Their LLM is a deterministic substitute: no inference, network or external effect occurs.
The synthetic portrait reference is neither opened nor rendered.

## The six slots

| Base | Responsibility for this call | Normal result type |
|---|---|---|
| Memory | Retrieve relevant content from explicit candidate material | `Vec<String>` |
| Emotion | Analyze the emotion expressed in the material | `Option<String>` |
| Event | Analyze a described event's meaning or possible impact | `Option<String>` |
| Prompt | Organize material according to this call's requirements | `String` |
| LLM | Generate text from input already prepared by the caller | `String` |
| Agent | Process a delegated task and report its outcome in text | `String` |

Each method returns a local boxed future containing a `Result`, usable through `dyn`. A normal
result may be empty; failures use `BaseCallError`. A capability return does not by itself end an
invocation or authorize an external operation. Exact semantics remain in the
[traits](crates/oclive_kernel_base/src/slot_base.rs) and [request/failure definitions](crates/oclive_kernel_base/src/requests.rs).

## Character profile

`character-profile` is on by default. It carries nonblank persona text and at least one nonblank
visual reference. **Seven images are recommended; one remains sufficient.** The Host resolves
and renders media. Construction/deserialization alone is not content validation.

Canonical names live in `character_profile`; the old `minimal_role` module and symbols remain
aliases of the same definitions. JSON/content rules and diagnostic wording are preserved;
the canonical type name changes derived Debug/type-name output.

## Integrating your own program

Pin a reviewed, remotely available full Git commit SHA when depending on `oclive_kernel_base`.
The following `rev` is a placeholder that must be replaced before use:

```toml
[dependencies]
oclive_kernel_base = { git = "https://github.com/linkaiheng2233-cyber/oclive-kernel", rev = "<reviewed-40-character-commit-sha>", default-features = false }
```

This selects Base-only use. To include the profile, remove `default-features = false`, or add
`features = ["character-profile"]` to opt in explicitly. Original OCLive's legacy re-exports are
maintained in that workspace; local absolute paths are not a distribution dependency.
Feature requests are additive: another selected dependency can enable the profile.
Run off-mode checks separately from the whole workspace, whose character Host enables defaults.

```sh
cargo test --locked -p oclive_kernel_base --no-default-features
cargo test --locked -p oclive_kernel_base --no-default-features --doc
cargo run --locked -p oclive_non_role_host --example local-tokio
```

The optional example demonstrates borrowed `!Send` state with a Host-owned Tokio LocalSet;
Tokio is only a development dependency of the example Host. Start with direct `.await` when
you already have an async task. Read [local future integration](docs/local-futures.md) before spawning tasks.

## Limits and verification

| Area | Boundary to account for |
|---|---|
| Threads and borrowing | Base does not require `Send`/`Sync`/`'static`; erased futures have no `Send` guarantee and cannot be passed directly to tasks that require it |
| Allocation and dispatch | Boxed futures/dynamic dispatch have allocation and indirection costs in the general case; there is no zero-allocation guarantee |
| Cancellation and retries | Dropping a future does not prove physical cancellation or rollback; the library supplies no automatic retry protocol |
| Authority and flow | The Host owns resource access and call organization; returned text does not grant authority, commit state or establish product success |
| Error display | Profile parse diagnostics omit supplied values; `BaseCallError.detail` is caller-supplied text that Hosts must redact before exposure where necessary |
| Characters and media | The profile validates content, not asset existence, decoding or rendering; it is not a complete character-pack loader |
| Compatibility and support | There is no stable binary ABI promise; toolchain, source compatibility and release policies are in CONTRIBUTING |

The complete responsibility specification stays in [boundary](docs/boundary.md).
The [Chinese landing page](README.md) also provides a repository map.
[CONTRIBUTING](CONTRIBUTING.md) lists full default/off/explicit-on verification commands.
The repository lockfile reproduces source examples and CI; downstream applications own their locks.
CI badges describe pushed commits, not unpublished local work or production-model acceptance.

Provenance: [extraction record](docs/extraction.md). Apache-2.0: [LICENSE](LICENSE), [NOTICE](NOTICE).
