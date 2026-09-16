# Issue #1036 validation evidence

Current validation head: `493d4b051335e43106c111d9535d22424df8aceb`.

- Native `csdlc proof 1036`: passed; 6 tests passed, 0 failed, inputs unchanged, and proof persisted at `.csdlc/v3/issues/1036/proof.json`.
- Native `csdlc validate 1036`: lifecycle digest, six-card structure, and semantic projection passed.
- `cargo fmt --manifest-path csdlc-v3/Cargo.toml -- --check`: passed before the final lifecycle-only update and is rerun below.
- `cargo clippy --manifest-path csdlc-v3/Cargo.toml --all-targets --all-features -- -D warnings`: passed before the final lifecycle-only update and is rerun below.
- `git diff --check`: required after final native rendering.

Review truth:

- The exact-head review at `740977763b` required changes.
- The remediation head `5a3d37d2b5` received a clean exact-head review with no actionable findings.
- Subsequent origin/main integration and bounded head-refresh/P2 repairs changed the head.
- The current head `493d4b051335e43106c111d9535d22424df8aceb` still requires final independent exact-head review.
- The PR remains open, draft, and unmerged. Publication readiness is not claimed until that final review completes.
