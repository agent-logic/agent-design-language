#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
# This entry point checks consistency only. Release mutations require separate
# operator authorization and are deliberately not dispatched by preflight.
if [[ $# == 1 && ( "$1" == --help || "$1" == -h ) ]]; then
  echo 'usage: adl/tools/release_ceremony.sh --request <native-release-preflight.json>'
  echo 'Read-only native v3 candidate preflight. No bypass, tag, push or release flags.'
  exit 0
fi
if [[ $# != 2 || "$1" != --request ]]; then
  echo 'release_ceremony: use --request <native-release-preflight.json>; bypass and mutation flags are unsupported' >&2
  exit 2
fi
NATIVE="$ROOT/.adl/bin/native-v3/csdlc"
[[ -x "$NATIVE" ]] || { echo 'release_ceremony: stable native v3 owner missing; install the reviewed owner before preflight' >&2; exit 2; }
# Resolve the request before changing directory.
REQUEST="$(cd "$(dirname "$2")" && pwd)/$(basename "$2")"
cd "$ROOT"
exec "$NATIVE" release-preflight --request "$REQUEST"
