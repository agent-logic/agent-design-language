#!/usr/bin/env bash
set -euo pipefail

root="$(git rev-parse --show-toplevel)"

python3 - <<'PY'
import json
from pathlib import Path

manifest = json.loads(Path("docs/csdlc-v3/v3-command-manifest.json").read_text())
expected = {"github", "github-issue", "github-pr", "pr-state", "publish", "review"}
routes = {entry["command"]: entry for entry in manifest["commands"]}
missing = sorted(expected - routes.keys())
if missing:
    raise SystemExit(f"missing #629 routes in manifest: {missing}")
for route in sorted(expected):
    entry = routes[route]
    if entry.get("owner_issue") != 629:
        raise SystemExit(f"{route} owner_issue is {entry.get('owner_issue')}, expected 629")
    if entry.get("authority_status") != "not_live":
        raise SystemExit(f"{route} authority_status is {entry.get('authority_status')}, expected not_live")
    if entry.get("implementation_status") not in {"fail_closed", "implemented"}:
        raise SystemExit(f"{route} has unsupported implementation_status {entry.get('implementation_status')}")
if manifest.get("one_binary") != "csdlc":
    raise SystemExit("manifest no longer declares one_binary=csdlc")
if manifest.get("operational_authority") is not False:
    raise SystemExit("v3 must remain non-authoritative before #505")
print("manifest handoff ok")
PY

if git diff --name-only origin/main...HEAD -- csdlc-v2 | grep -q .; then
  echo "csdlc-v2 source changed in #629 scope" >&2
  git diff --name-only origin/main...HEAD -- csdlc-v2 >&2
  exit 1
fi

if git diff --name-only origin/main...HEAD | grep -E '(^|/)private/tmp|/private/tmp' >/dev/null; then
  echo "private tmp path leaked into changed path set" >&2
  exit 1
fi

echo "no v2 source changes"

cargo test --manifest-path "$root/csdlc-v3/Cargo.toml" --test remote_publication_commands
