# Structured Output Record

Template: 1.0.0

Issue: 273

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented the bounded Shepherd serving-eligibility authority over a non-forgeable verified #272 foundation cut, with deterministic acquire, atomic replace, revoke, expiry, exact retry/restart, bounded capacity, receipts, and redacted projection.

## Artifacts

- adl-runtime/src/distributed/serving_authority.rs
- adl-runtime/src/distributed/shepherd_serving_eligibility.rs
- adl-runtime/src/distributed/mod.rs
- adl-runtime/tests/distributed_shepherd_serving_eligibility.rs
- .csdlc/evidence/273

## Execution

- Added a non-policy opaque VerifiedServingAuthorityCut returned only after existing sealed receipt/binding verification and durable Published commit.
- Added the Shepherd-only checkpointed eligibility store with fenced acquire, replace, revoke, expiry, retry, restart, and capacity behavior.
- Registered only the Shepherd module and added six focused deterministic tests; #274 and #205 remain untouched.

## Validation

[
  {
    "command": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_shepherd_serving_eligibility",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Run strict Clippy for the focused target.",
    "outcome": "passed",
    "evidence_ref": "issue273-clippy.log"
  },
  {
    "command": [
      "git",
      "diff",
      "--check",
      "origin/main...HEAD"
    ],
    "purpose": "Run exact candidate diff hygiene.",
    "outcome": "passed",
    "evidence_ref": "issue273-diff.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_shepherd_serving_eligibility",
      "--",
      "--test-threads=1"
    ],
    "purpose": "Run the exact issue-owned integration target.",
    "outcome": "passed",
    "evidence_ref": "issue273-focused.log"
  },
  {
    "command": [
      "python3",
      ".csdlc/prepared/issues/273/validate_scope.py"
    ],
    "purpose": "Run fail-closed issue scope proof.",
    "outcome": "passed",
    "evidence_ref": "issue273-scope.log"
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
