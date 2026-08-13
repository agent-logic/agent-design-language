# Structured Output Record

Template: 1.0.0

Issue: 299

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented #299 exact-authority archived-projection cleanup in a new isolated module, recovered the stale d655fb611 review assignment, normalized evidence hygiene, and recaptured review-readiness proof from clean committed base 299c5bb16577fc5ae462b09a5a011311204ae0bd. The cleanup engine validates cached #298 terminal merge authority and ancestry, requires ledger and archived tree atomicity, records operation and namespace intent/receipt checkpoints, captures receipt-owned regular files and empty directories through atomic exchange with operation-owned placeholders, rejects hardlinks, symlinks, non-empty directories, identity drift, mode/uid/gid/link-count drift, parent replacement, receipt corruption, and cross-device ledger placement, fsyncs receipt and parent boundaries, resumes safely across before/after intent, temp/write/fsync/rename, parent-fsync, unlink/rmdir, namespace creation, final receipt, and post-exchange/post-unlink crash boundaries, and preserves third-party public replacements.

## Artifacts

- csdlc-v2/src/projection_cleanup.rs
- csdlc-v2/tests/archived_projection_cleanup.rs
- .csdlc/evidence/299/archived-projection-cleanup-focused.log
- .csdlc/evidence/299/csdlc-v2-strict-clippy.log
- .csdlc/evidence/299/fmt-diff-check.log
- .csdlc/evidence/299/csdlc-v2-full-serial.log
- .csdlc/evidence/299/gate-github-actions-issue-read-isolated.log
- .csdlc/evidence/299/gate-github-actions-operation-marker-isolated.log

## Execution

- Added csdlc-v2/src/projection_cleanup.rs with typed cleanup request/result structures, terminal envelope and ancestry gate, same-device cleanup ledger guard, immutable receipt ledger, safe relative path checks, exact node identity including link count, atomic exchange, type-correct unlink/rmdir, parent fsync, receipt temp/write/fsync/rename/parent-fsync recovery, and idempotent replay.
- Added csdlc-v2/tests/archived_projection_cleanup.rs as the focused owned harness required by the pre-bind validator and expanded it from the initial 6 tests to 9 tests covering the issue/design restart and authority matrix.
- Expanded focused tests for before/after operation and namespace intent, receipt temp/write/fsync/rename/parent-fsync boundaries, exchange and unlink/rmdir restart adoption, final receipt restart, hardlink/link-count rejection, mode drift, parent replacement, and corrupt final receipt rejection.
- Preserved ownership boundaries: edited only csdlc-v2/src/projection_cleanup.rs and csdlc-v2/tests/archived_projection_cleanup.rs for source/test changes; did not edit projection_recovery.rs, store.rs, or gate5.rs.
- Recovered the stale d655fb611 review assignment before accepting any review result because pre-assignment evidence was bound to old HEAD e912bfc6 and committed range diff-check found evidence blank-line hygiene defects.
- Normalized older full-suite/rerun evidence logs so git diff --check origin/main...HEAD is clean at committed base 299c5bb16577fc5ae462b09a5a011311204ae0bd.
- Recaptured focused cleanup, strict Clippy, fmt, committed-range diff-check, and scoped gate_github_actions classification logs from clean committed base 299c5bb16577fc5ae462b09a5a011311204ae0bd with explicit HEAD, argv, worktree status digest, and command status.
- Preserved full-suite RED truth: the serial full suite remains failed in gate_github_actions outside the #299-owned cleanup surface; current clean isolated read and operation-marker reruns do not clear or convert the broad suite result to PASS.

## Validation

[
  {
    "command": [
      "env",
      "CARGO_TARGET_DIR=/Volumes/FastWork/adl-builds/299/csdlc-v2",
      "cargo",
      "nextest",
      "run",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "archived_projection_cleanup",
      "--no-tests=fail"
    ],
    "purpose": "Focused #299 archived-projection cleanup matrix after expanded restart, receipt, authority, drift, and corruption coverage; recaptured from clean committed base 299c5bb with explicit HEAD, argv, worktree status digest, and status.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/299/archived-projection-cleanup-focused.log"
  },
  {
    "command": [
      "env",
      "CARGO_TARGET_DIR=/Volumes/FastWork/adl-builds/299/csdlc-v2",
      "cargo",
      "clippy",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Strict Rust lint proof for #299 source and tests; recaptured from clean committed base 299c5bb with explicit HEAD, argv, worktree status digest, and status.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/299/csdlc-v2-strict-clippy.log"
  },
  {
    "command": [
      "cargo",
      "fmt",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--check",
      "&&",
      "git",
      "diff",
      "--check",
      "origin/main...HEAD"
    ],
    "purpose": "Formatting and committed-range whitespace hygiene from clean committed base 299c5bb; recaptured with explicit HEAD, argv, worktree status digest, and status.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/299/fmt-diff-check.log"
  },
  {
    "command": [
      "env",
      "CARGO_TARGET_DIR=/Volumes/FastWork/adl-builds/299/csdlc-v2",
      "cargo",
      "nextest",
      "run",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--workspace"
    ],
    "purpose": "Serial full csdlc-v2 suite attempt before review readiness; remains RED in gate_github_actions and is not converted to PASS by focused #299 proof or isolated reruns.",
    "outcome": "failed",
    "evidence_ref": ".csdlc/evidence/299/csdlc-v2-full-serial.log"
  },
  {
    "command": [
      "env",
      "CARGO_TARGET_DIR=/Volumes/FastWork/adl-builds/299/csdlc-v2-github-actions-isolated-read",
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate_github_actions",
      "issue_read_failures_are_typed_redacted_and_action_scoped",
      "--",
      "--test-threads=1"
    ],
    "purpose": "Clean isolated rerun of the gate_github_actions read-failure case from the serial full-suite RED; current clean-base rerun passed, which does not clear the preserved serial full-suite RED.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/299/gate-github-actions-issue-read-isolated.log"
  },
  {
    "command": [
      "env",
      "CARGO_TARGET_DIR=/Volumes/FastWork/adl-builds/299/csdlc-v2-github-actions-isolated-marker",
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate_github_actions",
      "operation_marker_is_stable_and_idempotent",
      "--",
      "--test-threads=1"
    ],
    "purpose": "Clean isolated rerun of the gate_github_actions operation-marker case from the serial full-suite RED; passed and does not clear the preserved serial full-suite RED.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/299/gate-github-actions-operation-marker-isolated.log"
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
