# Plan Status Board

Last updated: `2026-03-01`

## Baseline Used For This Status

1. JS reference: `node_modules/@cloudflare/workers-types/latest/index.d.ts`
2. Rust reference: `worker/src/*.rs`, `worker-sys/src/types/*.rs`
3. Detailed comparison: `docs/plan/17-rust-vs-js-api-diff.md`

## Closed (Already Landed)

1. `docs/plan/13-pr930-delay-drop-log.md`
2. `docs/plan/14-pr930-service-binding-http-version-fallback.md`
3. `docs/plan/15-pr930-hyperdrive-connect.md`
4. `docs/plan/01-binding-browser-rendering.md`
13. `docs/plan/05-binding-vectorize.md`
5. `docs/plan/02-binding-dynamic-worker-loaders.md`
6. `docs/plan/04-binding-mtls-certificates.md`
7. `docs/plan/07-event-email-handler.md`
1. `docs/plan/06-binding-workflows.md`

2. `docs/plan/10-rpc-class-instances.md`

3. `docs/plan/11-rpc-stub-forwarding.md`
2. `docs/plan/03-binding-images.md`
3. `docs/plan/05-binding-vectorize.md`
4. `docs/plan/06-binding-workflows.md`

## Open (Newly Split From Detailed Diff)

1. `docs/plan/18-email-binding-api-parity.md`
2. `docs/plan/19-tail-event-typed-api.md`
3. `docs/plan/20-dispatch-namespace-get-args-options.md`
4. `docs/plan/21-images-binding-pipeline-api-parity.md`
5. `docs/plan/22-event-macro-signature-validation.md`

## Scope Cleanup Notes

1. Items confirmed as already implemented were removed from active priority list.
2. API-shape gaps are now separated from entrypoint availability so progress is easier to track.
