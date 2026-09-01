#!/usr/bin/env bash
set -euo pipefail

for card in sip stp spp vpp srp sor; do
  test -f ".csdlc/issues/596/cards/${card}.md"
  test -f ".csdlc/issues/596/cards/${card}.values.json"
done

test -f ".csdlc/issues/596/index.json"
test -f ".csdlc/prepared/issues/596/pr-create-request.json"

python3 - <<'PY'
import json
from pathlib import Path

body = json.loads(Path(".csdlc/prepared/issues/596/pr-create-request.json").read_text())["body"]
if "Closes #596" not in body:
    raise SystemExit("PR body must visibly close #596")
for forbidden in ("Closes #505", "Fixes #505", "Resolves #505"):
    if forbidden in body:
        raise SystemExit(f"PR body must not close #505: {forbidden}")
for required in ("Part-Of #505", "Part-Of #534"):
    if required not in body:
        raise SystemExit(f"PR body must retain non-closing linkage: {required}")

index = json.loads(Path(".csdlc/issues/596/index.json").read_text())
cards = set(index["cards"])
if cards != {"sip", "stp", "spp", "vpp", "srp", "sor"}:
    raise SystemExit(f"unexpected card set: {sorted(cards)}")
if index["repository"] != "agent-logic/agent-design-language":
    raise SystemExit("issue repository drift")
PY
