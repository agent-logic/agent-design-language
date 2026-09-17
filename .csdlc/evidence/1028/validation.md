# Issue #1028 validation

Candidate scope: native C-SDLC v3 compatibility for
`issue_complete_coordination` when semantic observation is
`LegacyMigrationRequired`.

## Local proof

- `cargo test --manifest-path csdlc-v3/Cargo.toml --test installed_coordination_completion`
  - 8 passed; 0 failed.
  - Covers legacy success, exact replay without a second remote effect, retained
    parent/contract/evidence/child/head denials, non-coordination denial, unknown
    effect reporting, and retry-time guard revalidation.
- `cargo fmt --manifest-path csdlc-v3/Cargo.toml -- --check`
  - passed.
- `cargo clippy --manifest-path csdlc-v3/Cargo.toml --all-targets --all-features -- -D warnings`
  - passed.
- `git diff --check`
  - passed.

The compatibility path does not create semantic lifecycle state. Required CI,
independent exact-head review, publication, merge, and terminal closeout remain
pending.
