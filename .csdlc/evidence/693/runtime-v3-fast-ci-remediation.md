# Issue 693 Runtime v3 fast CI remediation

## Failed live gate

- PR: #696
- Failed run: https://github.com/agent-logic/agent-design-language/actions/runs/33989880590
- Failed job: `adl-runtime-v3-fast`
- Failed test: `tests/parity_b_live_kernel.rs::live_graph_executes_through_guardian_canonical_ingress`
- Failure: the production child exited with status 78 and stderr `runtime configuration generation environment is required`.
- Additional local sibling found before republish: `tests/production_acip_wss.rs::production_binary_acip_wss_produces_observed_receipt` failed with the same missing runtime configuration-generation environment.

## Remediation

The Runtime v3 production-style subprocess tests now provision and activate the config-generation receipt derived from the init TOML's configured kernel path before spawning `CARGO_BIN_EXE_adl-runtime-kernel`.

Touched test harnesses:

- `adl-runtime-kernel/tests/parity_b_live_kernel.rs`
- `adl-runtime-kernel/tests/production_acip_wss.rs`

No production A2A behavior was changed by this CI remediation.

## Local validation

All commands used `TMPDIR=/Volumes/FastWork/adl-worktrees/adl-issue-693-runtime-a2a-action-selection-reliability/.tmp`.

- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml --test parity_b_live_kernel live_graph_executes_through_guardian_canonical_ingress -- --nocapture` — passed, 1/1.
- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml --test production_acip_wss production_binary_acip_wss_produces_observed_receipt -- --nocapture` — passed, 1/1.
- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml` — passed, including the full Runtime kernel library/tests/doc-tests lane exercised by `adl-runtime-v3-fast`.
- `cargo fmt --manifest-path adl-runtime-kernel/Cargo.toml` — passed.
- `cargo clippy --manifest-path adl-runtime-kernel/Cargo.toml --all-targets -- -D warnings` — passed.
- `git diff --check` — passed.
- `csdlc-validate --root . issue --issue 693` — passed at generation 25 before the SOR update.
