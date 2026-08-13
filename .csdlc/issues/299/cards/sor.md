# Structured Output Record

Template: 1.0.0

Issue: 299

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented #299 exact-authority archived-projection cleanup, remediated prior review findings, and corrected final cleanup receipt shortcut handling. Existing final receipt detection now happens read-only immediately after deriving the operation root from an existing ledger and before creating operation directories, namespaces, or receipts. A forged final receipt is rejected after exact envelope validation and the focused regression snapshots the ledger tree bytes to prove no new ledger, namespace, or receipt bytes are created.

## Artifacts

- csdlc-v2/src/lib.rs
- csdlc-v2/src/projection_cleanup.rs
- csdlc-v2/tests/archived_projection_cleanup.rs
- .csdlc/evidence/299/archived-projection-cleanup-focused.log#sha256=b4ce772bd55bbd5884ac799474cfec0eaeb5733cf237e4e25783a764c53d8463
- .csdlc/evidence/299/csdlc-v2-strict-clippy.log#sha256=60fdbdf9e56caf7092c72dd0e3b1e76545de25d0621a79596cdbb9714803fc06
- .csdlc/evidence/299/fmt-diff-check.log#sha256=64d664ac62044a3f1b759ab13e67cd485dde07d657120968c66a6328897eca8d
- .csdlc/evidence/299/csdlc-v2-full-serial.log#sha256=cca12d5a364319d9baf25c251da5ab81812df855d8ec71b7b694e64e11854ce6
- .csdlc/evidence/299/gate-github-actions-issue-read-isolated.log#sha256=c3ed508f285c8ad1512e74158f6d164df4bd9c2bdf0407b820fc9eda79fd599c
- .csdlc/evidence/299/gate-github-actions-operation-marker-isolated.log#sha256=3b8cbe93e94b0f863d9eb406d9e295ba9bde4ce525a42a31330f53f6a3a4ddd6

## Execution

- Recovered the failed r6 review assignment without recording PASS. Preserved reviewer finding exactly in audit/SRP: P1 existing 900-cleanup-complete.json could falsely short-circuit cleanup because AlreadyCompleted only parsed a generic payload and did not validate sequence, state, previous_receipt_digest, or current request issue/operation_id/nodes.
- Moved existing final cleanup receipt detection to the earliest read-only point after an existing cleanup ledger is canonicalized and operation_root is derived. If 900-cleanup-complete.json exists, validation runs before operation root creation, operation-created receipt creation, private-delete namespace creation, or namespace receipt creation.
- Final receipt validation now checks schema, sequence == 900, state == cleanup-complete, exact previous_receipt_digest from the existing predecessor receipt chain, and exact current issue, operation_id, and node list payload.
- Added a coherent forged-final-envelope regression that preinstalls a forged final receipt and snapshots the entire ledger tree including file bytes; rejection must leave the snapshot unchanged, proving no new ledger, namespace, or receipt bytes are created.
- Preserved prior remediations: production crate API wiring through csdlc_v2, completed recovery receipt plus canonical archive manifest authority before cleanup mutation, coherent caller identity forgery rejection, temp receipt previous_receipt_digest and placeholder_identity validation, and archive-local ledger rejection before descendant creation.
- Preserved ownership boundaries: source/test edits are limited to csdlc-v2/src/lib.rs, csdlc-v2/src/projection_cleanup.rs, and csdlc-v2/tests/archived_projection_cleanup.rs; did not edit projection_recovery.rs, store.rs, or gate5.rs.
- Recaptured exact clean-head evidence at 938190766208ca7898dc14acfaaad587ca55e223: focused b4ce772bd55bbd5884ac799474cfec0eaeb5733cf237e4e25783a764c53d8463; strict Clippy 60fdbdf9e56caf7092c72dd0e3b1e76545de25d0621a79596cdbb9714803fc06; fmt/diff 64d664ac62044a3f1b759ab13e67cd485dde07d657120968c66a6328897eca8d; full serial RED cca12d5a364319d9baf25c251da5ab81812df855d8ec71b7b694e64e11854ce6; isolated read c3ed508f285c8ad1512e74158f6d164df4bd9c2bdf0407b820fc9eda79fd599c; isolated marker 3b8cbe93e94b0f863d9eb406d9e295ba9bde4ce525a42a31330f53f6a3a4ddd6.
- Preserved full-suite RED truth: the serial full suite remains failed in gate_github_actions outside the #299-owned cleanup surface; current focused proof and isolated gate reruns do not convert the broad suite result to PASS.

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
    "purpose": "Focused #299 archived-projection cleanup matrix after final receipt zero-mutation remediation; recaptured from clean committed head 938190766208ca7898dc14acfaaad587ca55e223 with explicit HEAD, argv, worktree status digest, and status. Proves 14/14 including crate-level API reachability, completed-recovery receipt plus canonical/archive manifest binding, coherent identity forgery rejection, no cleanup namespace before authority failure, temp receipt predecessor/placeholder negatives, archive-local ledger zero descendant creation, and forged final receipt rejection with exact ledger byte snapshot unchanged. Evidence sha256 b4ce772bd55bbd5884ac799474cfec0eaeb5733cf237e4e25783a764c53d8463.",
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
    "purpose": "Strict Rust lint proof for #299 source and tests after final receipt zero-mutation remediation; recaptured from clean committed head 938190766208ca7898dc14acfaaad587ca55e223 with explicit HEAD, argv, worktree status digest, and status; evidence sha256 60fdbdf9e56caf7092c72dd0e3b1e76545de25d0621a79596cdbb9714803fc06.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/299/csdlc-v2-strict-clippy.log"
  },
  {
    "command": [
      "cargo",
      "fmt",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--check"
    ],
    "purpose": "Formatting proof after final receipt zero-mutation remediation; recaptured from clean committed head 938190766208ca7898dc14acfaaad587ca55e223 with explicit HEAD, argv, worktree status digest, and status; evidence sha256 64d664ac62044a3f1b759ab13e67cd485dde07d657120968c66a6328897eca8d.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/299/fmt-diff-check.log"
  },
  {
    "command": [
      "git",
      "diff",
      "--check",
      "origin/main...HEAD"
    ],
    "purpose": "Committed-range whitespace hygiene after final receipt zero-mutation remediation; recaptured from clean committed head 938190766208ca7898dc14acfaaad587ca55e223 with explicit HEAD, argv, worktree status digest, and status; evidence sha256 64d664ac62044a3f1b759ab13e67cd485dde07d657120968c66a6328897eca8d.",
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
    "purpose": "Prior serial full csdlc-v2 suite attempt before review readiness; remains RED in gate_github_actions and was not rerun or converted to PASS by focused #299 remediation proof; evidence sha256 cca12d5a364319d9baf25c251da5ab81812df855d8ec71b7b694e64e11854ce6.",
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
    "purpose": "Prior clean isolated rerun of the gate_github_actions read-failure case from the serial full-suite RED; passed but does not clear the preserved serial full-suite RED; evidence sha256 c3ed508f285c8ad1512e74158f6d164df4bd9c2bdf0407b820fc9eda79fd599c.",
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
    "purpose": "Prior clean isolated rerun of the gate_github_actions operation-marker case from the serial full-suite RED; passed and does not clear the preserved serial full-suite RED; evidence sha256 3b8cbe93e94b0f863d9eb406d9e295ba9bde4ce525a42a31330f53f6a3a4ddd6.",
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
