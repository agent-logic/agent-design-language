#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
cd "$repo_root"

cargo fmt --manifest-path adl-runtime-kernel/Cargo.toml --all -- --check
cargo test --manifest-path adl-runtime-kernel/Cargo.toml agent_partial_checkpoint --lib
cargo test --manifest-path adl-runtime-kernel/Cargo.toml --test agent_roster
cargo test --manifest-path adl-runtime-kernel/Cargo.toml --test openapi_contract
node demos/html-observatory/tests/agent_continuity.test.mjs
infra/aws/runtime/agent-checkpoint-archive/validate.sh
git diff --check
