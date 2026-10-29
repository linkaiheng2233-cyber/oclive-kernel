# Working on this extracted public library

Read [README](README.md), [boundary](docs/boundary.md), and [extraction](docs/extraction.md) first.
Follow [CONTRIBUTING](CONTRIBUTING.md) for toolchain/compatibility checks and [SECURITY](SECURITY.md) for private reports.
The six Base signatures and their request/failure semantics come from the recorded OCLive source revision.
Do not add mandatory slots, `Send`/`Sync`/`'static`, a scheduler, authorization engine, registry, automatic retry, or product defaults without maintainer review.
The sample Host owns its implementations and call sequence. It is not a production Host or model-quality test.

Use narrow tests while editing; before freezing run `cargo test --locked --workspace`,
`cargo test --locked --workspace --doc`, `cargo clippy --locked --workspace --all-targets -- -D warnings`,
and `cargo fmt --all -- --check`. Public import changes also require checking existing downstream compatibility in OCLive.
The Host-only local-Tokio example has its own test/run commands in CONTRIBUTING; do not move that runtime into Base.
Keep the extraction record accurate. Never claim local checks are remote CI, or a sample call is product acceptance.
Do not publish a package, create a remote repository, or change the original repository without the task's authorization.
