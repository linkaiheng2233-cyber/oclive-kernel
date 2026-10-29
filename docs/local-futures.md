# Driving local Base futures

Base returns `BaseCallFuture<'a, T>`, a boxed future that can borrow the implementation and this
call's material for `'a`. Neither the trait nor its erased future requires `Send`, `Sync` or `'static`.
The exact binding remains in [slot_base.rs](../crates/oclive_kernel_base/src/slot_base.rs).

## Start with a direct await

Inside an async task that your Host already drives, borrow the slot and input, then await the Base
call directly. The [non-character Host](../examples/non-role-host/src/main.rs) shows this without a
Tokio dependency in its normal graph. Its synchronous entry point uses its own `futures-lite` executor.
You do not need to spawn a task for each capability call, nor run every slot.

## A compiled Tokio example

```sh
cargo run --locked -p oclive_non_role_host --example local-tokio
cargo test --locked -p oclive_non_role_host --example local-tokio
```

The full, compiled source is [local-tokio.rs](../examples/non-role-host/examples/local-tokio.rs).
It owns a current-thread runtime and `LocalSet`, then awaits the borrowed Base call inside
`LocalSet::run_until`. `Rc<Cell<usize>>` deliberately makes the implementation local; input strings
live on the stack, and a `yield_now().await` occurs before the material is inspected. Neither the slot
nor the request must be converted into `'static` data. Expected result: `selected=1 calls=1`, preserving
the statement that stale cache data is **not yet confirmed**. A second test checks a normal empty selection.

This example owns its runtime at the synchronous process entry point. Within an existing async Host,
await on the existing execution context instead of constructing/blocking a nested runtime. Tokio is
only a dev dependency of this example package; the library and either Host's normal dependencies
do not acquire it. This example does not choose a runtime for downstream Hosts.

## Spawning is a different requirement

| Choice | Relevant condition |
|---|---|
| Direct `.await` / `LocalSet::run_until` | The parent computation may borrow local state; keep the borrowed values alive until completion |
| `spawn_local` | The spawned outer future and its output still require `'static`; own the slot/material inside that outer future and borrow them internally |
| `tokio::spawn` | Requires a `Send + 'static` future; adding `Send + Sync` to a Base trait object does not restore `Send` after future erasure |
| `spawn_blocking` | Requires a `Send + 'static` closure and result; it cannot directly capture an existing `!Send` object from another thread |

If a Host uses a separate worker, construct its local state on that worker and exchange suitable owned
messages/results. Do not cast local futures to `Send`, widen Base bounds, or infer physical cancellation
or safe retry from dropping the local call. Task ownership and resource permissions stay with the Host.

Official API references: [LocalSet](https://docs.rs/tokio/1.53.3/tokio/task/struct.LocalSet.html),
[spawn_local](https://docs.rs/tokio/1.53.3/tokio/task/fn.spawn_local.html),
[spawn_blocking](https://docs.rs/tokio/1.53.3/tokio/task/fn.spawn_blocking.html).
