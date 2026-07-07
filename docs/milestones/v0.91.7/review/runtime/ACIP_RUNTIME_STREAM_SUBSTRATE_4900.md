# ACIP Runtime Stream Substrate Proof (#4900)

Issue #4900 is folded into WP-12 as a feeder for #4658 and #4659.

## Decision

- Selected substrate: WebSocket.
- Selected Rust crate: `tokio-tungstenite`.
- Role: carrier mechanics only. ACIP remains the domain contract above the transport.

`jsonrpsee` remains a possible later choice only if ACIP runtime streams become JSON-RPC method dispatch. The current WP-12 need is envelope/event carriage, so WebSocket mechanics are the smaller fit.

## Implemented Surface

- Added `adl::agent_comms::runtime_stream`.
- Added structured decision record type `AcipRuntimeStreamSubstrateDecisionV1`.
- Added structured loopback proof type `AcipRuntimeStreamLoopbackProofV1`.
- Added `prove_acip_runtime_stream_websocket_loopback_v1`, which opens a local WebSocket listener and client with `tokio-tungstenite`, sends one ACIP message envelope, validates it server-side, returns a validated ACIP response envelope, and closes the WebSocket.

## Failure Classification

The decision/proof surface classifies:

- reconnect after disconnect: fail closed, retry allowed only after a new handshake and ACIP revalidation.
- malformed message: fail closed, no retry without corrected payload.
- peer close before response: fail closed, retry allowed.
- response timeout: fail closed, retry allowed.
- auth/policy denial: fail closed, no authority expansion.

## Non-Claims

- This does not implement protobuf wire encoding; #4658 owns schema/protobuf projection.
- This does not prove production WebSocket authentication, TLS, reconnect scheduling, or cross-polis transport; #4659 owns the WP-12 transport path.
- This does not bypass ACIP access rules, Freedom Gate decisions, trace, replay, or policy boundaries.

## Validation

Command:

```sh
cargo test --manifest-path adl/Cargo.toml acip_runtime_stream -- --nocapture
```

Result:

- Passed.
- `agent_comms::tests::acip_runtime_stream_substrate_decision_selects_tokio_tungstenite`
- `agent_comms::tests::acip_runtime_stream_websocket_loopback_carries_validated_envelopes`

## Integration Notes

#4658 should consume the decision as carrier selection input while keeping protobuf/schema ownership separate.

#4659 should consume `prove_acip_runtime_stream_websocket_loopback_v1` as the first bounded loopback proof and extend from it toward the full WP-12 WebSocket transport path.
