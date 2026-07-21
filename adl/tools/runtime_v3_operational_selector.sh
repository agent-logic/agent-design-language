#!/usr/bin/env bash
set -euo pipefail

if [ "$#" -ne 3 ] || [ "$1" != "activate" ] || [ "$2" != "--selector" ]; then
  echo "usage: runtime_v3_operational_selector.sh activate --selector <selector-directory>" >&2
  exit 64
fi

selector=$3
state_dir=${ADL_RUNTIME_V3_SELECTOR_STATE_DIR:-}
if [ -z "$state_dir" ]; then
  echo "ADL_RUNTIME_V3_SELECTOR_STATE_DIR is required" >&2
  exit 64
fi
if [ ! -d "$selector" ] || [ ! -x "$selector/launch" ]; then
  echo "selector must be a directory containing an executable launch file" >&2
  exit 65
fi

selector=$(cd "$selector" && pwd -P)
mkdir -p "$state_dir"
state_dir=$(cd "$state_dir" && pwd -P)
pid_file="$state_dir/runtime.pid"
current_file="$state_dir/current-selector"
log_file="$state_dir/runtime.log"
shutdown_grace_ms=${ADL_RUNTIME_V3_SELECTOR_SHUTDOWN_GRACE_MS:-30000}
case "$shutdown_grace_ms" in
  *[!0-9]*|'') echo "selector shutdown grace must be a positive integer" >&2; exit 64 ;;
esac
if [ "$shutdown_grace_ms" -eq 0 ]; then
  echo "selector shutdown grace must be a positive integer" >&2
  exit 64
fi
shutdown_attempts=$(( (shutdown_grace_ms + 49) / 50 ))

stop_current() {
  if [ ! -f "$pid_file" ]; then
    return 0
  fi
  pid=$(cat "$pid_file")
  case "$pid" in *[!0-9]*|'') echo "selector PID state is invalid" >&2; exit 66 ;; esac
  if kill -0 "$pid" 2>/dev/null; then
    kill -TERM "$pid"
    attempts=0
    while kill -0 "$pid" 2>/dev/null; do
      attempts=$((attempts + 1))
      if [ "$attempts" -ge "$shutdown_attempts" ]; then
        kill -KILL "$pid" 2>/dev/null || true
        break
      fi
      sleep 0.05
    done
  fi
  rm -f "$pid_file"
}

stop_current
if [ -f "$log_file" ]; then
  mv "$log_file" "$log_file.previous"
fi
: > "$log_file"
"$selector/launch" >>"$log_file" 2>&1 &
pid=$!
sleep 0.1
if ! kill -0 "$pid" 2>/dev/null; then
  wait "$pid" || status=$?
  echo "selected Runtime v3 process exited before readiness (status=${status:-0})" >&2
  exit 70
fi

pid_tmp="$pid_file.tmp.$$"
current_tmp="$current_file.tmp.$$"
printf '%s\n' "$pid" > "$pid_tmp"
printf '%s\n' "$selector" > "$current_tmp"
mv "$pid_tmp" "$pid_file"
mv "$current_tmp" "$current_file"
printf 'runtime_v3_selector=active selector=%s pid=%s\n' "$selector" "$pid"
