#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
cd "$repo_root"

if [[ "${1:-}" == "--live-wuji" ]]; then
  echo "live Wuji acceptance is an issue #640 implementation deliverable" >&2
  exit 2
fi

cargo nextest run \
  --locked \
  --manifest-path adl-runtime-kernel/Cargo.toml \
  --test assembly \
  --test shepherd \
  --test control \
  --test governed_operations \
  --test agent_roster \
  --test openapi_contract \
  --no-tests=fail \
  -E 'test(resident_shepherd) | test(shepherd_provider) | test(shepherd_model_health) | test(shepherd_readiness_consistency)'

cargo fmt --manifest-path adl-runtime-kernel/Cargo.toml --check
git diff --check
