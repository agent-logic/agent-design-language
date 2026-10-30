#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
BIN="${ADL_ISSUE414_CONTINUITY_BIN:-$ROOT/adl/target/debug/adl_resident_shepherd_continuity}"
HOST_CLASS="${ADL_ISSUE414_RESTORE_HOST_CLASS:-aws}"
INPUT="${1:?usage: issue414_restore_and_admit.sh INPUT RUNTIME_ROOT OUTPUT}"
RUNTIME_ROOT="${2:?usage: issue414_restore_and_admit.sh INPUT RUNTIME_ROOT OUTPUT}"
OUTPUT="${3:?usage: issue414_restore_and_admit.sh INPUT RUNTIME_ROOT OUTPUT}"
expected="$(jq -er '.runtime_volume_identity_sha256|select(test("^[0-9a-f]{64}$"))' "$INPUT")"
if [[ "$HOST_CLASS" == aws ]]; then
  command -v findmnt >/dev/null; command -v lsblk >/dev/null
  source="$(findmnt -no SOURCE --target "$RUNTIME_ROOT")"
  serial="$(lsblk -ndo SERIAL "$source" | head -1 | tr -d '[:space:]')"
  [[ -n "$serial" ]] || { echo "restore-host retained volume serial unavailable" >&2; exit 66; }
  if [[ "$serial" == vol* && "$serial" != vol-* ]]; then serial="vol-${serial#vol}"; fi
  observed="$(printf '%s' "$serial" | shasum -a 256 | awk '{print $1}')"
  [[ "$observed" == "$expected" ]] || { echo "restore-host mounted EBS identity mismatch" >&2; exit 66; }
elif [[ "$HOST_CLASS" != reference ]]; then
  echo "restore host class must be aws or reference" >&2; exit 64
fi
"$BIN" restore --input "$INPUT" --runtime-root "$RUNTIME_ROOT" --output "$OUTPUT"
