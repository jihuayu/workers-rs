# Plan: Tail Event Typed API

## Background

Rust already supports `#[event(tail)]`, but `TailEvent` is currently opaque and lacks typed accessors.
JS API exposes structured fields and typed event handler variants.

## Goal

Provide typed Tail event access in Rust, with predictable parsing and ergonomics close to JS API.

## Design

1. Extend `worker-sys/src/types/tail.rs` with getters for core fields:
2. `invocationId`, `spanContext`, `timestamp`, `sequence`, `event`
3. Add typed Rust-side event model for common variants:
4. `outcome`, `spanOpen`, `spanClose`, `diagnosticChannel`, `exception`, `log`, `return`, `attributes`
5. Preserve raw access (`JsValue`) for unknown future variants.

## Work Items

1. Add wasm-bindgen extern methods for TailEvent core fields.
2. Add serde-based Rust model and parsing helpers in `worker/src/tail.rs`.
3. Add helper APIs for variant matching and safe fallback.
4. Add tests:
5. Known variant parse success
6. Unknown variant fallback without panic
7. Timestamp and sequence conversion checks
8. Add example in docs demonstrating typed pattern matching.

## Risks

1. Tail payload schema evolves; strict enums can become brittle.
2. Time/date conversion between JS and Rust can be a source of subtle bugs.

## Acceptance Criteria

1. Rust users can read core TailEvent fields through typed methods.
2. Common Tail variants can be matched without manual `JsValue` traversal.
3. Unknown variants are handled safely.
