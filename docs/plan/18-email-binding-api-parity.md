# Plan: Email Binding API Parity

## Background

Rust currently supports the `email` event entrypoint, but the message object is still an opaque wrapper.
Compared with JS API, key methods/properties are missing.

## Goal

Align Rust email APIs with JS-level capabilities for both:

1. Receiving email (`ForwardableEmailMessage` style)
2. Sending email (`SendEmail.send`)

## Design

1. Expand `worker-sys/src/types/email.rs` to expose message fields and methods:
2. `from`, `to`, `raw`, `headers`, `rawSize`
3. `setReject(reason)`, `forward(rcptTo, headers?)`, `reply(message)`
4. Add sender binding type in `worker-sys` and `worker` for `send(message)`.
5. Keep raw `JsValue` escape hatch for forward compatibility.

## Work Items

1. Add wasm-bindgen externs for email message and sender binding.
2. Implement typed wrapper methods in `worker/src/email.rs`.
3. Add `Env` accessor for sender binding (name finalized during review).
4. Add examples:
5. Reject path
6. Forward path
7. Send path
8. Add tests for field access and method calls (success + error cases).

## Risks

1. Runtime support for all email operations may vary between local dev and deployed environments.
2. Overly strict Rust structs may break if JS payload shape evolves; keep extensible fields where needed.

## Acceptance Criteria

1. Rust users can access core email fields without manual `JsValue` parsing.
2. Rust users can forward/reply/reject via typed methods.
3. Rust users can send email via a first-class binding wrapper.
