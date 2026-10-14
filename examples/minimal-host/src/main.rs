//! An explicit, deterministic Host call sequence; no shared runtime or reference Host.

mod modules;

use std::cell::Cell;

use modules::{DemoLlm, LiteralMemory, LiteralPrompt};
use oclive_kernel_base::{
    validate_minimal_role_definition, BaseCallError, LlmBase, LlmBaseRequest, MemoryBase,
    MemoryBaseRequest, MinimalRoleDefinition, PromptBase, PromptBaseRequest,
};

#[derive(Debug)]
enum HostError {
    InvalidRole(Vec<String>),
    Capability(&'static str, BaseCallError),
}

impl std::fmt::Display for HostError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidRole(errors) => {
                write!(formatter, "invalid logical role: {}", errors.join("; "))
            }
            Self::Capability(slot, error) => write!(formatter, "{slot}: {error}"),
        }
    }
}

impl std::error::Error for HostError {}

#[derive(Debug)]
struct HostOutput {
    selected: Vec<String>,
    reply: String,
}

async fn run_host(
    role: &MinimalRoleDefinition,
    request: MemoryBaseRequest<'_>,
    memory: &dyn MemoryBase,
    prompt: &dyn PromptBase,
    llm: &dyn LlmBase,
) -> Result<HostOutput, HostError> {
    validate_minimal_role_definition(role).map_err(HostError::InvalidRole)?;
    let selected = memory
        .retrieve(request)
        .await
        .map_err(|error| HostError::Capability("MemoryBase", error))?;
    let mut materials = vec![role.persona_prompt.as_str()];
    materials.extend(selected.iter().map(String::as_str));
    let prepared = prompt
        .assemble(PromptBaseRequest {
            materials: &materials,
            requirements: "Describe the supplied material without inventing facts.",
        })
        .await
        .map_err(|error| HostError::Capability("PromptBase", error))?;
    let reply = llm
        .generate(LlmBaseRequest { input: &prepared })
        .await
        .map_err(|error| HostError::Capability("LlmBase", error))?;
    Ok(HostOutput { selected, reply })
}

fn sample_role() -> MinimalRoleDefinition {
    MinimalRoleDefinition {
        persona_prompt: "A careful guide who preserves uncertainty.".into(),
        visual_assets: vec!["host-owned:portrait".into()],
    }
}

fn main() -> Result<(), HostError> {
    let role = sample_role();
    let materials = [
        "Alice may visit the river.",
        "Bob likes tea.",
        "Carol has not crossed the river.",
    ];
    let memory = LiteralMemory {
        limit: 2,
        calls: Cell::new(0),
    };
    let prompt = LiteralPrompt::default();
    let llm = DemoLlm::default();
    let output = futures_lite::future::block_on(run_host(
        &role,
        MemoryBaseRequest {
            materials: &materials,
            query: "river",
        },
        &memory,
        &prompt,
        &llm,
    ))?;
    println!("MemoryBase -> LiteralMemory (Host implementation)");
    println!("PromptBase -> LiteralPrompt (Host implementation)");
    println!("LlmBase -> DemoLlm (deterministic substitute, no model inference)");
    println!("EmotionBase / EventBase / AgentBase: not bound, not called");
    println!(
        "selected={} calls: memory={} prompt={} llm={}",
        output.selected.len(),
        memory.calls.get(),
        prompt.calls.get(),
        llm.calls.get()
    );
    println!("{}", output.reply);
    Ok(())
}

#[cfg(test)]
mod tests;
