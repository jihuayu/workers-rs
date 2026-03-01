# Plan: Images Binding Pipeline API Parity

## Background

Rust already has `Env::images(...)` and a single `transform(...)` call.
JS Images API provides a richer pipeline:

1. `info(stream, options?)`
2. `input(stream, options?) -> ImageTransformer`
3. `ImageTransformer.transform/draw/output`
4. `ImageTransformationResult.response/contentType/image`

## Goal

Align Rust Images binding with the JS pipeline model while retaining a simple one-shot helper.

## Design

1. Extend `worker-sys/src/types/images.rs` with:
2. `info`, `input`
3. `ImageTransformer` methods `transform`, `draw`, `output`
4. `ImageTransformationResult` accessors
5. Add Rust wrappers and typed option/result structs.
6. Keep existing `transform_json` as convenience wrapper layered on top.

## Work Items

1. Add missing wasm-bindgen types and methods in `worker-sys`.
2. Add Rust wrapper types in `worker/src/images.rs`.
3. Define typed options/outputs with serde and raw extension points.
4. Add tests for:
5. Metadata info path
6. Multi-step transform pipeline
7. Output conversion (`Response`, content type, bytes)
8. Update docs to clearly separate:
9. Request-level `cf.image`
10. Product binding `images` pipeline

## Risks

1. Stream ownership/lifetime across chained operations can be tricky.
2. Local runtime support may lag for some transformer operations.

## Acceptance Criteria

1. Rust can run the same image pipeline stages as JS API.
2. Multi-stage transform examples compile and run.
3. Existing one-shot transform flow remains available.
