# Structured Output Record

Template: 1.0.0

Issue: 239

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented root-aware terminal envelope validation for governed publication metadata-only heads with fail-closed regression coverage.

## Artifacts

- csdlc-v2/src/finish.rs
- csdlc-v2/src/bin/csdlc-finish.rs
- csdlc-v2/src/cleanup.rs
- csdlc-v2/tests/gate_finish.rs
- csdlc-v2/src/finish.rs
- csdlc-v2/src/bin/csdlc-finish.rs
- csdlc-v2/src/cleanup.rs
- csdlc-v2/tests/gate_finish.rs
- .csdlc/evidence/239

## Execution

- Added a root-aware envelope matcher while retaining exact-only compatibility behavior.
- Routed cached csdlc-finish validation and cleanup compatibility through the root-aware matcher.
- Reused git::metadata_only_changed_paths in publication lineage validation, preserving forward ancestry, safe path, per-commit, and typed card projection checks.
- Expanded the PR #238-shaped regression to cover positive metadata-only reconciliation plus rename drift, substantive drift, malformed publication revision, non-ancestor metadata, and missing commit rejection.
- Added root-aware matcher and retained exact-only compatibility matcher.
- Routed cached finish validation and cleanup compatibility through explicit repository root.
- Reused governed metadata-only ancestry/path/projection policy.
- Added PR #238-shaped positive and substantive, malformed, non-ancestor, missing-commit negatives.

## Validation

[
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Run diff whitespace validation.",
    "outcome": "passed",
    "evidence_ref": "diff-hygiene.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate_finish",
      "derived_terminal_accepts_publication_metadata_only_head_and_rejects_substantive_drift",
      "--",
      "--exact"
    ],
    "purpose": "Run the exact focused gate_finish regression.",
    "outcome": "passed",
    "evidence_ref": "terminal-envelope-metadata-head.log"
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
