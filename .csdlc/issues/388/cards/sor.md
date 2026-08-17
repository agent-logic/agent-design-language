# Structured Output Record

Template: 1.0.0

Issue: 388

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented bounded C-SDLC v2 implemented-phase SPP/VPP/SOR card-truth repair routes.

## Artifacts

- csdlc-v2/src/cards.rs
- csdlc-v2/src/store.rs
- csdlc-v2/tests/gate5.rs
- .csdlc/prepared/issues/388/validate_preparation_bundle.py
- .csdlc/evidence/388

## Execution

- Added typed semantic card operations for implemented-phase VPP summary repair, VPP failure-policy repair, and SOR follow-up replacement/removal after current review recovery.
- Relaxed SPP plan-summary repair to work after assignment/review recovery before publication while preserving actor/reason, active downstream truth, and duplicate-repair guards.
- Added recovery-epoch duplicate-field refusal and append-only audit payloads for the SPP/VPP/SOR repair operations.
- Added focused gate5 regressions proving SPP repair after assignment recovery, VPP/SOR repair guards, SOR empty-vector follow-up removal, blank-entry refusal, and compatibility with existing allowed intervening repairs.
- Repaired #388 lifecycle proof truth to use the real gate5 integration-test denominator instead of a zero-test --lib filter and restored the issue-owned preparation validator target in the bound worktree.

## Validation

[
  {
    "command": [
      "cargo",
      "clippy",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Run strict Clippy for all csdlc-v2 targets.",
    "outcome": "passed",
    "evidence_ref": "388-clippy.log"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Run git diff hygiene check.",
    "outcome": "passed",
    "evidence_ref": "388-diff.log"
  },
  {
    "command": [
      "/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-doctor",
      "--repo",
      "/Volumes/FastWork/adl-worktrees/adl-issue-388-implemented-card-truth-repair",
      "--issue",
      "388"
    ],
    "purpose": "Run C-SDLC v2 doctor for #388.",
    "outcome": "passed",
    "evidence_ref": "388-doctor.log"
  },
  {
    "command": [
      "cargo",
      "fmt",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--check"
    ],
    "purpose": "Run rustfmt check for csdlc-v2.",
    "outcome": "passed",
    "evidence_ref": "388-fmt.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate5",
      "implemented_card_truth_repair"
    ],
    "purpose": "Run the focused #388 C-SDLC v2 gate5 integration-test regressions.",
    "outcome": "passed",
    "evidence_ref": "388-focused-csdlc-store.log"
  },
  {
    "command": [
      "/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-validate",
      "--root",
      "/Volumes/FastWork/adl-worktrees/adl-issue-388-implemented-card-truth-repair",
      "issue",
      "--issue",
      "388"
    ],
    "purpose": "Run C-SDLC v2 typed issue validation for #388.",
    "outcome": "passed",
    "evidence_ref": "388-typed-validate.log"
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
