# WP-12 ACIP Schema And Protobuf Projection Proof (#4658)

## Metadata

- Issue: `#4658`
- Parent sprint: `#4639`
- Milestone: `v0.91.7`
- Status: integrated proven for schema/projection contract
- Machine-readable companion: `docs/milestones/v0.91.7/review/security/wp12_acip_schema_protobuf_projection_4658.json`

## Purpose

Finalize the bounded ACIP/A2A schema and protobuf projection contract that
WP-12 needs before `v0.92` can consume ACIP protocol readiness claims.

## Implemented Surface

- Added `adl::agent_comms::projection`.
- Added `AcipProtobufProjectionProfileV1`, message projection rows, enum
  projection rows, and validation for deterministic field numbers, JSON
  pointer bindings, duplicate rejection, and required-field parity.
- Added fail-closed checks that every enum field references a declared enum
  projection, every top-level JSON schema property is projected, and every
  projection row names the expected ACIP schema version.
- Bound the projection profile to the existing Rust `schemars` JSON schemas for:
  - `AcipMessageEnvelopeV1`
  - `AcipInvocationContractV1`
  - `AcipInvocationEventV1`
  - `AcipA2aAdapterBoundaryV1`
- Recorded consumption posture:
  - JSON projection remains the primary implemented consumption posture for
    `v0.91.7`.
  - WebSocket consumption uses text JSON frames from the #4900 loopback proof.
  - protobuf wire encoding and binary WebSocket frames remain non-claims until
    separately implemented and proven.

## Relationship To WP-12 Gates

This proof satisfies the `acip_a2a_schema_and_protobuf_projection` row in the
WP-12 security/CAV gate for the schema/projection contract only.

It does not satisfy:

- `#4659` WebSocket transport activation;
- `#4660` external-agent access rules;
- authentication, TLS, reconnect, or cross-polis transport claims;
- generated prost/protobuf wire encoding.

## Validation

Focused local validation:

```sh
git diff --check
cargo test --manifest-path adl/Cargo.toml acip --all-features
cargo test --manifest-path adl/Cargo.toml acip_protobuf_projection --all-features
```

Result:

- Passed after review fixes and retry.
- Initial review found incomplete enum coverage and insufficient drift checks
  for enum references, optional JSON properties, and schema version bindings.
  Those findings were fixed before publication.
- Focused projection tests passed with six projection-specific checks.
- The exact VPP command `cargo test --manifest-path adl/Cargo.toml acip --all-features`
  passed on the final rerun.
- Earlier full-lane attempts exposed an unrelated transient failure in
  `run_v0916_acip_aee_memory_integration` while moving `.adl/runtime_environment.json`;
  the failing binary test passed in isolation and the final exact full lane
  passed.

## Non-Claims

- This proof does not introduce generated protobuf Rust types.
- This proof does not implement protobuf wire encoding.
- This proof does not activate binary WebSocket frames.
- This proof does not grant external-agent trust or transport readiness.
