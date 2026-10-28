# Canonical ingress failure events (#852)

Canonical ingress now emits `domain_work_failed` when its actual dispatch returns
an error, before sending that error to the waiting caller. Successful dispatch
continues to emit `domain_work_completed`. Both use the accepted envelope's
correlation ID and the `canonical_ingress` component identity. The event carries
no provider error string or request payload; existing recorder sanitization and
stdout/stderr policy remain unchanged.

The production component loop, operation adapter, control service, TLS listener
and authenticated Observatory WSS projection are exercised by
`conversation_sessions_tests::resident_agent_conversation_uses_canonical_agent_runtime_wss_ingress`
in `adl-runtime-kernel/src/conversation_sessions_tests.rs`. Its test executor
runs a local `/bin/sh` process that exits 42 with a private stderr canary. The
actual adapter/dispatch error drives the event: no event is manually inserted
and no caller-provided event or failure flag controls Runtime emission.

The assertions cover:

- One completion event and no failure event for the successful request.
- One Runtime failure event and no completion event for the failing request.
- Failure response correlation and accepted sequence, with no synthetic reply.
- Exact equality between the recorded failure event and its WSS projection,
  including component, correlation, sequence and monotonic timestamp.
- Rejection of an authenticated client's forged Runtime event and an intent
  containing an injected event field.
- Rejection of a conflicting replay under a different correlation ID, without
  producing events under that unaccepted identity.
- Absence of private provider stderr and both bearer token canaries from the
  error response, recorder events and public WSS feed.

This is local deterministic fault injection through real ingress and WSS
production paths. The executor is a test implementation; this does not qualify
production provider subprocess recovery (#901), six residents (#900), remote
execution, release acceptance, or aggregate evidence admission (#902).

## Focused validation

Use an issue-owned `CARGO_TARGET_DIR` and run:

```sh
cargo test --locked --manifest-path adl-runtime-kernel/Cargo.toml --lib -- --list
cargo test --locked --manifest-path adl-runtime-kernel/Cargo.toml --lib conversation_sessions_tests::resident_agent_conversation_uses_canonical_agent_runtime_wss_ingress -- --exact --nocapture
cargo test --locked --manifest-path adl-runtime-kernel/Cargo.toml --lib ingress::tests::
cargo fmt --manifest-path adl-runtime-kernel/Cargo.toml --check
git diff --check
```

Enumeration identified 222 library tests. The WSS scenario and seven adjacent
ingress regressions are the focused execution surface. The 28-test
`governed_operations` target was enumerated separately; enumeration is not
execution proof. PR CI and exact-head independent review are separate gates.

## PVF classification

Lane: `runtime`; proof role: `runtime_regression`; determinism: isolated local
TLS listener, test PKI, explicit process exit and controlled fixture barriers;
resources: CPU, Rust build, local filesystem, ephemeral loopback port and
`/bin/sh`, with no external provider/network or paid infrastructure. This is a
required local regression for #852 and participates in ordinary Runtime CI;
it is not by itself a release qualification gate. Operational logs remain on
stderr; libtest output remains on stdout. No compatibility-log override is used.

The #851 source worktree is preserved unchanged. Only its bounded ingress,
telemetry and WSS test changes were adapted; its resident and provider work
remains outside this patch.
