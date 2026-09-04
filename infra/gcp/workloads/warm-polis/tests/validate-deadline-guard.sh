#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "$0")/.." && pwd)"
source "$root/deadline-guard.sh"
state_dir="$(mktemp -d "$root/.deadline-test.XXXXXX")"
trap 'rm -rf "$state_dir"' EXIT

paid_deadline_epoch="$(( $(date +%s) + 10 ))"
adl_run_paid_operation true

paid_deadline_epoch="$(( $(date +%s) + 1 ))"
started="$(date +%s)"
if adl_run_paid_operation bash -c '
  trap "exit 0" TERM
  bash -c '\''trap "" TERM; while :; do :; done'\'' &
  echo "$!" >"$1/child.pid"
  wait
' _ "$state_dir"; then
  echo "deadline guard accepted an operation that exceeded its window" >&2
  exit 1
else
  rc=$?
fi
[ "$rc" -eq 124 ]
child_pid="$(cat "$state_dir/child.pid")"
! kill -0 "$child_pid" 2>/dev/null || {
  kill -KILL "$child_pid" 2>/dev/null || true
  echo "deadline guard orphaned a TERM-resistant provider child" >&2
  exit 1
}
elapsed="$(( $(date +%s) - started ))"
[ "$elapsed" -le 8 ] || {
  echo "deadline guard did not force termination within its bounded grace" >&2
  exit 1
}
echo "issue670_deadline_guard=pass"
