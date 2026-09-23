"""Preserve the existing planning checks and their nonzero fixture denominator."""
import json
import subprocess
import sys
result = subprocess.run([sys.executable, "docs/milestones/v0.92.2/validate_planning.py", "--self-test"], capture_output=True, text=True)
print(result.stdout, end="")
print(result.stderr, end="", file=sys.stderr)
if result.returncode:
    raise SystemExit(result.returncode)
data = json.loads(result.stdout)
assert data["status"] == "pass" and data["failures"] == []
assert data["work_packages"] == 69
assert data["negative_fixtures"] == 298

# Current routing must agree with the retained authenticated deferral. Historical
# checkpoint objects remain unchanged and are not treated as current status.
from pathlib import Path
import copy
packet = Path("docs/milestones/v0.92.2/evidence/issue-916")
deferral = json.loads((packet / "SPRINT10_DEFERRAL.json").read_text())
decision = json.loads((packet / "QUALITY_DECISION.json").read_text())
def check_current_routing(value):
    row = next(r for r in value["stragglers"] if r["issue"] == deferral["source_issue"])
    assert row["state"] == deferral["closure"]["915"]["state"]
    assert row["state_reason"] == deferral["closure"]["915"]["state_reason"]
    assert row["delivery_status"] == "qualification_incomplete_deferred"
    assert row["successor_issues"] == deferral["follow_on_issues"]
    assert row["destination_milestone"] == "v0.93.1"
    actions = " ".join(value["next_actions"])
    assert "Consume #915 independent qualification" not in actions
    assert all(f"#{issue}" in actions for issue in deferral["follow_on_issues"])
    assert value["sprint10_scope_disposition"]["final_quality_decision"] == "not_proven"
check_current_routing(decision)
for field, old_value in [("state", "OPEN"), ("delivery_status", "outstanding_qualification")]:
    bad = copy.deepcopy(decision)
    next(r for r in bad["stragglers"] if r["issue"] == 915)[field] = old_value
    try:
        check_current_routing(bad)
    except AssertionError:
        pass
    else:
        raise AssertionError(f"accepted stale {field}")
bad = copy.deepcopy(decision)
bad["next_actions"][0] = "Consume #915 independent qualification against merged candidate"
try:
    check_current_routing(bad)
except AssertionError:
    pass
else:
    raise AssertionError("accepted obsolete qualification gate")
print("Current qualification routing: pass; 3 stale-state negative fixtures rejected")
