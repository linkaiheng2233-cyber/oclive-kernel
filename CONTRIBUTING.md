# Contributing

Start with the [landing page](README.en.md), [scope](docs/boundary.md) and
[source provenance](docs/extraction.md). The maintainer reviews changes in GitHub issues and pull
requests. Small fixes can go straight to a PR; for a contract or responsibility change, discuss a short
proposal first. A separate RFC repository or a formal RFC number is not required.

## Propose changes at the right boundary

Public contracts include trait bounds/signatures, request/result/failure semantics, profile validation,
serde fields, feature/default behavior and compatibility re-exports. Changes to these need a clear
before/after example, affected consumers, migration plan and maintainer review before implementation.

Adding required fields or preventing existing public struct-literal construction is a breaking source
change. Adding `#[non_exhaustive]` to an already public struct is not a compatibility-neutral cleanup.
The existing enum's non-exhaustive behavior and caller-owned error details should retain their documented meaning.
The profile's canonical rename preserves aliases and JSON, but changes derived Debug/type-name text.

Do not add mandatory slots, `Send`/`Sync`/`'static`, schedulers, authorization engines, registries,
automatic retry or product defaults as incidental fixes. Such proposals exceed the current extracted
surface and need explicit discussion. Hosts remain free to choose their own implementations and flow.
Do not copy the reference runtime or invent a second parser to work around the boundary.

Source consumers should pin a reviewed Git revision. Incompatible changes need an explicit versioned
migration and changelog entry; being pre-1.0 does not make an undocumented break harmless. A source
checkpoint is not a stable ABI promise. No crates.io publication or new release is automatic.

## Toolchain and dependencies

[rust-toolchain.toml](rust-toolchain.toml) pins **Rust 1.97.1**, including rustfmt and Clippy, for
contributors. CI verifies that same compiler. No earlier MSRV has been established. To declare one,
test the actual minimum under default/off features and the supported examples; setting a manifest
field alone is not proof. Workspace package metadata must also be inherited explicitly by each member.

Keep existing locked versions unless a change needs an update. `Cargo.lock` is intentionally tracked
because this repository distributes runnable examples and CI alongside the source library. Downstream
applications resolve and maintain their own locks. A lockfile change needs an explanation and audit.
Tokio is a dev dependency only of the non-character example Host; it is not a core runtime choice.

## Checks

While editing, run the checks affected by your change. Before freezing a source snapshot:

```sh
cargo test --locked --workspace
cargo test --locked --workspace --doc
cargo test --locked -p oclive_kernel_base --no-default-features
cargo test --locked -p oclive_kernel_base --no-default-features --doc
cargo test --locked -p oclive_kernel_base --no-default-features --features character-profile
cargo test --locked -p oclive_non_role_host
cargo test --locked -p oclive_non_role_host --example local-tokio
cargo clippy --locked --workspace --all-targets -- -D warnings
cargo clippy --locked -p oclive_kernel_base --no-default-features --all-targets -- -D warnings
cargo clippy --locked -p oclive_non_role_host --all-targets -- -D warnings
cargo fmt --all -- --check
cargo run --locked -p oclive_minimal_host
cargo run --locked -p oclive_non_role_host
cargo run --locked -p oclive_non_role_host --example local-tokio
cargo tree --locked -p oclive_kernel_base --edges normal
cargo tree --locked -p oclive_kernel_base --no-default-features --edges normal,build
cargo tree --locked -p oclive_non_role_host --edges normal,build,features
```

Default workspace testing unifies the character Host's feature request. Separately selected off-mode
checks are essential; a workspace-wide `--no-default-features` cannot prove profile absence.

Strict generated documentation, with the same shell-specific environment setting used in CI:

```sh
# POSIX shell
RUSTDOCFLAGS="-D warnings" cargo doc --locked -p oclive_kernel_base --no-deps
RUSTDOCFLAGS="-D warnings" cargo doc --locked -p oclive_kernel_base --no-default-features --no-deps
```

In PowerShell set `$env:RUSTDOCFLAGS = '-D warnings'` before those commands and restore its previous
value afterwards. Cargo supports `--offline` when dependencies are cached. For dependency changes,
run `cargo audit --deny warnings` with a fresh database; `cargo audit --no-fetch` only checks the available cached snapshot.

Examples prove their synthetic behavior, not real inference, authorization, platform media or product
recovery. Public import changes also require original OCLive consumer compatibility checks in an
isolated harness or a separately approved downstream migration. Do not silently change its dependency pin.

## PR and release evidence

Explain the problem, resulting behavior, meaningful checks, compatibility impact and untested boundaries.
Update the owning documentation rather than copying a second normative specification. Keep Chinese and
English entry points consistent in their claims, slot summaries, integration instructions and limitations;
they need not be sentence-for-sentence translations. Keep normative detail in the owning specification.
Code documentation is English. Never include keys or private user material.
Security reports follow [SECURITY](SECURITY.md), not a public issue.

A local pass is **Locally verified**. Remote evidence must name the exact pushed commit and run;
an ancestor's green badge does not validate an unpublished child. Keep unrelated changes out of the PR.
Release settings, tags and repository metadata are separate reviewed actions. Freeze the exact SHA, complete the applicable checks above, and verify the intended tag/prerelease against that SHA before announcing it. Preserve historical checkpoint tags; do not publish packages or alter downstream pins as a documentation cleanup side effect. One-time release drafts are kept outside source.
