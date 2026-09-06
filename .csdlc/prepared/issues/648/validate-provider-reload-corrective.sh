#!/usr/bin/env bash
set -euo pipefail

mode="${1:-all}"
root="$(git rev-parse --show-toplevel)"
cd "$root"

production() {
  cargo test --locked --manifest-path adl/Cargo.toml --lib provider_mod_profile -- --nocapture
  cargo test --locked --manifest-path adl/Cargo.toml --lib provider_reload -- --nocapture
  cargo test --locked --manifest-path adl/Cargo.toml --lib execute_sequential_retains_starting_provider_snapshot_for_in_flight_step -- --nocapture
  cargo test --locked --manifest-path adl/Cargo.toml --lib tick_adl_workflow_starts_hotload_owner_from_run_args -- --nocapture
}

safety() {
  cargo test --locked --manifest-path adl-runtime-kernel/Cargo.toml --lib config_reload -- --nocapture
  cargo test --locked --manifest-path adl/Cargo.toml --lib provider_mod_profile -- --nocapture
  cargo test --locked --manifest-path adl/Cargo.toml --lib provider_reload -- --nocapture
  cargo test --locked --manifest-path adl/Cargo.toml --lib execute_sequential_retains_starting_provider_snapshot_for_in_flight_step -- --nocapture
  cargo test --locked --manifest-path adl/Cargo.toml --lib tick_adl_workflow_starts_hotload_owner_from_run_args -- --nocapture
  cargo fmt --manifest-path adl/Cargo.toml --all -- --check
  cargo clippy --manifest-path adl/Cargo.toml --all-targets -- -D warnings
  git diff --check
}

case "$mode" in
  production) production ;;
  safety) safety ;;
  all) production; safety ;;
  *) echo "usage: $0 {production|safety|all}" >&2; exit 2 ;;
esac
