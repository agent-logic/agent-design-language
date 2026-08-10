# Structured Output Record

Template: 1.0.0

Issue: 5825

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented the deterministic WP-08 birthday decision contract and its narrowly issue-specific exact-head native macOS/Linux receipt workflow, with fail-closed lifecycle, evidence, integrity, privacy, path, continuity, and claim boundaries.

## Artifacts

- adl-runtime-kernel/src/birthday.rs
- adl-runtime-kernel/tests/birthday.rs
- adl-runtime-kernel/tests/fixtures/birthday
- docs/milestones/v0.92/features/FIRST_TRUE_GODEL_AGENT_BIRTHDAY_v0.92.md
- adl-runtime-kernel/src/birthday.rs
- adl-runtime-kernel/src/lib.rs
- adl-runtime-kernel/tests/birthday.rs
- adl-runtime-kernel/tests/fixtures/birthday
- docs/milestones/v0.92/features/FIRST_TRUE_GODEL_AGENT_BIRTHDAY_v0.92.md
- .csdlc/prepared/issues/5825/produce-native-receipt.rb
- .csdlc/prepared/issues/5825/validate-native-receipts.rb
- .github/workflows/wp08-native-birthday.yml

## Execution

- Added the deterministic birthday candidate and decision contract.
- Added positive and table-driven negative fixtures and integration proof.
- Recorded truthful feature non-claims and corrected split-repository receipt validation.
- Added the deterministic birthday candidate and decision contract with positive and exhaustive table-driven negative proof.
- Added a WP-08-only GitHub Actions workflow that produces native macOS/Linux receipts and validates both fragments against the unchanged exact PR head.
- Made native nextest evidence machine-readable and stable through structured suite output while preserving complete logs and source manifests.
- Resolved repository-relative semantic output from the Runtime kernel manifest root and rejected parent traversal.
- Removed the unrelated global Runtime v3 runner-policy changes from PR #104.

## Validation

[
  {
    "command": [
      "cargo",
      "nextest",
      "run",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "birthday",
      "--no-tests=fail",
      "--status-level",
      "all"
    ],
    "purpose": "Prove the deterministic birthday contract and its positive and negative matrix.",
    "outcome": "passed",
    "evidence_ref": "birthday-runtime-v3.log"
  },
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/5825/validate-native-receipts.rb",
      ".csdlc/evidence/5825/native-platform/macos.json",
      ".csdlc/evidence/5825/native-platform/linux.json"
    ],
    "purpose": "Validate native macOS arm64 and Linux x86_64 receipt fragments, exact source SHA, six-test structured summaries, manifests, logs, producer provenance, and byte-identical semantic output.",
    "outcome": "passed",
    "evidence_ref": "https://github.com/agent-logic/agent-design-language/actions/runs/31348649171"
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
