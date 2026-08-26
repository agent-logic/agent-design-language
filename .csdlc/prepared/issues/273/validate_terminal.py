#!/usr/bin/env python3
import json, pathlib, subprocess, sys
root = pathlib.Path(__file__).resolve().parents[4]
common = pathlib.Path(subprocess.check_output(["git","rev-parse","--path-format=absolute","--git-common-dir"],cwd=root,text=True).strip())
cache = common / "csdlc-v2/derived-terminal/273.json"
if not cache.is_file(): print("FAIL: #273 terminal cache absent",file=sys.stderr); raise SystemExit(1)
data=json.loads(cache.read_text())
if data.get("disposition")!="merged" or not data.get("merge_sha"): print("FAIL: #273 nonterminal",file=sys.stderr); raise SystemExit(1)
if subprocess.run(["git","merge-base","--is-ancestor",data["merge_sha"],"origin/main"],cwd=root).returncode: print("FAIL: #273 merge nonancestral",file=sys.stderr); raise SystemExit(1)
print("PASS: #273 terminal and ancestral")
