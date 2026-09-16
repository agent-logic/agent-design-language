#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
cd "$repo_root"

export CARGO_INCREMENTAL=0
export CARGO_TARGET_DIR="$repo_root/adl/target"

cargo test --quiet --manifest-path adl-provider-core/Cargo.toml
cargo test --quiet --manifest-path adl/Cargo.toml --test provider_moonshot_kimi_k3
cargo clippy --quiet --manifest-path adl-provider-core/Cargo.toml --all-targets --all-features -- -D warnings
cargo fmt --manifest-path adl-provider-core/Cargo.toml -- --check
cargo fmt --manifest-path adl/Cargo.toml -- --check
git diff --check

echo issue-970-provider-validation-passed
