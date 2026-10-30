#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$repo_root"

# Runtime origin authority moved with Runtime lifecycle authority. Exercise the
# canonical typed RuntimeInitConfig parser rather than the retired shell
# controller's unreachable init renderer.
cargo test \
  --manifest-path adl-runtime-kernel/Cargo.toml \
  --test configuration \
  runtime_init_accepts_supported_additional_origins \
  -- --exact
cargo test \
  --manifest-path adl-runtime-kernel/Cargo.toml \
  --test configuration \
  runtime_init_rejects_wildcard_duplicate_and_path_origins \
  -- --exact
