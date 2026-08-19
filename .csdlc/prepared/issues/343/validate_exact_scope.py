#!/usr/bin/env python3
import subprocess, sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[4]
BASE = subprocess.check_output(["git","merge-base","HEAD","origin/main"], cwd=ROOT, text=True).strip()
EXACT = {
    ".csdlc/locks/343.lock",
    ".csdlc/prepared/issues/343/design.md", ".csdlc/prepared/issues/343/diagram.mmd",
    ".csdlc/prepared/issues/343/bootstrap-request.json", ".csdlc/prepared/issues/343/validate_preparation_bundle.py",
    ".csdlc/prepared/issues/343/validate_sprint_readiness.py", ".csdlc/prepared/issues/343/validate_exact_scope.py",
    ".csdlc/evidence/343/terminal-children.json",
    ".csdlc/evidence/343/terminal-256.json", ".csdlc/evidence/343/terminal-341.json",
    ".csdlc/evidence/343/terminal-5835.json", ".csdlc/evidence/343/terminal-5839.json",
    ".csdlc/evidence/343/child-256-review.json", ".csdlc/evidence/343/child-256-checks.json",
    ".csdlc/evidence/343/child-341-review.json", ".csdlc/evidence/343/child-341-checks.json",
    ".csdlc/evidence/343/historical-wp17.json", ".csdlc/evidence/343/historical-wp19.json",
    ".csdlc/evidence/343/sprint-review.json",
    ".csdlc/evidence/343/preparation-contract.log",
    ".csdlc/evidence/343/terminal-child-census.log",
    "docs/milestones/v0.92/review/sprint_343/SPRINT_CLOSEOUT_PACKET.md",
    ".csdlc/issues/343/audit.jsonl", ".csdlc/issues/343/index.json",
    ".csdlc/issues/343/cards/sip.md", ".csdlc/issues/343/cards/sip.values.json",
    ".csdlc/issues/343/cards/stp.md", ".csdlc/issues/343/cards/stp.values.json",
    ".csdlc/issues/343/cards/spp.md", ".csdlc/issues/343/cards/spp.values.json",
    ".csdlc/issues/343/cards/vpp.md", ".csdlc/issues/343/cards/vpp.values.json",
    ".csdlc/issues/343/cards/srp.md", ".csdlc/issues/343/cards/srp.values.json",
    ".csdlc/issues/343/cards/sor.md", ".csdlc/issues/343/cards/sor.values.json",
}
def lines(argv): return {x for x in subprocess.check_output(argv, cwd=ROOT, text=True).splitlines() if x}
paths = lines(["git","diff","--name-only",BASE,"--"]) | lines(["git","diff","--cached","--name-only","--"]) | lines(["git","diff","--name-only","--"]) | lines(["git","ls-files","--others","--exclude-standard"])
outside = sorted(p for p in paths if p not in EXACT)
if outside: print("out-of-scope paths: " + ", ".join(outside), file=sys.stderr); raise SystemExit(1)
for argv in (["git","diff","--check",BASE,"--"],["git","diff","--check","--cached","--"],["git","diff","--check","--"]):
    if subprocess.run(argv, cwd=ROOT).returncode: raise SystemExit(1)
tracked = lines(["git","ls-files"])
for name in sorted(paths - tracked):
    path = ROOT / name
    if path.is_file():
        result = subprocess.run(["git","diff","--no-index","--check","/dev/null",str(path)], cwd=ROOT, text=True, capture_output=True)
        if result.stdout or result.stderr:
            sys.stdout.write(result.stdout); sys.stderr.write(result.stderr); raise SystemExit(1)
        if result.returncode not in (0,1): raise SystemExit(result.returncode)
print(f"PASS exact issue #343 scope ({len(paths)} paths)")
