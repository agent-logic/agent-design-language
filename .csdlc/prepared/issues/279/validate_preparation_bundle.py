#!/usr/bin/env python3
"""Validate the #279 preparation packet without mutating state."""

from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[4]
EXPECTED_TITLE = "[v0.92][WP-18C.07a][117.a] Prove Observatory accessibility and responsive UX"
EXPECTED_BRANCH = "codex/279-observatory-accessibility-responsive-ux-proof"
EXPECTED_WORKTREE = "/Volumes/FastWork/adl-worktrees/adl-issue-279-observatory-accessibility-responsive-ux-proof"
DEPENDENCIES = {
    111: "5dab282aa6b730efd057f0502dacd462d30cc1d0",
    112: "6172bfb067bd45ec231fbc2635e7efbb718ef415",
    113: "a260e14ab4a56b95fe5b37e4ffaff3f263bc58c1",
    114: "1d8685745b00df78f304cb03a6a559fa4e2cdec9",
    115: "22122c6c245b1f847aabcaf168a98660a3f11972",
    116: "557dd28d85746a8dc5109dcc674f5a606b8c9890",
    265: "301080a40c91c6882f34fead3c742524467c056d",
    270: "b1c38cd53573c03cdc4ad818ed5ead5eba570981",
    271: "6b200cfee83ea36a546123de4d24a6eda191b652",
    276: "3e249f9857f392f7f569560fbd5fbfbc36b95b2f",
    277: "3160fb8be575ba9a27748b05ea5dd911e4375deb",
    278: "c3ecaa615fbc29c1784d4e89f4fe38a98743ff02",
}


def fail(message: str) -> None:
    print(f"FAIL: {message}", file=sys.stderr)
    sys.exit(1)


def run(argv: list[str]) -> subprocess.CompletedProcess[str]:
    return subprocess.run(argv, cwd=ROOT, text=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)


def git_common_dir() -> Path:
    proc = run(["git", "rev-parse", "--git-common-dir"])
    if proc.returncode != 0:
        fail(f"cannot resolve git common dir: {proc.stderr.strip()}")
    common = Path(proc.stdout.strip())
    if not common.is_absolute():
        common = ROOT / common
    return common.resolve()


def owner_root() -> Path:
    return git_common_dir().parent


def read_text(path: str) -> str:
    try:
        return (ROOT / path).read_text(encoding="utf-8")
    except FileNotFoundError:
        fail(f"missing required artifact: {path}")


def read_json(path: Path) -> dict:
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except FileNotFoundError:
        fail(f"missing required JSON: {path}")
    except json.JSONDecodeError as exc:
        fail(f"invalid JSON in {path}: {exc}")


def assert_ancestral(sha: str) -> None:
    proc = run(["git", "merge-base", "--is-ancestor", sha, "origin/main"])
    if proc.returncode != 0:
        fail(f"dependency merge {sha} is not ancestral to origin/main")


def main() -> None:
    design = read_text(".csdlc/prepared/issues/279/design.md")
    diagram = read_text(".csdlc/prepared/issues/279/diagram.mmd")
    sip_packet = read_json(ROOT / ".csdlc/issues/279/cards/sip.values.json")
    stp_packet = read_json(ROOT / ".csdlc/issues/279/cards/stp.values.json")
    spp_packet = read_json(ROOT / ".csdlc/issues/279/cards/spp.values.json")
    vpp_packet = read_json(ROOT / ".csdlc/issues/279/cards/vpp.values.json")
    sip = sip_packet["content"]["values"]
    stp = stp_packet["content"]["values"]
    spp = spp_packet["content"]["values"]
    vpp = vpp_packet["content"]["values"]
    combined = "\n".join([
        design,
        diagram,
        json.dumps(sip_packet["identity"], sort_keys=True),
        json.dumps(stp_packet["identity"], sort_keys=True),
        json.dumps(spp_packet["identity"], sort_keys=True),
        json.dumps(vpp_packet["identity"], sort_keys=True),
        json.dumps(sip, sort_keys=True),
        json.dumps(stp, sort_keys=True),
        json.dumps(spp, sort_keys=True),
        json.dumps(vpp, sort_keys=True),
    ])

    for needle in [
        EXPECTED_TITLE,
        "accessibility",
        "responsive",
        "keyboard",
        "focus",
        "roles",
        "labels",
        "reduced-motion",
        "contrast",
        "#280",
        "#281",
        "#282",
        "#117",
        "#110",
        "Closes #279",
    ]:
        if needle not in combined:
            fail(f"missing required scope text: {needle}")

    expected_affected_areas = {
        "demos/html-observatory/index.html",
        "demos/html-observatory/app.js",
        "demos/html-observatory/styles.css",
        "demos/html-observatory/tests/accessibility_responsive.test.mjs",
        ".csdlc/prepared/issues/279",
        ".csdlc/prepared/issues/279/validate_preparation_bundle.py",
        ".csdlc/evidence/279",
        ".csdlc/issues/279",
    }
    affected_areas = set(spp.get("affected_areas", []))
    if affected_areas != expected_affected_areas:
        fail(f"unexpected affected areas: {sorted(affected_areas)}")

    owned_prefixes = (
        "demos/html-observatory/",
        ".csdlc/prepared/issues/279",
        ".csdlc/evidence/279",
        ".csdlc/issues/279",
    )
    owned_fields = (
        stp.get("deliverables", [])
        + spp.get("affected_areas", [])
        + vpp.get("lanes", [])[0].get("argv", [])
    )
    for path in owned_fields:
        if path.startswith(".csdlc/") or path.startswith("demos/"):
            if not path.startswith(owned_prefixes):
                fail(f"out-of-scope owned path: {path}")

    authority_boundary = "\n".join(sip.get("authority_boundary", []))
    non_goals = "\n".join(stp.get("non_goals", []))
    if "Runtime remains the sole communication" not in authority_boundary:
        fail("SIP authority boundary must keep Runtime as sole authority")
    for sibling in ["#280", "#281", "#282", "#117", "#110"]:
        if sibling not in authority_boundary and sibling not in non_goals:
            fail(f"missing structured exclusion for {sibling}")
    for required_non_goal in [
        "Large-Polis performance/recovery proof owned by #280",
        "Security/privacy/adversarial proof owned by #281",
        "Final production qualification assembly owned by #282",
        "Parent #117 or #110 implementation/closeout",
        "Cloud/public deployment, Unity feature implementation, provider credentials, or paid/optional jobs",
    ]:
        if required_non_goal not in non_goals:
            fail(f"missing required non-goal: {required_non_goal}")

    for forbidden in [
        "Runtime authority changes are allowed",
        "provider credentials are required",
        "cloud/public deployment is required",
        "Unity live host is required",
    ]:
        if forbidden in combined:
            fail(f"forbidden claim present: {forbidden}")

    for issue in DEPENDENCIES:
        if f"#{issue}" not in combined:
            fail(f"missing dependency reference: #{issue}")

    common = git_common_dir()
    owner = owner_root()
    for issue, merge_sha in DEPENDENCIES.items():
        validation = run([
            str(owner / ".adl" / "bin" / "csdlc-v2" / "csdlc-finish"),
            "--root",
            str(owner),
            "--validate-cached-issue",
            str(issue),
        ])
        if validation.returncode != 0:
            fail(f"issue {issue} cached terminal validation failed: {validation.stderr.strip() or validation.stdout.strip()}")
        try:
            packet = json.loads(validation.stdout)
        except json.JSONDecodeError as exc:
            fail(f"issue {issue} cached terminal validation returned invalid JSON: {exc}")
        if packet.get("canonical_match") is not True:
            fail(f"issue {issue} cached terminal validation is not canonical_match=true")
        terminal = read_json(common / "csdlc-v2" / "derived-terminal" / f"{issue}.json")
        if terminal.get("schema") != "csdlc.derived_terminal.v1":
            fail(f"issue {issue} terminal cache has wrong schema")
        if terminal.get("disposition") != "merged":
            fail(f"issue {issue} is not terminal merged")
        if terminal.get("merge_sha") != merge_sha:
            fail(f"issue {issue} merge SHA drift: {terminal.get('merge_sha')} != {merge_sha}")
        assert_ancestral(merge_sha)

    record = read_json(ROOT / ".csdlc" / "issues" / "279" / "index.json")
    if record.get("issue") != 279:
        fail(f"wrong issue index: {record.get('issue')}")
    if record.get("phase") not in ("initialized", "ready", "bound"):
        fail(f"unexpected #279 phase for preparation/bound validation: {record.get('phase')}")
    if record.get("phase") == "bound":
        if record.get("branch") != EXPECTED_BRANCH:
            fail(f"unexpected #279 bound branch: {record.get('branch')}")
        if record.get("worktree") != EXPECTED_WORKTREE:
            fail(f"unexpected #279 bound worktree: {record.get('worktree')}")
    else:
        if record.get("branch") not in (None, EXPECTED_BRANCH):
            fail(f"unexpected #279 branch before bind: {record.get('branch')}")
        if record.get("worktree") not in (None, EXPECTED_WORKTREE):
            fail(f"unexpected #279 worktree before bind: {record.get('worktree')}")

    print("PASS #279 preparation bundle validates dependency gates, accessibility/responsive scope, and sibling/parent exclusions")


if __name__ == "__main__":
    main()
