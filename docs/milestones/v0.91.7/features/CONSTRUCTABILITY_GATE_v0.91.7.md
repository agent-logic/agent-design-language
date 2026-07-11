# Constructability Gate

## Metadata

- Feature Name: Constructability Gate
- Milestone Target: `v0.91.7`
- Status: bounded Runtime v2 validator implemented
- Owner: ADL maintainers
- Doc Role: primary
- Feature Types: architecture, policy, schema
- Proof Modes: schema, review, tests

## Purpose

Define and implement the gate that separates provisional cognition from
authoritative shared reality.

## Scope

In scope:

- construction-event schema;
- external-anchor schema;
- admissibility validator;
- shared-reality boundary;
- proof path for constructed claims.

Out of scope:

- universal truth adjudication;
- CSM supervisor hosting, which remains a WP-07A runtime-component concern;
- replacing human/operator review.

## Runtime Status

ADL Runtime v2 exposes the bounded validator through:

```text
adl runtime-v2 constructability-anchor-validator --input <packet.json> --out <decision.json>
```

The command accepts a caller-supplied
`runtime_v2.constructability_anchor_validator.v1` packet, validates it
fail-closed, and emits canonical validated output at the caller-selected
repository-relative path. Omitting `--input` emits the built-in bounded proof
fixture for inspection. The packet records construction events, admissible
anchors, one validator decision per event, shared-reality promotion
requirements, retained fail-closed cases, validation commands, and explicit
non-claims.

The implementation lives in
`adl/src/runtime_v2/constructability_anchor_validator.rs`. It is host-agnostic
Runtime v2 core logic: this issue does not claim that the validator is already
hosted by the WP-07A CSM component supervisor.

## Required Decisions

- Which additional runtime event families may become construction events?
- Which additional external anchor kinds are admissible?
- Which WP-07A CSM component should host the validator?
- Which additional constructability proofs are required before `v0.92`?

## Dependencies

- Curiosity Engine feature doc.
- Security implementation readiness.
- ACIP/A2A implementation decisions.

## Validation And Review

- Review schemas for determinism and evidence boundaries.
- Validate that provisional claims cannot become public truth without anchors.
- Require implemented proof or evidence-backed blocker status for missing validators.

Focused proof commands:

```text
cargo test --manifest-path adl/Cargo.toml runtime_v2_constructability_anchor_validator -- --nocapture
cargo test --manifest-path adl/Cargo.toml trace_runtime_v2_constructability_anchor_validator -- --nocapture
adl/target/debug/adl runtime-v2 constructability-anchor-validator --input .adl/local-artifacts/wp10-constructability/anchor-validator.json --out .adl/local-artifacts/wp10-constructability/validated-anchor-decision.json
```

## v0.92 Consumption

`v0.92` may consume Constructability only as a reviewed boundary and proven or
evidence-backed blocked surface. It must not present provisional cognition as authoritative shared
reality.

## Non-Goals

- No WP-07A CSM supervisor-hosting claim.
- No universal epistemic authority claim.
- No public truth claim without anchors.
