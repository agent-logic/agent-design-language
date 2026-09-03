#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"

for path in AGENTS.md docs/onboarding.md docs/architecture/ADL_ARCHITECTURE.md; do
  test -f "$repo_root/$path"
done
test -f "$repo_root/docs/csdlc-v3/CUTOVER_READINESS_NOTICE.md"
test -f "$repo_root/csdlc-v3/README.md"

python3 - "$repo_root" <<'PY'
from pathlib import Path
import sys

root = Path(sys.argv[1])
agents = (root / "AGENTS.md").read_text()
notice = (root / "docs/csdlc-v3/CUTOVER_READINESS_NOTICE.md").read_text()
readme = (root / "csdlc-v3/README.md").read_text()

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
if "cleanup removes only the exact registered worktree after\n   terminal evidence permits it" not in architecture:
    raise SystemExit("architecture doc has malformed cleanup lifecycle guidance")

onboarding = (root / "docs/onboarding.md").read_text()
if "v3" not in onboarding.lower():
    raise SystemExit("onboarding does not mention v3 cutover readiness")

for text in [
    "Status: advance notice only",
    "C-SDLC v3 is not the live authority yet",
    "advertises the planned one-binary surface",
    "read-only or fail-closed",
    "Raw `gh` lifecycle writes remain prohibited",
    "Closes #<issue>",
]:
    if text not in notice:
        raise SystemExit(f"cutover readiness notice missing {text}")

if "CUTOVER_READINESS_NOTICE.md" not in readme:
    raise SystemExit("v3 README does not link the cutover readiness notice")

print("v3 guidance scan: pass")
PY
