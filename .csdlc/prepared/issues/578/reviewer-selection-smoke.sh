#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../.." && pwd)"
cd "$repo_root"

cargo test --manifest-path adl/Cargo.toml --test provider_tests \
  profiles::z_ai_glm_5_3_flash_profile_expands_for_reviewer_agent_selection
