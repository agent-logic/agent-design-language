#!/usr/bin/env bash
set -Eeuo pipefail

ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)
GIT_COMMON_DIR=$(git -C "$ROOT" rev-parse --path-format=absolute --git-common-dir)
LIVE_ROOT=$(cd "$(dirname "$GIT_COMMON_DIR")" && pwd -P)
RUNBOOK="$ROOT/docs/tooling/START_CSM_RUNBOOK.md"
SCRATCH=$(mktemp -d "$ROOT/.adl/csmctl-route-test.XXXXXX")
trap 'rm -rf "$SCRATCH"' EXIT

bash -n "$ROOT/CSMctl"
bash -n "$ROOT/start_CSM.sh"
bash -n "$ROOT/adl/tools/test_csmctl_linux_backend.sh"

mkdir -p "$SCRATCH/bin"
for command in curl launchctl kill; do
  cat > "$SCRATCH/bin/$command" <<'SH'
#!/usr/bin/env bash
printf '%s\n' "unexpected external command: $(basename "$0") $*" >> "${ADL_CSM_ROUTE_TEST_EXTERNAL_LOG:?}"
exit 97
SH
  chmod +x "$SCRATCH/bin/$command"
done

external_log="$SCRATCH/external.log"
: > "$external_log"
common=(
  env
  "PATH=$SCRATCH/bin:$PATH"
  "ADL_CSM_ROUTE_TEST_EXTERNAL_LOG=$external_log"
  ADL_CSM_TEST_MODE=1
  ADL_CSM_TEST_OS=Linux
  "ADL_CSM_REPO_ROOT=$ROOT"
)

runtime_verbs=(open start up restart status stop logs urls rotate-continuity-state)
for verb in "${runtime_verbs[@]}"; do
  output="$SCRATCH/$verb.out"
  if "${common[@]}" "$ROOT/CSMctl" "$verb" >"$output" 2>&1; then
    echo "legacy Runtime verb unexpectedly passed: $verb" >&2
    exit 1
  fi
  grep -F "reason=legacy_runtime_control_removed command=$verb" "$output" >/dev/null
  grep -F "Use: \"$LIVE_ROOT/.adl/runtime-v3/current/bin/csm\" runtime-v3 <start|stop|status|reload>" "$output" >/dev/null
done

if "${common[@]}" "$ROOT/CSMctl" >"$SCRATCH/default.out" 2>&1; then
  echo "empty legacy invocation unexpectedly passed" >&2
  exit 1
fi
grep -F 'reason=legacy_runtime_control_removed command=default' "$SCRATCH/default.out" >/dev/null
grep -F "Use: \"$LIVE_ROOT/.adl/runtime-v3/current/bin/csm\" runtime-v3 <start|stop|status|reload>" "$SCRATCH/default.out" >/dev/null

(
  cd "$SCRATCH"
  if "${common[@]}" "$ROOT/CSMctl" status >"$SCRATCH/alternate-cwd.out" 2>&1; then
    echo "alternate-cwd legacy Runtime status unexpectedly passed" >&2
    exit 1
  fi
)
grep -F "Use: \"$LIVE_ROOT/.adl/runtime-v3/current/bin/csm\"" "$SCRATCH/alternate-cwd.out" >/dev/null
grep -F -- "--init \"$LIVE_ROOT/.adl/runtime-v3/live/runtime-init.toml\"" "$SCRATCH/alternate-cwd.out" >/dev/null

(
  cd "$SCRATCH"
  "$ROOT/CSMctl" --help >"$SCRATCH/alternate-cwd-help.out"
)
grep -F "\"$LIVE_ROOT/.adl/runtime-v3/current/bin/csm\" runtime-v3" "$SCRATCH/alternate-cwd-help.out" >/dev/null
grep -F -- "--init \"$LIVE_ROOT/.adl/runtime-v3/live/runtime-init.toml\"" "$SCRATCH/alternate-cwd-help.out" >/dev/null

[[ ! -s "$external_log" ]]

help_output=$("$ROOT/CSMctl" --help)
[[ "$help_output" == *'Usage: ./CSMctl observatory'* ]]
[[ "$help_output" == *'.adl/runtime-v3/current/bin/csm runtime-v3'* ]]
[[ "$help_output" != *'Usage: ./CSMctl [open|start'* ]]

if "${common[@]}" "$ROOT/CSMctl" observatory status >"$SCRATCH/observatory.out" 2>&1; then
  echo "Linux Observatory control unexpectedly passed" >&2
  exit 1
fi
grep -F 'observatory_control_not_supported_on_Linux' "$SCRATCH/observatory.out" >/dev/null

grep -F '.adl/runtime-v3/current/bin/csm' "$RUNBOOK" >/dev/null
grep -F '.adl/runtime-v3/live/runtime-init.toml' "$RUNBOOK" >/dev/null
grep -F 'com.agentlogic.adl-runtime-v3' "$RUNBOOK" >/dev/null
grep -F 'service_loaded' "$RUNBOOK" >/dev/null
grep -F 'listener_ready' "$RUNBOOK" >/dev/null
grep -F 'guardian_process_id' "$RUNBOOK" >/dev/null
grep -F 'runtime_process_id' "$RUNBOOK" >/dev/null
grep -F 'active_init_hash' "$RUNBOOK" >/dev/null
grep -F 'observability_ready' "$RUNBOOK" >/dev/null

if grep -E 'macOS backend: launchd label `com\.agentlogic\.start-csm`|Runtime binary: `\.adl/runtime-v3-service/' "$RUNBOOK" >/dev/null; then
  echo "runbook still presents the legacy Runtime route as authoritative" >&2
  exit 1
fi

echo "PASS: canonical Runtime route, legacy refusal, Observatory separation, and runbook guards"
