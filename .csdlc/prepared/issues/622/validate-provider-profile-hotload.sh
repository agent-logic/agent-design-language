#!/usr/bin/env bash
set -euo pipefail

mode="${1:-all}"
root="$(git rev-parse --show-toplevel)"
cd "$root"

production() {
  cargo test --locked --manifest-path adl/Cargo.toml provider_mod_profile -- --nocapture
  if test -f adl/tests/provider_profile_hot_reload.rs; then
    cargo test --locked --manifest-path adl/Cargo.toml --test provider_profile_hot_reload -- --nocapture
  fi
}

safety() {
  cargo test --locked --manifest-path adl-runtime-kernel/Cargo.toml --lib config_reload -- --nocapture
  cargo test --locked --manifest-path adl/Cargo.toml provider_mod_profile -- --nocapture
  if test -f adl/tests/provider_profile_hot_reload.rs; then
    cargo test --locked --manifest-path adl/Cargo.toml --test provider_profile_hot_reload -- --nocapture
  fi
  git diff --check
}

case "$mode" in
  production) production ;;
  safety) safety ;;
  all) production; safety ;;
  *) echo "usage: $0 {production|safety|all}" >&2; exit 2 ;;
esac
