# OCLive Kernel Base

[中文主页](README.md) · [Scope and responsibilities](docs/boundary.md) · [Contributing](CONTRIBUTING.md) · [Security](SECURITY.md) · [Changelog](CHANGELOG.md)

Six independently usable Rust Base contracts, with composition owned by your Host.
Extracted from OCLive's implemented public surface, this source library provides borrowed
requests, explicit call failures and a default-on optional character conformance profile.
It owns no Host state, scheduler, storage, transport or model adapter. Package publication is disabled.

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

The six contracts cover retrieval, expressed emotion analysis, described-event analysis, prompt
assembly, text generation and delegated work reporting. Each returns a local boxed future.
See the [traits](crates/oclive_kernel_base/src/slot_base.rs) for exact call semantics;
this brief introduction does not replace that specification.

## Character profile and integration

`character-profile` is on by default. It carries nonblank persona text and at least one nonblank
visual reference. **Seven images are recommended; one remains sufficient.** The Host resolves
and renders media. Construction/deserialization alone is not content validation.

Canonical names live in `character_profile`; the old `minimal_role` module and symbols remain
aliases of the same definitions. JSON/content rules and diagnostic wording are preserved;
the canonical type name changes derived Debug/type-name output.

Pin a reviewed, remotely available full Git commit SHA when depending on `oclive_kernel_base`.
Set `default-features = false` for Base-only use, or explicitly opt in to `character-profile`.
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

No `Send`/`Sync`/`'static` bound is required by Base. Erased futures do not preserve a concrete
future's possible `Send` property. Dropping a future proves neither physical cancellation nor
rollback. There is no automatic retry, authorization engine, media loader or stable binary ABI.
`BaseCallError.detail` is caller-supplied display text; Hosts must redact it where necessary.

The [Chinese landing page](README.md) has the six-slot and limitation tables and repository map.
[CONTRIBUTING](CONTRIBUTING.md) lists full default/off/explicit-on verification commands.
The repository lockfile reproduces source examples and CI; downstream applications own their locks.
CI badges describe pushed commits, not unpublished local work or production-model acceptance.

Provenance: [extraction record](docs/extraction.md). Apache-2.0: [LICENSE](LICENSE), [NOTICE](NOTICE).
