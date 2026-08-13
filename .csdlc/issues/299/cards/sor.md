# Structured Output Record

Template: 1.0.0

Issue: 299

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Recovered failed exact review r11 without recording PASS, remediated the P2 pre-final real 900 receipt race, and recaptured exact clean-head evidence at 39453c4d3ac2d754e5ee2ace6b46c5c251bfa783. Pre-final operation-ledger validation now allows only the legitimate 900-cleanup-complete.json.tmp crash-recovery artifact; completed-ledger validation remains the only path that allows a real 900-cleanup-complete.json. A new deterministic boundary failpoint proves a real mismatched final receipt appearing after the early shortcut and before pre-final validation fails closed without creating or rewriting ledger, namespace, or receipt bytes.

## Artifacts

- csdlc-v2/src/lib.rs
- csdlc-v2/src/projection_cleanup.rs
- csdlc-v2/tests/archived_projection_cleanup.rs
- .csdlc/evidence/299/archived-projection-cleanup-focused.log#sha256=0d92d247a0d0ecd76014c7951ff91458294f326fe03cc2539a4954b4c446e351
- .csdlc/evidence/299/csdlc-v2-full-hosted-geometry.log#sha256=b73b1917ee770434504700a8fc199cb2e0ed2cc3733a841cbab74053da078be8
- .csdlc/evidence/299/csdlc-v2-strict-clippy.log#sha256=ee14b3c110049648a8bdfdf42f41293130fd4606bcc38675563815a728e395bd
- .csdlc/evidence/299/fmt-check.log#sha256=c022eddd5d94ff365acc7aa70d124e06898baefa21a31e165c659f879170ca49
- .csdlc/evidence/299/diff-check-origin-main-head.log#sha256=ff5eb2489c2deaa459beb3cb2fb1a213f4a43959aa689166259aaea17fc42448

## Execution

- Recovered failed exact review r11 at git-blake3:a13666d1be493e63a91318d36294526a6af812b1:3401b82eceab18d98e0842a5c06f9d58cc5f2eda9d5e09572af16998cdb11b5c without recording PASS. Preserved reviewer finding exactly in audit/SRP: P2 pre-final validation still allowed a real 900-cleanup-complete.json, not only 900-cleanup-complete.json.tmp, so a real final receipt appearing after the early completed-ledger shortcut but before pre-final validation could be adopted by receipt() if bytes matched the deterministic final payload.
- Changed pre-final operation-ledger allowlisting in csdlc-v2/src/projection_cleanup.rs so pre-final validation permits 900-cleanup-complete.json.tmp but rejects real 900-cleanup-complete.json; completed-ledger validation remains strict and idempotent for real 900-cleanup-complete.json.
- Added before_prefinal_receipt_chain_validation failpoint immediately before pre-final chain validation, allowing deterministic proof of the race window after the early shortcut and before final receipt creation.
- Added cleanup_rejects_real_final_receipt_race_before_prefinal_validation, which stops at the new boundary, injects a valid-looking mismatched real 900-cleanup-complete.json, reruns cleanup, and asserts CorruptRecord with byte-for-byte ledger snapshot preservation.
- Preserved existing r10 remediation and proof: pre-final chain validation rejects unexpected valid-looking 777-removed.json before final receipt creation; completed-ledger validation still rejects unexpected receipt entries and coherent rehashed seq1/seq2/extra-entry forgeries with zero-mutation assertions.
- Preserved idempotent completed-ledger behavior: a fully valid completed cleanup ledger remains accepted as AlreadyCompleted.
- Preserved ownership boundaries: source/test edits remain limited to csdlc-v2/src/projection_cleanup.rs and csdlc-v2/tests/archived_projection_cleanup.rs; did not edit projection_recovery.rs, store.rs, gate5.rs, or any #298 worktree/evidence/lifecycle.
- Recaptured exact evidence at 39453c4d3ac2d754e5ee2ace6b46c5c251bfa783: focused 17/17 PASS, full hosted-geometry cargo test PASS, strict Clippy PASS, fmt PASS, and diff-check PASS.

## Validation

[
  {
    "command": [
      "env",
      "CARGO_TARGET_DIR=/Volumes/FastWork/adl-worktrees/adl-issue-299-exact-authority-archived-projection-cleanup/adl/target-focused-r14",
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
    "purpose": "Focused #299 archived-projection cleanup matrix after r11 pre-final real 900 receipt race remediation; recaptured at 39453c4d3ac2d754e5ee2ace6b46c5c251bfa783 with explicit SHA, argv, and status. Proves 17/17 including the new real-final-race zero-mutation regression.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/299/archived-projection-cleanup-focused.log"
  },
  {
    "command": [
      "env",
      "CARGO_TARGET_DIR=/Volumes/FastWork/adl-worktrees/adl-issue-299-exact-authority-archived-projection-cleanup/adl/target-hosted-r14",
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml"
    ],
    "purpose": "Exact local reproduction of the hosted csdlc-v2-standalone command geometry after r11 remediation; recaptured at 39453c4d3ac2d754e5ee2ace6b46c5c251bfa783 with explicit SHA, argv, and status.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/299/csdlc-v2-full-hosted-geometry.log"
  },
  {
    "command": [
      "env",
      "CARGO_TARGET_DIR=/Volumes/FastWork/adl-worktrees/adl-issue-299-exact-authority-archived-projection-cleanup/adl/target-clippy-r14",
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
    "purpose": "Strict Rust lint proof for #299 after r11 remediation; recaptured at 39453c4d3ac2d754e5ee2ace6b46c5c251bfa783 with explicit SHA, argv, and status.",
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
    "purpose": "Formatting proof for #299 after r11 remediation; recaptured at 39453c4d3ac2d754e5ee2ace6b46c5c251bfa783 with explicit SHA, argv, and status.",
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
    "purpose": "Committed-range whitespace hygiene for #299 after r11 remediation; recaptured at 39453c4d3ac2d754e5ee2ace6b46c5c251bfa783 with explicit SHA, argv, and status.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/299/diff-check-origin-main-head.log"
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
