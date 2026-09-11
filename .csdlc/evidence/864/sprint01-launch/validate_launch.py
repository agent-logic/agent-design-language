#!/usr/bin/env python3
"""PVF: deterministic local issue-launch contract; no implementation proof."""
import copy
import hashlib
import json
from pathlib import Path
import re

ROOT = Path(__file__).resolve().parent
KEYS = {"SIM-UMBRELLA"} | {f"SIM-{n:02}" for n in range(1, 10)}


def canonical_body(key, item, issues, draft):
    body = draft + "\n\n## Canonical sprint links\n\nPlanning and issue-creation owner: #864.\n"
    if key != "SIM-UMBRELLA":
        body += f'Sprint umbrella: #{issues["SIM-UMBRELLA"]["number"]} (coordination; does not gate SIM-01 startup).\n'
        if key != "SIM-01":
            previous = f"SIM-{int(key[-2:]) - 1:02}"
            body += f'Execution prerequisite: #{issues[previous]["number"]} ({previous}), with accepted merged output before dependent execution.\n'
    else:
        body += "Completion depends on SIM-09 and all nine accepted child results; coordination opens immediately.\n"
        for n in range(1, 10):
            child = f"SIM-{n:02}"
            body += f'- [ ] #{issues[child]["number"]} ({child}) accepted result reconciled.\n'
    body += f'\nCreation contract reviewed at `{item["source_revision"]}`. Issue creation is not implementation start or live activation.\n'
    return body


def check(issues, readbacks, review_rows=None, receipts=None):
    failures = []
    if set(issues) != KEYS or set(readbacks) != KEYS:
        failures.append("exact ten-issue denominator required")
    if len({row["number"] for row in issues.values()}) != len(issues):
        failures.append("duplicate issue identity")
    if review_rows is None:
        review_rows = json.loads((ROOT / "precreation-review.json").read_text())["issues"]
    reviews = {r["task"]: r for r in review_rows}
    if len(review_rows) != 10 or set(reviews) != KEYS:
        failures.append("exact independent review denominator required")
    expected_reviewers = {key: "/root/docs_final_review" if key in {"SIM-UMBRELLA", "SIM-04", "SIM-05", "SIM-06"} else "/root/task_contract_review" for key in KEYS}
    for key in KEYS & issues.keys() & readbacks.keys():
        item, remote = issues[key], readbacks[key]
        draft = (ROOT / "drafts" / f"{key}.md").read_text()
        digest = hashlib.sha256(draft.encode()).hexdigest()
        review = reviews.get(key, {})
        if review.get("result") != "pass" or review.get("reviewer") != expected_reviewers[key] or review.get("findings") != [] or review.get("draft_sha256") != digest or item["draft_sha256"] != digest:
            failures.append(f"{key}: draft differs from independently reviewed bytes")
        body = item["body_without_operation_marker"]
        if (set(issues) == KEYS and body != canonical_body(key, item, issues, draft)) or hashlib.sha256(body.encode()).hexdigest() != item["body_sha256"]:
            failures.append(f"{key}: published intent lost reviewed contract")
        if remote["number"] != item["number"] or remote["title"] != item["title"]:
            failures.append(f"{key}: remote identity/title mismatch")
        if remote["state"] != "OPEN" or remote["milestone"]["number"] != 2 or "version:v0.92.2" not in {x["name"] for x in remote["labels"]}:
            failures.append(f"{key}: remote routing/state mismatch")
        normalized = re.sub(r"\n*<!-- csdlc-v3-operation:[0-9a-f]{64} -->\s*$", "", remote["body"]).rstrip()
        if normalized != body.rstrip():
            failures.append(f"{key}: remote body differs from reviewed intent")
        result = receipts[key] if receipts is not None else json.loads((ROOT / item["final_receipt"]).read_text())["result"]["outcome"]["result"]
        receipt, reconciliation = result["receipt"], result["reconciliation"]
        if not receipt["authenticated"] or not reconciliation["authenticated"] or receipt["issue"] != item["number"] or receipt["repository"] != "agent-logic/agent-design-language":
            failures.append(f"{key}: native authenticated receipt mismatch")
        if any(record.get("issue") != item["number"] or record.get("repository") != "agent-logic/agent-design-language" or record.get("expected_head_sha") != item["source_revision"] for record in (receipt, reconciliation)):
            failures.append(f"{key}: receipt/reconciliation source identity mismatch")
        for field in ("operation_digest", "readback_digest"):
            if not re.fullmatch(r"[0-9a-f]{64}", receipt.get(field, "")) or receipt.get(field) != reconciliation.get(field):
                failures.append(f"{key}: receipt/reconciliation {field} mismatch")
        expected_marker = f'<!-- csdlc-v3-operation:{receipt["operation_digest"]} -->'
        if reconciliation["operation_marker"] != expected_marker or reconciliation["operation_marker"] not in remote["body"]:
            failures.append(f"{key}: native operation marker missing")
        if key != "SIM-UMBRELLA" and "SIM-UMBRELLA" in issues:
            if f'Sprint umbrella: #{issues["SIM-UMBRELLA"]["number"]}' not in body:
                failures.append(f"{key}: numeric umbrella link missing")
        if key not in {"SIM-01", "SIM-UMBRELLA"}:
            previous = f"SIM-{int(key[-2:]) - 1:02}"
            if previous in issues and f'Execution prerequisite: #{issues[previous]["number"]}' not in body:
                failures.append(f"{key}: numeric execution dependency missing")
        if key == "SIM-UMBRELLA":
            for child in KEYS - {key}:
                if child in issues and f'#{issues[child]["number"]} ({child})' not in body:
                    failures.append(f"umbrella missing {child}")
    inventory = (ROOT / "drafts" / "command-inventory.md").read_text().strip()
    if "SIM-03" in readbacks and inventory not in readbacks["SIM-03"]["body"]:
        failures.append("SIM-03 missing frozen inventory")
    return failures


def main():
    issues = json.loads((ROOT / "issues.json").read_text())
    readbacks = json.loads((ROOT / "readbacks.json").read_text())
    failures = check(issues, readbacks)
    cases = []
    for key in sorted(KEYS):
        broken = copy.deepcopy(readbacks)
        broken.pop(key)
        cases.append((f"missing {key}", issues, broken))
    for field, value in (("body", "contract omitted"), ("number", 1), ("title", "wrong title"), ("state", "CLOSED"), ("milestone", {"number": 1}), ("labels", [])):
        broken = copy.deepcopy(readbacks)
        broken["SIM-03"][field] = value
        cases.append((f"remote {field} drift", issues, broken))
    broken = copy.deepcopy(issues)
    broken["SIM-02"]["number"] = broken["SIM-01"]["number"]
    cases.append(("duplicate issue", broken, readbacks))
    broken, remote = copy.deepcopy(issues), copy.deepcopy(readbacks)
    extra = "\nCompletion exception: all installed execution proofs may be skipped.\n"
    broken["SIM-03"]["body_without_operation_marker"] += extra
    broken["SIM-03"]["body_sha256"] = hashlib.sha256(broken["SIM-03"]["body_without_operation_marker"].encode()).hexdigest()
    remote["SIM-03"]["body"] = broken["SIM-03"]["body_without_operation_marker"] + re.search(r"<!-- csdlc-v3-operation:[0-9a-f]{64} -->", remote["SIM-03"]["body"])[0]
    cases.append(("unreviewed contract suffix", broken, remote))
    for name, candidate, remote in cases:
        if not check(candidate, remote):
            failures.append(f"negative fixture admitted: {name}")
    extra_cases = 0
    reviews = json.loads((ROOT / "precreation-review.json").read_text())["issues"]
    for field, value in (("reviewer", ""), ("findings", ["unresolved finding"])):
        broken = copy.deepcopy(reviews)
        broken[0][field] = value
        if not check(issues, readbacks, review_rows=broken):
            failures.append(f"negative fixture admitted: invalid review {field}")
        extra_cases += 1
    if not check(issues, readbacks, review_rows=reviews[:-1]):
        failures.append("negative fixture admitted: missing independent review")
    extra_cases += 1
    receipts = {key: json.loads((ROOT / item["final_receipt"]).read_text())["result"]["outcome"]["result"] for key, item in issues.items()}
    for owner in ("receipt", "reconciliation"):
        for field, value in (("repository", "other/repo"), ("issue", 1), ("expected_head_sha", "0" * 40), ("operation_digest", "0" * 64), ("readback_digest", "0" * 64)):
            broken = copy.deepcopy(receipts)
            broken["SIM-03"][owner][field] = value
            if not check(issues, readbacks, receipts=broken):
                failures.append(f"negative fixture admitted: {owner} {field}")
            extra_cases += 1
    print(json.dumps({"status": "fail" if failures else "pass", "issues": len(issues), "negative_fixtures": len(cases) + extra_cases, "failures": failures, "nonclaim": "Issue creation and reviewed contract parity only; no implementation or activation proof"}, indent=2))
    return bool(failures)


if __name__ == "__main__":
    raise SystemExit(main())
