# Structured Output Record

Template: 1.0.0

Issue: 721

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented v3 GitHub issue-create parity, corrected post-cutover terminal finish operational-authority reporting, documented the real --execute issue-create path, recorded adjacent v3 defects as backlog issues, repaired executable install authority evidence, and fixed read-only v3 diagnostic fallback so CI merge-checkout environments without the configured FastWork worktree parent still emit construction JSON for doctor and eligibility instead of failing with shadow_output_not_json.

## Artifacts

- csdlc-v3/src/main.rs
- csdlc-v3/src/commands/proof.rs
- csdlc-v3/src/commands/remote/mod.rs
- csdlc-v3/src/adapters/mod.rs
- csdlc-v3/src/commands/remote/tests.rs
- csdlc-v3/src/commands/terminal.rs
- csdlc-v3/tests/proof_parity_install_commands.rs
- csdlc-v3/tests/terminal_cleanup_cutover_commands.rs
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
- Extended the same read-only construction fallback to CI environments where the configured operational worktree parent is unavailable, preserving mutation denial while keeping proof and shadow stdout JSON-valid.

## Validation

[
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
    "purpose": "focused proof, shadow, soak, and install lane regression that failed csdlc-v3-standalone CI",
    "outcome": "passed",
    "evidence_ref": "terminal output 2026-09-08 in bound worktree: 6 passed after read-only worktree-parent fallback repair"
  },
  {
    "command": [
      "ADL_CARGO_BUILD_ROOT=/Volumes/home/builds/adl-issue-721-csdlc-v3-ci-repro",
      "bash",
      "adl/tools/run_cargo_validation.sh",
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v3/Cargo.toml",
      "--test",
      "proof_parity_install_commands"
    ],
    "purpose": "CI-wrapper reproduction of the failing csdlc-v3-standalone proof test lane with an external build root",
    "outcome": "passed",
    "evidence_ref": "terminal output 2026-09-08 in bound worktree: wrapper proof_parity_install_commands passed 6/6"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v3/Cargo.toml"
    ],
    "purpose": "CI-equivalent csdlc-v3 test command",
    "outcome": "passed",
    "evidence_ref": "terminal output 2026-09-08 in bound worktree: all csdlc-v3 lib, binary, integration, and doc tests passed"
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
