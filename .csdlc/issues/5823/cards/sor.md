# Structured Output Record

Template: 1.0.0

Issue: 5823

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Repaired the provider-neutral remote-validation contract after independent review. AWS and Nessus now preserve the complete portable request in canonical results; AWS enforces nonzero and projected cost bounds, instance CPU/memory capacity, cancellation, and pre-emission redaction; Nessus enforces revision, capacity, timeout, and cancellation; retained platform receipts are opened, hashed, cross-checked, and tied to truthful operator authorization. The prior authorized paid AWS run remains the live Linux proof; no second paid run was needed because the changed enforcement is covered by deterministic focused tests.

## Artifacts

- tools/remote_validation
- tools/aws_remote_validation/src/aws_remote_validation.rs
- tools/aws_remote_validation/src/bin/adl_aws_remote_validation.rs
- tools/aws_remote_validation/tests/portable_adapter.rs
- adl/tools/run_aws_spot_remote_validation_lane.sh
- adl/tools/test_run_aws_spot_remote_validation_lane.sh
- adl/tools/run_nessus_remote_validation.sh
- adl/tools/test_run_nessus_remote_validation.sh
- .csdlc/prepared/issues/5823/validate-platform-matrix.rb
- .csdlc/evidence/5823

## Execution

- Preserved resource budget, artifact policy, cancellation file, fallback policy, and command-profile digest through adapter plans and canonical portable results.
- Added fail-closed AWS cost-ceiling, projected-cost, instance-capacity, cancellation, and stdout/stderr redaction enforcement with negative tests.
- Added Nessus exact-revision, CPU/memory, timeout, cancellation, artifact hashing, and canonical result enforcement with focused shell tests.
- Strengthened platform evidence validation to open receipts, verify SHA-256 and artifact bytes, compare revisions and profile digests, and validate budget, redaction, cleanup, first-attempt disposition, and operator authorization.
- Reconciled retained portable result schemas and fixed all bounded diff whitespace findings without another paid provider run.

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "tools/remote_validation/Cargo.toml",
      "--test",
      "contract"
    ],
    "purpose": "Prove complete portable request/result preservation, canonical artifact hashing, exact revision, timeout, cancellation, cleanup, redaction, and fallback semantics.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5823/final-portable-contract.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "tools/aws_remote_validation/Cargo.toml",
      "--test",
      "portable_adapter"
    ],
    "purpose": "Prove AWS portable mapping, redaction before retention, provider-failure classification, and cleanup behavior.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5823/final-aws-adapter.log"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_run_aws_spot_remote_validation_lane.sh"
    ],
    "purpose": "Prove projected-cost, CPU/memory capacity, cancellation, exact revision, and wrapper redaction controls without provider use.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5823/final-aws-shell.log"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_run_nessus_remote_validation.sh"
    ],
    "purpose": "Prove Nessus complete portable mapping, exact revision, capacity, timeout, cancellation, artifact hashing, and canonical result behavior.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5823/final-nessus-shell.log"
  },
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/5823/validate-platform-matrix.rb"
    ],
    "purpose": "Verify retained receipt SHA-256, artifact digests, revisions, profile digests, authorization, budget, redaction, cleanup, and first-attempt disposition.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5823/final-platform-matrix.log"
  },
  {
    "command": [
      "git",
      "diff",
      "--check",
      "origin/main"
    ],
    "purpose": "Reject whitespace errors across the complete issue diff.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5823/final-diff-hygiene.log"
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
