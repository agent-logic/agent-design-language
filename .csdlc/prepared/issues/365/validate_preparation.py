#!/usr/bin/env python3
import json, os, pathlib, subprocess
ROOT = pathlib.Path(__file__).resolve().parents[4]
BASE = "26de2a048cea436e5140a8ab5afa7524324b3b39"
FINISH = os.environ.get("CSDLC_FINISH", "csdlc-finish")
OWNED = (
 "adl-runtime/src/distributed/shepherd_serving_eligibility.rs",
 "adl-runtime/src/distributed/observatory_serving_eligibility.rs",
 "adl-runtime/tests/distributed_shepherd_serving_eligibility.rs",
 "adl-runtime/tests/distributed_observatory_serving_eligibility.rs",
)
def run(*args): return subprocess.run(args, cwd=ROOT, text=True, capture_output=True, check=True).stdout
if run("git","rev-parse","HEAD").strip() != BASE: raise SystemExit("FAIL: wrong #365 base")
for issue in (272,273,274):
 p=subprocess.run([FINISH,"--root",".","--validate-cached-issue",str(issue)],cwd=ROOT,text=True,capture_output=True)
 if p.returncode: raise SystemExit(f"FAIL cache #{issue}: {p.stdout}{p.stderr}")
 d=json.loads(p.stdout)
 if not d.get("canonical_match"): raise SystemExit(f"FAIL noncanonical #{issue}")
 if subprocess.run(["git","merge-base","--is-ancestor",d["terminal"]["merge_sha"],BASE],cwd=ROOT).returncode: raise SystemExit(f"FAIL ancestry #{issue}")
design=(ROOT/".csdlc/prepared/issues/365/design.md").read_text()
for path in OWNED:
 if path not in design: raise SystemExit(f"FAIL omitted path {path}")
for forbidden in ("#275 and #205 remain frozen", "no public or feature-gated caller constructor", "Existing acquire/replace/revoke/expire policy is byte-for-byte unchanged", "Existing acquire/renew/transfer/revoke/expiry policy is unchanged"):
 if forbidden not in design: raise SystemExit(f"FAIL boundary text: {forbidden}")
print("PASS: #365 base, terminal ancestry, four-path ownership, and no-policy boundary")
