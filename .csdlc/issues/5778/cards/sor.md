# Structured Output Record

Template: 1.0.0

Issue: 5778

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Added one idempotent C-SDLC v2 finish command that preserves exact-head gates, derives terminal authority from live GitHub state, and eliminates tracked post-merge closeout writes.

## Artifacts

- csdlc-v2/src/finish.rs
- csdlc-v2/src/bin/csdlc-finish.rs
- csdlc-v2/tests/gate_finish.rs
- csdlc-v2/operator/skills/csdlc-v2-finish/SKILL.md
- .csdlc/evidence/5778/post-finalize-remediation.md

## Execution

- Added typed csdlc-finish request, result, derived-terminal envelope, and operator route.
- Reused the exact-head merge primitive and restricted review approval to the current PR head.
- Added a minimal serialized and symlink-safe Git-common terminal cache that logically releases stale claims without becoming canonical lifecycle state.
- Kept legacy csdlc-closeout available only for migration and repair while changing the default workflow to finish.

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--locked",
      "--lib",
      "--test",
      "gate_finish",
      "--test",
      "gate7_lifecycle",
      "--test",
      "gate10a",
      "--test",
      "gate10b",
      "--test",
      "gate_github_actions"
    ],
    "purpose": "Prove exact-head review normalization, terminal derivation, concurrency, stale-claim release, legacy lifecycle compatibility, installation, and GitHub action behavior.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5778/post-finalize-remediation.md"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--locked",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Prove warning-free C-SDLC v2 code across all targets.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5778/post-finalize-remediation.md"
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
