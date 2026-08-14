#!/usr/bin/env python3
import json, pathlib, subprocess, sys

ROOT = pathlib.Path(__file__).resolve().parents[4]
ISSUE = ROOT / ".csdlc/issues/273"
INDEX = ISSUE / "index.json"
DESIGN = ROOT / ".csdlc/prepared/issues/273/design.md"
DIAGRAM = ROOT / ".csdlc/prepared/issues/273/diagram.mmd"

def fail(message):
    print(f"FAIL: {message}", file=sys.stderr)
    raise SystemExit(1)

for path in (INDEX, DESIGN, DIAGRAM):
    if not path.is_file() or path.is_symlink(): fail(f"missing or unsafe {path}")
index = json.loads(INDEX.read_text())
if index.get("issue") != 273 or index.get("phase") != "initialized": fail("wrong typed identity/phase")
cards = index.get("cards", {})
if set(cards) != {"sip","stp","spp","vpp","srp","sor"}: fail("six-card identity mismatch")
for name in cards:
    for suffix in (f"cards/{name}.md", f"cards/{name}.values.json"):
        if not (ISSUE / suffix).is_file(): fail(f"missing {suffix}")

design = DESIGN.read_text()
required = [
    "adl-runtime/src/distributed/shepherd_serving_eligibility.rs",
    "adl-runtime/tests/distributed_shepherd_serving_eligibility.rs",
    "#273 merges and becomes terminal/ancestral first",
    "must not edit `serving_authority.rs`",
    "VerifiedServingAuthorityCut",
    "No Observatory quorum lifecycle (#274)",
]
for marker in required:
    if marker not in design: fail(f"design marker absent: {marker}")

common = pathlib.Path(subprocess.check_output(["git","rev-parse","--path-format=absolute","--git-common-dir"], cwd=ROOT, text=True).strip())
for dep in (191,199,200,201,202,203,272):
    cache = common / f"csdlc-v2/derived-terminal/{dep}.json"
    if not cache.is_file(): fail(f"terminal cache absent #{dep}")
    data = json.loads(cache.read_text())
    if data.get("disposition") != "merged" or not data.get("merge_sha"): fail(f"nonterminal cache #{dep}")
    rc = subprocess.run(["git","merge-base","--is-ancestor",data["merge_sha"],"HEAD"], cwd=ROOT).returncode
    if rc != 0: fail(f"nonancestral dependency #{dep}")

for forbidden in (ROOT / ".csdlc/issues/274", ROOT / ".csdlc/prepared/issues/274"):
    if forbidden.exists(): fail(f"cross-child projection in #273 root: {forbidden}")
print(json.dumps({"result":"pass","issue":273,"phase":index["phase"],"generation":index["generation"],"digest":index["digest"],"base":subprocess.check_output(["git","rev-parse","HEAD"],cwd=ROOT,text=True).strip(),"serial_order":[273,274]}))
