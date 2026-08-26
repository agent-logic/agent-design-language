#!/usr/bin/env python3
import argparse
import hashlib
import json
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
EVIDENCE = ROOT / ".csdlc/evidence/307/child-sequence.json"
READBACK = ROOT / ".csdlc/evidence/307/github-pr-readback.json"
FINAL_CEREMONY = ROOT / ".csdlc/evidence/307/issue-319-final-ceremony-receipt.json"
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

def repo_file(value: object, *, label: str) -> Path | None:
    if not isinstance(value, str) or not value or Path(value).is_absolute() or ".." in Path(value).parts:
        errors.append(f"{label} must be a contained repo-relative path")
        return None
    path = ROOT / value
    if not path.is_file():
        errors.append(f"{label} is missing: {value}")
        return None
    return path

try:
    readback_packet = json.loads(READBACK.read_text())
except (OSError, json.JSONDecodeError) as exc:
    readback_packet = {}
    errors.append(f"GitHub PR readback is missing or invalid: {exc}")
readback_rows = {
    row.get("issue"): row
    for row in readback_packet.get("rows", [])
    if isinstance(row, dict)
}
no_pr_rows = {
    row.get("issue"): row
    for row in readback_packet.get("no_pr_dispositions", [])
    if isinstance(row, dict)
}

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
    for index, item in enumerate(children):
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
        review_path = repo_file(item.get("review_evidence"), label=f"child #{issue} review_evidence")
        reviewed_head = item.get("reviewed_head")
        if review_path is not None:
            review_text = review_path.read_text(errors="replace")
            if issue in {308, 309, 311, 312, 313, 315, 316, 317, 318, 319}:
                if f'git-blake3:{reviewed_head}:' not in review_text or "Result: pass" not in review_text:
                    errors.append(f"child #{issue} review evidence does not bind its passed reviewed_head")
            elif issue == 310:
                try:
                    universe = json.loads(review_text)
                    row = next(entry for entry in universe["issues"] if entry["issue"] == 310)
                except (KeyError, StopIteration, json.JSONDecodeError):
                    row = None
                if (
                    not row
                    or row.get("terminal_route") != "recordless_closeout"
                    or row.get("head") != reviewed_head
                    or row.get("merge") != merge_sha
                ):
                    errors.append("child #310 lacks its disclosed retrospective recordless-review evidence")
            elif issue == 314:
                try:
                    validation = json.loads(review_text)
                except json.JSONDecodeError:
                    validation = {}
                if validation.get("ready_for_315_handoff") is not True:
                    errors.append("child #314 external review is not validated for WP-27 handoff")
        if issue == 314:
            no_pr = no_pr_rows.get(314)
            if not no_pr or no_pr.get("reviewed_head") != reviewed_head or no_pr.get("integration_merge") != merge_sha:
                errors.append("child #314 no-PR disposition does not match retained readback")
            handoff = repo_file(
                no_pr.get("handoff") if isinstance(no_pr, dict) else None,
                label="child #314 handoff",
            )
            if handoff is not None and "#315" not in handoff.read_text(errors="replace"):
                errors.append("child #314 handoff does not route to #315")
            if item.get("pull_request") is not None:
                errors.append("child #314 must not claim a pull request")
        else:
            observed = readback_rows.get(issue)
            expected = {
                "pull_request": item.get("pull_request"),
                "head_sha": item.get("publication_head"),
                "merge_sha": merge_sha,
                "state": "closed",
                "merged": True,
                "adl_ci": "success",
                "adl_coverage": "success",
                "adl_path_policy": "success",
            }
            if not observed or any(observed.get(field) != value for field, value in expected.items()):
                errors.append(f"child #{issue} live PR/check readback does not match the ledger")
        if index + 1 < len(children):
            successor_merge = children[index + 1].get("merge_sha")
            if not isinstance(successor_merge, str) or not git_ok("merge-base", "--is-ancestor", successor_merge, "origin/main"):
                errors.append(f"child #{issue} successor handoff is not integrated")
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
            else:
                common_dir = subprocess.run(
                    ["git", "-C", str(ROOT), "rev-parse", "--git-common-dir"],
                    text=True,
                    capture_output=True,
                    check=True,
                ).stdout.strip()
                common_path = Path(common_dir)
                if not common_path.is_absolute():
                    common_path = ROOT / common_path
                terminal_path = common_path / "csdlc-v2" / "derived-terminal" / f"{issue}.json"
                try:
                    terminal = json.loads(terminal_path.read_text())
                except (OSError, json.JSONDecodeError):
                    terminal = {}
                if terminal.get("digest") != value or terminal.get("merge_sha") != merge_sha:
                    errors.append(f"child #{issue} terminal cache does not match the ledger")
        if not isinstance(item.get("residual_risk"), list):
            errors.append(f"child #{issue} lacks residual_risk list")
final_receipt_ref = children[-1].get("final_ceremony_receipt") if isinstance(children, list) and children else None
final_receipt_path = repo_file(final_receipt_ref, label="child #319 final_ceremony_receipt")
if final_receipt_path is not None:
    try:
        final_receipt = json.loads(final_receipt_path.read_text())
    except json.JSONDecodeError:
        final_receipt = {}
    output_path = repo_file(final_receipt.get("ceremony_output_ref"), label="child #319 ceremony output")
    output_digest = hashlib.sha256(output_path.read_bytes()).hexdigest() if output_path is not None else None
    required_final = {
        "main_sha": "aa5766d71864713b97210abdb5aa8e5c2481ed31",
        "origin_main_sha": "aa5766d71864713b97210abdb5aa8e5c2481ed31",
        "branch": "main",
        "worktree_clean": True,
        "ceremony_preflight": "passed_check_only",
        "ceremony_output_sha256": output_digest,
        "merge_sha": "aa5766d71864713b97210abdb5aa8e5c2481ed31",
        "canonical_match": True,
        "execution_worktree_cleaned": True,
        "tag_created": False,
        "tag_pushed": False,
        "release_created": False,
        "release_published": False,
        "release_mutation_authorized": False,
        "v093_activated": False,
    }
    if any(final_receipt.get(field) != value for field, value in required_final.items()):
        errors.append("child #319 final ceremony receipt is incomplete or inconsistent")
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
