# Issue #1061 local validation

Candidate scope: native C-SDLC v3 legacy coordination contract installation.

## Proving checks

- `cargo test --manifest-path csdlc-v3/Cargo.toml --lib commands::remote::coordination`: 10 passed, 0 failed.
- `cargo test --locked --manifest-path csdlc-v3/Cargo.toml --test installed_coordination_completion`: 17 passed, 0 failed.
- `cargo test --locked --manifest-path csdlc-v3/Cargo.toml --test remote_module_decomposition`: 3 passed, 0 failed.
- `cargo fmt --manifest-path csdlc-v3/Cargo.toml -- --check`: passed.
- `cargo clippy --manifest-path csdlc-v3/Cargo.toml --all-targets --all-features -- -D warnings`: passed.
- `git diff --check`: passed.

The installed suite covers atomic marker installation and completion, byte-exact preservation of trailing and whitespace-only bodies, complete outbound-body size rejection, exact replay, stale body and timestamp, malformed or pre-existing markers, wrong repository/issue/children, missing approval, changed replay, semantic-state rejection, authenticated child/readback gates, and continued denial of ordinary legacy mutations.

GitHub CI exposed and rejected an upward module dependency from `transport.rs` to
`coordination.rs`. The shared target-body construction now lives in the lower
transport layer, and the focused module-decomposition regression passes locally.
GitHub CI remains the integration gate for the corrected head.
