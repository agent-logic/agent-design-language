# Structured Output Record

Template: 1.0.0

Issue: 721

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented v3 GitHub issue-create parity, corrected post-cutover terminal finish operational-authority reporting, documented the real --execute issue-create path, recorded adjacent v3 defects as backlog issues, and repaired the csdlc-v3 standalone CI lane by preserving read-only issue-worktree diagnostics and fail-closed executable install authority.

## Artifacts

- csdlc-v3/src/commands/remote/mod.rs
- csdlc-v3/src/adapters/mod.rs
- csdlc-v3/src/commands/remote/tests.rs
- csdlc-v3/src/commands/terminal.rs
- csdlc-v3/src/commands/proof.rs
- csdlc-v3/tests/proof_parity_install_commands.rs
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
- Created backlog issues #723, #724, and #725 for adjacent v3 defects outside the original #721 implementation surface.
- Repaired csdlc-v3 read-only doctor and eligibility route behavior so issue-worktree diagnostics fall back to non-mutating construction reports when operational roots are intentionally invalid for that worktree.
- Tightened executable install authority so stable install execution verifies typed cutover approval evidence content and digest and requires exact_head to match the current checkout before invoking the canonical v3 selector/install path.

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
      "finish"
    ],
    "purpose": "focused terminal finish authority-reporting regressions",
    "outcome": "passed",
    "evidence_ref": "terminal output 2026-09-08 in bound worktree: 7 passed"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v3/Cargo.toml",
      "--test",
      "proof_parity_install_commands"
    ],
    "purpose": "focused proof, shadow, soak, and install lane regression that previously failed csdlc-v3-standalone CI",
    "outcome": "passed",
    "evidence_ref": "terminal output 2026-09-08 in bound worktree: 6 passed"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v3/Cargo.toml",
      "--all-targets"
    ],
    "purpose": "broad csdlc-v3 standalone parity with the failing CI lane",
    "outcome": "passed",
    "evidence_ref": "terminal output 2026-09-08 in bound worktree: all csdlc-v3 lib, integration, and binary tests passed"
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

worktree_only

## Publication

Publication: not_published

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
