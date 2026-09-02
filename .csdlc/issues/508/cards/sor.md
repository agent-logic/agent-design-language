# Structured Output Record

Template: 1.0.0

Issue: 508

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented and remediated the DRT-C final distributed Runtime qualification decision. The retained qualification evidence now binds to the immutable Runtime qualification source revision, models both required #187 soak windows with exact command/model/receipt digests, independent replay, clock bounds, and cleanup readback, and keeps the decision within #508 without absorbing #509/GCP or Observatory product redesign scope.

## Artifacts

- adl-runtime/Cargo.toml
- adl-runtime/src/qualification/mod.rs
- adl-runtime/tests/distributed_failure/drt_c_qualification.rs
- docs/milestones/v0.92.1/evidence/runtime/drt-c/qualification.json
- .csdlc/prepared/issues/508/validate-readiness.rb
- .csdlc/prepared/issues/508/validate-implementation.rb
- .csdlc/evidence/508/drt-c-rust-fmt-r2.log
- .csdlc/evidence/508/drt-c-focused-r2.log
- .csdlc/evidence/508/drt-c-implementation-validator-r2.log
- .csdlc/evidence/508/drt-c-diff-check-r2.log
- .csdlc/issues/508
- .csdlc/prepared/issues/508

## Execution

- Added a deterministic DRT-C qualification decision model and validator to the Runtime qualification module.
- Added a focused DRT-C integration test under the distributed_failure surface that proves requirements #185-#187, fail-closed identity/provider/transport cases, Runtime-authentic redacted Observatory evidence, bounded soak, cleanup-zero, and retained evidence equality.
- Removed the stale hardcoded parent #507 SHA from the focused test; the test now derives the qualification subject from retained evidence.
- Strengthened #187 soak evidence from a single positive duration into two required windows with source revision, command digest, model digest, clock bounds, receipt digest, independent replay, and cleanup readback.
- Strengthened the issue-owned implementation validator so retained evidence fails closed unless runtime_revision equals the latest Runtime qualification source commit and the full soak denominator is present.
- Retained the exact DRT-C qualification decision artifact under docs/milestones/v0.92.1/evidence/runtime/drt-c/qualification.json.

## Validation

[
  {
    "command": [
      "cargo",
      "fmt",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--check"
    ],
    "purpose": "Rust formatting check for the DRT-C Runtime qualification model and focused test.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/508/drt-c-rust-fmt-r2.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_failure_drt_c",
      "drt_c_final_qualification_decision_is_exact",
      "--",
      "--nocapture"
    ],
    "purpose": "Prove the deterministic DRT-C Runtime qualification decision validates and matches retained evidence exactly, including the Runtime source revision and #187 soak denominator.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/508/drt-c-focused-r2.log"
  },
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/508/validate-implementation.rb"
    ],
    "purpose": "Prove retained DRT-C qualification JSON binds exact Runtime source revision, fail-closed cases, Runtime-authentic Observatory evidence, two-window bounded soak denominator, synthesis, and cleanup-zero.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/508/drt-c-implementation-validator-r2.log"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Reject whitespace and conflict-marker residue across the #508 remediation diff.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/508/drt-c-diff-check-r2.log"
  }
]

## Integration

worktree_only

## Publication

Publication: not_published

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
