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
wave_owners = {}
for line in wave.splitlines():
    if not line.startswith("| #"):
        continue
    columns = [column.strip() for column in line.strip("|").split("|")]
    issue = int(columns[0].removeprefix("#"))
    observed.append(issue)
    wave_owners[issue] = columns[6]
assert observed == ORDERED, f"unexpected child wave: {observed}"
assert "- Owner: `Planning #5` for umbrella coordination" in packet
assert wave_owners[916] == "Planning #11; Worker #9 read-only audit"
assert wave_owners[917] == "Planning #4.5"
assert wave_owners[918] == "Worker #10"
assert wave_owners[922] == "Planning #11"
assert "Sprint 10 qualification did not pass" in packet
assert (
    "This packet does not authorize v0.93 execution, Beta 1 launch, release, "
    "provider calls, product repair, deployment, publication, or merge."
) in packet
assert "release, or public launch" in packet

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
by_kind = {event["event"]: event for event in events}
assert by_kind["coordination_wave_recorded"]["actor"] == "Planning #5"
wave_event = by_kind["coordination_wave_recorded"]["details"]
assert wave_event["active_decision_owner"] == {
    "issue": 916,
    "owner": "Planning #11",
}
assert wave_event["read_only_support"] == {
    "issue": 916,
    "owner": "Worker #9",
}
assert wave_event["preparation_lanes"] == [
    {"issue": 917, "owner": "Planning #4.5"},
    {"issue": 918, "owner": "Worker #10"},
    {"issue": 922, "owner": "Planning #11"},
]
assert wave_event["sprint10_disposition"]["owner"] == "Planning #7.3"
assert wave_event["sprint10_closeout"] == {
    "issue": 936,
    "owner": "Planning #11",
}
assert wave_event["successors"] == [1148, 1149, 1150]
assert "preparation overlap grants no early acceptance" in wave_event["acceptance_rule"]

closed_event = by_kind["sprint10_disposition_closed"]["details"]
assert closed_event["issue_915"]["state"] == "closed"
assert closed_event["issue_915"]["disposition"] == "NOT_PLANNED"
assert closed_event["issue_936"]["state"] == "closed"
assert closed_event["issue_936"]["disposition"] == "NOT_PLANNED"
assert closed_event["successors"] == [1148, 1149, 1150]
assert closed_event["qualification_claim"] == "incomplete_deferred_not_pass"

spp = load_json(ROOT / ".csdlc/v3/issues/937/cards/spp.values.json")
dependencies = spp["dependencies_inline"]
assert "#916 remains not_proven" in dependencies
assert "#916 through #925" in dependencies
assert "#910 and #911" in dependencies
assert "Final acceptance remains strictly serial" in dependencies
assert spp["step_3_status"] == "in_progress"
assert spp["step_4_status"] == "pending"
assert spp["step_5_status"] == "pending"
acceptance = spp["acceptance_criteria_inline"]
for phrase in [
    "#917, #918, and #922 preparation does not become early acceptance",
    "no product repair",
    "provider call",
    "deployment",
    "publication",
    "merge",
    "release",
    "public launch",
]:
    assert phrase in acceptance, f"missing acceptance boundary: {phrase}"

print("sprint coordination validation: ok; exact 10-child roster and evidence verified")
