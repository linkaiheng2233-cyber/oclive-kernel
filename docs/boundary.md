# Scope of this extracted library

This repository packages the **already implemented** Base call surface and minimal logical role validation.
It does not turn the entire reference-runtime contracts/types/validation crates into the small Kernel.

- `oclive_kernel_base` provides six independently callable Base traits, borrowed requests, call failures,
  and minimum logical role content. The trait and data documentation is the call-level source of truth.
- The sample Host chooses implementations, owns its material and executor, and arranges its own calls.
  Its Memory → Prompt → LLM dependency is an example, not a universal six-slot workflow.
- The original shared minimal-role consumers remain outside the small Kernel. Other Hosts may reuse
  such support or bind Base implementations directly.
- The reference Host retains rich role lifecycle, resource policy, retries/fallbacks it already owns,
  database, transport, model adapters, plugins and product state. None is copied into this public library.

Existing responsibility specification:
[OCLive ownership and Base semantics](https://github.com/linkaiheng2233-cyber/oclivenewnew/blob/80ab85824849a466674169a3be34d94a92117886/handoff/MODULE_MAP_AND_HANDOFF.md).
Read confirmed responsibility, candidate public semantics, and actual implementations separately.
This extraction introduces no new authorization power, public scheduling rule or mandatory extension.

## What the call binding does not prove

The local boxed future does not require `Send`, `Sync` or `'static`. Its executor belongs to the caller.
Dropping a future does not prove physical cancellation, rollback or safe retry. A capability return value,
empty or otherwise, is not an invocation terminal state or proof of product success.

Applicable purpose, dependency, eligibility and invocation cutoff constraints remain responsibilities
under the existing specification. They are not a newly implemented centralized enforcement engine here.
The example visibly preserves the dependency it chooses: generation uses the successful Prompt result;
an upstream failure starts no dependent call. This limited example does not certify all lifecycle boundaries.

Minimum role validation checks nonblank persona text and one or more nonblank visual references.
It does not grant file access, resolve assets, activate a product role or define a richer pack format.
Its JSON projection ignores unknown product fields; it is not a lossless rich-pack editor.

## Acceptance and maintenance

Library tests prove the behavior they actually exercise; the Host tests prove this synthetic local flow.
There is no real model, network, database, tool effect, media playback, crash-recovery or hardware acceptance.
No production TTS, all-platform resource guarantee or stable binary ABI is claimed.

This repository is the defining source for the extracted symbols. Original-workspace compatibility is
verified separately: the transition uses this crate plus re-exports at old import paths, rather than
permanently maintaining two independent definitions. This repository's CI does not certify that a
downstream migration has been merged or that a product has passed acceptance; see [extraction](extraction.md).
