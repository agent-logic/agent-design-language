#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repo_root="$(git -C "${script_dir}" rev-parse --show-toplevel)"
cd "${repo_root}"

cargo test --offline --locked --manifest-path adl/Cargo.toml --bin adl csmctl_agent
cargo test --offline --locked --manifest-path adl-runtime-kernel/Cargo.toml --lib agent_lifecycle
cargo test --offline --locked --manifest-path adl-runtime-kernel/Cargo.toml --test openapi_contract
git diff --check
git diff --check origin/main...HEAD
