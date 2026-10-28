//! A non-character Host: select explicit diagnostic material and prepare a text report.
//! The three bindings and their sequence belong to this example, not to the library.

#[path = "../../common/demo_modules.rs"]
mod modules;

use std::cell::Cell;

use modules::{DemoLlm, LiteralMemory, LiteralPrompt};
use oclive_kernel_base::{
    BaseCallError, LlmBase, LlmBaseRequest, MemoryBase, MemoryBaseRequest, PromptBase,
    PromptBaseRequest,
};

#[derive(Debug)]
struct Report {
    selected: Vec<String>,
    text: String,
}

async fn prepare_report(
    request: MemoryBaseRequest<'_>,
    memory: &dyn MemoryBase,
    prompt: &dyn PromptBase,
    llm: &dyn LlmBase,
) -> Result<Report, BaseCallError> {
    let selected = memory.retrieve(request).await?;
    let materials: Vec<&str> = selected.iter().map(String::as_str).collect();
    let prepared = prompt
        .assemble(PromptBaseRequest {
            materials: &materials,
            requirements: "Report the supplied diagnostic material; preserve uncertainty.",
        })
        .await?;
    let text = llm.generate(LlmBaseRequest { input: &prepared }).await?;
    Ok(Report { selected, text })
}

fn main() -> Result<(), BaseCallError> {
    let materials = [
        "cache: stale data may explain the delay; not yet confirmed.",
        "network: response received.",
        "cache: a second candidate observation.",
    ];
    let memory = LiteralMemory {
        limit: 1,
        calls: Cell::new(0),
    };
    let prompt = LiteralPrompt::default();
    let llm = DemoLlm::default();
    let report = futures_lite::future::block_on(prepare_report(
        MemoryBaseRequest {
            materials: &materials,
            query: "cache",
        },
        &memory,
        &prompt,
        &llm,
    ))?;
    println!("Non-character text report: MemoryBase -> PromptBase -> LlmBase");
    println!("EmotionBase / EventBase / AgentBase: not bound, not called");
    println!(
        "selected={} calls: memory={} prompt={} llm={}",
        report.selected.len(),
        memory.calls.get(),
        prompt.calls.get(),
        llm.calls.get()
    );
    println!("{}", report.text);
    Ok(())
}

#[cfg(test)]
mod tests;
