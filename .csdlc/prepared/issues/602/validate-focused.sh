#!/usr/bin/env bash
set -euo pipefail

cargo test --locked --manifest-path adl/Cargo.toml --bin adl csmctl_agent
cargo test --locked --manifest-path adl-runtime-kernel/Cargo.toml --lib agent_lifecycle
cargo test --locked --manifest-path adl-runtime-kernel/Cargo.toml --test openapi_contract
git diff --check
