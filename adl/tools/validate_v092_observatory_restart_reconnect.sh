#!/usr/bin/env bash
set -Eeuo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
MODE="${1:---contract}"
CSMCTL="${ROOT_DIR}/CSMctl"
APP_JS="${ROOT_DIR}/demos/html-observatory/app.js"
HTML_TEST="${ROOT_DIR}/adl/tools/test_html_observatory.sh"

GIT_COMMON_DIR="$(git -C "$ROOT_DIR" rev-parse --path-format=absolute --git-common-dir 2>/dev/null || true)"
PRIMARY_REPO_ROOT="$ROOT_DIR"
if [[ -n "$GIT_COMMON_DIR" && -d "$GIT_COMMON_DIR" ]]; then
  PRIMARY_REPO_ROOT="$(cd "$(dirname "$GIT_COMMON_DIR")" && pwd)"
fi
if [[ ! -d "$PRIMARY_REPO_ROOT/.adl" ]]; then
  PRIMARY_REPO_ROOT="$(git -C "$ROOT_DIR" rev-parse --show-toplevel 2>/dev/null || printf '%s' "$ROOT_DIR")"
fi
CSM="${ADL_CSM_RUNTIME_BIN:-$PRIMARY_REPO_ROOT/.adl/runtime-v3/current/bin/csm}"
RUNTIME_INIT="${ADL_CSM_RUNTIME_INIT:-$PRIMARY_REPO_ROOT/.adl/runtime-v3/live/runtime-init.toml}"
SERVICE_ROOT="$ROOT_DIR"
if [[ ! -d "$SERVICE_ROOT/.adl/runtime-v3-service" && -d "$PRIMARY_REPO_ROOT/.adl/runtime-v3-service" ]]; then
  SERVICE_ROOT="$PRIMARY_REPO_ROOT"
fi

SERVICE_DIR="${ADL_CSM_SERVICE_DIR:-$SERVICE_ROOT/.adl/runtime-v3-service}"
CONFIG_FILE="${ADL_CSM_CONFIG_FILE:-$SERVICE_DIR/CSMctl.conf}"
if [[ ! -f "$CONFIG_FILE" && -f "$SERVICE_DIR/start_CSM.conf" ]]; then
  CONFIG_FILE="$SERVICE_DIR/start_CSM.conf"
fi
if [[ -f "$CONFIG_FILE" ]]; then
  # shellcheck source=/dev/null
  . "$CONFIG_FILE"
fi

fail() {
  printf 'validate_v092_observatory_restart_reconnect status=failed reason=%s\n' "$*" >&2
  exit 1
}

pass() {
  printf 'validate_v092_observatory_restart_reconnect status=pass mode=%s\n' "$MODE"
}

require_file() {
  [[ -f "$1" ]] || fail "missing_file:$1"
}

require_executable() {
  [[ -x "$1" ]] || fail "missing_executable:$1"
}

require_source() {
  local path="$1"
  local needle="$2"
  grep -F -- "$needle" "$path" >/dev/null || fail "missing_contract:$needle in $path"
}

contract_check() {
  require_executable "$CSMCTL"
  require_executable "$HTML_TEST"
  require_file "$APP_JS"
  require_source "$APP_JS" 'health_endpoint: "/v1/health"'
  require_source "$APP_JS" 'fetchRuntimeV3Health(base)'
  require_source "$APP_JS" 'response.status !== 200'
  require_source "$CSMCTL" 'OBSERVATORY_SERVER_BIN'
  require_source "$CSMCTL" 'adl-observatory-static'
  require_source "$CSMCTL" '--daemon'
  require_source "${ROOT_DIR}/adl-runtime/src/bin/adl-observatory-static.rs" 'Router::new()'
  require_source "${ROOT_DIR}/adl-runtime/src/bin/adl-observatory-static.rs" 'path.push("index.html")'
  require_source "$CSMCTL" 'printf '\''OBSERVATORY_RUNTIME_BASE=%q\n'\'' "$OBSERVATORY_RUNTIME_BASE"'
  require_source "$CSMCTL" 'load_observatory_state || true'
  "$HTML_TEST"
}

curl_code() {
  local url="$1"
  if [[ "${ADL_CSM_PROBE_INSECURE_TLS:-0}" == "1" ]]; then
    curl -k -sS --max-time "${ADL_CSM_VALIDATOR_CURL_TIMEOUT_SECONDS:-5}" \
      -o /dev/null -w '%{http_code}' "$url" 2>/dev/null || printf '000'
  elif [[ -n "${ADL_CSM_API_TLS_TRUST_ROOTS:-}" ]]; then
    curl --cacert "$ADL_CSM_API_TLS_TRUST_ROOTS" -sS --max-time "${ADL_CSM_VALIDATOR_CURL_TIMEOUT_SECONDS:-5}" \
      -o /dev/null -w '%{http_code}' "$url" 2>/dev/null || printf '000'
  else
    curl -sS --max-time "${ADL_CSM_VALIDATOR_CURL_TIMEOUT_SECONDS:-5}" \
      -o /dev/null -w '%{http_code}' "$url" 2>/dev/null || printf '000'
  fi
}

curl_body() {
  local url="$1"
  local output="$2"
  if [[ "${ADL_CSM_PROBE_INSECURE_TLS:-0}" == "1" ]]; then
    curl -k -sS --max-time "${ADL_CSM_VALIDATOR_CURL_TIMEOUT_SECONDS:-5}" \
      -o "$output" "$url" 2>/dev/null
  elif [[ -n "${ADL_CSM_API_TLS_TRUST_ROOTS:-}" ]]; then
    curl --cacert "$ADL_CSM_API_TLS_TRUST_ROOTS" -sS --max-time "${ADL_CSM_VALIDATOR_CURL_TIMEOUT_SECONDS:-5}" \
      -o "$output" "$url" 2>/dev/null
  else
    curl -sS --max-time "${ADL_CSM_VALIDATOR_CURL_TIMEOUT_SECONDS:-5}" \
      -o "$output" "$url" 2>/dev/null
  fi
}

curl_options_code() {
  local url="$1"
  if [[ "${ADL_CSM_PROBE_INSECURE_TLS:-0}" == "1" ]]; then
    curl -k -sS --max-time "${ADL_CSM_VALIDATOR_CURL_TIMEOUT_SECONDS:-5}" \
      -X OPTIONS -H 'Origin: https://localhost:8765' -H 'Access-Control-Request-Method: POST' \
      -o /dev/null -w '%{http_code}' "$url" 2>/dev/null || printf '000'
  elif [[ -n "${ADL_CSM_API_TLS_TRUST_ROOTS:-}" ]]; then
    curl --cacert "$ADL_CSM_API_TLS_TRUST_ROOTS" -sS --max-time "${ADL_CSM_VALIDATOR_CURL_TIMEOUT_SECONDS:-5}" \
      -X OPTIONS -H 'Origin: https://localhost:8765' -H 'Access-Control-Request-Method: POST' \
      -o /dev/null -w '%{http_code}' "$url" 2>/dev/null || printf '000'
  else
    curl -sS --max-time "${ADL_CSM_VALIDATOR_CURL_TIMEOUT_SECONDS:-5}" \
      -X OPTIONS -H 'Origin: https://localhost:8765' -H 'Access-Control-Request-Method: POST' \
      -o /dev/null -w '%{http_code}' "$url" 2>/dev/null || printf '000'
  fi
}

curl_post_json_code() {
  local url="$1"
  if [[ "${ADL_CSM_PROBE_INSECURE_TLS:-0}" == "1" ]]; then
    curl -k -sS --max-time "${ADL_CSM_VALIDATOR_CURL_TIMEOUT_SECONDS:-5}" \
      -X POST -H 'Content-Type: application/json' --data '{}' \
      -o /dev/null -w '%{http_code}' "$url" 2>/dev/null || printf '000'
  elif [[ -n "${ADL_CSM_API_TLS_TRUST_ROOTS:-}" ]]; then
    curl --cacert "$ADL_CSM_API_TLS_TRUST_ROOTS" -sS --max-time "${ADL_CSM_VALIDATOR_CURL_TIMEOUT_SECONDS:-5}" \
      -X POST -H 'Content-Type: application/json' --data '{}' \
      -o /dev/null -w '%{http_code}' "$url" 2>/dev/null || printf '000'
  else
    curl -sS --max-time "${ADL_CSM_VALIDATOR_CURL_TIMEOUT_SECONDS:-5}" \
      -X POST -H 'Content-Type: application/json' --data '{}' \
      -o /dev/null -w '%{http_code}' "$url" 2>/dev/null || printf '000'
  fi
}

ws_status_code() {
  local url="$1"
  local origin="${2:-https://localhost:8766}"
  local cafile="${ADL_CSM_API_TLS_TRUST_ROOTS:-}"
  python3 - "$url" "$origin" "$cafile" <<'PY'
import base64
import os
import socket
import ssl
import sys
from urllib.parse import urlparse

url, origin, cafile = sys.argv[1:4]
parsed = urlparse(url)
host = parsed.hostname
port = parsed.port or 443
path = parsed.path or "/"
if parsed.query:
    path += "?" + parsed.query
context = ssl.create_default_context(cafile=cafile or None)
if os.environ.get("ADL_CSM_PROBE_INSECURE_TLS") == "1":
    context = ssl._create_unverified_context()
key = base64.b64encode(os.urandom(16)).decode("ascii")
request = (
    f"GET {path} HTTP/1.1\r\n"
    f"Host: {host}:{port}\r\n"
    "Upgrade: websocket\r\n"
    "Connection: Upgrade\r\n"
    "Sec-WebSocket-Version: 13\r\n"
    f"Sec-WebSocket-Key: {key}\r\n"
    f"Origin: {origin}\r\n"
    "\r\n"
).encode("ascii")
try:
    with socket.create_connection((host, port), timeout=5) as raw:
        with context.wrap_socket(raw, server_hostname=host) as tls:
            tls.sendall(request)
            response = tls.recv(4096).decode("iso-8859-1", errors="replace")
    first = response.splitlines()[0] if response else ""
    parts = first.split()
    print(parts[1] if len(parts) > 1 and parts[1].isdigit() else "000")
except Exception:
    print("000")
PY
}

json_field() {
  local file="$1"
  local field="$2"
  python3 - "$file" "$field" <<'PY'
import json
import sys
path, field = sys.argv[1:3]
with open(path, "r", encoding="utf-8") as handle:
    value = json.load(handle)
for part in field.split("."):
    value = value.get(part, {}) if isinstance(value, dict) else {}
print(value if isinstance(value, (str, int)) else "")
PY
}

agent_id_from_observatory() {
  local file="$1"
  python3 - "$file" <<'PY'
import json
import sys
with open(sys.argv[1], "r", encoding="utf-8") as handle:
    value = json.load(handle)
sample = (((value.get("agents") or {}).get("sample")) or [])
for agent in sample:
    ident = agent.get("id")
    if isinstance(ident, str) and ident:
        print(ident)
        break
PY
}

assert_get_200() {
  local runtime_base="$1"
  local endpoint="$2"
  [[ "$(curl_code "$runtime_base$endpoint")" == "200" ]] || fail "runtime_endpoint_not_200:$endpoint"
}

assert_options_allowed() {
  local runtime_base="$1"
  local endpoint="$2"
  local code
  code="$(curl_options_code "$runtime_base$endpoint")"
  [[ "$code" == "200" || "$code" == "204" ]] || fail "runtime_options_not_allowed:$endpoint:$code"
}

assert_post_reaches_route() {
  local runtime_base="$1"
  local endpoint="$2"
  local allowed="$3"
  local code
  code="$(curl_post_json_code "$runtime_base$endpoint")"
  [[ " $allowed " == *" $code "* ]] || fail "runtime_post_unexpected_status:$endpoint:$code"
}

assert_wss_reaches_route() {
  local runtime_base="$1"
  local endpoint="$2"
  local allowed="$3"
  local code
  code="$(ws_status_code "${runtime_base/https:/wss:}$endpoint")"
  [[ " $allowed " == *" $code "* ]] || fail "runtime_wss_unexpected_status:$endpoint:$code"
}

assert_exposed_read_routes() {
  local runtime_base="$1"
  local state_dir="$2"
  local observatory_body="$state_dir/CSMctl.observatory.feed.json"
  local agent_id
  for endpoint in \
    /v1/health \
    /v1/ready \
    /v1/metrics \
    /v1/openapi.json \
    /v1/docs/ \
    /v1/observatory \
    /v1/observatory/openapi.json \
    /v1/observatory/docs/ \
    /v1/agents; do
    assert_get_200 "$runtime_base" "$endpoint"
  done
  curl_body "$runtime_base/v1/observatory" "$observatory_body" || fail "runtime_observatory_body_unavailable"
  agent_id="$(agent_id_from_observatory "$observatory_body" || true)"
  if [[ -n "$agent_id" ]]; then
    assert_get_200 "$runtime_base" "/v1/agents/$agent_id"
  else
    printf 'validate_v092_observatory_restart_reconnect event=no_runtime_agent_sample route=/v1/agents/{agent_id} status=skipped\n'
  fi
  assert_options_allowed "$runtime_base" /v1/ready
  assert_options_allowed "$runtime_base" /v1/control
  assert_options_allowed "$runtime_base" /v1/layer8/recipient-acknowledgement
  assert_post_reaches_route "$runtime_base" /v1/control "400 401 403 409 410 503"
  assert_post_reaches_route "$runtime_base" /v1/layer8/recipient-acknowledgement "400 503"
  assert_wss_reaches_route "$runtime_base" /v1/acip/ws "101 401"
  assert_wss_reaches_route "$runtime_base" /v1/observatory/ws "101 403"
}

live_check() {
  require_executable "$CSMCTL"
  require_executable "$CSM"
  require_file "$RUNTIME_INIT"
  local state_dir="${ADL_CSM_STATE_DIR:-$ROOT_DIR/.adl/runtime-v3-service/state}"
  local runtime_base="${ADL_CSM_RUNTIME_BASE:-https://localhost:20997}"
  local observatory_state="${ADL_CSM_OBSERVATORY_STATE_FILE:-$state_dir/CSMctl.observatory.env}"
  local observatory_feed="$state_dir/CSMctl.observatory.feed.json"
  local first_status="$state_dir/csm.runtime-v3.status.before.json"
  local second_status="$state_dir/csm.runtime-v3.status.after.json"

  "$CSM" runtime-v3 start --init "$RUNTIME_INIT" --json
  "$CSM" runtime-v3 status --init "$RUNTIME_INIT" --json > "$first_status"
  assert_exposed_read_routes "$runtime_base" "$state_dir"
  "$CSMCTL" observatory stop >/dev/null 2>&1 || true
  "$CSMCTL" observatory start
  require_file "$observatory_state"
  # shellcheck source=/dev/null
  . "$observatory_state"
  local urls_output="$state_dir/CSMctl.urls.out"
  "$CSMCTL" observatory urls > "$urls_output"
  grep -F "observatory=$OBSERVATORY_URL" "$urls_output" >/dev/null \
    || fail "observatory_urls_cmd_stale_url"
  grep -F "observatory_runtime_api_base=$OBSERVATORY_RUNTIME_BASE" "$urls_output" >/dev/null \
    || fail "observatory_urls_cmd_stale_runtime_api_base"
  [[ "${OBSERVATORY_URL:-}" == *"runtime=v3"* ]] || fail "observatory_url_missing_runtime_v3"
  [[ "${OBSERVATORY_URL:-}" == *"runtimeApiBase="* ]] || fail "observatory_url_missing_runtime_api_base"
  [[ "$(curl_code "${OBSERVATORY_BASE:-}/")" == "200" ]] || fail "observatory_root_not_200"
  [[ "$(curl_code "${OBSERVATORY_BASE:-}/index.html")" == "200" ]] || fail "observatory_index_not_200"
  local expected_observatory_url="$OBSERVATORY_URL"
  local expected_observatory_runtime_base="$OBSERVATORY_RUNTIME_BASE"

  local first_incarnation_id
  first_incarnation_id="$(json_field "$observatory_feed" runtime_incarnation_id)"
  [[ -n "$first_incarnation_id" ]] || fail "runtime_feed_missing_initial_incarnation_id"
  "$CSM" runtime-v3 stop --init "$RUNTIME_INIT" --json
  local restart_output="$state_dir/CSMctl.restart.out"
  "$CSM" runtime-v3 start --init "$RUNTIME_INIT" --json > "$restart_output"
  "$CSM" runtime-v3 status --init "$RUNTIME_INIT" --json > "$second_status"
  "$CSMCTL" observatory urls > "$urls_output"
  grep -F "observatory=$expected_observatory_url" "$urls_output" >/dev/null \
    || fail "observatory_restart_stale_url"
  grep -F "observatory_runtime_api_base=$expected_observatory_runtime_base" "$urls_output" >/dev/null \
    || fail "observatory_restart_stale_runtime_api_base"
  assert_exposed_read_routes "$runtime_base" "$state_dir"
  local second_incarnation_id
  second_incarnation_id="$(json_field "$observatory_feed" runtime_incarnation_id)"
  [[ -n "$second_incarnation_id" ]] || fail "runtime_feed_missing_restarted_incarnation_id"
  [[ "$first_incarnation_id" != "$second_incarnation_id" ]] \
    || fail "runtime_restart_did_not_refresh_runtime_incarnation"
}

case "$MODE" in
  --contract)
    contract_check
    pass
    ;;
  --live)
    live_check
    pass
    ;;
  *)
    fail "unknown_mode:$MODE"
    ;;
esac
