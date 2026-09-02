#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"

for path in AGENTS.md docs/onboarding.md docs/architecture/ADL_ARCHITECTURE.md; do
  test -f "$repo_root/$path"
done

python3 - "$repo_root" <<'PY'
from pathlib import Path
import sys

root = Path(sys.argv[1])
agents = (root / "AGENTS.md").read_text()

required_terms = [
    ["v2", "operational authority"],
    ["C-SDLC v3", "construction", "cutover"],
    ["Closes #"],
]
for terms in required_terms:
    if not all(term in agents for term in terms):
        raise SystemExit(f"AGENTS.md missing required guidance terms: {terms}")

architecture = (root / "docs/architecture/ADL_ARCHITECTURE.md").read_text()
if "SIP -> STP -> SPP -> VPP -> SRP -> SOR" not in architecture:
    raise SystemExit("architecture doc does not preserve six-card lifecycle including VPP")

onboarding = (root / "docs/onboarding.md").read_text()
if "v3" not in onboarding.lower():
    raise SystemExit("onboarding does not mention v3 cutover readiness")

print("v3 guidance scan: pass")
PY
