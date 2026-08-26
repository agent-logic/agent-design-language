#!/usr/bin/env python3
import argparse
import json
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
EVIDENCE = ROOT / ".csdlc/evidence/307/child-sequence.json"
parser = argparse.ArgumentParser()
parser.add_argument("--terminal", action="store_true")
args = parser.parse_args()
if not args.terminal:
    print(json.dumps({"schema":"adl.issue307.sequence.v1","status":"pass","preparation_only":True}))
    raise SystemExit(0)
errors = []

def git_ok(*argv: str) -> bool:
    return subprocess.run(
        ["git", "-C", str(ROOT), *argv],
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
        check=False,
    ).returncode == 0

if not EVIDENCE.is_file():
    errors.append("child sequence evidence missing")
    packet = {}
else:
    try:
        packet = json.loads(EVIDENCE.read_text())
    except (OSError, json.JSONDecodeError) as exc:
        packet = {}
        errors.append(f"child sequence evidence unreadable: {exc}")
sequence = packet.get("sequence", [])
expected_sequence = list(range(308, 320))
if sequence != expected_sequence:
    errors.append(f"child sequence must equal {expected_sequence}")
children = packet.get("children")
if not isinstance(children, list) or [item.get("issue") for item in children] != expected_sequence:
    errors.append("children must contain exactly one ordered row for every issue #308 through #319")
else:
    for item in children:
        issue = item["issue"]
        for field in ("merge_ancestral", "handoff_accepted"):
            if item.get(field) is not True:
                errors.append(f"child #{issue} lacks {field}=true")
        if item.get("review_result") != "passed" or item.get("required_checks") != "passed":
            errors.append(f"child #{issue} lacks passed review/check truth")
        for field, length in (("reviewed_head", 40), ("merge_sha", 40)):
            value = item.get(field)
            if not isinstance(value, str) or len(value) != length or any(ch not in "0123456789abcdef" for ch in value):
                errors.append(f"child #{issue} has invalid {field}")
            elif not git_ok("cat-file", "-e", f"{value}^{{commit}}"):
                errors.append(f"child #{issue} {field} does not resolve to a commit")
        merge_sha = item.get("merge_sha")
        if isinstance(merge_sha, str) and len(merge_sha) == 40 and not git_ok(
            "merge-base", "--is-ancestor", merge_sha, "origin/main"
        ):
            errors.append(f"child #{issue} merge_sha is not ancestral to origin/main")
        closeout = item.get("closeout")
        if closeout not in ("async_pending", "reconciled"):
            errors.append(f"child #{issue} closeout must be async_pending or reconciled")
        if closeout == "reconciled":
            for field in ("terminal", "canonical_match", "worktree_cleaned"):
                if item.get(field) is not True:
                    errors.append(f"child #{issue} reconciled closeout lacks {field}=true")
            value = item.get("terminal_digest")
            if not isinstance(value, str) or len(value) != 64 or any(ch not in "0123456789abcdef" for ch in value):
                errors.append(f"child #{issue} has invalid terminal_digest")
        if not isinstance(item.get("residual_risk"), list):
            errors.append(f"child #{issue} lacks residual_risk list")
remediation = packet.get("issue_471_remediation_subissue")
if not isinstance(remediation, dict) or remediation.get("issue") != 471:
    errors.append("exact #471 remediation subissue row is missing")
else:
    if remediation.get("parent_issue") != 315 or remediation.get("wp") != "WP-27":
        errors.append("#471 must be recorded as a WP-27/#315 child")
    if remediation.get("release_tail_lane") is not False:
        errors.append("#471 must not be recorded as an independent release-tail lane")
carryover = packet.get("issue_268_result")
if not isinstance(carryover, dict) or carryover.get("issue") != 268:
    errors.append("exact #268 result row is missing")
else:
    if carryover.get("state") != "closed":
        errors.append("#268 result state must be closed")
    qualification = carryover.get("qualification")
    if qualification == "passed":
        if carryover.get("claimed_pass") is not True or carryover.get("terminal") is not True or carryover.get("result") != "passed":
            errors.append("passed #268 requires explicit typed terminal/result truth")
        for field, length in (("reviewed_head", 40), ("merge_sha", 40), ("terminal_digest", 64)):
            value = carryover.get(field)
            if not isinstance(value, str) or len(value) != length or any(ch not in "0123456789abcdef" for ch in value):
                errors.append(f"passed #268 has invalid {field}")
    elif qualification in ("pending", "failed", "cancelled"):
        if carryover.get("claimed_pass") is not False:
            errors.append("non-passing #268 must remain unclaimed")
        if not isinstance(carryover.get("residual_risk"), str) or not carryover["residual_risk"].strip():
            errors.append("non-passing #268 requires residual risk")
    else:
        errors.append("#268 qualification must be pending, passed, failed, or cancelled")
print(json.dumps({"schema":"adl.issue307.sequence.v1","status":"fail" if errors else "pass","errors":errors}, sort_keys=True))
sys.exit(1 if errors else 0)
