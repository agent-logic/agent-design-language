#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "$0")/.." && pwd)"
source "$root/deadline-guard.sh"

paid_deadline_epoch="$(( $(date +%s) + 10 ))"
adl_run_paid_operation true

paid_deadline_epoch="$(( $(date +%s) + 1 ))"
started="$(date +%s)"
if adl_run_paid_operation bash -c 'trap "" TERM; while :; do :; done'; then
  echo "deadline guard accepted an operation that exceeded its window" >&2
  exit 1
else
  rc=$?
fi
[ "$rc" -eq 124 ]
elapsed="$(( $(date +%s) - started ))"
[ "$elapsed" -le 8 ] || {
  echo "deadline guard did not force termination within its bounded grace" >&2
  exit 1
}
echo "issue670_deadline_guard=pass"
