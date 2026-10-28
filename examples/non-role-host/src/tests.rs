#![allow(clippy::unwrap_used)]

use oclive_kernel_base::{BaseCallErrorKind, BaseCallFuture};

use super::*;

struct FailedMemory;

impl MemoryBase for FailedMemory {
    fn retrieve<'a>(&'a self, _: MemoryBaseRequest<'a>) -> BaseCallFuture<'a, Vec<String>> {
        Box::pin(async {
            Err(BaseCallError {
                kind: BaseCallErrorKind::TimedOut,
                detail: Some("fixture timeout".into()),
            })
        })
    }
}

struct FailedPrompt;

impl PromptBase for FailedPrompt {
    fn assemble<'a>(&'a self, _: PromptBaseRequest<'a>) -> BaseCallFuture<'a, String> {
        Box::pin(async {
            Err(BaseCallError {
                kind: BaseCallErrorKind::Unavailable,
                detail: Some("fixture unavailable".into()),
            })
        })
    }
}

#[test]
fn the_three_bindings_preserve_selected_material_and_uncertainty() {
    let materials = [
        "cache: may be stale; not yet confirmed.",
        "network: response received.",
        "cache: another candidate.",
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
    ))
    .unwrap();
    assert_eq!(report.selected, [materials[0]]);
    assert!(report.text.contains(materials[0]));
    assert!(!report.text.contains(materials[1]));
    assert!(!report.text.contains(materials[2]));
    assert_eq!(
        (memory.calls.get(), prompt.calls.get(), llm.calls.get()),
        (1, 1, 1)
    );
}

#[test]
fn memory_failure_preserves_its_reason_and_starts_no_successor() {
    let prompt = LiteralPrompt::default();
    let llm = DemoLlm::default();
    let error = futures_lite::future::block_on(prepare_report(
        MemoryBaseRequest {
            materials: &[],
            query: "cache",
        },
        &FailedMemory,
        &prompt,
        &llm,
    ))
    .unwrap_err();
    assert_eq!(error.kind, BaseCallErrorKind::TimedOut);
    assert_eq!(error.detail.as_deref(), Some("fixture timeout"));
    assert_eq!((prompt.calls.get(), llm.calls.get()), (0, 0));
}

#[test]
fn prompt_failure_preserves_its_reason_and_starts_no_generation() {
    let memory = LiteralMemory {
        limit: 1,
        calls: Cell::new(0),
    };
    let llm = DemoLlm::default();
    let error = futures_lite::future::block_on(prepare_report(
        MemoryBaseRequest {
            materials: &[],
            query: "cache",
        },
        &memory,
        &FailedPrompt,
        &llm,
    ))
    .unwrap_err();
    assert_eq!(error.kind, BaseCallErrorKind::Unavailable);
    assert_eq!(error.detail.as_deref(), Some("fixture unavailable"));
    assert_eq!((memory.calls.get(), llm.calls.get()), (1, 0));
}
