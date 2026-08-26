# Structured Output Record

Template: 1.0.0

Issue: 299

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Recovered failed exact review r13 without recording PASS, remediated the P2 final receipt predecessor-digest race, merged current origin/main 5178164c, and recaptured exact evidence at 586b24441513f8062b9495eac4fdc70e0b9e9929. Final cleanup receipt creation now uses the predecessor digest returned by read-only pre-final chain validation instead of rescanning the live ledger, and a deterministic after-validation hook proves an unexpected real receipt inserted between validation and final creation fails closed without writing 900-cleanup-complete.json. The prior r10/r11 pre-final allowlist fixes, idempotent completed-ledger behavior, and #301 gate_github_actions base repair are preserved.

## Artifacts

- csdlc-v2/src/lib.rs
- csdlc-v2/src/projection_cleanup.rs
- csdlc-v2/tests/archived_projection_cleanup.rs
- .csdlc/evidence/299/archived-projection-cleanup-focused.log#sha256=6781030b03b51de8b7e9d7118e66714ea9ff01de8790ba750691d3e4c12b93a8
- .csdlc/evidence/299/csdlc-v2-full-hosted-geometry.log#sha256=1b6c961839060c7091b70383a01de2575651996d6505b574a3136a3553aead40
- .csdlc/evidence/299/csdlc-v2-strict-clippy.log#sha256=5c431a7cc595adf2c921c5f7af9e37ae5d8aa76729c9da1d72483eb6f8190817
- .csdlc/evidence/299/fmt-check.log#sha256=511fdd73696a4bd5180fbbd5d0933702a66fa14b066f23b7c5ba25aaa7eb9c41
- .csdlc/evidence/299/diff-check-origin-main-head.log#sha256=5ff3f1eafa71c2b99ea61ed24f27b2b4999bf4c536d11b508443ef1e031c2451

## Execution

- Recovered failed exact review r13 at git-blake3:6f25eb7ed9616c0a9bd5f3d0288ffbaedb95d205:353542f238b864bd2ee059c15dc1764033d4ffc5594d47ce04a2a1021374e31b without recording PASS. Preserved reviewer finding exactly in audit/SRP: P2 pre-final chain validation returned a trusted predecessor digest, but final receipt creation discarded it and recomputed previous_receipt_digest from the live directory, allowing an unexpected 777 receipt inserted after validation and before final predecessor selection to become the final predecessor.
- Changed final cleanup creation in csdlc-v2/src/projection_cleanup.rs to thread the validated predecessor digest from validate_prefinal_receipt_chain into receipt_with_previous for 900-cleanup-complete.json, avoiding a second live predecessor scan after validation.
- Added an after-pre-final-validation deterministic hook that can inject a valid-looking 777-removed.json after successful chain validation and before final receipt creation; production then rechecks the pre-final allowlist and rejects the unexpected receipt before writing 900-cleanup-complete.json.
- Added cleanup_rejects_extra_receipt_race_after_prefinal_validation_without_final_mutation, which snapshots the ledger at the pre-final boundary, injects a raced 777 receipt at the precise boundary, expects CorruptRecord, asserts no 900-cleanup-complete.json exists, and verifies the final ledger bytes equal the pre-final snapshot plus only the injected raced receipt.
- Preserved r11 remediation requested by the operator: a preexisting fully valid completed cleanup ledger remains idempotent, while a real 900-cleanup-complete.json appearing after the early shortcut and before pre-final validation is rejected with byte-exact no new mutation.
- Preserved r10 remediation and proof: pre-final validation rejects unexpected operation receipts before final creation, validates exact seq1/seq2 semantic authority for completed ledgers, rejects coherent rehashed seq1, seq2, and extra-entry forgeries, and asserts byte-for-byte zero mutation.
- Merged current origin/main 5178164c into the issue branch; this introduced #301 gate_github_actions fixture/owner repairs and resolved the earlier r15 full-suite gate_github_actions RED classification on the resynced broad run.
- Preserved ownership boundaries: source/test edits remain limited to csdlc-v2/src/projection_cleanup.rs and csdlc-v2/tests/archived_projection_cleanup.rs; did not edit projection_recovery.rs, store.rs, gate5.rs, or any #298 worktree/evidence/lifecycle.
- Recaptured exact evidence at 586b24441513f8062b9495eac4fdc70e0b9e9929: focused archived_projection_cleanup 18/18 PASS, full hosted-geometry cargo test PASS, strict Clippy PASS, fmt PASS, and diff-check PASS.

## Validation

[
  {
    "command": [
      "env",
      "CARGO_TARGET_DIR=/Volumes/FastWork/adl-worktrees/adl-issue-299-exact-authority-archived-projection-cleanup/adl/target-focused-r16",
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "archived_projection_cleanup",
      "--",
      "--nocapture"
    ],
    "purpose": "Focused #299 archived-projection cleanup matrix after r13 final-predecessor race remediation and resync over origin/main 5178164c; recaptured at 586b24441513f8062b9495eac4fdc70e0b9e9929 with explicit SHA, argv, and status. Proves 18/18 including the after-pre-final-validation 777 race and prior real-900 pre-final race regressions.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/299/archived-projection-cleanup-focused.log"
  },
  {
    "command": [
      "env",
      "CARGO_TARGET_DIR=/Volumes/FastWork/adl-worktrees/adl-issue-299-exact-authority-archived-projection-cleanup/adl/target-hosted-r16",
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml"
    ],
    "purpose": "Exact local reproduction of hosted csdlc-v2-standalone command geometry after r13 remediation and resync over origin/main 5178164c; recaptured at 586b24441513f8062b9495eac4fdc70e0b9e9929.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/299/csdlc-v2-full-hosted-geometry.log"
  },
  {
    "command": [
      "env",
      "CARGO_TARGET_DIR=/Volumes/FastWork/adl-worktrees/adl-issue-299-exact-authority-archived-projection-cleanup/adl/target-clippy-r16",
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--all-targets",
      "--all-features",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Strict Rust lint proof for #299 after r13 remediation and resync; recaptured at 586b24441513f8062b9495eac4fdc70e0b9e9929 with explicit SHA, argv, and status.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/299/csdlc-v2-strict-clippy.log"
  },
  {
    "command": [
      "cargo",
      "fmt",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--",
      "--check"
    ],
    "purpose": "Formatting proof for #299 after r13 remediation and resync; recaptured at 586b24441513f8062b9495eac4fdc70e0b9e9929 with explicit SHA, argv, and status.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/299/fmt-check.log"
  },
  {
    "command": [
      "git",
      "diff",
      "--check",
      "origin/main...HEAD"
    ],
    "purpose": "Committed-range whitespace hygiene for #299 after r13 remediation and resync; recaptured at 586b24441513f8062b9495eac4fdc70e0b9e9929 with explicit SHA, argv, and status.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/299/diff-check-origin-main-head.log"
  }
]

## Integration

pr_open

## Publication

Publication: ready

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
