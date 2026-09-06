#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$repo_root"

echo "issue713_a2a_history: checking durable A2A transcript store" >&2
cargo test --manifest-path adl-runtime-kernel/Cargo.toml --test conversation_history

echo "issue713_a2a_history: checking live-style non-Shepherd A2A history projection" >&2
cargo test --manifest-path adl-runtime-kernel/Cargo.toml agent_to_agent_model_action_from_conversation_delivers_peer_response

echo "issue713_a2a_history: checking checkpoint/rehydration transcript restore" >&2
cargo test --manifest-path adl-runtime-kernel/Cargo.toml archived_restore_rehydrates_complete_a2a_transcript_history

echo "issue713_a2a_history: checking Observatory browser restore and redaction" >&2
node --test \
  demos/html-observatory/tests/conversation_sessions.test.mjs \
  demos/html-observatory/tests/security_privacy_adversarial.test.mjs

echo "issue713_a2a_history: PASS" >&2
