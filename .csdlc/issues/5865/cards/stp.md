# Structured Task Prompt

Template: 1.0.0

Issue: 5865

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Pin reviewed quinn, rustls, prost, and openraft versions in the child-owned manifest and lockfile, implement only the bounded authenticated QUIC/TLS adapter, and leave the OpenRaft authority-ledger source implementation to WP-04.07.

## Deliverables

- A bounded authenticated QUIC/TLS adapter using maintained quinn and rustls without custom cryptography or framing
- Canonical length-delimited protobuf messages using prost
- Reviewed quinn, rustls, prost, and openraft versions pinned together in adl-runtime/Cargo.toml and adl-runtime/Cargo.lock
- Focused positive and negative tests with dependency-lock parity
- Digest-bound execution proof
- Reviewed rollback evidence

## Acceptance

1. Implement only the declared exclusive paths
2. Preserve Guardian, authentication, bounds, determinism, durability, and redaction invariants
3. Run the exact named test with nonzero test enforcement
4. Prove applicable stale, replay, malformed, unauthorized, failure, and recovery cases
5. Bind all evidence to the exact source revision and artifact digests
6. Complete independent review and child-owned typed closeout

## Dependencies

- WP-04.02 issue #5864
- WP-04-IMP issue 5862
- Architecture/security gate issue 5821 terminal

## Inputs

- docs/milestones/v0.92/features/DISTRIBUTED_GUARDIAN_POLIS_v0.92.md
- .csdlc/prepared/issues/5821/design.md
- adl-runtime/src/guardian.rs
- adl-runtime/src/networking.rs
- adl-runtime/src/runtime_api.rs

## Non Goals

- Sibling WP-04 paths
- Runtime v2 fallback
- Custom cryptography or plaintext
- WP-14, consumer UI, or v0.93 work
- Self-attested completion
