#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
target_root="/Volumes/FastWork/adl-5624-cargo-target"

cd "$repo_root"
cargo fmt --manifest-path csdlc-v2/Cargo.toml --all -- --check
cargo test --locked --manifest-path csdlc-v2/Cargo.toml --target-dir "$target_root" --test gate7 prune_guard
cargo test --locked --manifest-path csdlc-v2/Cargo.toml --target-dir "$target_root" --test gate7_lifecycle prune
cargo clippy --locked --manifest-path csdlc-v2/Cargo.toml --target-dir "$target_root" --all-targets -- -D warnings
