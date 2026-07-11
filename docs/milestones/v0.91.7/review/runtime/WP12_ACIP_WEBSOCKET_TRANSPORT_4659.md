# WP-12 ACIP WebSocket Transport Proof (#4659)

## Summary

#4659 extends the #4900 substrate proof into a bounded WebSocket transport path
implementation for ACIP runtime streams. The implemented path uses
`tokio-tungstenite` for real local client/server WebSocket mechanics, validates
ACIP JSON envelopes at the transport boundary, applies a fail-closed access
policy, and retains positive and negative proof evidence.

## Implemented Surface

- `adl::agent_comms::runtime_stream::prove_acip_runtime_stream_websocket_transport_path_v1`
  executes the transport proof.
- `AcipRuntimeStreamWebSocketTransportProofV1` records the transport proof
  contract.
- `AcipRuntimeStreamAccessPolicyV1` records allowed sender prefixes, denied
  sender prefixes, allowed recipients, required trace posture, and the
  no-authority-expansion invariant.
- `run_wp12_acip_websocket_transport_proof` writes the retained review packet.

## Proof Coverage

The retained proof covers:

- positive ACIP envelope delivery over a local WebSocket server/client path;
- malformed JSON frame rejection;
- sender-policy denial for external sender claims;
- peer close before request/response completion;
- response timeout while the peer remains idle.

All negative cases are classified as fail-closed and record error evidence.

## Retained Evidence

- `docs/milestones/v0.91.7/review/runtime/wp12_acip_websocket_transport_4659/acip_websocket_transport_proof.json`
- `docs/milestones/v0.91.7/review/runtime/wp12_acip_websocket_transport_4659/evidence_index.json`
- `docs/milestones/v0.91.7/review/runtime/wp12_acip_websocket_transport_4659/audit/artifact_safety_scan.json`
- `docs/milestones/v0.91.7/review/runtime/wp12_acip_websocket_transport_4659/reviewer_walkthrough.md`

## Validation

```sh
cargo test --manifest-path adl/Cargo.toml acip_runtime_stream --lib -- --nocapture
cargo run --manifest-path adl/Cargo.toml --bin run_wp12_acip_websocket_transport_proof -- --out docs/milestones/v0.91.7/review/runtime/wp12_acip_websocket_transport_4659
```

Both commands passed locally during #4659 execution.

## Non-Claims

- This proof does not implement protobuf wire encoding; #4658 owns schema and
  protobuf projection.
- This proof does not claim production TLS termination, production
  authentication, reconnect scheduling, or cross-polis networking.
- This proof does not bypass ACIP access rules, Freedom Gate decisions, trace,
  replay, or policy boundaries.
