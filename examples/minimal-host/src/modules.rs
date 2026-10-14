//! Example implementations owned by this Host, not by the public Kernel library.

use std::cell::Cell;

use oclive_kernel_base::{
    BaseCallFuture, LlmBase, LlmBaseRequest, MemoryBase, MemoryBaseRequest, PromptBase,
    PromptBaseRequest,
};

pub(crate) struct LiteralMemory {
    pub limit: usize,
    pub calls: Cell<usize>,
}

impl MemoryBase for LiteralMemory {
    fn retrieve<'a>(&'a self, request: MemoryBaseRequest<'a>) -> BaseCallFuture<'a, Vec<String>> {
        Box::pin(async move {
            self.calls.set(self.calls.get() + 1);
            Ok(request
                .materials
                .iter()
                .filter(|material| material.contains(request.query))
                .take(self.limit)
                .map(|material| (*material).to_owned())
                .collect())
        })
    }
}

#[derive(Default)]
pub(crate) struct LiteralPrompt {
    pub calls: Cell<usize>,
}

impl PromptBase for LiteralPrompt {
    fn assemble<'a>(&'a self, request: PromptBaseRequest<'a>) -> BaseCallFuture<'a, String> {
        Box::pin(async move {
            self.calls.set(self.calls.get() + 1);
            Ok(format!(
                "Requirement: {}\nReference material:\n{}",
                request.requirements,
                request.materials.join("\n")
            ))
        })
    }
}

#[derive(Default)]
pub(crate) struct DemoLlm {
    pub calls: Cell<usize>,
}

impl LlmBase for DemoLlm {
    fn generate<'a>(&'a self, request: LlmBaseRequest<'a>) -> BaseCallFuture<'a, String> {
        Box::pin(async move {
            self.calls.set(self.calls.get() + 1);
            Ok(format!(
                "[deterministic demo; no inference]\n{}",
                request.input
            ))
        })
    }
}
