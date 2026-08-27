#!/usr/bin/env bash
set -euo pipefail

test -f ".csdlc/issues/558/index.json"
test -d ".csdlc/evidence/558"

python3 - <<'PY'
import json
from pathlib import Path

index = json.loads(Path(".csdlc/issues/558/index.json").read_text())
review = index.get("review")
publication = index.get("publication")

if review is not None:
    assert review.get("completed") is True, "review exists but is not complete"

if publication is not None:
    assert publication.get("issue") == 558, "publication must bind issue 558"
PY
