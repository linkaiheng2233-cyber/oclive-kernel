//! OCLive's six-slot Base surface, with a default-on character conformance profile.
//!
//! `character-profile` provides persona/visual-reference content and validation. It
//! recommends seven visual references and requires at least one. Disable default
//! features for Base-only use; no slot requires profile content as an input.
//!
//! This crate owns no Host state, resource binding, capability implementation,
//! turn scheduler, storage, transport, or model adapter. Capability return values
//! are not invocation terminal states or authorization credentials.
//! See the repository's `docs/boundary.md` for the preserved responsibility scope.

#![cfg_attr(
    not(feature = "character-profile"),
    doc = r#"
# Profile-disabled import boundary

Canonical and compatibility imports are absent when `character-profile` is disabled:

```compile_fail
use oclive_kernel_base::character_profile::CharacterProfileDefinition;
```

```compile_fail
use oclive_kernel_base::minimal_role::MinimalRoleDefinition;
```

```compile_fail
use oclive_kernel_base::CharacterProfileDefinition;
```

```compile_fail
use oclive_kernel_base::MinimalRoleDefinition;
```
"#
)]

#[cfg(feature = "character-profile")]
pub mod character_profile;
pub mod requests;
pub mod slot_base;

/// Compatibility module alias, available with the default-on character profile.
#[cfg(feature = "character-profile")]
pub use character_profile as minimal_role;

#[cfg(feature = "character-profile")]
pub use character_profile::{
    parse_character_profile_definition, parse_minimal_role_definition,
    validate_character_profile_definition, validate_minimal_role_definition,
    CharacterProfileDefinition, MinimalRoleDefinition,
};
pub use requests::{
    AgentBaseRequest, BaseCallError, BaseCallErrorKind, EmotionBaseRequest, EventBaseRequest,
    LlmBaseRequest, MemoryBaseRequest, PromptBaseRequest,
};
pub use slot_base::{
    AgentBase, BaseCallFuture, EmotionBase, EventBase, LlmBase, MemoryBase, PromptBase,
};
