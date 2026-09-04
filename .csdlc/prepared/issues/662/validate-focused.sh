#!/usr/bin/env bash
set -euo pipefail

cargo test \
  --manifest-path adl-runtime-kernel/Cargo.toml \
  --lib \
  agent_to_agent_initiation
