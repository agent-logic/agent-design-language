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

def git_stdout(*argv: str) -> str | None:
    result = subprocess.run(
        ["git", "-C", str(ROOT), *argv],
        text=True,
        capture_output=True,
        check=False,
    )
    if result.returncode != 0:
        return None
    return result.stdout

def git_changed_paths(base: str, head: str) -> list[str]:
    output = git_stdout("diff", "--name-only", base, head)
    if output is None:
        return []
    return [line for line in output.splitlines() if line]

def git_blob_id(commit: str, path: str) -> str | None:
    output = git_stdout("ls-tree", commit, path)
    if output is None:
        return None
    parts = output.strip().split()
    if len(parts) < 3 or parts[1] != "blob":
        return None
    return parts[2]

def git_blob_sha256(commit: str, path: str) -> str | None:
    result = subprocess.run(
        ["git", "-C", str(ROOT), "show", f"{commit}:{path}"],
        text=False,
        capture_output=True,
        check=False,
    )
    if result.returncode != 0:
        return None
    return hashlib.sha256(result.stdout).hexdigest()

def issue_lifecycle_paths(issue: int) -> set[str]:
    prefix = f".csdlc/issues/{issue}"
    return {
        f"{prefix}/audit.jsonl",
        f"{prefix}/index.json",
        f"{prefix}/cards/sip.md",
        f"{prefix}/cards/sip.values.json",
        f"{prefix}/cards/stp.md",
        f"{prefix}/cards/stp.values.json",
        f"{prefix}/cards/spp.md",
        f"{prefix}/cards/spp.values.json",
        f"{prefix}/cards/vpp.md",
        f"{prefix}/cards/vpp.values.json",
        f"{prefix}/cards/srp.md",
        f"{prefix}/cards/srp.values.json",
        f"{prefix}/cards/sor.md",
        f"{prefix}/cards/sor.values.json",
    }

def validate_metadata_only_publication(issue: int, reviewed_head: str, publication_head: str) -> None:
    if not git_ok("merge-base", "--is-ancestor", reviewed_head, publication_head):
        errors.append(f"child #{issue} reviewed_head is not ancestral to publication_head")
        return
    changed = set(git_changed_paths(reviewed_head, publication_head))
    if not changed:
        errors.append(f"child #{issue} has no reviewed-to-publication diff paths")
        return
    unexpected = sorted(changed - issue_lifecycle_paths(issue))
    if unexpected:
        errors.append(f"child #{issue} reviewed-to-publication diff is not lifecycle-only: {unexpected}")

def validate_issue_312_receipt(item: dict) -> None:
    receipt_path = repo_file(item.get("retrospective_review_receipt"), label="child #312 retrospective_review_receipt")
    if receipt_path is None:
        return
    try:
        receipt = json.loads(receipt_path.read_text())
    except json.JSONDecodeError:
        errors.append("child #312 final-content review receipt is invalid JSON")
        return
    if (
        receipt.get("issue") != 312
        or receipt.get("reviewed_publication_head") != item.get("publication_head")
        or receipt.get("original_reviewed_head") != item.get("reviewed_head")
        or receipt.get("review_result") != "pass"
    ):
        errors.append("child #312 final-content review receipt identity is inconsistent")
    changed = git_changed_paths(item["reviewed_head"], item["publication_head"])
    if receipt.get("publication_diff_paths") != changed:
        errors.append("child #312 final-content receipt does not match exact publication diff path set")
    scoped_files = {
        row.get("path"): row
        for row in receipt.get("retrospectively_reviewed_files", [])
        if isinstance(row, dict)
    }
    expected_paths = [
        "docs/milestones/v0.92/WP_EXECUTION_READINESS_v0.92.md",
        "docs/reviews/v0.92/docs-release-truth-312/inventory.json",
    ]
    if sorted(scoped_files) != expected_paths:
        errors.append("child #312 final-content receipt must bind exactly the two non-lifecycle files")
        return
    for path in expected_paths:
        row = scoped_files[path]
        if row.get("publication_blob") != git_blob_id(item["publication_head"], path):
            errors.append(f"child #312 final-content receipt blob mismatch for {path}")
        if row.get("publication_sha256") != git_blob_sha256(item["publication_head"], path):
            errors.append(f"child #312 final-content receipt sha256 mismatch for {path}")

def validate_issue_315_scope_receipt(item: dict, receipt: dict) -> None:
    if receipt.get("publication_head") != item.get("publication_head"):
        errors.append("child #315 historical review receipt does not bind the PR publication head")
    scoped_files = receipt.get("review_scope_files")
    if not isinstance(scoped_files, list) or len(scoped_files) != 2:
        errors.append("child #315 historical review receipt must bind exactly two review-scope files")
        return
    expected_paths = [
        "adl-runtime-kernel/src/production_birthday.rs",
        "adl-runtime-kernel/tests/production_birthday.rs",
    ]
    by_path = {row.get("path"): row for row in scoped_files if isinstance(row, dict)}
    if sorted(by_path) != expected_paths:
        errors.append("child #315 historical review receipt has the wrong scoped files")
        return
    for path in expected_paths:
        row = by_path[path]
        reviewed_blob = git_blob_id(item["reviewed_head"], path)
        publication_blob = git_blob_id(item["publication_head"], path)
        if row.get("reviewed_blob") != reviewed_blob or row.get("publication_blob") != publication_blob:
            errors.append(f"child #315 scoped receipt blob mismatch for {path}")
        if reviewed_blob != publication_blob:
            errors.append(f"child #315 scoped file changed after review: {path}")
        if row.get("sha256") != git_blob_sha256(item["publication_head"], path):
            errors.append(f"child #315 scoped receipt sha256 mismatch for {path}")

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
        if item.get("merge_ancestral") is not True:
            errors.append(f"child #{issue} lacks merge_ancestral=true")
        expected_disposition = {
            310: ("not_tracked_disclosed", "passed"),
            314: ("intake_completed_with_blockers", "not_applicable_no_pr"),
            315: ("passed_then_remediated_by_476", "passed"),
        }.get(issue, ("passed", "passed"))
        if (item.get("review_result"), item.get("required_checks")) != expected_disposition:
            errors.append(f"child #{issue} review/check disposition is not truthful")
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
        publication_head = item.get("publication_head")
        if issue in {308, 309, 311, 313, 316, 317, 318, 319}:
            validate_metadata_only_publication(issue, reviewed_head, publication_head)
        elif issue == 312:
            validate_issue_312_receipt(item)
        if review_path is not None:
            review_text = review_path.read_text(errors="replace")
            if issue in {308, 309, 311, 312, 313, 316, 317, 318, 319}:
                if f'git-blake3:{reviewed_head}:' not in review_text or "Result: pass" not in review_text:
                    errors.append(f"child #{issue} review evidence does not bind its passed reviewed_head")
            elif issue == 315:
                try:
                    receipt = json.loads(review_text)
                    historical = subprocess.run(
                        ["git", "-C", str(ROOT), "show", f'{receipt["source_merge"]}:{receipt["source_path"]}'],
                        text=False,
                        capture_output=True,
                        check=True,
                    ).stdout
                except (KeyError, json.JSONDecodeError, subprocess.CalledProcessError):
                    receipt, historical = {}, b""
                if (
                    receipt.get("issue") != 315
                    or receipt.get("reviewed_head") != reviewed_head
                    or receipt.get("review_result") != "pass"
                    or hashlib.sha256(historical).hexdigest() != receipt.get("source_sha256")
                    or f'git-blake3:{reviewed_head}:' not in historical.decode(errors="replace")
                    or "Result: pass" not in historical.decode(errors="replace")
                ):
                    errors.append("child #315 historical review receipt does not bind its exact passed head")
                validate_issue_315_scope_receipt(item, receipt)
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
        handoff_proof = item.get("handoff_proof")
        if index + 1 < len(children):
            successor = children[index + 1]
            if issue == 314:
                if handoff_proof != "validated_handoff_to_315_and_integration_merge":
                    errors.append("child #314 lacks its validated WP-27 handoff proof")
            elif issue == 315:
                if handoff_proof != "documented_concurrent_lane_with_reviewed_follow_on_476":
                    errors.append("child #315 lacks its documented concurrent-lane proof")
                plan = repo_file(
                    "docs/milestones/v0.92/V092_TERMINAL_CLOSEOUT_PLAN_317.md",
                    label="child #315 concurrent-lane plan",
                )
                if plan is not None:
                    plan_text = plan.read_text(errors="replace")
                    if "#316 and #317 did not serialize" not in plan_text:
                        errors.append("child #315 concurrent-lane exception is not documented")
            else:
                if handoff_proof != "successor_publication_contains_predecessor_merge":
                    errors.append(f"child #{issue} lacks ancestry-bound successor handoff proof")
                successor_head = successor.get("publication_head")
                if not isinstance(successor_head, str) or not git_ok(
                    "merge-base", "--is-ancestor", merge_sha, successor_head
                ):
                    errors.append(f"child #{issue} merge is not consumed by successor #{successor.get('issue')} publication")
        elif handoff_proof != "final_clean_main_ceremony_receipt":
            errors.append("child #319 lacks final ceremony handoff proof")
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
follow_on = packet.get("issue_476_follow_on")
if not isinstance(follow_on, dict) or follow_on.get("issue") != 476 or follow_on.get("parent_issue") != 315:
    errors.append("issue #476 follow-on remediation row is missing")
else:
    expected_follow_on = {
        "reviewed_head": "bf7031e3c9ba57557efd01922e88ccc65d33f108",
        "publication_head": "9ba0a8ffbd10bb7719ec004ab07ec6fe63a47380",
        "pull_request": 477,
        "merge_sha": "19d479541df4f58e9a40f09f6711593f7829a1d3",
        "review_result": "passed",
        "required_checks": "passed",
        "merge_ancestral": True,
        "terminal_digest": "7ae5e2e6840d71612f5a2e31d3596fa409459b4d4c5b40132c002e311bf37689",
    }
    if any(follow_on.get(field) != value for field, value in expected_follow_on.items()):
        errors.append("issue #476 follow-on identity or disposition is inconsistent")
    follow_on_review = repo_file(follow_on.get("review_evidence"), label="issue #476 review_evidence")
    if follow_on_review is not None:
        review_text = follow_on_review.read_text(errors="replace")
        if f'git-blake3:{follow_on["reviewed_head"]}:' not in review_text or "Result: pass" not in review_text:
            errors.append("issue #476 review evidence does not bind its exact passed head")
    observed = readback_rows.get(476)
    if not observed or any(observed.get(field) != value for field, value in {
        "pull_request": 477,
        "head_sha": follow_on.get("publication_head"),
        "merge_sha": follow_on.get("merge_sha"),
        "state": "closed",
        "merged": True,
        "adl_ci": "success",
        "adl_coverage": "success",
        "adl_path_policy": "success",
    }.items()):
        errors.append("issue #476 live PR/check readback does not match follow-on evidence")
    if not git_ok("merge-base", "--is-ancestor", follow_on.get("merge_sha", ""), "origin/main"):
        errors.append("issue #476 merge is not ancestral to origin/main")
    if isinstance(follow_on.get("reviewed_head"), str) and isinstance(follow_on.get("publication_head"), str):
        validate_metadata_only_publication(476, follow_on["reviewed_head"], follow_on["publication_head"])
    common_dir = subprocess.run(
        ["git", "-C", str(ROOT), "rev-parse", "--git-common-dir"],
        text=True,
        capture_output=True,
        check=True,
    ).stdout.strip()
    terminal_path = Path(common_dir) / "csdlc-v2" / "derived-terminal" / "476.json"
    try:
        terminal = json.loads(terminal_path.read_text())
    except (OSError, json.JSONDecodeError):
        terminal = {}
    if terminal.get("digest") != follow_on.get("terminal_digest") or terminal.get("merge_sha") != follow_on.get("merge_sha"):
        errors.append("issue #476 terminal cache does not match follow-on evidence")
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
