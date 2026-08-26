#!/usr/bin/env python3
"""Post-finish canonical terminal and ancestry guard for issue #274."""
import json, pathlib, subprocess

root = pathlib.Path(__file__).resolve().parents[4]
common = pathlib.Path(subprocess.check_output(["git","rev-parse","--path-format=absolute","--git-common-dir"], cwd=root, text=True).strip())
record = json.loads((common / "csdlc-v2/derived-terminal/274.json").read_text())
ok = record.get("issue") == 274 and record.get("repository") == "agent-logic/agent-design-language" and record.get("disposition") == "merged" and record.get("issue_state") == "closed_by_merged_pr"
merge = record.get("merge_sha")
if not ok or not merge or subprocess.run(["git","merge-base","--is-ancestor",merge,"origin/main"], cwd=root).returncode:
    print(json.dumps({"status":"failed","record":record})); raise SystemExit(1)
print(json.dumps({"schema":"adl.issue274.terminal.v1","status":"passed","merge_sha":merge,"digest":record.get("digest")}))
