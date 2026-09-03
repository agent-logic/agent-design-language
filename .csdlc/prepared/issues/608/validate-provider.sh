#!/usr/bin/env bash
set -euo pipefail

cargo fmt --manifest-path adl/Cargo.toml --check
cargo test --manifest-path adl/Cargo.toml --lib vertex_ai_ -- --nocapture
git diff --check -- adl/src/provider/http_family.rs adl/src/provider/http_family/tests.rs
cargo check --manifest-path adl/Cargo.toml -p adl

