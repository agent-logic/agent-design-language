#!/usr/bin/env bash
# Product owns predicates and live validation; wrapper also rejects missing artifacts.
set -euo pipefail
if [[ $# -lt 2 || "$1" != "--binary" || ! -x "$2" ]]; then
  echo 'codefriend fitness CI requires --binary <installed-adl>' >&2
  exit 2
fi
binary="$2"
shift 2
script_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
python3 "$script_dir/codefriend_fitness_ci_artifacts.py" before 0 "$@"
if "$binary" codefriend fitness ci-run "$@"; then
  adapter_exit=0
else
  adapter_exit=$?
fi
# No pipe/upload result can mask the original failure. Only consistent artifacts
# may return the same 0/1/2 code; malformed/missing output always becomes error2.
python3 "$script_dir/codefriend_fitness_ci_artifacts.py" after "$adapter_exit" "$@"
