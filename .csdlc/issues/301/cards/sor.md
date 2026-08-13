# Structured Output Record

Template: 1.0.0

Issue: 301

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Recovered stale #301 publication after origin/main advanced to 193f77d24a693f955a2fcf3bdfc759ad1db8aff4, merged current main into codex/301-title-only-operation-provenance without manual conflicts, and preserved title-only GitHub issue update provenance behavior.

## Artifacts

- csdlc-v2/src/github.rs
- csdlc-v2/tests/gate_github_actions.rs
- .csdlc/issues/301

## Execution

- Recovered stale review/publication truth through typed csdlc-review recover after PR #304 was observed behind current origin/main with overlapping #258/#301 paths.
- Merged origin/main 193f77d24a693f955a2fcf3bdfc759ad1db8aff4 into codex/301-title-only-operation-provenance.
- Observed the merge completed cleanly with no manual conflict resolution.
- Preserved #301's durable title-only operation provenance implementation and focused regression coverage.

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
    "purpose": "Formatter check for the #301 branch after merging origin/main 193f77d24.",
    "outcome": "passed",
    "evidence_ref": "local-command:18323f4c4d5456fe3f19023203665e932d8ec356:cargo-fmt-csdlc-v2-check:passed"
  },
  {
    "command": [
      "csdlc-validate",
      "--root",
      ".",
      "issue",
      "--issue",
      "301"
    ],
    "purpose": "Typed C-SDLC issue validation after stale-publication recovery and current-main resync.",
    "outcome": "passed",
    "evidence_ref": "local-command:18323f4c4d5456fe3f19023203665e932d8ec356:csdlc-validate-issue-301:passed"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate_github_actions"
    ],
    "purpose": "Focused GitHub issue owner proof including #301 title-only provenance regressions and current main's action-scoped redaction coverage.",
    "outcome": "passed",
    "evidence_ref": "local-command:18323f4c4d5456fe3f19023203665e932d8ec356:gate-github-actions:10-passed"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate_github_actions",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Strict warning-free proof for the touched GitHub owner test target after current-main resync.",
    "outcome": "passed",
    "evidence_ref": "local-command:18323f4c4d5456fe3f19023203665e932d8ec356:clippy-gate-github-actions:passed"
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
