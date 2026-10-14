//! OCLive's extracted six-slot Base surface and minimum logical role content.
//!
//! This crate owns no Host state, resource binding, capability implementation,
//! turn scheduler, storage, transport, or model adapter. Capability return values
//! are not invocation terminal states or authorization credentials.
//! See the repository's `docs/boundary.md` for the preserved responsibility scope.

pub mod minimal_role;
pub mod requests;
pub mod slot_base;

pub use minimal_role::{
    parse_minimal_role_definition, validate_minimal_role_definition, MinimalRoleDefinition,
};
pub use requests::{
    AgentBaseRequest, BaseCallError, BaseCallErrorKind, EmotionBaseRequest, EventBaseRequest,
    LlmBaseRequest, MemoryBaseRequest, PromptBaseRequest,
};
pub use slot_base::{
    AgentBase, BaseCallFuture, EmotionBase, EventBase, LlmBase, MemoryBase, PromptBase,
};
