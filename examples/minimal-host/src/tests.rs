#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::cell::RefCell;

use oclive_kernel_base::{BaseCallErrorKind, BaseCallFuture};

use super::*;

#[derive(Default)]
struct InspectLlm {
    inputs: RefCell<Vec<String>>,
}

impl LlmBase for InspectLlm {
    fn generate<'a>(&'a self, request: LlmBaseRequest<'a>) -> BaseCallFuture<'a, String> {
        Box::pin(async move {
            self.inputs.borrow_mut().push(request.input.to_owned());
            Ok(request.input.to_owned())
        })
    }
}

struct FailedPrompt;

impl PromptBase for FailedPrompt {
    fn assemble<'a>(&'a self, _request: PromptBaseRequest<'a>) -> BaseCallFuture<'a, String> {
        Box::pin(async {
            Err(BaseCallError {
                kind: BaseCallErrorKind::Unavailable,
                detail: Some("fixture unavailable".into()),
            })
        })
    }
}

struct FailedMemory;

impl MemoryBase for FailedMemory {
    fn retrieve<'a>(&'a self, _request: MemoryBaseRequest<'a>) -> BaseCallFuture<'a, Vec<String>> {
        Box::pin(async {
            Err(BaseCallError {
                kind: BaseCallErrorKind::TimedOut,
                detail: Some("fixture timeout".into()),
            })
        })
    }
}

fn request<'a>(materials: &'a [&'a str]) -> MemoryBaseRequest<'a> {
    MemoryBaseRequest {
        materials,
        query: "river",
    }
}

#[test]
fn selected_implementation_is_called_and_its_result_is_used_verbatim() {
    let memory = LiteralMemory {
        limit: 2,
        calls: Cell::new(0),
    };
    let prompt = LiteralPrompt::default();
    let llm = InspectLlm::default();
    let materials = [
        "Alice may visit the river.",
        "Bob likes tea.",
        "Carol has not crossed the river.",
    ];
    let output = futures_lite::future::block_on(run_host(
        &sample_role(),
        request(&materials),
        &memory,
        &prompt,
        &llm,
    ))
    .unwrap();
    assert_eq!(output.selected, [materials[0], materials[2]]);
    assert_eq!(memory.calls.get(), 1);
    assert_eq!(prompt.calls.get(), 1);
    assert_eq!(llm.inputs.borrow().as_slice(), [output.reply.as_str()]);
    assert!(output.reply.contains(materials[0]));
    assert!(output.reply.contains(materials[2]));
    assert!(!output.reply.contains(materials[1]));
}

#[test]
fn replacing_the_memory_binding_changes_the_actual_result() {
    let role = sample_role();
    let materials = ["first river", "second river"];
    let prompt = LiteralPrompt::default();
    let llm = DemoLlm::default();
    let two = LiteralMemory {
        limit: 2,
        calls: Cell::new(0),
    };
    let one = LiteralMemory {
        limit: 1,
        calls: Cell::new(0),
    };
    let original =
        futures_lite::future::block_on(run_host(&role, request(&materials), &two, &prompt, &llm))
            .unwrap();
    let replaced =
        futures_lite::future::block_on(run_host(&role, request(&materials), &one, &prompt, &llm))
            .unwrap();
    assert_eq!(original.selected, materials);
    assert_eq!(replaced.selected, [materials[0]]);
    assert_ne!(original.reply, replaced.reply);
    assert_eq!((two.calls.get(), one.calls.get()), (1, 1));
}

#[test]
fn invalid_role_calls_no_capability() {
    let mut role = sample_role();
    role.persona_prompt.clear();
    let memory = LiteralMemory {
        limit: 2,
        calls: Cell::new(0),
    };
    let prompt = LiteralPrompt::default();
    let llm = DemoLlm::default();
    let result =
        futures_lite::future::block_on(run_host(&role, request(&[]), &memory, &prompt, &llm));
    assert!(matches!(result, Err(HostError::InvalidRole(_))));
    assert_eq!(
        (memory.calls.get(), prompt.calls.get(), llm.calls.get()),
        (0, 0, 0)
    );
}

#[test]
fn memory_failure_preserves_reason_and_starts_no_successor() {
    let prompt = LiteralPrompt::default();
    let llm = DemoLlm::default();
    let result = futures_lite::future::block_on(run_host(
        &sample_role(),
        request(&[]),
        &FailedMemory,
        &prompt,
        &llm,
    ));
    match result {
        Err(HostError::Capability("MemoryBase", error)) => {
            assert_eq!(error.kind, BaseCallErrorKind::TimedOut);
            assert_eq!(error.detail.as_deref(), Some("fixture timeout"));
        }
        other => panic!("unexpected result: {other:?}"),
    }
    assert_eq!((prompt.calls.get(), llm.calls.get()), (0, 0));
}

#[test]
fn prompt_failure_preserves_reason_and_starts_no_generation() {
    let memory = LiteralMemory {
        limit: 2,
        calls: Cell::new(0),
    };
    let llm = DemoLlm::default();
    let result = futures_lite::future::block_on(run_host(
        &sample_role(),
        request(&[]),
        &memory,
        &FailedPrompt,
        &llm,
    ));
    match result {
        Err(HostError::Capability("PromptBase", error)) => {
            assert_eq!(error.kind, BaseCallErrorKind::Unavailable);
            assert_eq!(error.detail.as_deref(), Some("fixture unavailable"));
        }
        other => panic!("unexpected result: {other:?}"),
    }
    assert_eq!(memory.calls.get(), 1);
    assert_eq!(llm.calls.get(), 0);
}
