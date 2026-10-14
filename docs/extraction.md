# Extraction record

Status: **Locally verified** extraction candidate; remote repository/package publication and original-workspace migration are not completed.
Controller implementation and semantic self-review: `independent=false`.

The maintainer selected: extract the existing public library plus a minimal Host, retain the boundary
specification, and introduce no unified invocation/permission/scheduling engine.

Source: OCLive `80ab85824849a466674169a3be34d94a92117886`.
The inspection worktree was `4847ab0c3b88a7c8b58dc3c8b1267703893c3b76`; its `kernel/` tree is unchanged
from that SDK source revision. The later video-preparation documents were not copied.

| Original path under `kernel/crates/` | Destination | Original SHA256 |
|---|---|---|
| `oclive_kernel_contracts/src/slot_base.rs` | `crates/oclive_kernel_base/src/slot_base.rs` | `F69D1DBAB4F644809A0A58D69DF5A1A20DF79410271E4BA96EC88E3A6E57FA18` |
| `oclive_kernel_types/src/slot_base.rs` | `crates/oclive_kernel_base/src/requests.rs` | `AE86477CC9090D2BD77D05D820F775C341D22C9A114453F2E46CD7C2935158B8` |
| `oclive_validation/src/minimal_role.rs` | `crates/oclive_kernel_base/src/minimal_role.rs` | `0EF6BDD7735AE41052628FF475E640C82042A6FB7AA2E5C6EEDA2AE9D256F426` |
| `oclive_kernel_contracts/tests/base_only_fixture.rs` | `crates/oclive_kernel_base/tests/base_only_fixture.rs` | `77064ED653AB58B5F74F78D4D995E035AE9FA512898D67631C8970E8255681CE` |

Necessary extraction differences: crate imports become `oclive_kernel_base` / `crate`; rustdoc imports
follow the new path; provenance text points to the original repository; a rustdoc link to the reference
Host's `AppError` becomes plain text because that type is intentionally not included. Line endings are LF.
The preserved minimal-role unit tests allow `clippy::unwrap_used` inside their `cfg(test)` module only;
production code remains subject to the workspace lint. The first Clippy attempt exposed this extraction
configuration difference; its failed log is retained, and no assertion was weakened.
The existing trait methods, borrowed requests, error reasons, logical role checks and fixture assertions are preserved.
LICENSE and NOTICE are carried from the source repository. No models, user data, artwork or rich role packs are copied.

The new library root and Cargo manifests package these existing definitions. The sample Host code is new,
private to the example, and owns its deterministic implementations. `serde`/`serde_json` are library dependencies
for the existing logical role and JSON projection; `futures-lite` is a Host dependency only.

## Planned downstream migration

The source package is named `oclive_kernel_base`, with an initial extraction version of `0.1.0`.
`publish = false` remains until distribution and compatibility have been frozen.

The intended single-source transition is:

1. Freeze this independent library and choose its immutable distribution identity.
2. Make original contracts/types/validation consume it and re-export the original public symbols.
3. Check that new and legacy paths identify the **same** Rust traits and data, not duplicate similar definitions.
4. Run the affected shared consumer, source SDK and reference Host regressions, plus explicit workspace doctests.
5. Freeze the original-workspace migration separately; retain historical SDK tags and source commits.

The new package must not depend back on those old crates. Copying whole old crates, or permanently editing
two independent Base/type definitions, would defeat this extraction. This local candidate does not yet
claim that the above downstream steps have happened.

## Measurement

Measure the library source/documentation, tests, example Host and dependency graph separately.
Report physical lines with a stated method; rustdoc and inline tests are not executable-line counts.
The Host's executable includes its executor, dependencies, standard library and linker/profile choices;
its size is not the library's intrinsic size. Cross-platform and production performance need actual targets.

Reproducible checks are in the [README](../README.md). Local command logs are kept by the current controller
outside this repository; they are not remote CI evidence and do not automatically close `K-CORE-BOUNDARY-01`.

Measured on 2026-10-14, before the initial local freeze. Physical lines use UTF-8 `splitlines()`;
source includes comments, rustdoc and blank lines. Inline unit tests are explicitly included in the library row.

| Group | Physical lines | UTF-8 bytes |
|---|---:|---:|
| Library source: lib + traits + requests/failure + minimal role, including inline tests/rustdoc | 771 | 33274 |
| Base integration fixture | 817 | 28978 |
| Host assembly and private implementations, excluding separate tests | 175 | 5137 |
| Host tests | 175 | 5162 |

These figures do not include Cargo dependencies, compiler output or the repository's license/documentation files.
They describe the extracted **implemented surface**, not a new engine implementing every candidate invocation constraint.

## Local validation at freeze

Environment: Windows, Rust/Cargo 1.97.1, locked dependencies, offline builds, separate target directory.
No original-workspace runtime/Host crate is a path dependency. `serde_json` is locked to source version 1.0.151;
this extraction did not require a JSON-library upgrade.

- Workspace tests: 11 library unit tests + 16 Base integration tests + 5 Host tests passed; 5 doctests also passed.
- Explicit workspace doctest: 5 passed. Strict `cargo doc --no-deps` with `RUSTDOCFLAGS=-D warnings`: passed.
- Clippy `--all-targets -D warnings`, fmt check and actual console Host run: exit 0.
- Source comparison: four files' noncomment text matches after import-namespace/whitespace normalization
  and accounting for the test-only lint allowance. This is a limited textual check, not semantic proof;
  actual behavior is separately exercised by the tests.
- Markdown links, UTF-8 encoding and workflow YAML parsing: passed. The workflow has one verification
  job expanded across Windows/Linux; remote execution is **NOT RUN**.
- `cargo audit --no-fetch`: exit 0 using the existing local advisory database (1308 loaded advisories).
  This is not a fresh database fetch or proof against unknown vulnerabilities.

First Clippy attempt: exit 101 because the preserved minimal-role test module used seven unwrap/unwrap_err
expressions under the new warning lint. The allowance is confined to that test module, matching the
test/production distinction; production checks and all assertions remain intact. Its raw failed log is retained.

Evidence: `E:/OCLive/_worktrees/wiring-view-20261013-r0/.cursor/plans/kernel-extraction-20261014-r0/`.
This local path is for the current controller; it is not a portable dependency or required build input.
No original repository source, remote setting, SDK tag, user data or protected evidence was changed.
