# Structured Output Record

Template: 1.0.0

Issue: 506

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented DRT-A distributed qualification contract surfaces for deterministic topology requirements, ACIP authority binding, replay/idempotency receipts, and fail-closed negative vectors without cloud/provider execution.

## Artifacts

- adl-runtime/src/qualification/mod.rs
- adl-runtime/tests/distributed_contract/main.rs
- adl-runtime/tests/distributed_contract/validate_drt_a.sh
- docs/milestones/v0.92.1/evidence/runtime/drt-a/qualification-contract.json

## Execution

- Added adl-runtime qualification contract types and deterministic validation helpers for DRT-A.
- Added focused integration tests that verify qualification denominator mapping, ACIP authority envelope binding, replay receipt stability, and fail-closed invalid vectors.
- Added the DRT-A qualification contract evidence packet under the v0.92.1 runtime evidence tree.
- Added the issue-owned validation script for the four DRT-A proof lanes.

## Validation

[
  {
    "command": [
      "bash",
      "adl-runtime/tests/distributed_contract/validate_drt_a.sh",
      "qualification-contract"
    ],
    "purpose": "Prove deterministic DRT-A requirement mapping and topology qualification.",
    "outcome": "passed",
    "evidence_ref": "qualification_contract test passed locally: 1 passed, 0 failed, 3 filtered out"
  },
  {
    "command": [
      "bash",
      "adl-runtime/tests/distributed_contract/validate_drt_a.sh",
      "acip-authority"
    ],
    "purpose": "Prove ACIP authority envelope binding for qualified participants.",
    "outcome": "passed",
    "evidence_ref": "acip_authority test passed locally: 1 passed, 0 failed, 3 filtered out"
  },
  {
    "command": [
      "bash",
      "adl-runtime/tests/distributed_contract/validate_drt_a.sh",
      "replay-conformance"
    ],
    "purpose": "Prove replay receipt stability and duplicate/reordered delivery classification.",
    "outcome": "passed",
    "evidence_ref": "replay_conformance test passed locally: 1 passed, 0 failed, 3 filtered out"
  },
  {
    "command": [
      "bash",
      "adl-runtime/tests/distributed_contract/validate_drt_a.sh",
      "negative-matrix"
    ],
    "purpose": "Prove fail-closed stale, duplicate, reordered, malformed, unsigned, wrong-domain, cross-Polis, and authority-mutation cases.",
    "outcome": "passed",
    "evidence_ref": "negative_matrix test passed locally: 1 passed, 0 failed, 3 filtered out"
  },
  {
    "command": [
      "git",
      "diff",
      "--check",
      "origin/main...HEAD"
    ],
    "purpose": "Verify exact-range diff hygiene before implementation review.",
    "outcome": "passed",
    "evidence_ref": "local command produced no output and exited 0"
  }
]

## Integration

not_started

## Publication

Publication: not_published

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
