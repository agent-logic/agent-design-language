# Structured Output Record

Template: 1.0.0

Issue: 90

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented a typed, audited recovery transaction that fills an absent code_repository only from the exact clean bound worktree and matching GitHub origin identities, while preserving review and publication authority.

## Artifacts

- csdlc-v2/src/migration.rs
- csdlc-v2/src/store.rs
- csdlc-v2/src/git.rs
- csdlc-v2/src/bin/csdlc-issue.rs
- csdlc-v2/src/schema.rs
- csdlc-v2/tests/code_repository_migration.rs
- csdlc-v2/operator/skills/csdlc-v2-init/SKILL.md

## Execution

- Added the versioned migration request, evidence, report, and csdlc-issue command.
- Added binding-lock then issue-lock authorization with exact CAS, phase, branch, worktree, cleanliness, and fetch/push origin checks.
- Added atomic record/card projection updates, credential-safe audit evidence, schema exposure, operator guidance, and focused regressions.

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--locked",
      "--test",
      "code_repository_migration"
    ],
    "purpose": "Prove all allowed phases, representative origin/topology/cleanliness guards, CAS, deterministic retry, credential-safe audit, schema/CLI exposure, and reviewed publication compatibility.",
    "outcome": "passed",
    "evidence_ref": "local FastWork exact-head run: 8 passed, 0 failed"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--locked",
      "--test",
      "gate6",
      "split_authority"
    ],
    "purpose": "Prove existing qualified split-authority publication linkage remains unchanged.",
    "outcome": "passed",
    "evidence_ref": "local FastWork run: 1 passed, 0 failed"
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
    "purpose": "Prove warning-free source and test integration after formatting verification.",
    "outcome": "passed",
    "evidence_ref": "local FastWork exact-head run: completed without warnings"
  },
  {
    "command": [
      "csdlc-install",
      "install",
      "--repo",
      "<issue-worktree>",
      "--destination",
      "/Volumes/FastWork/adl-install-smoke/issue-90/csdlc-v2"
    ],
    "purpose": "Prove the clean implementation installs into an isolated generation directory and installed csdlc-issue exposes migrate-code-repository --request.",
    "outcome": "passed",
    "evidence_ref": "/Volumes/FastWork/adl-install-smoke/issue-90/csdlc-v2/install-receipt.json; source_revision git:9041657e7ebcbc3e2bd2f1cdba3028d769e84dad"
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
