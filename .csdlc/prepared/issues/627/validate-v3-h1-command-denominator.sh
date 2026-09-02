#!/usr/bin/env bash
set -euo pipefail

mode="${1:-all}"
repo_root="$(git rev-parse --show-toplevel)"
manifest="${repo_root}/docs/csdlc-v3/v3-command-manifest.json"

validate_manifest() {
  python3 - "$manifest" <<'PY'
import json
import sys

path = sys.argv[1]
with open(path, "r", encoding="utf-8") as handle:
    data = json.load(handle)

commands = data.get("commands", [])
names = [entry.get("command") for entry in commands]
if data.get("one_binary") != "csdlc":
    raise SystemExit("one_binary must be csdlc")
if data.get("operational_authority") is not False:
    raise SystemExit("v3 manifest must remain non-authoritative before #505")
if data.get("denominator", {}).get("v2_entrypoints") != 21:
    raise SystemExit("v2 denominator must be 21")
if data.get("denominator", {}).get("remaining_replacement_routes") != 19:
    raise SystemExit("remaining replacement route count must be 19")
if len(names) != len(set(names)):
    raise SystemExit("duplicate v3 command names")
required = {
    "bind", "clean", "cutover", "doctor", "edit", "eligibility", "finish",
    "github", "github-issue", "github-pr", "install", "issue", "pr-state",
    "proof", "publish", "review", "schedule", "shadow", "shepherd", "soak",
    "validate",
}
present = set(names)
missing = sorted(required - present)
if missing:
    raise SystemExit(f"missing command routes: {missing}")
fail_closed = {
    entry["command"]
    for entry in commands
    if entry.get("implementation_status") == "fail_closed"
}
remaining = required - {"foundation", "local"}
already_partial = {"shadow", "validate"}
if len(fail_closed & (remaining - already_partial)) != 19:
    raise SystemExit("the 19 remaining replacement routes must fail closed in #627")
for command in already_partial:
    entry = next(item for item in commands if item["command"] == command)
    if entry.get("implementation_status") != "partial":
        raise SystemExit(f"{command} must remain partial construction evidence in #627")
for entry in commands:
    if entry.get("authority_status") not in {"read_only_construction", "not_live"}:
        raise SystemExit(f"bad authority status for {entry.get('command')}")
    if not entry.get("owner_issue"):
        raise SystemExit(f"missing owner issue for {entry.get('command')}")
print("manifest ok")
PY
}

validate_no_v2_source_change() {
  if git diff --name-only origin/main...HEAD -- csdlc-v2 | grep -q .; then
    echo "csdlc-v2 source changed in #627 diff" >&2
    git diff --name-only origin/main...HEAD -- csdlc-v2 >&2
    exit 1
  fi
  echo "no v2 source changes"
}

case "$mode" in
  manifest)
    validate_manifest
    ;;
  no-v2-source-change)
    validate_no_v2_source_change
    ;;
  all)
    validate_manifest
    validate_no_v2_source_change
    ;;
  *)
    echo "usage: $0 [manifest|no-v2-source-change|all]" >&2
    exit 2
    ;;
esac
