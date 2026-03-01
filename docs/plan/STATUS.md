# Plan Status Board

Last updated: `2026-03-01`

## Baseline Used For This Status

1. JS reference: `node_modules/@cloudflare/workers-types/latest/index.d.ts`
2. Rust reference: `worker/src/*.rs`, `worker-sys/src/types/*.rs`
3. Detailed comparison: `docs/plan/17-rust-vs-js-api-diff.md`

## Closed (Already Landed)

1. `docs/plan/01-binding-browser-rendering.md`
2. `docs/plan/02-binding-dynamic-worker-loaders.md`
3. `docs/plan/03-binding-images.md`
4. `docs/plan/04-binding-mtls-certificates.md`
5. `docs/plan/05-binding-vectorize.md`
6. `docs/plan/06-binding-workflows.md`
7. `docs/plan/07-event-email-handler.md`
8. `docs/plan/08-event-tail-handler.md`
9. `docs/plan/09-rpc-function-args-returns.md`
10. `docs/plan/10-rpc-class-instances.md`
11. `docs/plan/11-rpc-stub-forwarding.md`
12. `docs/plan/12-env-version-metadata.md`
13. `docs/plan/13-pr930-delay-drop-log.md`
14. `docs/plan/14-pr930-service-binding-http-version-fallback.md`
15. `docs/plan/15-pr930-hyperdrive-connect.md`
16. `docs/plan/16-pr930-r2-start-after-and-put-only-if.md`
17. `docs/plan/18-email-binding-api-parity.md`

## Open (Newly Split From Detailed Diff)

1. `docs/plan/19-tail-event-typed-api.md`
2. `docs/plan/20-dispatch-namespace-get-args-options.md`
3. `docs/plan/21-images-binding-pipeline-api-parity.md`
4. `docs/plan/22-event-macro-signature-validation.md`

## Scope Cleanup Notes

1. Items confirmed as already implemented are tracked in the closed list.
2. API-shape gaps remain in the open list for incremental delivery.
