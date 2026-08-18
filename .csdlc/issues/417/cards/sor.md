# Structured Output Record

Template: 1.0.0

Issue: 417

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Permit implemented authored-design refresh after bounded recovery repairs and typed design-review recovery while preserving the originating review-recovery epoch and cleared downstream authority.

## Artifacts

- csdlc-v2/src/store.rs
- csdlc-v2/tests/gate5.rs
- .csdlc/issues/417
- .csdlc/prepared/issues/417

## Execution

- Add a refresh-specific implemented recovery-epoch predicate without widening the shared card-repair classifier.
- Accept the exact recover_review, supported repair, recover_design_review, authored refresh sequence while retaining immediate and iterative compatibility.
- Record the originating recovery sequence and generation in authored-refresh audit evidence.
- Add exact public-operation success, authority-clear, provenance, compatibility, and unlisted-operation regressions.

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--lib"
    ],
    "purpose": "Run the C-SDLC v2 library test suite.",
    "outcome": "passed",
    "evidence_ref": "csdlc-v2-library-regression.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate5",
      "implemented_design_refresh"
    ],
    "purpose": "Run focused issue #417 gate5 regressions.",
    "outcome": "passed",
    "evidence_ref": "implemented-design-refresh-recovery-focused.log"
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
