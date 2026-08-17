# Structured Output Record

Template: 1.0.0

Issue: 400

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

#400 implemented a narrow typed C-SDLC v2 recovery route for implemented-phase SPP plan-step status truth and STP dependency truth after recorded review recovery, without weakening review, publication, or terminal guardrails. Review findings were resolved by making SPP plan-step recovery status-only and retaining/recording full gate5 proof.

## Artifacts

- csdlc-v2/src/cards.rs
- csdlc-v2/src/store.rs
- csdlc-v2/tests/gate5.rs
- .csdlc/issues/400
- .csdlc/evidence/400

## Execution

- Added typed semantic operation correct_stp_dependencies_after_recovery for STP dependency denominator repairs.
- Added typed semantic operation correct_plan_steps_after_recovery for SPP execution-step status repairs.
- Gated both operations on implemented phase plus current recorded-review recovery provenance with cleared review/publication/readiness/terminal truth.
- Recorded audit snapshots for previous/new STP dependencies or SPP steps and the associated recovery sequence/generation.
- Added focused gate5 regression coverage for #117-style STP dependency repairs, SPP step repairs, guardrail failures, and public schema exposure.
- Retained full gate5 regression proof under .csdlc/evidence/400/gate5-full.log after review finding 400-r2-p2-full-gate5-proof-not-retained.
- Restricted correct_plan_steps_after_recovery to status-only changes: step IDs, actions, acceptance IDs, and cardinality must match prior SPP plan truth.

## Validation

[
  {
    "command": [
      "cargo",
      "fmt",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--check"
    ],
    "purpose": "Formatting proof for touched C-SDLC v2 Rust files.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/400/cargo-fmt-check.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate5",
      "recovered_implemented_issue_can_correct",
      "--",
      "--nocapture"
    ],
    "purpose": "Focused proof for implemented-phase STP dependency and SPP step recovery behavior.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/400/recovery-focused-tests.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate5",
      "public_edit_schema_exposes_implemented_recovery_card_repairs",
      "--",
      "--nocapture"
    ],
    "purpose": "Schema exposure proof for the new typed recovery operations.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/400/schema-focused-test.log"
  },
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
    "purpose": "Strict lint proof for touched C-SDLC v2 targets.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/400/strict-clippy.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate5",
      "--",
      "--nocapture"
    ],
    "purpose": "Full gate5 regression proof after #400 changes; retained result: 64 passed, 0 failed.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/400/gate5-full.log"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Whitespace and patch hygiene proof.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/400/diff-hygiene.log"
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
