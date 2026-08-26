#!/usr/bin/env bash
set -euo pipefail

runtime_base="${ADL_CSM_LIVE_RUNTIME_BASE:-https://wuji.dev.csm.agent-logic.ai:20997}"
local_observatory="${ADL_CSM_LOCAL_OBSERVATORY_URL:-http://localhost:8000/index.html?runtime=v3&runtimeApiBase=${runtime_base}&live=1}"
trusted_observatory="${ADL_CSM_TRUSTED_OBSERVATORY_URL:-https://wuji.dev.csm.agent-logic.ai:8765/index.html?runtime=v3&runtimeApiBase=${runtime_base}&live=1}"
evidence_dir=".csdlc/evidence/550/live-wuji"
mkdir -p "$evidence_dir"

curl -fsS --max-time 8 "$local_observatory" >/dev/null
curl -fsS --max-time 8 "$trusted_observatory" >/dev/null

for endpoint in /v1/health /v1/ready /v1/observatory; do
  curl -fsS --max-time 8 \
    -H "Origin: http://localhost:8000" \
    -D "$evidence_dir/${endpoint//\//_}-headers.txt" \
    -o "$evidence_dir/${endpoint//\//_}-body.json" \
    "${runtime_base}${endpoint}"
  grep -Fi "access-control-allow-origin: http://localhost:8000" "$evidence_dir/${endpoint//\//_}-headers.txt" >/dev/null
done
