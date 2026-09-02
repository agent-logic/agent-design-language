#!/usr/bin/env bash
set -euo pipefail

required=(
  "ADL_VERTEX_AI_LIVE_APPROVED"
  "ADL_VERTEX_AI_PROJECT"
  "ADL_VERTEX_AI_LOCATION"
  "ADL_VERTEX_AI_MODEL"
)

for name in "${required[@]}"; do
  if [[ -z "${!name:-}" ]]; then
    echo "live Vertex AI smoke is not authorized or configured: missing $name" >&2
    exit 2
  fi
done

if [[ "${ADL_VERTEX_AI_LIVE_APPROVED}" != "1" ]]; then
  echo "live Vertex AI smoke requires ADL_VERTEX_AI_LIVE_APPROVED=1" >&2
  exit 2
fi

echo "live Vertex AI smoke entrypoint is configured; implementation command is supplied by #528 postbind proof"
