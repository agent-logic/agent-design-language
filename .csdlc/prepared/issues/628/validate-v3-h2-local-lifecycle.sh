#!/usr/bin/env bash
set -euo pipefail

mode="${1:-all}"
root="$(git rev-parse --show-toplevel)"
manifest="$root/docs/csdlc-v3/v3-command-manifest.json"

check_manifest_handoff() {
  python3 - "$manifest" <<'PY'
import json, sys
path = sys.argv[1]
with open(path, "r", encoding="utf-8") as handle:
    data = json.load(handle)
commands = {entry["command"]: entry for entry in data["commands"]}
expected = ["issue", "bind", "edit", "validate", "doctor", "schedule", "shepherd", "eligibility"]
missing = [name for name in expected if name not in commands]
if missing:
    raise SystemExit(f"missing local routes in #627 manifest: {missing}")
if data.get("one_binary") != "csdlc":
    raise SystemExit("one_binary must be csdlc")
if data.get("operational_authority") is not False:
    raise SystemExit("v3 must remain non-authoritative before #505 cutover")
bad = [
    name for name in expected
    if commands[name].get("owner_issue") != 628
    or commands[name].get("implementation_status") != "implemented"
    or commands[name].get("authority_status") != "not_live"
]
if bad:
    raise SystemExit(f"local routes are not owned/statused for #628: {bad}")
print("manifest handoff ok")
PY
}

check_no_v2_source_change() {
  if git diff --name-only origin/main...HEAD -- csdlc-v2 | grep -q .; then
    echo "unexpected csdlc-v2 source diff" >&2
    git diff --name-only origin/main...HEAD -- csdlc-v2 >&2
    exit 1
  fi
  echo "no v2 source changes"
}

check_route_specific_proof() {
  cargo test --manifest-path "$root/csdlc-v3/Cargo.toml" --test local_commands
  cargo test --manifest-path "$root/csdlc-v3/Cargo.toml" --test real_issue_canary
}

case "$mode" in
  all)
    check_manifest_handoff
    check_no_v2_source_change
    check_route_specific_proof
    ;;
  manifest)
    check_manifest_handoff
    ;;
  no-v2-source-change)
    check_no_v2_source_change
    ;;
  route-specific-proof)
    check_route_specific_proof
    ;;
  *)
    echo "usage: $0 [all|manifest|no-v2-source-change|route-specific-proof]" >&2
    exit 64
    ;;
esac
