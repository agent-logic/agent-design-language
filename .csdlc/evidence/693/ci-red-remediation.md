# Issue #693 CI red remediation

PR #696 initially published at `7da2475b1a6e779a7a23e0cfd8efcb4b1b5c409c`.
GitHub Actions run `33988188478` failed `adl-runtime-v3-fast` in `tests/guardian_soak.rs`.

The failed subprocess tests exited before readiness because the test harness
launched `adl-runtime-kernel serve` without the now-required Runtime v3
configuration-generation environment:

- `ADL_RUNTIME_V3_CONFIG_GENERATION`
- `ADL_RUNTIME_V3_CONFIG_RECEIPT_DIGEST`

The remediation provisions and activates a config-generation receipt from the
final test init TOML before each positive subprocess launch, derives the
compatible binary generation from the TOML's declared `binaries.kernel_path`,
and passes the resulting environment to the child runtime.

Local validation after the fix, with `TMPDIR` inside the #693 worktree:

- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml --test guardian_soak -- --nocapture`
  - result: passed, 8 tests.
  - retained log: `.csdlc/evidence/693/runtime-guardian-soak-ci-remediation.log`.
- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml --lib agent_to_agent_ -- --nocapture`
  - result: passed, 5 tests.
- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml --lib provider_conversation -- --nocapture`
  - result: passed, 10 tests.
- `cargo fmt --manifest-path adl-runtime-kernel/Cargo.toml`
  - result: passed.
- `cargo clippy --manifest-path adl-runtime-kernel/Cargo.toml --all-targets -- -D warnings`
  - result: passed.
- `git diff --check`
  - result: passed.
