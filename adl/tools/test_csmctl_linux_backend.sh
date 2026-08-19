#!/usr/bin/env bash
set -Eeuo pipefail

ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)
for script in "$ROOT/CSMctl" "$ROOT/start_CSM.sh" "$ROOT/adl/tools/test_csmctl_linux_backend.sh"; do
  bash -n "$script"
done
SCRATCH=$(mktemp -d "$ROOT/.adl/csmctl-linux-test.XXXXXX")
cleanup() {
  if [[ -f "$SCRATCH/state/supervisor.pid" ]]; then
    read -r pid _ < "$SCRATCH/state/supervisor.pid" || true
    [[ "$pid" =~ ^[0-9]+$ ]] && kill -TERM "$pid" 2>/dev/null || true
  fi
  if [[ "${ADL_CSM_TEST_KEEP_SCRATCH:-0}" == "1" ]]; then
    printf 'retained scratch: %s\n' "$SCRATCH" >&2
  else
    rm -rf "$SCRATCH"
  fi
}
trap cleanup EXIT

mkdir -p "$SCRATCH/bin" "$SCRATCH/service" "$SCRATCH/state" "$SCRATCH/generated/bin"
cat > "$SCRATCH/bin/curl" <<'SH'
#!/usr/bin/env bash
pid_file=${ADL_CSM_PID_FILE:?}
output=""
while [[ $# -gt 0 ]]; do
  case "$1" in
    -o) output=$2; shift 2 ;;
    -w) shift 2 ;;
    *) shift ;;
  esac
done
code=000
if [[ "${ADL_CSM_TEST_CURL_ALWAYS_READY:-0}" == "1" ]]; then
  code=200
elif [[ -s "$pid_file" ]]; then
  pid=$(tr -d '[:space:]' < "$pid_file")
  if [[ "$pid" =~ ^[0-9]+$ ]] && kill -0 "$pid" 2>/dev/null; then code=200; fi
fi
[[ -n "$output" ]] && printf '{}\n' > "$output"
printf '%s' "$code"
SH
cat > "$SCRATCH/bin/kernel" <<'SH'
#!/usr/bin/env bash
trap 'exit 0' TERM INT
while true; do sleep 1; done
SH
cat > "$SCRATCH/bin/vector" <<'SH'
#!/usr/bin/env bash
exit 0
SH
cat > "$SCRATCH/bin/launchctl" <<'SH'
#!/usr/bin/env bash
printf '%s\n' "$*" >> "${ADL_CSM_TEST_LAUNCHCTL_LOG:?}"
if [[ "$1" == print ]]; then printf 'pid = 4242\n'; fi
SH
chmod +x "$SCRATCH/bin/"*

cat > "$SCRATCH/service/runtime.env" <<'EOF'
ADL_RUNTIME_CONTROL_PUBLIC_KEY_HEX=aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
ADL_RUNTIME_OPERATION_PUBLIC_KEY_HEX=bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb
ADL_RUNTIME_CONTINUITY_SIGNING_KEY_HEX=cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc
ADL_RUNTIME_OBSERVATORY_TOKEN=issue426-test-token
EOF
chmod 600 "$SCRATCH/service/runtime.env"

process_match_env=()
if [[ "$(uname -s)" != "Linux" ]]; then
  process_match_env=(ADL_CSM_TEST_PROCESS_MATCH=1)
fi
common=(
  env
  "PATH=$SCRATCH/bin:$PATH"
  ADL_CSM_TEST_MODE=1
  ADL_CSM_TEST_OS=Linux
  "${process_match_env[@]}"
  "ADL_CSM_REPO_ROOT=$ROOT"
  "ADL_CSM_SERVICE_DIR=$SCRATCH/service"
  "ADL_CSM_STATE_DIR=$SCRATCH/state"
  "ADL_CSM_GENERATED_DIR=$SCRATCH/generated"
  "ADL_CSM_GENERATED_BIN_DIR=$SCRATCH/generated/bin"
  "ADL_CSM_ENV_FILE=$SCRATCH/service/runtime.env"
  "ADL_CSM_PID_FILE=$SCRATCH/state/kernel.pid"
  "ADL_CSM_SUPERVISOR_PID_FILE=$SCRATCH/state/supervisor.pid"
  "ADL_CSM_LEASE_PID_FILE=$SCRATCH/state/lease.pid"
  "ADL_CSM_LEASE_INFO_FILE=$SCRATCH/state/lease.env"
  "ADL_CSM_LEASE_SERVER_FILE=$SCRATCH/generated/lease.py"
  "ADL_CSM_RUNNER_FILE=$SCRATCH/generated/runner.sh"
  "ADL_CSM_PLIST_FILE=$SCRATCH/generated/runtime.plist"
  "ADL_CSM_LOG_FILE=$SCRATCH/state/runtime.log"
  "ADL_CSM_PROBE_FILE=$SCRATCH/state/probe.json"
  "ADL_CSM_READY_PROBE_FILE=$SCRATCH/state/ready.json"
  "ADL_CSM_KERNEL_BIN=$SCRATCH/bin/kernel"
  "ADL_CSM_VECTOR_BIN=$SCRATCH/bin/vector"
  "ADL_CSM_LAUNCH_WORKING_DIR=$ROOT"
)
if [[ -L "/proc/$$/exe" && "$(basename "$(readlink "/proc/$$/exe")")" == qemu-* ]]; then
  common+=(ADL_CSM_TEST_ALLOW_EMULATED_EXE=1)
  [[ " ${common[*]} " == *" ADL_CSM_TEST_ALLOW_EMULATED_EXE=1 "* ]]
fi

absent_status_output=$("${common[@]}" ADL_CSM_TEST_CURL_ALWAYS_READY=1 "$ROOT/CSMctl" status)
[[ "$absent_status_output" == *"pid=none state=not_started_by_start_CSM"* ]]
[[ "$absent_status_output" == *"status=pass"* ]]
if "${common[@]}" ADL_CSM_TEST_CURL_ALWAYS_READY=1 "$ROOT/CSMctl" restart >"$SCRATCH/unowned-restart.out" 2>&1; then
  echo "unowned Linux restart unexpectedly passed" >&2
  exit 1
fi
grep -F 'restart_requires_owned_runtime_service' "$SCRATCH/unowned-restart.out" >/dev/null

start_output=$("${common[@]}" "$ROOT/CSMctl" start)
[[ "$start_output" == *"backend=linux-process host_os=Linux"* ]]
[[ "$start_output" == *"status=pass"* ]]
supervisor_record=$(<"$SCRATCH/state/supervisor.pid")
read -r supervisor_pid supervisor_start_time <<<"$supervisor_record"
kill -0 "$supervisor_pid"
if [[ -r "/proc/$supervisor_pid/cmdline" ]]; then
  tr '\0' '\n' < "/proc/$supervisor_pid/cmdline" | grep -Fx "$SCRATCH/generated/runner.sh" >/dev/null
fi

status_output=$("${common[@]}" "$ROOT/start_CSM.sh" status)
[[ "$status_output" == *"state=running"* ]]
[[ "$status_output" == *"status=pass"* ]]

foreign_pid=$$
foreign_start_time=1
foreign_match_override=(ADL_CSM_TEST_PROCESS_MATCH=0)
if [[ -r "/proc/$foreign_pid/stat" ]]; then
  foreign_start_time=$(python3 - "$foreign_pid" <<'PY'
import sys
stat = open(f"/proc/{sys.argv[1]}/stat", encoding="utf-8").read()
print(stat[stat.rfind(")") + 2:].split()[19])
PY
)
  foreign_match_override=()
fi
printf '%s %s\n' "$foreign_pid" "$foreign_start_time" > "$SCRATCH/state/supervisor.pid"
if "${common[@]}" "${foreign_match_override[@]}" "$ROOT/CSMctl" stop >"$SCRATCH/foreign.out" 2>&1; then
  echo "foreign Linux PID ownership unexpectedly passed" >&2
  exit 1
fi
grep -F 'linux_supervisor_pid_not_owned' "$SCRATCH/foreign.out" >/dev/null
if [[ -r "/proc/$supervisor_pid/stat" ]]; then
  printf '%s %s\n' "$supervisor_pid" "$((supervisor_start_time + 1))" > "$SCRATCH/state/supervisor.pid"
  if "${common[@]}" "$ROOT/CSMctl" stop >"$SCRATCH/stale.out" 2>&1; then
    echo "stale Linux process identity unexpectedly passed" >&2
    exit 1
  fi
  grep -F 'linux_supervisor_pid_not_owned' "$SCRATCH/stale.out" >/dev/null
fi
printf '%s\n' "$supervisor_record" > "$SCRATCH/state/supervisor.pid"

restart_output=$("${common[@]}" "$ROOT/start_CSM.sh" restart)
[[ "$restart_output" == *"status=stopped"* ]]
[[ "$restart_output" == *"status=pass"* ]]
read -r restarted_pid _ < "$SCRATCH/state/supervisor.pid"
[[ "$restarted_pid" != "$supervisor_pid" ]]

stop_output=$("${common[@]}" "$ROOT/start_CSM.sh" stop)
[[ "$stop_output" == *"status=stopped"* ]]
[[ ! -e "$SCRATCH/state/supervisor.pid" ]]

: > "$SCRATCH/launchctl.log"
darwin_output=$(env PATH="$SCRATCH/bin:$PATH" ADL_CSM_TEST_MODE=1 ADL_CSM_TEST_OS=Darwin \
  ADL_CSM_TEST_CURL_ALWAYS_READY=1 \
  ADL_CSM_TEST_LAUNCHCTL_LOG="$SCRATCH/launchctl.log" ADL_CSM_REPO_ROOT="$ROOT" \
  ADL_CSM_SERVICE_DIR="$SCRATCH/service" ADL_CSM_STATE_DIR="$SCRATCH/state" \
  ADL_CSM_PID_FILE="$SCRATCH/state/kernel.pid" \
  ADL_CSM_RUNTIME_BASE=https://localhost:20997 "$ROOT/CSMctl" status)
[[ "$darwin_output" == *"launch_label="*"pid=4242"* ]]
grep -F 'print gui/' "$SCRATCH/launchctl.log" >/dev/null

if env ADL_CSM_TEST_MODE=1 ADL_CSM_TEST_OS=Plan9 "$ROOT/CSMctl" urls >"$SCRATCH/unsupported.out" 2>&1; then
  echo "unsupported operating system unexpectedly passed" >&2
  exit 1
fi
grep -F 'unsupported_operating_system:Plan9' "$SCRATCH/unsupported.out" >/dev/null

if "${common[@]}" "$ROOT/CSMctl" observatory status >"$SCRATCH/observatory.out" 2>&1; then
  echo "Linux Observatory control unexpectedly passed" >&2
  exit 1
fi
grep -F 'observatory_control_not_supported_on_Linux' "$SCRATCH/observatory.out" >/dev/null

echo "PASS: CSMctl Linux backend lifecycle and platform routing"
