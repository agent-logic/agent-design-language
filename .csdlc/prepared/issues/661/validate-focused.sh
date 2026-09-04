#!/usr/bin/env bash
set -euo pipefail

cargo test --manifest-path adl-runtime-kernel/Cargo.toml --lib \
  shepherd_conversation_invokes_configured_provider
