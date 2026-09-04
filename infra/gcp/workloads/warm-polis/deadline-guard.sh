#!/usr/bin/env bash

# Run one create-capable operation only inside the immutable qualification
# window. Terraform providers are children of the CLI, so terminate them before
# the parent and force termination after a short grace period.
adl_run_paid_operation() {
  [ "$(date +%s)" -lt "$paid_deadline_epoch" ] || return 124
  "$@" &
  operation_pid=$!
  while kill -0 "$operation_pid" 2>/dev/null; do
    if [ "$(date +%s)" -ge "$paid_deadline_epoch" ]; then
      pkill -TERM -P "$operation_pid" 2>/dev/null || true
      kill -TERM "$operation_pid" 2>/dev/null || true
      for _ in 1 2 3 4 5; do
        kill -0 "$operation_pid" 2>/dev/null || break
        sleep 1
      done
      if kill -0 "$operation_pid" 2>/dev/null; then
        pkill -KILL -P "$operation_pid" 2>/dev/null || true
        kill -KILL "$operation_pid" 2>/dev/null || true
      fi
      wait "$operation_pid" 2>/dev/null || true
      echo "paid operation exceeded the immutable qualification deadline" >&2
      return 124
    fi
    sleep 2
  done
  wait "$operation_pid"
}
