# Structured Output Record

Template: 1.0.0

Issue: 515

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented a local-only, non-authoritative provider shadow completion helper that preserves the authoritative provider output as the only accepted result, records shadow observations separately, keeps authority/shadow channels constructor-controlled, converts returned shadow errors and shadow panics into redacted observation metadata, suppresses raw shadow panic-hook payload leakage, and emits redacted digest/class comparison evidence.

## Artifacts

- adl/src/provider/mod.rs
- adl/tests/provider_shadow_isolation.rs
- adl/tests/provider_shadow_comparison.rs
- adl/tests/provider_shadow_fallback.rs
- docs/milestones/v0.92.1/evidence/provider/prov-b/local-model-shadow-comparison.json

## Execution

- Added distinct provider shadow authority/shadow channel types and redacted comparison records in adl/src/provider/mod.rs.
- Added complete_with_local_model_shadow so authority executes first and shadow success, returned failure, or panic cannot replace or mask authoritative output.
- Kept authoritative and shadow channel markers constructor-controlled with read-only accessors so callers cannot construct shadow observations as authoritative results.
- Added a scoped shadow panic-hook guard so raw shadow panic payloads do not leak through stderr/log hooks before redacted observation handling restores the previous hook.
- Added issue-owned integration tests for shadow isolation, deterministic comparison, returned-error fallback, authoritative-first fallback, panicking-shadow fallback behavior, and panic-hook payload suppression.
- Added redacted PROV-B evidence under docs/milestones/v0.92.1/evidence/provider/prov-b/.

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl/Cargo.toml",
      "-p",
      "adl",
      "--test",
      "provider_shadow_isolation"
    ],
    "purpose": "Prove authority and shadow paths are distinguishable and shadow output cannot mutate authoritative state.",
    "outcome": "passed",
    "evidence_ref": "terminal:provider_shadow_isolation:1 passed"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl/Cargo.toml",
      "-p",
      "adl",
      "--test",
      "provider_shadow_comparison"
    ],
    "purpose": "Prove exact deterministic comparison inputs and rule set for authority-versus-shadow observations.",
    "outcome": "passed",
    "evidence_ref": "terminal:provider_shadow_comparison:1 passed"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl/Cargo.toml",
      "-p",
      "adl",
      "--test",
      "provider_shadow_fallback"
    ],
    "purpose": "Prove shadow returned errors, panics, and suppressed panic-hook payloads preserve the authoritative result and do not convert shadow success into authority.",
    "outcome": "passed",
    "evidence_ref": "terminal:provider_shadow_fallback:3 passed"
  },
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/515/validate-provider-shadow-redaction.sh"
    ],
    "purpose": "Prove PROV-B evidence excludes common credential, private payload, prompt, and host-local path markers.",
    "outcome": "passed",
    "evidence_ref": "docs/milestones/v0.92.1/evidence/provider/prov-b/local-model-shadow-comparison.json"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Prove exact branch diff hygiene before review.",
    "outcome": "passed",
    "evidence_ref": "terminal:git diff --check"
  },
  {
    "command": [
      "cargo",
      "fmt",
      "--manifest-path",
      "adl/Cargo.toml",
      "--check"
    ],
    "purpose": "Prove Rust formatting for the touched provider and test surfaces.",
    "outcome": "passed",
    "evidence_ref": "terminal:cargo fmt --check"
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
