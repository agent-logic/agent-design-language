#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
cd "$repo_root"

if [[ "${1:-}" == "--live-wuji" ]]; then
  primary_root="$(dirname "$(git rev-parse --git-common-dir)")"
  init="$primary_root/.adl/runtime-v3/live/runtime-init.toml"
  csm="$primary_root/.adl/runtime-v3/current/bin/csm"
  evidence_dir="$repo_root/.adl/runs/issue-640"
  mkdir -p "$evidence_dir"
  status="$evidence_dir/wuji-service-status.json"
  feed="$evidence_dir/wuji-observatory-feed.json"
  "$csm" runtime-v3 status --init "$init" --json >"$status"
  public_base_url="$(python3 -c 'import sys,tomllib; print(tomllib.load(open(sys.argv[1], "rb"))["api"]["public_base_url"])' "$init")"
  curl -fsSk "$public_base_url:20997/v1/observatory" >"$feed"
  python3 - "$status" "$feed" <<'PY'
import json, sys
status = json.load(open(sys.argv[1]))
feed = json.load(open(sys.argv[2]))
assert status["service_loaded"] and status["listener_ready"] and status["observability_ready"]
shepherd = next(agent for agent in feed["agents"]["sample"] if agent["id"] == "shepherd")
assert shepherd["name"] == "beacon.axioma"
assert shepherd["provider"] == "ollama" and shepherd["model"] == "qwen3:8b"
assert shepherd["state"] == "ready" and shepherd["communication_eligible"] is True
assert feed["health"]["observability_ready"] is True
print(json.dumps({"schema":"adl.issue_640.wuji_acceptance.v1","status":"pass","shepherd":shepherd}, indent=2))
PY
  ollama ps | grep -F 'qwen3:8b' | grep -F 'Forever'
  exit 0
fi

cargo nextest run \
  --locked \
  --manifest-path adl-runtime-kernel/Cargo.toml \
  --test assembly \
  --test shepherd \
  --test control \
  --test governed_operations \
  --test agent_roster \
  --test openapi_contract \
  --no-tests=fail \
  -E 'test(resident_shepherd) | test(shepherd_provider) | test(shepherd_model_health) | test(shepherd_readiness_consistency)'

cargo fmt --manifest-path adl-runtime-kernel/Cargo.toml --check
git diff --check
