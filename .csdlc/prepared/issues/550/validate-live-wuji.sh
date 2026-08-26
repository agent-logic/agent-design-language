#!/usr/bin/env bash
set -euo pipefail

runtime_base="${ADL_CSM_LIVE_RUNTIME_BASE:-https://wuji.dev.csm.agent-logic.ai:20997}"
local_observatory="${ADL_CSM_LOCAL_OBSERVATORY_URL:-http://localhost:8000/index.html?runtime=v3&runtimeApiBase=${runtime_base}&live=1}"
trusted_observatory="${ADL_CSM_TRUSTED_OBSERVATORY_URL:-https://wuji.dev.csm.agent-logic.ai:8765/index.html?runtime=v3&runtimeApiBase=${runtime_base}&live=1}"
mkdir -p .adl/runs
evidence_dir="$(mktemp -d .adl/runs/issue-550-live-validation.XXXXXX)"
trap 'rm -rf "$evidence_dir"' EXIT

curl -fsS --max-time 8 "$local_observatory" >/dev/null
curl -fsS --max-time 8 "$trusted_observatory" >/dev/null

for origin in "http://localhost:8000" "https://wuji.dev.csm.agent-logic.ai:8765"; do
  origin_label="$(printf '%s' "$origin" | tr -c '[:alnum:]' '_')"
  for endpoint in /v1/health /v1/ready /v1/observatory; do
    curl -fsS --max-time 8 \
      -H "Origin: $origin" \
      -D "$evidence_dir/${origin_label}-${endpoint//\//_}-headers.txt" \
      -o /dev/null \
      "${runtime_base}${endpoint}"
    grep -Fi "access-control-allow-origin: $origin" \
      "$evidence_dir/${origin_label}-${endpoint//\//_}-headers.txt" >/dev/null
  done
done

RUNTIME_BASE="$runtime_base" node <<'NODE'
const runtimeBase = new URL(process.env.RUNTIME_BASE);
runtimeBase.protocol = "wss:";
runtimeBase.pathname = "/v1/observatory/ws";
runtimeBase.search = "";
runtimeBase.hash = "";

await new Promise((resolve, reject) => {
  const socket = new WebSocket(runtimeBase);
  const timeout = setTimeout(() => {
    socket.close();
    reject(new Error("trusted Runtime WSS open timed out"));
  }, 8000);
  socket.addEventListener("open", () => {
    clearTimeout(timeout);
    socket.close(1000, "issue-550 validation complete");
    resolve();
  }, { once: true });
  socket.addEventListener("error", () => {
    clearTimeout(timeout);
    reject(new Error("trusted Runtime WSS open failed"));
  }, { once: true });
});
NODE
