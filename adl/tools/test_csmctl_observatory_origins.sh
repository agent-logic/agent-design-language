#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$repo_root"
mkdir -p "$repo_root/.adl/runs"
scratch="$(mktemp -d "$repo_root/.adl/runs/issue-550-origin.XXXXXX")"
trap 'rm -rf "$scratch"' EXIT

mkdir -p "$scratch/bin"
cat > "$scratch/bin/curl" <<'SH'
#!/usr/bin/env bash
output=""
while [[ $# -gt 0 ]]; do
  case "$1" in
    -o) output="$2"; shift 2 ;;
    -w) shift 2 ;;
    *) shift ;;
  esac
done
[[ -n "$output" ]] && printf '{}\n' > "$output"
printf '200'
SH
cat > "$scratch/bin/kernel" <<'SH'
#!/usr/bin/env bash
exit 0
SH
cat > "$scratch/bin/vector" <<'SH'
#!/usr/bin/env bash
exit 0
SH
chmod +x "$scratch/bin/"*

cat > "$scratch/runtime.env" <<'EOF'
ADL_RUNTIME_CONTROL_PUBLIC_KEY_HEX=aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
ADL_RUNTIME_OPERATION_PUBLIC_KEY_HEX=bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb
ADL_RUNTIME_CONTINUITY_SIGNING_KEY_HEX=cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc
ADL_RUNTIME_OBSERVATORY_TOKEN=issue550-test-token
EOF
chmod 600 "$scratch/runtime.env"

require_source() {
  local needle="$1"
  grep -F "$needle" CSMctl >/dev/null || {
    printf 'missing CSMctl source fragment: %s\n' "$needle" >&2
    exit 1
  }
}

render_origins() {
  local name="$1"
  local localhost_enabled="$2"
  local public_origin="$3"
  local expected="$4"
  local case_root="$scratch/$name"
  mkdir -p "$case_root/service" "$case_root/state" "$case_root/generated"
  env \
    "PATH=$scratch/bin:$PATH" \
    ADL_CSM_TEST_MODE=1 \
    ADL_CSM_TEST_OS=Darwin \
    "ADL_CSM_REPO_ROOT=$repo_root" \
    "ADL_CSM_SERVICE_DIR=$case_root/service" \
    "ADL_CSM_STATE_DIR=$case_root/state" \
    "ADL_CSM_GENERATED_DIR=$case_root/generated" \
    "ADL_CSM_ENV_FILE=$scratch/runtime.env" \
    "ADL_CSM_KERNEL_BIN=$scratch/bin/kernel" \
    "ADL_CSM_VECTOR_BIN=$scratch/bin/vector" \
    "ADL_CSM_ALLOW_LOCALHOST_8000_ORIGIN=$localhost_enabled" \
    "ADL_CSM_OBSERVATORY_PUBLIC_ORIGIN=$public_origin" \
    ./CSMctl start >"$case_root/output.log"
  grep -Fx "$expected" "$case_root/generated/runtime-init.current.toml" >/dev/null || {
    printf 'generated origin mismatch for %s; expected %s\n' "$name" "$expected" >&2
    grep -F 'additional_allowed_origins' "$case_root/generated/runtime-init.current.toml" >&2 || true
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

render_origins empty 0 "" 'additional_allowed_origins = []'
render_origins localhost-only 1 "" 'additional_allowed_origins = ["http://localhost:8000"]'
render_origins public-only 0 "https://WUJI.DEV.CSM.AGENT-LOGIC.AI:8765" \
  'additional_allowed_origins = ["https://wuji.dev.csm.agent-logic.ai:8765"]'
render_origins combined 1 "https://WUJI.DEV.CSM.AGENT-LOGIC.AI:8765" \
  'additional_allowed_origins = ["http://localhost:8000", "https://wuji.dev.csm.agent-logic.ai:8765"]'
