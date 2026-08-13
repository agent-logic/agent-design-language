#!/usr/bin/env python3
import pathlib, subprocess, sys
root = pathlib.Path(__file__).resolve().parents[4]
allowed = {
 "adl-runtime/src/distributed/shepherd_serving_eligibility.rs",
 "adl-runtime/tests/distributed_shepherd_serving_eligibility.rs",
 "adl-runtime/src/distributed/mod.rs",
 "adl-runtime/src/distributed/serving_authority.rs",
 "adl/tools/check_coverage_impact.sh",
 "adl/tools/run_pr_fast_coverage_lane.sh",
 "adl/tools/test_check_coverage_impact.sh",
 "adl/tools/test_run_pr_fast_coverage_lane.sh",
}
changed = set(subprocess.check_output(["git","diff","--name-only","origin/main...HEAD"],cwd=root,text=True).splitlines())
permitted = allowed | {
 p for p in changed
 if p.startswith(".csdlc/issues/273/")
 or p.startswith(".csdlc/evidence/273/")
 or p.startswith(".csdlc/prepared/issues/273/")
}
unexpected = changed - permitted
if not allowed.issubset(changed):
 print(f"FAIL: missing product paths {sorted(allowed - changed)}",file=sys.stderr); raise SystemExit(1)
if unexpected:
 print(f"FAIL: undeclared changed paths {sorted(unexpected)}",file=sys.stderr); raise SystemExit(1)
if any(p.startswith(".csdlc/issues/205/") or p.startswith(".csdlc/issues/274/") or "observatory_serving_eligibility" in p for p in changed):
 print("FAIL: parent/cross-child collision",file=sys.stderr); raise SystemExit(1)
print("PASS: exact #273 product and coverage-policy allowlist")
