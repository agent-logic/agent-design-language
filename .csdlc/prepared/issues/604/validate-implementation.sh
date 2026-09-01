#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../.." && pwd)"
cd "$repo_root"

mkdir -p ".csdlc/evidence/604/tmp"
export TMPDIR="$repo_root/.csdlc/evidence/604/tmp"

cargo test --locked --manifest-path csdlc-v2/Cargo.toml --test publication_ready
cargo test --locked --manifest-path csdlc-v2/Cargo.toml --test gate_github_route_policy
cargo fmt --manifest-path csdlc-v2/Cargo.toml --all -- --check
git diff --check
