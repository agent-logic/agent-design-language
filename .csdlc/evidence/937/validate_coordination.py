#!/usr/bin/env python3
import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[3]
EVIDENCE = ROOT / ".csdlc/evidence/937"
ORDERED = list(range(916, 926))


def load_json(path: Path):
    return json.loads(path.read_text(encoding="utf-8"))


packet = (EVIDENCE / "SPRINT_EXECUTION_PACKET.md").read_text(encoding="utf-8")
required_sections = [
    "## Child Issue Wave",
    "## Recommended Execution Order",
    "## Watcher Policy",
    "## Budget And Goal Accounting",
    "## Watcher Plan",
    "## Safe Parallel Lanes",
    "## Candidate Parallel Lanes",
    "## Serial Gates",
    "## Parallelism Outcome Plan",
    "## Sprint Closeout Rollup Expectations",
    "## Non-Claims",
]
for section in required_sections:
    assert section in packet, f"missing packet section: {section}"

wave = packet.split("## Child Issue Wave", 1)[1].split("## Dependency Graph", 1)[0]
observed = []
for line in wave.splitlines():
    if not line.startswith("| #"):
        continue
    observed.append(int(line.split("#", 1)[1].split(" ", 1)[0]))
assert observed == ORDERED, f"unexpected child wave: {observed}"

state = load_json(EVIDENCE / "sprint-state.json")
assert state["sprint_issue_number"] == 937
assert state["ordered_issue_numbers"] == ORDERED
assert [record["issue_number"] for record in state["issue_records"]] == ORDERED

events = [
    json.loads(line)
    for line in (EVIDENCE / "activity.jsonl").read_text(encoding="utf-8").splitlines()
    if line.strip()
]
assert events, "activity log is empty"
assert all(event["sprint_issue"] == 937 for event in events)
assert events[-1]["details"]["successors"] == [1148, 1149, 1150]

spp = load_json(ROOT / ".csdlc/v3/issues/937/cards/spp.values.json")
dependencies = spp["dependencies_inline"]
assert "#916 remains not_proven" in dependencies
assert "#916 through #925" in dependencies
assert "#910 and #911" in dependencies
assert spp["step_3_status"] == "in_progress"
assert spp["step_4_status"] == "pending"
assert spp["step_5_status"] == "pending"

print("sprint coordination validation: ok; exact 10-child roster and evidence verified")
