# Structured Output Record

Template: 1.0.0

Issue: 721

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented v3 GitHub issue-create parity, corrected post-cutover terminal finish operational-authority reporting, documented the real --execute issue-create path, and recorded adjacent v3 defects as backlog issues.

## Artifacts

- csdlc-v3/src/commands/remote/mod.rs
- csdlc-v3/src/adapters/mod.rs
- csdlc-v3/src/commands/remote/tests.rs
- csdlc-v3/src/commands/terminal.rs
- csdlc-v3/tests/terminal_cleanup_cutover_commands.rs
- csdlc-v3/src/main.rs
- csdlc-v3/README.md
- .csdlc/evidence/v3-defects/backlog-requests/issue-candidates.md
- https://github.com/agent-logic/agent-design-language/issues/723
- https://github.com/agent-logic/agent-design-language/issues/724
- https://github.com/agent-logic/agent-design-language/issues/725

## Execution

- Added v3 IssueCreate mutation support for title, body, labels, assignees, and milestone through the existing typed GitHub mutation dispatcher.
- Added operation-marker based issue-create reconciliation through a read-only issues-by-marker adapter lookup and retained the assigned GitHub issue number in reconciliation and final receipts.
- Changed authenticated terminal finish reporting so post-cutover persisted terminal state reports operational_authority=true while pre-cutover persistence denial remains blocked and non-authoritative.
- Documented the distinction between local v3 issue state and real GitHub issue creation in csdlc-v3/README.md, including the implemented --execute path.
- Created backlog issues #723, #724, and #725 for adjacent v3 defects outside #721 scope.

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v3/Cargo.toml",
      "--lib",
      "commands::remote::tests"
    ],
    "purpose": "focused v3 remote mutation tests including issue-create reconciliation and receipt assignment",
    "outcome": "passed",
    "evidence_ref": "terminal output 2026-09-08 in bound worktree: 23 passed; includes issue_create_mutation_posts_and_reconciles_assigned_issue_number"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v3/Cargo.toml",
      "--test",
      "terminal_cleanup_cutover_commands",
      "post_cutover_finish_persists_typed_state_and_receipt_idempotently"
    ],
    "purpose": "focused post-cutover terminal finish authority-reporting regression",
    "outcome": "passed",
    "evidence_ref": "terminal output 2026-09-08 in bound worktree: 1 passed"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v3/Cargo.toml",
      "--test",
      "terminal_cleanup_cutover_commands",
      "pre_cutover_finish_denies_terminal_persistence"
    ],
    "purpose": "focused pre-cutover fail-closed terminal persistence regression",
    "outcome": "passed",
    "evidence_ref": "terminal output 2026-09-08 in bound worktree: 1 passed"
  },
  {
    "command": [
      "cargo",
      "fmt",
      "--manifest-path",
      "csdlc-v3/Cargo.toml",
      "--all",
      "--",
      "--check"
    ],
    "purpose": "Rust formatting",
    "outcome": "passed",
    "evidence_ref": "terminal output 2026-09-08 in bound worktree: fmt check passed"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "csdlc-v3/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Rust lint safety across v3 targets",
    "outcome": "passed",
    "evidence_ref": "terminal output 2026-09-08 in bound worktree: clippy finished successfully"
  },
  {
    "command": [
      "cargo",
      "run",
      "--locked",
      "--manifest-path",
      "csdlc-v3/Cargo.toml",
      "--bin",
      "csdlc",
      "--",
      "github-issue",
      "--help"
    ],
    "purpose": "operator-visible command help documents real --execute path",
    "outcome": "passed",
    "evidence_ref": "terminal output 2026-09-08 in bound worktree: usage includes [--execute]"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "exact diff whitespace safety",
    "outcome": "passed",
    "evidence_ref": "terminal output 2026-09-08 in bound worktree: no whitespace errors"
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
