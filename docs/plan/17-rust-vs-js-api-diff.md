# Plan: Rust vs JS API Diff (Detailed)

## Scope and Baseline

This document is the detailed parity diff between:

1. JS API baseline: `node_modules/@cloudflare/workers-types/latest/index.d.ts`
2. Rust API baseline: `worker/src/*.rs` and `worker-sys/src/types/*.rs`

Checked on: `2026-03-01`.

## Parity Matrix

| Area | JS SDK Surface | Rust Current Surface | Gap Type | Plan |
|---|---|---|---|---|
| Hyperdrive connect | `Hyperdrive.connect()` + getters (`connectionString`, `host`, `port`, `user`, `password`, `database`) | `Hyperdrive::connect` + getters already exist in `worker/src/hyperdrive.rs:58` | Closed | `docs/plan/15-pr930-hyperdrive-connect.md` |
| HTTP version fallback | Unknown `httpProtocol` should not panic | Unknown protocol already falls back to `HTTP_11` in `worker/src/http/request.rs:18` | Closed | `docs/plan/14-pr930-service-binding-http-version-fallback.md` |
| Delay drop log | Drop path should stay safe without noisy logs | Drop path only clears timeout now, no debug logging in `worker/src/delay.rs:93` | Closed | `docs/plan/13-pr930-delay-drop-log.md` |
| R2 list pagination | JS `R2ListOptions.startAfter` and `R2Bucket.list(options)` support `startAfter` | `ListOptionsBuilder` has `limit/prefix/cursor/delimiter/include` only (`worker/src/r2/builder.rs:360`) | Missing field | `docs/plan/16-pr930-r2-start-after-and-put-only-if.md` |
| Email event payload API | `ForwardableEmailMessage` has `from/to/raw/headers/rawSize/setReject/forward/reply`; `SendEmail.send` exists | `EmailMessage` is currently opaque wrapper only (`worker/src/email.rs:5`, `worker-sys/src/types/email.rs:7`) | Missing typed API | `docs/plan/18-email-binding-api-parity.md` |
| Tail event typed model | JS `TailEvent` includes `invocationId/spanContext/timestamp/sequence/event` + typed handler object | `TailEvent` is currently opaque wrapper only (`worker/src/tail.rs:5`, `worker-sys/src/types/tail.rs:7`) | Missing typed API | `docs/plan/19-tail-event-typed-api.md` |
| Images binding API shape | JS has `info(...)`, `input(...) -> ImageTransformer`, then `transform/draw/output` chain | Rust exposes only `ImagesBinding::transform` (`worker/src/images.rs:16`, `worker-sys/src/types/images.rs:10`) | Shape mismatch / partial | `docs/plan/21-images-binding-pipeline-api-parity.md` |
| Dispatch namespace dynamic args/options | JS `DispatchNamespace.get(name, args?, options?)` | Rust has `DynamicDispatcher::get(name)` only (`worker/src/dynamic_dispatch.rs:22`) | Missing overload/options | `docs/plan/20-dispatch-namespace-get-args-options.md` |
| Vectorize full API | JS `describe/query/queryById/insert/upsert/deleteByIds/getByIds` (+ typed options) | Rust currently `upsert/query/delete` generic bridge only (`worker/src/vectorize.rs:16`) | Partial | `docs/plan/05-binding-vectorize.md` |
| Workflows full API | JS has `Workflow.get/create/createBatch`, `WorkflowInstance` lifecycle methods, and step primitives (`do/sleep/sleepUntil/waitForEvent`) | Rust currently has `trigger/get_status` only (`worker/src/workflows.rs:16`) | Partial | `docs/plan/06-binding-workflows.md` |
| Event macro signature guarantees | Handler signatures are strongly typed in JS examples/tooling | `worker-macros` still has `TODO: validate the inputs / signature` (`worker-macros/src/event.rs:47`) | Missing validation | `docs/plan/22-event-macro-signature-validation.md` |

## Notes

1. `put(...).only_if(...) -> Result<Option<Object>>` is already landed; the remaining PR930 gap on R2 is `start_after`.
2. `#[event(email)]` and `#[event(tail)]` entrypoints already exist. The remaining work is API completeness of payload/event models.
3. `docs/plan/03-binding-images.md` completed the initial binding availability; full JS-shape alignment is tracked separately by plan 21.
