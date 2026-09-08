#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
cd "$repo_root"

test ! -e adl-runtime-kernel/src/config_generation.rs
test ! -e adl/tests/csm_runtime_v3_generation.rs

if rg -n 'ADL_RUNTIME_CONFIG_GENERATION|ADL_RUNTIME_CONFIG_RECEIPT' \
  adl/src/cli/csm_runtime_v3_cmd.rs \
  adl-runtime/src/bin/adl-runtime-guardian.rs \
  adl-runtime-kernel/src/bin/adl-runtime-kernel.rs; then
  printf 'retired cross-binary startup receipt contract remains\n' >&2
  exit 1
fi

cargo test --manifest-path adl/Cargo.toml \
  -p adl --lib csm_runtime_v3_cmd_tests --no-fail-fast

cargo test --manifest-path adl-runtime/Cargo.toml \
  --bin adl-runtime-guardian \
  --test guardian_cli \
  --test runtime_guardian_lifecycle \
  --no-fail-fast

cargo test --manifest-path adl-runtime-kernel/Cargo.toml \
  --test production_acip_wss \
  --no-fail-fast

cargo test --manifest-path adl-runtime-kernel/Cargo.toml \
  --lib \
  agent_to_agent_model_action_from_conversation_delivers_peer_response \
  --no-fail-fast

cargo test --manifest-path adl-runtime-kernel/Cargo.toml \
  --lib \
  archived_restore_rehydrates_complete_a2a_transcript_history \
  --no-fail-fast

cargo test --manifest-path adl-runtime-kernel/Cargo.toml \
  --test configuration \
  continuity_compatibility_v1_allows_hot_load_policy_and_binds_security_identity \
  --no-fail-fast

cargo test --manifest-path adl-runtime-kernel/Cargo.toml \
  --test control \
  running_kernel_reload_publishes_candidate_hash_and_updates_presentation_atomically \
  --no-fail-fast

cargo clippy --manifest-path adl-runtime-kernel/Cargo.toml \
  --lib --bins --tests -- -D warnings

cargo clippy --manifest-path adl-runtime/Cargo.toml \
  --lib \
  --bin adl-runtime-guardian \
  --test guardian_cli \
  --test runtime_guardian_lifecycle \
  -- -D warnings

git diff --check origin/main...HEAD
