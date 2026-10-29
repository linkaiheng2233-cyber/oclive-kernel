# Scope of this extracted library

This repository packages the **already implemented** Base call surface and character-content validation.
The latter is named the **character conformance profile** and is enabled by default through
`character-profile`. It can be disabled without changing any Base trait or request.
It does not turn the entire reference-runtime contracts/types/validation crates into the small Kernel.

- `oclive_kernel_base` provides six independently callable Base traits, borrowed requests and call failures.
  The optional profile adds persona/visual-reference content and validation. The trait and data
  documentation is the call-level source of truth; no Base request requires a profile.
- The sample Host chooses implementations, owns its material and executor, and arranges its own calls.
  Its Memory → Prompt → LLM dependency is an example, not a universal six-slot workflow.
- The original shared character consumers remain outside the small Kernel. Other Hosts may reuse
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

`BaseCallError.detail` is caller-supplied human-readable text; its `Display` includes a nonempty
detail verbatim. A Host must redact that text where necessary before exposing errors or logs.
The profile parser's input-omitting diagnostics are not a library-wide redaction engine.

Applicable purpose, dependency, eligibility and invocation cutoff constraints remain responsibilities
under the existing specification. They are not a newly implemented centralized enforcement engine here.
The example visibly preserves the dependency it chooses: generation uses the successful Prompt result;
an upstream failure starts no dependent call. This limited example does not certify all lifecycle boundaries.

## Character conformance profile

`character_profile::CharacterProfileDefinition` defines a content floor, not a mandatory workflow.
The parser/validator require nonblank `persona_prompt` text and **at least one** nonblank entry in
`visual_assets`. **Seven images are recommended**, not required: existing single-image definitions
remain valid. The profile does not require seven named emotions, distinct references, a URI scheme
or specific filenames. Text, reference order and duplicates are preserved verbatim.

The library guarantees that these definitions and checks are included by default. A Host adopting
the profile must resolve the references, establish that the media can be used, and implement its
rendering. A successful content check alone does not guarantee that an arbitrary Host can render
the character. Enabling a Cargo feature does not automatically run validation or rendering;
deserializing/constructing the public struct alone does not validate its contents.

There is no file access, asset decoding, permission grant, role activation, slot binding, model call,
state management, scheduler or rich pack loader in this profile. Its JSON projection ignores unknown
product fields; it is not a lossless rich-pack editor. The guaranteed content floor is persona plus
one or more references, not a seven-image renderer or a universal on-disk character-pack format.

For source compatibility, `minimal_role` aliases the module, and the old type/parser/validator names
alias the same definitions at the module and crate root. JSON fields, content rules and diagnostic
wording are unchanged. The canonical type name changes debug/type-name output; this is not a binary
ABI promise. All canonical and compatibility profile imports are gated by the same feature.

## Cargo feature boundary

The default feature set enables `character-profile`, which enables the profile-only optional
`serde`/`serde_json` dependencies. A dependency with `default-features = false` can use Base without
the profile or those normal/build dependencies. It can opt back in with
`features = ["character-profile"]`. The non-character Host does exactly this and binds only
Memory, Prompt and LLM; its example flow is still chosen by that Host.

[Cargo features are additive and unified across applicable dependency edges](https://doc.rust-lang.org/cargo/reference/features.html#feature-unification).
One edge disabling defaults cannot veto another selected edge enabling the profile. Disable defaults
on every relevant dependency edge and inspect the resolved feature graph. This is not restricted to
the top-level manifest. Selecting both workspace Hosts enables the profile through the character
Host, so the pure-library and non-character checks run separately in CI. Resolver 2 does not remove
this unification for normal dependencies on the same package.

With the profile disabled, both new and legacy profile imports are absent. Dedicated compile-fail
doctests check those import boundaries; the unchanged Base fixture and a running non-character
example check that core calls still work. This separation does not claim `no_std`, a stable binary
ABI, or a complete application without its own Host implementations.

Host execution examples and task-spawning conditions are in [local futures](local-futures.md).
That material chooses a runtime only for an example; it adds no Base bound or execution obligation.

## Acceptance and maintenance

Library tests prove the behavior they actually exercise; the Host tests prove this synthetic local flow.
There is no real model, network, database, tool effect, media playback, crash-recovery or hardware acceptance.
No production TTS, all-platform resource guarantee or stable binary ABI is claimed.

This repository is the defining source for the extracted symbols. Original-workspace compatibility is
verified separately: the transition uses this crate plus re-exports at old import paths, rather than
permanently maintaining two independent definitions. This repository's CI does not certify that a
downstream migration has been merged or that a product has passed acceptance; see [extraction](extraction.md).
