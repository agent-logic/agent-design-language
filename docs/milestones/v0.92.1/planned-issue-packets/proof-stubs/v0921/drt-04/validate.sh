#!/usr/bin/env bash
set -euo pipefail

root="$(git rev-parse --show-toplevel)"
runner="$root/adl/tools/v0921/drt-04/run.sh"

if [[ ! -x "$runner" ]]; then
  echo "DRT-04 implementation must provide executable $runner" >&2
  exit 66
fi

"$runner" "$@"
ruby "$root/.csdlc/prepared/issues/184/validate-outcome.rb"
