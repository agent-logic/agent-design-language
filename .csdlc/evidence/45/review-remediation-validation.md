# Issue 45 review-remediation validation

Exact worktree: `/Volumes/FastWork/adl-issue-45-doctor-split-authority`

All commands completed successfully on 2026-08-08 after remediation of the
initial exact-head review findings.

- `cargo test --locked --manifest-path csdlc-v2/Cargo.toml --test gate2`
  - Result: 1 passed, 0 failed.
  - Proof: same-repository and explicit split acceptance; missing, unparseable,
    mismatched, and retry-substituted origin authority fail closed.
- `cargo test --locked --manifest-path csdlc-v2/Cargo.toml absent_code_repository_preserves_pre_field_record_and_receipt_digests`
  - Result: 1 passed, 0 failed.
  - Proof: pre-field live index and retained terminal-receipt digests remain
    stable when `code_repository` is absent.
- `cargo run --locked --manifest-path csdlc-v2/Cargo.toml --bin csdlc-validate -- --root . issue --issue 45`
  - Result: `status=pass`, `findings=[]`, generation 9, phase `implemented`.
  - Proof: all six typed cards and canonical issue state validate.
- `cargo test --locked --manifest-path csdlc-v2/Cargo.toml --test gate6 public_schema_keeps_publication_and_drops_merged_reconciliation`
  - Result: 1 passed, 0 failed.
  - Proof: public schema contract remains valid.
- `cargo clippy --locked --manifest-path csdlc-v2/Cargo.toml --all-targets -- -D warnings`
  - Result: passed with no warnings.
- `cargo fmt --all --manifest-path csdlc-v2/Cargo.toml -- --check`
  - Result: passed.
- `bash adl/tools/test_install_adl_pr_cycle_skill.sh`
  - Result: `PASS test_install_adl_pr_cycle_skill`.
- `git diff --check`
  - Result: passed.

The narrow temporary local editor authorization used to repair the stale STP
deliverable list was reverted before this evidence was recorded and is absent
from the product diff.
