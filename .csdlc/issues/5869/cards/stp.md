# Structured Task Prompt

Template: 1.0.0

Issue: 5869

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Implement monotonic epochs and bounded leases as prerequisites for distributed ownership decisions.

## Deliverables

- OpenRaft authority and leases whose endorsements use a stable majority or both constituent majorities during joint membership
- A negative test proving a union majority without either constituent majority is rejected
- AuthorityCertificateV1 using the fixed protobuf tag and wire-type table, closed operation classes, exact identity bytes, and unsigned lexicographic signer ordering
- Ed25519 verification using ed25519-dalek VerifyingKey::verify_strict, 32-byte public keys, 64-byte R || S signatures, the exact ADL-AUTHORITY-CERTIFICATE-V1\0 domain, SHA-256, and deterministic prost encoding
- Negative tests for wrong algorithms, malformed key/signature lengths, unknown or duplicate fields, non-minimal varints, noncanonical scalar or point encodings, unsorted or duplicate signers, decode/re-encode mismatch, and noncanonical field ordering
- Digest-bound execution proof and reviewed rollback evidence
- Authority recovery only from a quorum-proven committed prefix; local durability, a leader assertion, or a minority history never grants authority

## Acceptance

1. Implement only the declared exclusive paths
2. Preserve Guardian, authentication, bounds, determinism, durability, and redaction invariants
3. Run the exact named test with nonzero test enforcement
4. Prove applicable stale, replay, malformed, unauthorized, failure, and recovery cases
5. Bind all evidence to the exact source revision and artifact digests
6. Complete independent review and child-owned typed closeout

## Dependencies

- WP-04.05 issue #5867
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
