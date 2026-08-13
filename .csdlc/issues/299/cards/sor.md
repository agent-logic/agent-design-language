# Structured Output Record

Template: 1.0.0

Issue: 299

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Recovered #299 after PR #325 hosted CI failed csdlc-v2-standalone at head 3ce1716eeaf277a30d00ff11c92abebe1002962e. Typed recovery cleared publication/review truth back to implemented, the branch was resynced over origin/main 2ad315b33, and the replacement-race fixtures were hardened to force CleanupNodeIdentity drift on Linux/CI rather than relying on remove-and-recreate content drift. The production cleanup contract remains identity-authority based; the repair is test/proof-only and preserves prior final-chain authority behavior.

## Artifacts

- csdlc-v2/src/lib.rs
- csdlc-v2/src/projection_cleanup.rs
- csdlc-v2/tests/archived_projection_cleanup.rs
- .csdlc/evidence/299/archived-projection-cleanup-focused.log#sha256=578730329c4b0b8fb9c0c9b52afa7219441412191ef3b83eecf8053205819fd2
- .csdlc/evidence/299/csdlc-v2-full-hosted-geometry.log#sha256=1a76b79c029f612d1e8f255b279624d0927944b994ae8b28a63edd0d9c879dc2
- .csdlc/evidence/299/csdlc-v2-strict-clippy.log#sha256=abbc7048e9218779d0c05f7a73b2a356d2b048fff557bf0562ade6988c2c0699
- .csdlc/evidence/299/fmt-check.log#sha256=5e276467dcb6a823995d84d349e199c9e87c780b42b5491ec883c9910f7b46b5
- .csdlc/evidence/299/diff-check-origin-main-head.log#sha256=0a07f1ecd579d8fb940e30c97830d246698ab7dc68eee75b614301a8b504d861

## Execution

- Observed PR #325 run 31682167016 terminal RED: csdlc-v2-standalone job 94390045054 failed in tests/archived_projection_cleanup.rs with cleanup_rejects_replacement_inode_before_capture and cleanup_preserves_public_replacement_before_placeholder_disposal unexpectedly returning Completed; adl-ci failed as the split-lane aggregator.
- Diagnosed the failure as #299 test/proof-related rather than merge-ready or main-advance-only: the two replacement fixtures changed file contents but did not force a modeled CleanupNodeIdentity drift, so Linux/CI could produce a replacement still accepted by the identity-only authority model.
- Applied typed review/publication recovery at generation 23/digest 6986b30c852e4955266b995269d6ac36df6a9ef0310c6814eb48ac60b94a1d8e; resulting issue state is implemented generation 24/digest 1a085da34e9cff7dca95ba1c5d51dbde6a11fc8c97c234523adb5ae614a50495 with review and publication cleared.
- Committed recovery and initial fixture repair at 9e227ea9d, merged origin/main 2ad315b33 (#324 typed bound issue identity/repository migration) cleanly into the #299 worktree at c2215e38c, then committed the final rustfmt-only fixture shape at 9306ad0c9205f2c270aab2351edbf55b61025e11.
- Changed only csdlc-v2/tests/archived_projection_cleanup.rs for the CI repair: added write_distinct_replacement to write replacement content and set mode 0600, and used it in the two replacement-preservation regressions.
- Preserved the production #299 behavior from the reviewed implementation: final receipt detection remains early/read-only, final-chain validation still binds exact seq1 operation authority, seq2 namespace authority, expected receipt filenames, and unexpected receipt rejection with zero ledger/namespace/receipt mutation.
- Preserved ownership boundaries: no #298 worktree/files were touched; projection_recovery.rs, store.rs, and gate5.rs remain untouched by this remediation.
- Recaptured exact evidence at 9306ad0c9205f2c270aab2351edbf55b61025e11 after resync and fixture repair: focused 15/15 PASS, full hosted-geometry cargo test PASS, strict Clippy PASS, fmt PASS, and diff-check PASS.

## Validation

[
  {
    "command": [
      "env",
      "CARGO_TARGET_DIR=/Volumes/FastWork/adl-worktrees/adl-issue-299-exact-authority-archived-projection-cleanup/adl/target",
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
    "purpose": "Focused #299 archived-projection cleanup matrix after hosted CI fixture remediation and base resync; recaptured at 9306ad0c9205f2c270aab2351edbf55b61025e11 with explicit SHA, argv, and status. Proves 15/15 including replacement identity drift regressions, final receipt read-only shortcut validation, seq1/seq2 authority binding, unexpected receipt rejection, coherent fully rehashed forgery negatives, and byte-for-byte zero ledger/namespace/receipt mutation.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/299/archived-projection-cleanup-focused.log"
  },
  {
    "command": [
      "env",
      "CARGO_TARGET_DIR=/Volumes/FastWork/adl-worktrees/adl-issue-299-exact-authority-archived-projection-cleanup/adl/target-hosted-geometry",
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml"
    ],
    "purpose": "Exact local reproduction of the hosted csdlc-v2-standalone command geometry after the PR #325 CI failure; recaptured at 9306ad0c9205f2c270aab2351edbf55b61025e11 with explicit SHA, argv, and status. The formerly failing archived_projection_cleanup replacement tests passed, and the full csdlc-v2 test command exited 0.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/299/csdlc-v2-full-hosted-geometry.log"
  },
  {
    "command": [
      "env",
      "CARGO_TARGET_DIR=/Volumes/FastWork/adl-worktrees/adl-issue-299-exact-authority-archived-projection-cleanup/adl/target",
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
    "purpose": "Strict Rust lint proof for #299 after hosted CI fixture remediation and base resync; recaptured at 9306ad0c9205f2c270aab2351edbf55b61025e11 with explicit SHA, argv, and status.",
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
    "purpose": "Formatting proof for #299 after hosted CI fixture remediation and base resync; recaptured at 9306ad0c9205f2c270aab2351edbf55b61025e11 with explicit SHA, argv, and status.",
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
    "purpose": "Committed-range whitespace hygiene for #299 after hosted CI fixture remediation and base resync; recaptured at 9306ad0c9205f2c270aab2351edbf55b61025e11 with explicit SHA, argv, and status.",
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
