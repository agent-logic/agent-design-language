# Structured Output Record

Template: 1.0.0

Issue: 5913

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Repair #5913 adl-review read-only compatibility routing for verify-repo-contract and deterministic fixture code-review without provider credentials or v1 lifecycle resurrection.

## Artifacts

- adl/src/cli/mod.rs
- adl/tools/test_adl_review_compatibility.sh

## Execution

- Route adl-review verify-repo-contract to a direct markdown repository-review contract verifier instead of the removed v1 tooling multiplexer.
- Route adl-review code-review fixture mode to the deterministic CodeBuddy/CodeFriend showcase smoke path and validator, rejecting provider-backed backends for this bounded issue.
- Update the focused compatibility regression so it no longer uses adl tooling as an oracle and covers good/bad contract packets plus deterministic fixture smoke.

## Validation

[
  {
    "command": [
      "bash",
      "adl/tools/test_adl_review_compatibility.sh"
    ],
    "purpose": "Prove the repaired adl-review read-only compatibility surface and fail-closed command boundaries",
    "outcome": "passed",
    "evidence_ref": "adl-review-compatibility.log"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--manifest-path",
      "adl/Cargo.toml",
      "--bin",
      "adl-review",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Prove the repaired adl-review Rust dispatch compiles cleanly under strict Clippy",
    "outcome": "passed",
    "evidence_ref": "strict-clippy-adl-review.log"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_adl_review_compatibility.sh"
    ],
    "purpose": "Focused compatibility regression after stale-help finding fix",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5913/review-finding-fix-compatibility.log"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--manifest-path",
      "adl/Cargo.toml",
      "--bin",
      "adl-review",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Strict relevant Rust lint after stale-help finding fix",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5913/review-finding-fix-clippy.log"
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
