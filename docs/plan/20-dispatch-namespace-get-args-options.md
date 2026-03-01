# Plan: Dispatch Namespace `get` Args/Options Parity

## Background

JS API supports:

1. `DispatchNamespace.get(name, args?, options?)`

Rust currently exposes only:

1. `DynamicDispatcher::get(name)`

## Goal

Support typed `args` and invocation `options` in Rust dynamic dispatch calls while keeping backward compatibility.

## Design

1. Keep existing `get(name)` as a convenience method.
2. Add extended API:
3. `get_with_args(name, args)`
4. `get_with_options(name, args, options)`
5. Add typed options struct (serde) with raw extension field for unknown keys.
6. Update `worker-sys` extern signature(s) to match runtime call shape.

## Work Items

1. Confirm runtime JS signature and argument encoding behavior.
2. Update `worker-sys/src/types/dynamic_dispatcher.rs` accordingly.
3. Implement Rust wrapper overload methods in `worker/src/dynamic_dispatch.rs`.
4. Add serialization tests for args/options payload.
5. Add integration test:
6. worker receives args correctly
7. options propagated correctly
8. Update docs with migration examples.

## Risks

1. Incorrect assumption on runtime signature can break compatibility.
2. Over-typing options can cause unnecessary churn if platform adds fields quickly.

## Acceptance Criteria

1. Rust supports dispatch invocation with args/options.
2. Existing `get(name)` users are not broken.
3. API docs show both simple and advanced usage.
