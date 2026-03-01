# Plan: Event Macro Signature Validation

## Background

`worker-macros` currently parses `#[event(...)]` attributes but still has a TODO for signature validation.
That means invalid handler signatures may fail later with less actionable errors.

## Goal

Provide strict, compile-time validation for event handler signatures with clear diagnostics.

## Design

1. Validate argument and return signatures per handler type:
2. `fetch`
3. `scheduled`
4. `email`
5. `tail`
6. `queue` (feature-gated)
7. `start`
8. Emit focused compiler errors showing expected signature forms.
9. Add compile-fail tests using `trybuild`.

## Work Items

1. Implement signature checks in `worker-macros/src/event.rs`.
2. Add positive and negative macro test fixtures.
3. Add tests for `respond_with_errors` interaction where relevant.
4. Update event macro docs with canonical signatures.

## Risks

1. Overly rigid validation may block legitimate advanced generic signatures.
2. Error messages can become hard to maintain if macro logic grows too complex.

## Acceptance Criteria

1. Invalid signatures fail at compile time with actionable messages.
2. Existing valid examples keep compiling.
3. CI includes compile-fail coverage for each event type.
