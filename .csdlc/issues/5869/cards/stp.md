# Structured Task Prompt

Template: 1.0.0

Issue: 5869

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Implement monotonic epochs and bounded leases as prerequisites for distributed ownership decisions.

## Deliverables

- OpenRaft authority whose endorsements use a stable majority or both constituent majorities during joint membership, with a negative union-majority test
- AuthorityCertificateV1 using the fixed protobuf tag and wire-type table, closed operation classes, exact identity bytes, unsigned lexicographic signer ordering, and canonical body hashing under ADL-AUTHORITY-CERTIFICATE-BODY-V1\0
- AuthorityEndorsementPayloadV1 signing the body digest, signer Guardian identity, certificate generation, and algorithm under ADL-AUTHORITY-ENDORSEMENT-V1\0
- Ed25519 verification using ed25519-dalek VerifyingKey::verify_strict, 32-byte public keys, 64-byte R || S signatures, SHA-256, and deterministic prost encoding
- Negative tests for copied signatures under another identity or generation, stale generations, duplicate signer identities or effective control keys, wrong algorithms, malformed lengths, unknown or duplicate fields, non-minimal varints, noncanonical scalar or point encodings, unsorted signers, and decode/re-encode mismatch
- Digest-bound execution proof, reviewed rollback evidence, and quorum-only authority recovery

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
