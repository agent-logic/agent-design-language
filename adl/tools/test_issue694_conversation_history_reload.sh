#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"

node "$repo_root/adl/tools/validate_v092_observatory_transcript_history.mjs"
cargo test \
  --locked \
  --manifest-path "$repo_root/adl-runtime-kernel/Cargo.toml" \
  production_conversation_history_restores_operator_and_agent_once_in_order \
  -- --nocapture
cargo test \
  --locked \
  --manifest-path "$repo_root/adl-runtime-kernel/Cargo.toml" \
  --test durable_conversation_history_integration
cargo test \
  --locked \
  --manifest-path "$repo_root/adl-runtime-kernel/Cargo.toml" \
  control::layer8_conversation_ingress_tests::production_conversation_history_restores_operator_and_agent_once_in_order \
  -- \
  --exact
