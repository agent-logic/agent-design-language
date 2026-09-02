#!/usr/bin/env bash
set -euo pipefail

for card in sip stp spp vpp srp sor; do
  test -f ".csdlc/issues/596/cards/${card}.md"
  test -f ".csdlc/issues/596/cards/${card}.values.json"
done

test -f ".csdlc/issues/596/index.json"
test -f ".csdlc/prepared/issues/596/pr-create-request.json"
test -f ".csdlc/prepared/issues/596/pr-state-request.json"
test ! -e ".csdlc/prepared/issues/596/pr-update-after-review-fixes-request.json"

python3 - <<'PY'
import json
import subprocess
from pathlib import Path

state_result = subprocess.run(
    [
        ".adl/bin/csdlc-v2/csdlc-github-pr",
        "state",
        "--request",
        ".csdlc/prepared/issues/596/pr-state-request.json",
    ],
    text=True,
    stdout=subprocess.PIPE,
    stderr=subprocess.PIPE,
    check=True,
)
live_state = json.loads(state_result.stdout)
local_head = subprocess.run(
    ["git", "rev-parse", "HEAD"],
    text=True,
    stdout=subprocess.PIPE,
    stderr=subprocess.PIPE,
    check=True,
).stdout.strip()
if live_state["schema"] != "csdlc.github_pr_state.v1":
    raise SystemExit(f"PR #615 readback used unexpected schema: {live_state['schema']}")
if live_state["repository"] != "agent-logic/agent-design-language":
    raise SystemExit("PR #615 live readback repository drift")
if live_state["pull_request"] != 615:
    raise SystemExit("PR #615 live readback must target PR #615")
if live_state["linked_issue"] != 596:
    raise SystemExit(f"PR #615 must close only issue #596; observed linked issue {live_state['linked_issue']}")
if live_state["linkage_source"] != "github_closing_issues_references":
    raise SystemExit(f"PR #615 linkage must come from GitHub closing references: {live_state['linkage_source']}")
if live_state["state"] != "open" or live_state["draft"] is not False:
    raise SystemExit("PR #615 must be open and non-draft for publication-readiness proof")
if live_state["merge_state"] != "clean":
    raise SystemExit(f"PR #615 merge state must be clean: {live_state['merge_state']}")
if live_state["head_sha"] != local_head:
    raise SystemExit(
        f"PR #615 live readback drifted from local HEAD: "
        f"live={live_state['head_sha']} local={local_head}"
    )

body = live_state["body"]
for required in ("Closes #596", "issue 505", "issue 534"):
    if required not in body:
        raise SystemExit(f"Live PR body must retain required lifecycle linkage: {required}")
for forbidden in ("Fixes #596", "Resolves #596", "#505", "#534", "Closes issue 505", "Fixes issue 505", "Resolves issue 505", "Closes issue 534", "Fixes issue 534", "Resolves issue 534"):
    if forbidden in body:
        raise SystemExit(f"Live PR body has forbidden lifecycle linkage: {forbidden}")

index = json.loads(Path(".csdlc/issues/596/index.json").read_text())
cards = set(index["cards"])
if cards != {"sip", "stp", "spp", "vpp", "srp", "sor"}:
    raise SystemExit(f"unexpected card set: {sorted(cards)}")
if index["repository"] != "agent-logic/agent-design-language":
    raise SystemExit("issue repository drift")

pr_state = json.loads(Path(".csdlc/prepared/issues/596/pr-state-request.json").read_text())
expected_pr_state_keys = {
    "repository",
    "pull_request",
    "required_checks",
    "require_review",
    "token_file",
    "linked_issue",
    "linked_issue_repository",
}
if set(pr_state) != expected_pr_state_keys:
    raise SystemExit(f"PR #615 state request uses stale schema keys: {sorted(pr_state)}")
if pr_state["repository"] != "agent-logic/agent-design-language":
    raise SystemExit("PR #615 state request repository drift")
if pr_state["pull_request"] != 615 or pr_state["linked_issue"] != 596:
    raise SystemExit("PR #615 state request must assert issue #596 linkage only")

import subprocess
changed = subprocess.run(
    ["git", "diff", "--name-only", "origin/main...HEAD", "--", "csdlc-v2"],
    text=True,
    stdout=subprocess.PIPE,
    stderr=subprocess.PIPE,
    check=True,
)
if changed.stdout.strip():
    raise SystemExit(f"v3 cutover remediation must not mutate csdlc-v2 sources/tests: {changed.stdout.strip()}")

owner_lanes = json.loads(Path("docs/csdlc-v3/owner-proof-lanes.json").read_text())
for source in owner_lanes["sources"]:
    source_path = source["source_path"]
    if source_path.startswith("/"):
        raise SystemExit(f"owner proof lane source must be repo-relative: {source_path}")
    if ".." in Path(source_path).parts:
        raise SystemExit(f"owner proof lane source must not traverse: {source_path}")
    if source["owner_issue"] == 505 and source_path != "docs/csdlc-v3/owner-lane-sources/issue-505-vpp.values.json":
        raise SystemExit(f"owner #505 lane source must be repo-contained snapshot: {source_path}")
PY
