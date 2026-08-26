# Structured Output Record

Template: 1.0.0

Issue: 291

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented typed initialized-phase post-decomposition card recovery in csdlc-edit, including transaction journaling, generic graph validation, #114 golden fixture proof, and legacy terminal-census compatibility needed for the full csdlc-v2 denominator.

## Artifacts

- csdlc-v2/src/store.rs
- csdlc-v2/src/bin/csdlc-edit.rs
- csdlc-v2/src/lib.rs
- csdlc-v2/src/schema.rs
- csdlc-v2/src/cleanup.rs
- csdlc-v2/tests/initialized_decomposition_recovery.rs
- .csdlc/evidence/291/bind-staging-omission-candidate.json
- .csdlc/evidence/291/review-r2-findings.json
- .csdlc/evidence/291/review-r2-historical-findings.json
- .csdlc/evidence/291/review-r4-findings.json

## Execution

- Added csdlc-edit recover-initialized-decomposition typed request/result/schema surfaces.
- Implemented initialized-only CAS recovery with preserved design/diagram evidence checks, generic typed decomposition graph validation, closed replacement fields, review-authority recovery truth, and write-ahead journal roll-forward semantics.
- Added focused regression coverage including crash recovery, stale CAS, and read-only #114 generation 35 golden fixture copy proof.
- Adjusted legacy terminal-census cleanup indexing to compare retained historical receipts to projections without retroactively requiring modern terminal card completion shape.

## Validation

[
  {
    "command": [
      "/usr/bin/env",
      "CARGO_TARGET_DIR=/Volumes/FastWork/adl-worktrees/adl-issue-291-initialized-decomposition-recovery/csdlc-v2/target",
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Run strict Clippy across all csdlc-v2 targets.",
    "outcome": "passed",
    "evidence_ref": "csdlc-v2-clippy.log"
  },
  {
    "command": [
      "/usr/bin/env",
      "CARGO_TARGET_DIR=/Volumes/FastWork/adl-worktrees/adl-issue-291-initialized-decomposition-recovery/csdlc-v2/target",
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml"
    ],
    "purpose": "Run the full csdlc-v2 Cargo test denominator before PVF writes any issue evidence logs that would trip dirty-source guard tests.",
    "outcome": "passed",
    "evidence_ref": "csdlc-v2-full.log"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Reject whitespace errors and support exact-head review.",
    "outcome": "passed",
    "evidence_ref": "diff-hygiene.log"
  },
  {
    "command": [
      "/usr/bin/env",
      "ADL_CSDLC_291_GOLDEN_114_ROOT=/Volumes/FastWork/adl-worktrees/adl-issue-114-durable-history-preparation",
      "CARGO_TARGET_DIR=/Volumes/FastWork/adl-worktrees/adl-issue-291-initialized-decomposition-recovery/csdlc-v2/target",
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "initialized_decomposition_recovery"
    ],
    "purpose": "Run focused initialized recovery tests, including live read-only #114 generation 35 golden fixture copy proof.",
    "outcome": "passed",
    "evidence_ref": "initialized-decomposition-recovery-focused.log"
  }
]

## Integration

pr_open

## Publication

Publication: ready

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
