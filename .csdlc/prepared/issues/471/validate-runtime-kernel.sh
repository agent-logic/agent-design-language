#!/usr/bin/env bash
set -Eeuo pipefail

ROOT="$(git rev-parse --show-toplevel)"
MANIFEST="$ROOT/adl-runtime-kernel/Cargo.toml"
TARGET_DIR="${ADL_RUNTIME_KERNEL_TARGET_DIR:-$ROOT/adl-runtime-kernel/target}"

env CARGO_TARGET_DIR="$TARGET_DIR" cargo test --manifest-path "$MANIFEST" --locked
env CARGO_TARGET_DIR="$TARGET_DIR" cargo clippy --manifest-path "$MANIFEST" --all-targets --locked -- -D warnings
cargo fmt --manifest-path "$MANIFEST" --all -- --check
git -C "$ROOT" diff --check
