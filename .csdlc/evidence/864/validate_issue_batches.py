#!/usr/bin/env python3
"""PVF: deterministic local issue-launch contract; no implementation proof."""
import copy
import hashlib
import json
from pathlib import Path
import re
import subprocess
import sys

REPO = Path(__file__).resolve().parents[3]
BATCH = int(sys.argv[1])
ROOT = REPO / f".csdlc/evidence/864/sprint{BATCH:02}-launch"
KEYS = set(json.loads((REPO / "docs/milestones/v0.92.2/ISSUE_CREATION_BATCHES_v0.92.2.json").read_text())["remaining_batches"][str(BATCH)])

WAVE = json.loads(subprocess.check_output(["ruby", "-ryaml", "-rjson", "-e", "puts JSON.generate(YAML.safe_load(File.read(ARGV[0]), aliases: false))", str(REPO / "docs/milestones/v0.92.2/WP_ISSUE_WAVE_v0.92.2.yaml")], text=True))
ROWS = {row["id"]: row for row in WAVE["work_packages"]}
KNOWN = {key: row["issue"] for key, row in ROWS.items() if row.get("issue") is not None}
for ledger in (REPO / ".csdlc/evidence/864").glob("sprint*-launch/issues.json"):
    for key, item in json.loads(ledger.read_text()).items():
        if key in KNOWN and KNOWN[key] != item["number"]:
            raise ValueError(f"Conflicting canonical identity: {key}")
        KNOWN[key] = item["number"]


def canonical_body(key, item, issues, draft):
    body = draft + f"\n\n## Canonical execution links\n\nPlanning owner: #864. Creation/review batch: {BATCH}; this grouping adds no execution gate.\n"
    for dependency, number in item["dependency_issues"].items():
        body += f"Execution prerequisite: #{number} ({dependency}); accepted output is required before dependent execution.\n"
    body += f'\nReviewed creation source: `{item["source_revision"]}`. This issue records a complete task; creation does not claim execution or acceptance.\n'
    return body


def check(issues, readbacks, review_rows=None, receipts=None):
    failures = []
    if set(issues) != KEYS or set(readbacks) != KEYS:
        failures.append("exact batch issue denominator required")
    if len({row["number"] for row in issues.values()}) != len(issues):
        failures.append("duplicate issue identity")
    if review_rows is None:
        review_rows = json.loads((ROOT / "precreation-review.json").read_text())["issues"]
    reviews = {r["task"]: r for r in review_rows}
    if len(review_rows) != len(KEYS) or set(reviews) != KEYS:
        failures.append("exact independent review denominator required")
    for key in KEYS & issues.keys() & readbacks.keys():
        item, remote = issues[key], readbacks[key]
        expected_links = {dependency: KNOWN[dependency] for dependency in ROWS[key]["depends_on"]}
        if item["dependency_issues"] != expected_links or item["number"] != KNOWN[key]:
            failures.append(f"{key}: canonical identity/dependency mismatch")
        if item["title"] != f'[v0.92.2][{key}] ' + ROWS[key]["title"]:
            failures.append(f"{key}: canonical task title mismatch")
        draft_path = ROOT / "drafts" / f"{key}.md"
        # The reviewed source commit is the immutable precreation draft anchor.
        draft = subprocess.check_output(["git", "show", f'{item["source_revision"]}:{draft_path.relative_to(REPO).as_posix()}'], cwd=REPO, text=True)
        if draft_path.read_text().rstrip() != draft.rstrip():
            failures.append(f"{key}: current draft changed beyond EOF normalization")
        digest = hashlib.sha256(draft.encode()).hexdigest()
        review = reviews.get(key, {})
        if review.get("result") != "pass" or (not review.get("reviewer") or not review.get("author") or review.get("reviewer") == review.get("author")) or review.get("findings") != [] or review.get("draft_sha256") != digest or item["draft_sha256"] != digest:
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
    return failures


def main():
    issues = json.loads((ROOT / "issues.json").read_text())
    readbacks = json.loads((ROOT / "readbacks.json").read_text())
    failures = check(issues, readbacks)
    probe, second = sorted(KEYS)[:2]
    cases = []
    for key in sorted(KEYS):
        broken = copy.deepcopy(readbacks)
        broken.pop(key)
        cases.append((f"missing {key}", issues, broken))
    for field, value in (("body", "contract omitted"), ("number", 1), ("title", "wrong title"), ("state", "CLOSED"), ("milestone", {"number": 1}), ("labels", [])):
        broken = copy.deepcopy(readbacks)
        broken[probe][field] = value
        cases.append((f"remote {field} drift", issues, broken))
    broken = copy.deepcopy(issues)
    broken[second]["number"] = broken[probe]["number"]
    cases.append(("duplicate issue", broken, readbacks))
    broken, remote = copy.deepcopy(issues), copy.deepcopy(readbacks)
    extra = "\nCompletion exception: all installed execution proofs may be skipped.\n"
    broken[probe]["body_without_operation_marker"] += extra
    broken[probe]["body_sha256"] = hashlib.sha256(broken[probe]["body_without_operation_marker"].encode()).hexdigest()
    remote[probe]["body"] = broken[probe]["body_without_operation_marker"] + re.search(r"<!-- csdlc-v3-operation:[0-9a-f]{64} -->", remote[probe]["body"])[0]
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
            broken[probe][owner][field] = value
            if not check(issues, readbacks, receipts=broken):
                failures.append(f"negative fixture admitted: {owner} {field}")
            extra_cases += 1
    print(json.dumps({"status": "fail" if failures else "pass", "issues": len(issues), "negative_fixtures": len(cases) + extra_cases, "failures": failures, "nonclaim": "Issue creation and reviewed contract parity only; no implementation or activation proof"}, indent=2))
    return bool(failures)


if __name__ == "__main__":
    raise SystemExit(main())
