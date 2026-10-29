//! Drive a borrowed, deliberately !Send Base future on a Host-owned local executor.

use std::cell::Cell;
use std::rc::Rc;

use oclive_kernel_base::{BaseCallFuture, MemoryBase, MemoryBaseRequest};

struct LocalMemory {
    calls: Rc<Cell<usize>>,
}

impl MemoryBase for LocalMemory {
    fn retrieve<'a>(&'a self, request: MemoryBaseRequest<'a>) -> BaseCallFuture<'a, Vec<String>> {
        Box::pin(async move {
            // Both local state and caller material remain borrowed across a yield.
            tokio::task::yield_now().await;
            self.calls.set(self.calls.get() + 1);
            Ok(request
                .materials
                .iter()
                .filter(|item| item.contains(request.query))
                .map(|item| (*item).to_owned())
                .collect())
        })
    }
}

fn select_locally(query: &str) -> Result<(Vec<String>, usize), Box<dyn std::error::Error>> {
    let first = String::from("cache: data may be stale; not yet confirmed.");
    let second = String::from("network: response received.");
    let materials = [first.as_str(), second.as_str()];
    let memory = LocalMemory {
        calls: Rc::new(Cell::new(0)),
    };
    let runtime = tokio::runtime::Builder::new_current_thread().build()?;
    let local = tokio::task::LocalSet::new();
    let selected = runtime.block_on(local.run_until(async {
        memory
            .retrieve(MemoryBaseRequest {
                materials: &materials,
                query,
            })
            .await
    }))?;
    Ok((selected, memory.calls.get()))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let query = String::from("cache");
    let (selected, calls) = select_locally(&query)?;
    println!("Host-owned current-thread runtime + LocalSet; no spawned task");
    println!("selected={} calls={calls}", selected.len());
    println!("{}", selected.join("\n"));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn local_state_and_borrowed_material_survive_a_yield() -> Result<(), Box<dyn std::error::Error>>
    {
        let query = String::from("cache");
        let (selected, calls) = select_locally(&query)?;
        assert_eq!(selected, ["cache: data may be stale; not yet confirmed."]);
        assert_eq!(calls, 1);
        Ok(())
    }

    #[test]
    fn no_match_is_a_normal_empty_result() -> Result<(), Box<dyn std::error::Error>> {
        let (selected, calls) = select_locally("unmatched")?;
        assert!(selected.is_empty());
        assert_eq!(calls, 1);
        Ok(())
    }
}
