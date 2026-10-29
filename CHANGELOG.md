# Changelog

This file describes source changes. Package publication is disabled. **Unreleased** entries are not
a tag, GitHub Release or claim of remote CI success; verify the actual distribution SHA and evidence.

## Unreleased

### Added

- Default-on `character-profile` in the same library, with canonical `character_profile` module,
  `CharacterProfileDefinition` and matching parser/validator names.
- A non-character Memory/Prompt/LLM Host with default features disabled, and new/legacy profile
  type-identity and profile-disabled import checks.
- A runnable Host-owned Tokio LocalSet example, with tests for local state and borrowed material
  surviving a yield. Tokio is an example Host dev dependency only.
- Chinese/English entry points, maintenance/security/contribution policies, pinned contributor toolchain,
  strict rustdoc CI and a separate scheduled/manual dependency-audit workflow.

### Changed

- Profile-only serde dependencies become optional; disabling the profile leaves Base calls available.
- The minimum image policy is explicitly **seven recommended, one required**. Existing single-image
  definitions, JSON fields, validation rules and diagnostic wording are retained.
- Old `minimal_role` module and symbols alias the canonical definitions; derived Debug/type-name output
  uses the new concrete Rust name. This is source/JSON compatibility, not a stable binary ABI guarantee.
- README now separates capabilities, Host responsibilities, integration and known limits. Error
  redaction claims apply to profile parse diagnostics, not to caller-supplied `BaseCallError.detail`.

The six Base traits, borrowed requests, failure semantics and public struct construction are unchanged.
Original OCLive consumer compatibility for the profile change was checked in an isolated local patched
consumer; that did not update the original workspace's immutable dependency pin.

### Security

- The example-only runtime is locked to Tokio 1.53.3, a patched version for
  [RUSTSEC-2026-0386](https://rustsec.org/advisories/RUSTSEC-2026-0386).
  No advisory exemption is configured; the separate audit workflow denies warnings as well as vulnerabilities.

## Source checkpoints — 2026-10-14

- [Extraction checkpoint](https://github.com/linkaiheng2233-cyber/oclive-kernel/tree/checkpoint/kernel-base-extraction-20261014-r0):
  `605914cfb48c977db1e24675e039b8b198857b55`.
- [Source checkpoint](https://github.com/linkaiheng2233-cyber/oclive-kernel/tree/checkpoint/kernel-base-source-20261014-r0):
  `60c6cc0bb911bf8e15efd4352f00561da6a6ee4f`.

These are historical checkpoints, not semver releases or promises of indefinite maintenance.
See [extraction](docs/extraction.md) for original source mappings and historical validation.
