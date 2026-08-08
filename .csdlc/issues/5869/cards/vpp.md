# Validation Planning Prompt

Template: 1.0.0

Issue: 5869

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5869/design.md

Diagram: .csdlc/prepared/issues/5869/diagram.mmd

## Selected Lanes

[
  {
    "lane": "exact-child-tests",
    "proof_role": "Exact nextest target distributed_lease proves stable-majority and joint old-plus-new majority endorsement; rejects a union majority lacking either constituent majority; enforces the fixed AuthorityCertificateBodyV1 and AuthorityEndorsementV1 protobuf tag and wire-type table, closed operation classes, exact identity bytes, unsigned lexicographic unique signer ordering, canonical decode/re-encode byte equality, Ed25519 algorithm identity, 32-byte public keys, 64-byte R || S signatures, ADL-AUTHORITY-CERTIFICATE-V1\\0 domain separation, SHA-256, and VerifyingKey::verify_strict; rejects wrong algorithms, malformed lengths, unknown or duplicate fields, non-minimal varints, unsorted or duplicate signers, noncanonical scalar or point encodings, and byte-mismatched re-encoding; and proves activation possession, applied-index checks, monotonic epochs, renewal, expiry, revocation, quorum loss, malicious-leader denial, clock uncertainty, stale-holder denial, and restart recovery.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 1800,
    "budget_tokens": 8000,
    "argv": [
      "cargo",
      "nextest",
      "run",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_lease",
      "--no-tests=fail"
    ],
    "parallel_group": "child",
    "defer_reason": null
  },
  {
    "lane": "exact-revision-proof-receipt",
    "proof_role": "Recompute source, command, nonzero test, artifact, negative-case, and native receipt bindings. [preexec_rejection exit=1 diagnostic_sha256=67c48658cb585b04994978da823640a32be4602799401649195121fd796ce598]",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 3000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5869/validate-proof-receipt.rb"
    ],
    "parallel_group": "receipt",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `cargo nextest run --manifest-path adl-runtime/Cargo.toml --test distributed_lease --no-tests=fail`
- `ruby .csdlc/prepared/issues/5869/validate-proof-receipt.rb`

## Failure Semantics

Fail closed on stale dependencies, path overlap, zero tests, invalid evidence, insecure fallback, or unresolved review findings.

## Handoff

Retain typed evidence before convergence.
