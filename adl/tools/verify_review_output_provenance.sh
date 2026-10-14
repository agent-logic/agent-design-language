#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="${ADL_TOOLING_MANIFEST_ROOT:-$(cd "$SCRIPT_DIR/../.." && pwd)}"
BINARY="${CSDLC_HISTORICAL_REVIEW_PROVENANCE_BIN:-}"
if [[ -z "$BINARY" || ! -f "$BINARY" || ! -x "$BINARY" ]]; then
  echo "ERROR: set CSDLC_HISTORICAL_REVIEW_PROVENANCE_BIN to the delivered C-SDLC historical provenance executable" >&2
  exit 1
fi
if [[ ! -d "$ROOT" ]]; then
  echo "ERROR: review input root does not exist" >&2
  exit 1
fi
# Historical card_review_output.v1 validation only; no current lifecycle authority.
exec "$BINARY" --root "$ROOT" "$@"
