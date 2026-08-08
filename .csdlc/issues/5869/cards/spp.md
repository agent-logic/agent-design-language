# Structured Planning Prompt

Template: 1.0.0

Issue: 5869

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Verify gates, implement the exclusive slice, run exact proving tests and negatives, validate rollback, resolve review, and close through child authority.

## Plan

Revision 6

## Steps

[
  {
    "id": "S1",
    "action": "Verify #5821 terminal ancestry, dependency receipts, exact paths, and source contracts.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement the bounded WP-04.07 outcome in the exclusive paths.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run exact positive, negative, failure, recovery, and receipt validation.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Resolve independent review and complete child-owned publication and closeout.",
    "acceptance_ids": [
      "AC-6"
    ],
    "status": "pending"
  }
]

## Invariants

- Exclusive paths remain disjoint; Guardian stays process 0; queues and waits remain bounded; no insecure or Runtime v2 fallback is permitted
- Evidence is exact-revision and digest bound
- Joint membership requires separate strict majorities of the old and new voter sets; a union majority alone never grants authority
- AuthorityCertificateV1 uses frozen protobuf tags and wire types, closed operation classes, exact identity bytes, unsigned lexicographic signer ordering, and hashes canonical body bytes under ADL-AUTHORITY-CERTIFICATE-BODY-V1\0
- AuthorityEndorsementPayloadV1 signs the body digest, exact signer identity, certificate generation, and algorithm under ADL-AUTHORITY-ENDORSEMENT-V1\0
- Enrollment and verification reject duplicate effective control public keys, while quorum counting deduplicates both signer identity and control key
- Authority certificates accept only frozen Ed25519 encodings, SHA-256, deterministic prost encoding, byte-equal decode/re-encode, and VerifyingKey::verify_strict

## Risks

- Dependency contract drift
- Cross-child path overlap
- False-green zero-test selection
- Self-attested platform or recovery evidence

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/5869/design.md

Digest: 132f5437e67cb71961df3c6cb1b88fed79e68d88ac39b44d8f421f630d468125

## Diagram

.csdlc/prepared/issues/5869/diagram.mmd

Digest: b6214f3f6d8281d9eabcf54b42092e69517d8f3cbb616f27313f0b7265a97d07

## Stop Conditions

- #5821 is not terminal
- A dependency is not terminal
- Any declared path overlaps an active claim
- The exact test target is absent or selects zero tests
- Scope or rollback authority must widen

## Handoff

Proceed only after doctor readiness.
