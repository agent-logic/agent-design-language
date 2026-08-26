#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$repo_root"
mkdir -p "$repo_root/.adl/runs"
scratch="$(mktemp -d "$repo_root/.adl/runs/issue-550-origin.XXXXXX")"
trap 'rm -rf "$scratch"' EXIT

require_source() {
  local needle="$1"
  grep -F "$needle" CSMctl >/dev/null || {
    printf 'missing CSMctl source fragment: %s\n' "$needle" >&2
    exit 1
  }
}

expect_invalid_origin() {
  local origin="$1"
  local expected="$2"
  local log="$scratch/invalid-$(printf '%s' "$origin" | tr -c 'A-Za-z0-9' '_').log"
  if ADL_CSM_TEST_MODE=1 \
    ADL_CSM_TEST_OS=Darwin \
    ADL_CSM_CONFIG_FILE="$scratch/missing-CSMctl.conf" \
    ADL_CSM_OBSERVATORY_CONFIG_FILE="$scratch/missing-observatory.conf" \
    ADL_CSM_OBSERVATORY_PUBLIC_ORIGIN="$origin" \
    ./CSMctl start >"$log" 2>&1; then
    printf 'CSMctl accepted invalid public origin: %s\n' "$origin" >&2
    exit 1
  fi
  grep -F "$expected" "$log" >/dev/null || {
    printf 'CSMctl rejected %s for the wrong reason; expected %s\n' "$origin" "$expected" >&2
    sed -n '1,40p' "$log" >&2
    exit 1
  }
}

bash -n CSMctl
require_source 'ALLOW_LOCALHOST_8000_ORIGIN="${ADL_CSM_ALLOW_LOCALHOST_8000_ORIGIN:-0}"'
require_source 'OBSERVATORY_PUBLIC_ORIGIN="${ADL_CSM_OBSERVATORY_PUBLIC_ORIGIN:-}"'
require_source 'additional_allowed_origins = []'
require_source 'additional_allowed_origins = ["http://localhost:8000"]'
require_source 'additional_allowed_origins = [\"http://localhost:8000\", \"$OBSERVATORY_PUBLIC_ORIGIN\"]'
require_source 'additional_allowed_origins = [\"$OBSERVATORY_PUBLIC_ORIGIN\"]'
require_source 'validate_observatory_public_origin "$OBSERVATORY_PUBLIC_ORIGIN"'

expect_invalid_origin "http://wuji.dev.csm.agent-logic.ai:8765" "invalid_observatory_public_origin expected=https_origin"
expect_invalid_origin "https://user:pass@wuji.dev.csm.agent-logic.ai:8765" "invalid_observatory_public_origin expected=host_and_optional_port"
expect_invalid_origin "https://wuji.dev.csm.agent-logic.ai:8765/path" "invalid_observatory_public_origin expected=host_and_optional_port"
expect_invalid_origin "https://localhost:8765" "invalid_observatory_public_origin expected=dns_host"
expect_invalid_origin "https://observatory.dev.agent-logic.ai" "duplicate_observatory_public_origin"
expect_invalid_origin "https://OBSERVATORY.DEV.AGENT-LOGIC.AI:443" "duplicate_observatory_public_origin"
expect_invalid_origin "https://wuji..agent-logic.ai:8765" "invalid_observatory_public_origin expected=dns_host"
expect_invalid_origin "https://-wuji.agent-logic.ai:8765" "invalid_observatory_public_origin expected=host_and_optional_port"
expect_invalid_origin "https://wuji-.agent-logic.ai:8765" "invalid_observatory_public_origin expected=dns_host"
expect_invalid_origin "https://wuji.dev.csm.agent-logic.ai:0" "invalid_observatory_public_origin expected=valid_port"
expect_invalid_origin "https://wuji.dev.csm.agent-logic.ai:65536" "invalid_observatory_public_origin expected=valid_port"
