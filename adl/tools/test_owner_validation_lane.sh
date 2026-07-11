#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
RUNNER="$ROOT_DIR/adl/tools/run_owner_validation_lane.sh"
INSTALLER="$ROOT_DIR/adl/tools/install_owner_binaries.sh"

plan_output="$(bash "$RUNNER" all --build --print-plan)"
installer_bins="$(
  awk '
    /^  BINS=\($/ { capture = 1; next }
    capture && /^  \)$/ { exit }
    capture { print }
  ' "$INSTALLER" |
    tr -d '\\' |
    xargs -n1 |
    LC_ALL=C sort
)"
runner_bins="$(
  grep -o -- '--bin [[:alnum:]-]*' <<<"$plan_output" |
    awk '{ print $2 }' |
    LC_ALL=C sort
)"
if ! diff -u \
  <(printf '%s\n' "$installer_bins") \
  <(printf '%s\n' "$runner_bins"); then
  echo "owner validation build set does not match installer defaults" >&2
  exit 1
fi

for expected in \
  "cargo build owner binaries" \
  "C-SDLC wrapper migration contract" \
  "C-SDLC run ambiguity policy" \
  "C-SDLC control-plane observability contract" \
  "runtime CSM binary availability contract" \
  "runtime CSM binary availability guard" \
  "runtime compatibility boundary" \
  "review compatibility boundary" \
  "PASS run_owner_validation_lane surface=all"; do
  grep -Fq -- "$expected" <<<"$plan_output" || {
    echo "missing expected lane plan entry: $expected" >&2
    echo "$plan_output" >&2
    exit 1
  }
done

bash "$RUNNER" csdlc

set +e
bad_output="$(bash "$RUNNER" unknown 2>&1)"
bad_status=$?
set -e
[[ "$bad_status" -ne 0 ]] || {
  echo "unsupported lane unexpectedly passed" >&2
  exit 1
}
grep -Fq "unsupported argument 'unknown'" <<<"$bad_output" || {
  echo "unsupported lane did not report a useful error" >&2
  echo "$bad_output" >&2
  exit 1
}

echo "PASS test_owner_validation_lane"
