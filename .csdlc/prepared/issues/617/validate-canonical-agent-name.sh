#!/usr/bin/env bash
set -euo pipefail

cargo nextest run \
  --locked \
  --manifest-path adl-runtime-kernel/Cargo.toml \
  --test configuration \
  --test agent_roster \
  --test control \
  --test observatory \
  --test openapi_contract \
  --no-tests=fail \
  -E 'test(canonical_name) | test(production_shepherd_construction_uses_configured_canonical_name)'

cargo nextest run \
  --locked \
  --manifest-path adl-runtime-kernel/Cargo.toml \
  --lib \
  --no-tests=fail \
  -E 'test(agent_lifecycle_is_idempotent_portable_and_restart_safe)'
