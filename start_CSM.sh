#!/usr/bin/env bash
set -Eeuo pipefail

# Start or diagnose the real repo-local CSM / Runtime v3 service.
# This script does not print credentials. It uses the existing local service
# package and stable repo binaries, and writes only local runtime state.

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="${ADL_CSM_REPO_ROOT:-$SCRIPT_DIR}"
GIT_COMMON_DIR="$(git -C "$REPO_ROOT" rev-parse --path-format=absolute --git-common-dir 2>/dev/null || true)"
PRIMARY_REPO_ROOT="$REPO_ROOT"
if [[ -n "$GIT_COMMON_DIR" && -d "$GIT_COMMON_DIR" ]]; then
  PRIMARY_REPO_ROOT="$(cd "$(dirname "$GIT_COMMON_DIR")" && pwd)"
fi
if [[ ! -d "$PRIMARY_REPO_ROOT/.adl" ]]; then
  PRIMARY_REPO_ROOT="$(git -C "$REPO_ROOT" rev-parse --show-toplevel 2>/dev/null || printf '%s' "$REPO_ROOT")"
fi

SERVICE_ROOT="$REPO_ROOT"
if [[ ! -d "$SERVICE_ROOT/.adl/runtime-v3-service" && -d "$PRIMARY_REPO_ROOT/.adl/runtime-v3-service" ]]; then
  SERVICE_ROOT="$PRIMARY_REPO_ROOT"
fi
BIN_ROOT="$REPO_ROOT"
if [[ ! -x "$BIN_ROOT/.adl/bin/adl-runtime-kernel" && -x "$PRIMARY_REPO_ROOT/.adl/bin/adl-runtime-kernel" ]]; then
  BIN_ROOT="$PRIMARY_REPO_ROOT"
fi

SERVICE_DIR="${ADL_CSM_SERVICE_DIR:-$SERVICE_ROOT/.adl/runtime-v3-service}"
ENV_FILE="${ADL_CSM_ENV_FILE:-$SERVICE_DIR/runtime.env}"
STATE_DIR="${ADL_CSM_STATE_DIR:-$SERVICE_DIR/state}"
GENERATED_DIR="${ADL_CSM_GENERATED_DIR:-$SERVICE_DIR/generated}"
CREDENTIAL_DIR="$GENERATED_DIR/credentials"
RUNTIME_PORT="${ADL_CSM_RUNTIME_PORT:-20997}"
RUNTIME_BASE="${ADL_CSM_RUNTIME_BASE:-https://localhost:$RUNTIME_PORT}"
RUNTIME_ADDRESS="${ADL_CSM_RUNTIME_ADDRESS:-127.0.0.1:$RUNTIME_PORT}"
RUNTIME_PUBLIC_BASE_URL="${ADL_CSM_RUNTIME_PUBLIC_BASE_URL:-$RUNTIME_BASE}"
RUNTIME_SERVER_NAME="${ADL_CSM_RUNTIME_SERVER_NAME:-localhost}"
INIT_FILE="${ADL_CSM_RUNTIME_INIT:-$GENERATED_DIR/runtime-init.current.toml}"
PID_FILE="${ADL_CSM_PID_FILE:-$STATE_DIR/start_CSM.pid}"
LOG_FILE="${ADL_CSM_LOG_FILE:-$STATE_DIR/start_CSM.log}"
PROBE_FILE="$STATE_DIR/start_CSM.probe"
KERNEL_BIN="${ADL_CSM_KERNEL_BIN:-$BIN_ROOT/.adl/bin/adl-runtime-kernel}"
VECTOR_BIN="${ADL_CSM_VECTOR_BIN:-$BIN_ROOT/.adl/bin/vector}"
CERT_FILE="${ADL_CSM_TLS_CERT:-$SERVICE_DIR/tls/localhost-cert.pem}"
KEY_FILE="${ADL_CSM_TLS_KEY:-$SERVICE_DIR/tls/localhost-key.pem}"
TRUST_ROOTS_FILE="${ADL_CSM_TLS_TRUST_ROOTS:-$SERVICE_DIR/tls/test-ca-cert.pem}"
OBSERVATORY_ENTRY="${ADL_CSM_OBSERVATORY_ENTRY:-$REPO_ROOT/demos/html-observatory/index.html}"

usage() {
  cat <<'USAGE'
Usage: ./start_CSM.sh [up|status|stop|logs|urls]

Commands:
  up      Start the real Runtime v3 service if needed, probe it, and print URLs.
  status  Probe the Runtime v3 service without starting anything.
  stop    Stop only the Runtime process started by this script.
  logs    Show recent service logs from this script.
  urls    Print Runtime and Observatory URLs/paths.
USAGE
}

info() {
  printf 'start_CSM %s\n' "$*"
}

fail() {
  printf 'start_CSM status=failed reason=%s\n' "$*" >&2
  exit 1
}

require_file() {
  [[ -f "$1" ]] || fail "missing_$2: $1"
}

require_executable() {
  [[ -x "$1" ]] || fail "missing_or_not_executable_$2: $1"
}

load_service_env() {
  require_file "$ENV_FILE" "service_env"
  local shell_options="$-"
  set +u
  set -a
  # shellcheck source=/dev/null
  . "$ENV_FILE"
  set +a
  if [[ "$shell_options" == *u* ]]; then
    set -u
  fi
}

require_env() {
  local name="$1"
  [[ -n "${!name:-}" ]] || fail "missing_required_service_env_$name"
}

write_secret_file() {
  umask 077
  printf '%s\n' "$2" > "$1"
}

ensure_current_init() {
  load_service_env
  require_executable "$KERNEL_BIN" "kernel_binary"
  require_executable "$VECTOR_BIN" "vector_binary"
  require_file "$CERT_FILE" "tls_certificate"
  require_file "$KEY_FILE" "tls_private_key"
  require_file "$TRUST_ROOTS_FILE" "tls_trust_roots"

  require_env ADL_RUNTIME_CONTROL_PUBLIC_KEY_HEX
  require_env ADL_RUNTIME_OPERATION_PUBLIC_KEY_HEX
  require_env ADL_RUNTIME_CONTINUITY_SIGNING_KEY_HEX
  require_env ADL_RUNTIME_OBSERVATORY_TOKEN

  mkdir -p "$STATE_DIR" "$GENERATED_DIR" "$CREDENTIAL_DIR"
  write_secret_file "$CREDENTIAL_DIR/control-public-key.hex" "$ADL_RUNTIME_CONTROL_PUBLIC_KEY_HEX"
  write_secret_file "$CREDENTIAL_DIR/operation-public-key.hex" "$ADL_RUNTIME_OPERATION_PUBLIC_KEY_HEX"
  write_secret_file "$CREDENTIAL_DIR/migration-decision-public-key.hex" "$ADL_RUNTIME_OPERATION_PUBLIC_KEY_HEX"
  write_secret_file "$CREDENTIAL_DIR/continuity-signing-key.hex" "$ADL_RUNTIME_CONTINUITY_SIGNING_KEY_HEX"
  write_secret_file "$CREDENTIAL_DIR/observatory-token.txt" "$ADL_RUNTIME_OBSERVATORY_TOKEN"
  write_secret_file "$CREDENTIAL_DIR/acip-write-token.txt" "${ADL_RUNTIME_ACIP_WRITE_TOKEN:-$ADL_RUNTIME_OBSERVATORY_TOKEN}"

  umask 077
  cat > "$CREDENTIAL_DIR/birth-witness-trust.json" <<EOF
{
  "schema": "adl.runtime.birth_witness_trust.v1",
  "authority_context": "runtime-v3-local-service",
  "authorities": [{
    "witness_id": "local-operation-witness",
    "role": "runtime_operator",
    "signing_key_id": "runtime-operations",
    "verifying_key": "$ADL_RUNTIME_OPERATION_PUBLIC_KEY_HEX"
  }]
}
EOF

  cat > "$INIT_FILE" <<EOF
schema = "adl.runtime_v3.init.v1"
state_root = "$STATE_DIR"

[binaries]
kernel_path = "$KERNEL_BIN"

[paths]
continuity_dir = "continuity"
tls_dir = "tls"
credentials_dir = "credentials"
observability_dir = "observability"

[api]
address = "$RUNTIME_ADDRESS"
public_base_url = "$RUNTIME_PUBLIC_BASE_URL"
bind_attempts = 20
bind_retry_millis = 100
websocket_auth_timeout_millis = 5000
websocket_refresh_millis = 1000
websocket_max_frame_bytes = 65536

[api.tls]
certificate_chain_path = "$CERT_FILE"
private_key_path = "$KEY_FILE"
trust_roots_path = "$TRUST_ROOTS_FILE"
server_name = "$RUNTIME_SERVER_NAME"

[observatory]
allowed_origins = ["https://localhost:8765", "https://localhost:8766", "https://observatory.dev.agent-logic.ai"]

[kernel]
recorder_capacity = 1024
control_history_capacity = 1024
checkpoint_channel_capacity = 4
component_readiness_timeout_millis = 5000
observability_poll_millis = 50
weather_stale_after_millis = 2000
guardian_lease_connect_millis = 5000
guardian_lease_auth_millis = 5000
trusted_time_sample_timeout_millis = 3000
trusted_time_max_offset_millis = 5000
trusted_time_max_round_trip_millis = 2000
trusted_time_retry_millis = 1000
trusted_time_refresh_millis = 60000

[credentials]
control_public_key_path = "$CREDENTIAL_DIR/control-public-key.hex"
control_key_id = "operator"
control_principal = "operator"
operation_public_key_path = "$CREDENTIAL_DIR/operation-public-key.hex"
operation_key_id = "runtime-operations"
migration_decision_public_key_path = "$CREDENTIAL_DIR/migration-decision-public-key.hex"
migration_decision_key_id = "runtime-migration-decisions"
migration_decision_key_generation = 1
continuity_signing_key_path = "$CREDENTIAL_DIR/continuity-signing-key.hex"
continuity_key_id = "runtime-continuity"
observatory_token_path = "$CREDENTIAL_DIR/observatory-token.txt"
acip_write_token_path = "$CREDENTIAL_DIR/acip-write-token.txt"
birth_witness_trust_manifest_path = "$CREDENTIAL_DIR/birth-witness-trust.json"
continuity_min_generation = 0
sntp_server = "time.cloudflare.com"

[shutdown]
checkpoint_deadline_millis = 5000
kernel_grace_millis = 10000
api_drain_millis = 3000
guardian_margin_millis = 500

[guardian]
restart_budget = 3
backoff_base_millis = 100
backoff_cap_millis = 5000
healthy_window_millis = 60000
lease_auth_timeout_millis = 5000
lease_auth_attempts = 3
capture_max_bytes = 65536
capture_drain_grace_millis = 2000
configuration_exit_codes = [64]

[qualification]
readiness_timeout_millis = 10000
readiness_poll_millis = 50
shutdown_wait_millis = 50000

[observability_pipeline]
vector_binary_path = "$VECTOR_BIN"
service_name = "adl-runtime-v3"
revision = "0.92.0"
guardian_id = "start-csm-local"
lifecycle_suite = "runtime-v3"
lifecycle_run = "start-csm-local"
lifecycle_cycle = "start-csm-local-cycle"
trace_filter = "adl_runtime_kernel=info,adl_runtime=info"
otlp_timeout_millis = 5000
vector_startup_attempts = 3
vector_startup_backoff_millis = 100
vector_shutdown_limit_millis = 3000
drain_timeout_millis = 5000
vector_config_path = "config/runtime-v3-vector.json"
ingress_spool_path = "spool/runtime-v3.current.jsonl"
master_log_path = "durable/master.log.jsonl"
audit_path = "durable/master-log-audit.json"
sequence_checkpoint_path = "durable/sequence.json"
vector_data_dir = "vector-data"
spool_max_bytes = 8388608
spool_retained_files = 4

[weather]
sample_millis = 1000
history_capacity = 60
disk_warning_free_bytes = 5368709120
disk_stop_free_bytes = 2147483648
disk_recover_free_bytes = 8589934592
memory_warning_used_basis_points = 8500
memory_stop_used_basis_points = 9500
memory_recover_used_basis_points = 7500
cpu_warning_basis_points = 9000
cpu_stop_basis_points = 9800
cpu_recover_basis_points = 8000
checkpoint_deadline_millis = 5000
snapshot_concurrency = 4
EOF
}

pid_is_alive() {
  local pid="$1"
  [[ "$pid" =~ ^[0-9]+$ ]] && kill -0 "$pid" 2>/dev/null
}

recorded_pid() {
  [[ -f "$PID_FILE" ]] || return 1
  local pid
  pid="$(tr -d '[:space:]' < "$PID_FILE")"
  [[ -n "$pid" ]] || return 1
  printf '%s' "$pid"
}

curl_code() {
  mkdir -p "$STATE_DIR"
  local url="$1"
  curl -k -sS --max-time 3 -o "$PROBE_FILE" -w '%{http_code}' "$url" 2>/dev/null || printf '000'
}

probe_runtime() {
  local ready_code observatory_code health_code
  ready_code="$(curl_code "$RUNTIME_BASE/v1/ready")"
  observatory_code="$(curl_code "$RUNTIME_BASE/v1/observatory")"
  health_code="$(curl_code "$RUNTIME_BASE/v1/health")"
  info "probe /v1/ready http=$ready_code"
  info "probe /v1/observatory http=$observatory_code"
  info "probe /v1/health http=$health_code"
  [[ "$observatory_code" == "200" && "$health_code" == "200" && ( "$ready_code" == "200" || "$ready_code" == "503" ) ]]
}

urls_cmd() {
  info "runtime=$RUNTIME_BASE"
  info "observatory=$OBSERVATORY_ENTRY?runtime=v3&runtimeApiBase=$RUNTIME_BASE&live=1"
}

status_cmd() {
  info "repo_root=$REPO_ROOT"
  info "service_dir=$SERVICE_DIR"
  info "kernel_bin=$KERNEL_BIN"
  info "init_file=$INIT_FILE"
  local pid
  if pid="$(recorded_pid 2>/dev/null)" && pid_is_alive "$pid"; then
    info "pid=$pid state=alive"
  else
    info "pid=none state=not_started_by_start_CSM"
  fi
  if probe_runtime; then
    info "status=pass runtime_base=$RUNTIME_BASE"
  else
    fail "runtime_not_ready_or_not_serving"
  fi
}

up_cmd() {
  ensure_current_init
  if probe_runtime; then
    info "status=pass runtime_base=$RUNTIME_BASE"
    urls_cmd
    return 0
  fi

  local pid
  if pid="$(recorded_pid 2>/dev/null)" && pid_is_alive "$pid"; then
    fail "owned_runtime_pid_alive_but_probe_failed pid=$pid log=$LOG_FILE"
  fi

  : > "$LOG_FILE"
  info "starting kernel=$KERNEL_BIN"
  (
    cd "$REPO_ROOT"
    nohup "$KERNEL_BIN" serve --init "$INIT_FILE" >> "$LOG_FILE" 2>&1 < /dev/null &
    printf '%s\n' "$!" > "$PID_FILE"
  )

  for _ in $(seq 1 80); do
    if probe_runtime; then
      info "status=pass runtime_base=$RUNTIME_BASE"
      urls_cmd
      return 0
    fi
    sleep 0.25
  done
  fail "runtime_started_but_not_ready log=$LOG_FILE"
}

stop_cmd() {
  local pid
  if pid="$(recorded_pid 2>/dev/null)" && pid_is_alive "$pid"; then
    kill "$pid"
    info "status=stop_requested pid=$pid"
  else
    info "status=not_running"
  fi
}

logs_cmd() {
  if [[ -f "$LOG_FILE" ]]; then
    tail -80 "$LOG_FILE"
  else
    info "log_absent=$LOG_FILE"
  fi
}

case "${1:-up}" in
  up) up_cmd ;;
  status) status_cmd ;;
  stop) stop_cmd ;;
  logs) logs_cmd ;;
  urls) urls_cmd ;;
  -h|--help|help) usage ;;
  *) usage >&2; fail "unknown_command=$1" ;;
esac
