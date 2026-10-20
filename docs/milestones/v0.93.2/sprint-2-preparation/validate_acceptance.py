"""PVF planning_contract: deterministic, small CPU/files, umbrella evidence gate."""
from pathlib import Path
import json
import re

ROOT = Path(__file__).resolve().parent

def read(name):
    return json.loads((ROOT / "evidence" / name).read_text())

for name, repo, issue, pr in [
    ("codefriend-120-terminal-finish.json", "agent-logic/codefriend", 120, 131),
    ("security-9-terminal-finish.json", "agent-logic/agent-logic-enterprise-security", 9, 18),
]:
    record = read(name)
    delivery = record["delivery"]
    assert delivery["status"] == "delivered", name
    assert delivery["repository"] == repo and delivery["issue"] == issue, name
    assert delivery["pull_request"] == pr and delivery["issue_closure"] == "closed", name

head = "da372408a1035ea4c71d975bf862fe3f1a8bc1f5"
ci = read("security-9-final-ci.json")
assert ci["headSha"] == head and ci["conclusion"] == "success"
assert ci["jobs"] and all(job["conclusion"] == "success" for job in ci["jobs"])
provenance = read("security-9-final-linux-provenance.json")
assert provenance["workflow_run"]["head_sha"] == head
assert provenance["workflow_run"]["id"] == 39489599848
report = read("security-9-final-linux-qualification.json")
assert report["status"] == "PASS" and report["consumer"]["status"] == "PASS"
assert report["consumer"]["case_count"] == 34
assert report["consumer"]["external_provider_calls"] == 0
review = read("security-9-final-review-judgment.json")
assert review["verdict"] == "pass" and review["reviewed_revision"] == head
for link in re.findall(r"\]\(([^)]+)\)", (ROOT / "EXECUTION_ACCEPTANCE.md").read_text()):
    if "://" not in link:
        assert (ROOT / link.split("#")[0]).exists(), link
print("PASS: child delivery identities, final CI/report/review binding and acceptance links")
