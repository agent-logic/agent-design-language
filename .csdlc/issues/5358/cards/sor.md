# Structured Output Record

Template: 1.0.0

Issue: 5358

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented fail-closed ready-publication validation and deterministic recovery after ambiguous remote success or local CAS failure.

## Artifacts

- csdlc-v2/src/publication.rs
- csdlc-v2/src/bin/csdlc-publish.rs
- csdlc-v2/src/lib.rs
- csdlc-v2/tests/gate7_lifecycle.rs

## Execution

- Validate remote open/draft state and exact base/head repository, ref, and SHA before and after mark-ready.
- Record ready state only from confirmed remote observations and support typed ready reconciliation from reviewed or governed draft state.
- Add command-level loopback HTTP tests for success, identity drift, closed/non-draft state, remote failures, ambiguous confirmation, and CAS recovery.

## Validation

[
  {
    "command": [
      "csdlc-validate --request .csdlc/prepared/issues/5358/validation-request.json",
      "csdlc-doctor --repo . --issue 5358",
      "git diff --check"
    ],
    "purpose": "Validate the typed #5358 six-card bundle, retained design integrity, bound issue state, and issue-local patch hygiene only.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5358/preparation-validation"
  },
  {
    "command": [
      "csdlc-validate --request .csdlc/prepared/issues/5358/validation-request.json",
      "csdlc-doctor --repo . --issue 5358",
      "git status --short -- .csdlc/issues/5358 .csdlc/prepared/issues/5358 .csdlc/evidence/5358"
    ],
    "purpose": "Validate typed record/card integrity and explicitly enumerate the full untracked #5358 review scope. This replaces the earlier git diff --check evidence, which did not cover untracked artifacts.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5358/preparation-validation"
  },
  {
    "command": [
      "CARGO_TARGET_DIR=/Volumes/FastWork/adl-5358/csdlc-v2-test-target cargo test --manifest-path csdlc-v2/Cargo.toml --test gate7_lifecycle",
      "CARGO_TARGET_DIR=/Volumes/FastWork/adl-5358/csdlc-v2-clippy-target cargo clippy --manifest-path csdlc-v2/Cargo.toml --all-targets -- -D warnings",
      "cargo fmt --manifest-path csdlc-v2/Cargo.toml -- --check"
    ],
    "purpose": "Prove the ready-publication command matrix, recovery/CAS invariants, warning-free all-target compilation, and canonical formatting.",
    "outcome": "passed",
    "evidence_ref": "local-fastwork:gate7-21-pass-clippy-fmt"
  },
  {
    "command": [
      "CARGO_TARGET_DIR=/Volumes/FastWork/adl-5358/csdlc-v2-test-target cargo test --locked --manifest-path csdlc-v2/Cargo.toml --all-targets"
    ],
    "purpose": "Prove the complete C-SDLC v2 test surface, including clean-source installer and provenance contracts, after committing the publication repair implementation.",
    "outcome": "passed",
    "evidence_ref": "local-fastwork:csdlc-v2-all-targets-pass-167e8b9b4"
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
