# Issue #1061 local validation

Candidate scope: native C-SDLC v3 legacy coordination contract installation.

## Proving checks

- `cargo test --manifest-path csdlc-v3/Cargo.toml --lib commands::remote::coordination`: 10 passed, 0 failed.
- `cargo test --manifest-path csdlc-v3/Cargo.toml --test installed_coordination_completion`: 16 passed, 0 failed.
- `cargo fmt --manifest-path csdlc-v3/Cargo.toml -- --check`: passed.
- `cargo clippy --manifest-path csdlc-v3/Cargo.toml --all-targets --all-features -- -D warnings`: passed.
- `git diff --check`: passed.

The installed suite covers atomic marker installation and completion, byte-exact preservation of trailing body whitespace, complete outbound-body size rejection, exact replay, stale body and timestamp, malformed or pre-existing markers, wrong repository/issue/children, missing approval, changed replay, semantic-state rejection, authenticated child/readback gates, and continued denial of ordinary legacy mutations.

A broader `cargo test --manifest-path csdlc-v3/Cargo.toml` run was stopped after the user directed this small tooling issue to completion. Before cancellation it reported no failures, including 167 library tests and the 14-test installed coordination suite. This partial broad run is not claimed as full-suite proof; GitHub CI remains the integration gate.
