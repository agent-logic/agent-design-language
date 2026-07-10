#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
ADL_DIR="$ROOT_DIR/adl"
FAMILY_CONFIG="$ADL_DIR/config/slow_proof_families.v0.91.6.json"

family=""
mode="run"
partition=""

usage() {
  cat <<'USAGE'
Usage:
  adl/tools/run_slow_proof_family.sh --family <id|all> [--list|--run|--print-plan|--json] [--partition <spec>]

Modes:
  --list        Run `cargo nextest list` for the selected family feature.
  --run         Run `cargo nextest run` for the selected family feature. Default.
  --print-plan  Print key=value plan lines and exit.
  --json        Print the selected family plan as JSON and exit.
  --partition   Optional cargo-nextest partition spec, for example count:1/4.
USAGE
}

while [ "$#" -gt 0 ]; do
  case "$1" in
    --family)
      family="${2:-}"
      shift 2
      ;;
    --list)
      mode="list"
      shift
      ;;
    --run)
      mode="run"
      shift
      ;;
    --print-plan)
      mode="print-plan"
      shift
      ;;
    --json)
      mode="json"
      shift
      ;;
    --partition)
      partition="${2:-}"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "unknown argument: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

if [ -z "$family" ]; then
  echo "run_slow_proof_family: --family is required" >&2
  usage >&2
  exit 2
fi

family_payload="$(
  python3 - "$FAMILY_CONFIG" "$family" <<'PY'
import json
import sys
from pathlib import Path

config = json.loads(Path(sys.argv[1]).read_text())
if config.get("schema_version") != "adl.slow_proof_families.v1":
    raise SystemExit("unsupported slow-proof family config schema")
family_id = sys.argv[2]
def family_payload(family):
    selectors = family.get("module_selectors", [])
    if not selectors:
        raise SystemExit(f"slow-proof family is missing module_selectors: {family.get('id')}")
    return {
        "id": family["id"],
        "feature": family["feature"],
        "proof_role": family.get("proof_role", "slow_proof"),
        "description": family.get("description", ""),
        "module_selectors": selectors,
        "sample_tests": family.get("sample_tests", []),
        "umbrella_feature": config.get("umbrella_feature", "slow-proof-tests"),
    }
families = config.get("families", [])
if family_id == "all":
    selectors = []
    seen = set()
    for family in families:
        for selector in family.get("module_selectors", []):
            if selector not in seen:
                selectors.append(selector)
                seen.add(selector)
    if not selectors:
        raise SystemExit("slow-proof family config is missing module selectors")
    print(json.dumps({
        "id": "all",
        "feature": config.get("umbrella_feature", "slow-proof-tests"),
        "proof_role": "slow_proof",
        "description": "All configured slow-proof runtime_v2 families.",
        "module_selectors": selectors,
        "sample_tests": [
            sample
            for family in families
            for sample in family.get("sample_tests", [])
        ],
        "umbrella_feature": config.get("umbrella_feature", "slow-proof-tests"),
    }))
    raise SystemExit(0)
for family in families:
    if family.get("id") == family_id:
        print(json.dumps(family_payload(family)))
        break
else:
    raise SystemExit(f"unknown slow-proof family: {family_id}")
PY
)" || {
  echo "run_slow_proof_family: failed to resolve family '$family'" >&2
  exit 2
}

feature="$(python3 -c 'import json,sys; print(json.loads(sys.stdin.read())["feature"])' <<<"$family_payload")"
description="$(python3 -c 'import json,sys; print(json.loads(sys.stdin.read())["description"])' <<<"$family_payload")"
filter_expression="$(
  python3 -c 'import json,re,sys; p=json.loads(sys.stdin.read()); print(" | ".join("test(/^" + re.escape(s) + "/)" for s in p["module_selectors"]))' <<<"$family_payload"
)"

command_list=(cargo nextest list --lib --features "$feature" -E "$filter_expression")
command_run=(cargo nextest run --lib --features "$feature" -E "$filter_expression")
if [ -n "$partition" ]; then
  command_run+=(--partition "$partition")
fi
command_run+=(--status-level all --final-status-level slow)

case "$mode" in
  print-plan)
    printf 'family=%s\n' "$family"
    printf 'feature=%s\n' "$feature"
    printf 'description=%s\n' "$description"
    printf 'filter_expression=%s\n' "$filter_expression"
    printf 'list_command=%q ' "${command_list[@]}"
    printf '\n'
    printf 'run_command=%q ' "${command_run[@]}"
    printf '\n'
    ;;
  json)
    python3 - <<'PY' "$family_payload"
import json
import re
import sys

payload = json.loads(sys.argv[1])
filter_expression = " | ".join(
    "test(/^" + re.escape(selector) + "/)"
    for selector in payload["module_selectors"]
)
payload["list_command"] = [
    "cargo", "nextest", "list", "--lib", "--features", payload["feature"], "-E", filter_expression
]
payload["run_command"] = [
    "cargo", "nextest", "run", "--lib", "--features", payload["feature"],
    "-E", filter_expression,
    "--status-level", "all", "--final-status-level", "slow",
]
print(json.dumps(payload, indent=2, sort_keys=True))
PY
    ;;
  list)
    (
      cd "$ADL_DIR"
      "${command_list[@]}"
    )
    ;;
  run)
    (
      cd "$ADL_DIR"
      "${command_run[@]}"
    )
    ;;
esac
