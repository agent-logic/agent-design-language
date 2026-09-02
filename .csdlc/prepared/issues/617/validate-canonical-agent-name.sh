#!/usr/bin/env bash
set -euo pipefail

cargo nextest run \
  --locked \
  --manifest-path adl-runtime-kernel/Cargo.toml \
  --test agent_roster \
  --test control \
  --test observatory \
  --test openapi_contract \
  --no-tests=fail \
  -E 'test(canonical_name)'
