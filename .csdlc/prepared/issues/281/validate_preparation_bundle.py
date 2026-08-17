#!/usr/bin/env python3
"""Validate the #281 preparation packet without mutating state."""

from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[4]
EXPECTED_TITLE = "[v0.92][WP-18C.07c][117.c] Prove Observatory security, privacy, and adversarial behavior"
EXPECTED_BRANCH = "codex/281-observatory-security-privacy-adversarial-proof"
EXPECTED_WORKTREE = "/Volumes/FastWork/adl-worktrees/adl-issue-281-observatory-security-privacy-adversarial-proof-bound"
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


def run(argv: list[str], cwd: Path = ROOT) -> subprocess.CompletedProcess[str]:
    return subprocess.run(argv, cwd=cwd, text=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)


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
    design = read_text(".csdlc/prepared/issues/281/design.md")
    diagram = read_text(".csdlc/prepared/issues/281/diagram.mmd")
    packets = {
        name: read_json(ROOT / ".csdlc/issues/281/cards" / f"{name}.values.json")
        for name in ["sip", "stp", "spp", "vpp", "srp"]
    }
    combined = "\n".join(
        [
            design,
            diagram,
            *[json.dumps(packet, sort_keys=True) for packet in packets.values()],
        ]
    )

    for needle in [
        EXPECTED_TITLE,
        "security",
        "privacy",
        "adversarial",
        "XSS",
        "Credential",
        "token",
        "Origin",
        "replay",
        "confused-deputy",
        "stale-data",
        "denial",
        "redaction",
        "#279",
        "#280",
        "#282",
        "#117",
        "#110",
    ]:
        if needle not in combined:
            fail(f"missing required scope text: {needle}")

    spp = packets["spp"]["content"]["values"]
    stp = packets["stp"]["content"]["values"]
    sip = packets["sip"]["content"]["values"]
    vpp = packets["vpp"]["content"]["values"]

    expected_affected_areas = {
        "demos/html-observatory/index.html",
        "demos/html-observatory/app.js",
        "demos/html-observatory/styles.css",
        "demos/html-observatory/tests/security_privacy_adversarial.test.mjs",
        ".csdlc/prepared/issues/281",
        ".csdlc/prepared/issues/281/validate_preparation_bundle.py",
        ".csdlc/evidence/281",
        ".csdlc/issues/281",
    }
    affected_areas = set(spp.get("affected_areas", []))
    if affected_areas != expected_affected_areas:
        fail(f"unexpected affected areas: {sorted(affected_areas)}")

    owned_prefixes = (
        "demos/html-observatory/",
        ".csdlc/prepared/issues/281",
        ".csdlc/evidence/281",
        ".csdlc/issues/281",
    )
    owned_fields = stp.get("deliverables", []) + spp.get("affected_areas", [])
    for lane in vpp.get("lanes", []):
        owned_fields.extend(lane.get("argv", []))
    for path in owned_fields:
        if path.startswith(".csdlc/") or path.startswith("demos/"):
            if not path.startswith(owned_prefixes):
                fail(f"out-of-scope owned path: {path}")

    boundary = "\n".join(sip.get("authority_boundary", []))
    non_goals = "\n".join(stp.get("non_goals", []))
    if "Runtime remains the sole communication" not in boundary:
        fail("SIP authority boundary must keep Runtime as sole authority")
    for required_non_goal in [
        "Accessibility/responsive UX proof owned by #279",
        "Large-Polis performance/recovery proof owned by #280",
        "Final production qualification assembly owned by #282",
        "Parent #117 or #110 implementation/closeout",
    ]:
        if required_non_goal not in non_goals:
            fail(f"missing required non-goal: {required_non_goal}")

    for forbidden in [
        "provider credentials are required",
        "cloud/public deployment is required",
        "Unity live host is required",
        "raw provider payload inspection is required",
        "private cognition is required",
    ]:
        if forbidden in combined:
            fail(f"forbidden claim present: {forbidden}")

    for issue in DEPENDENCIES:
        if f"#{issue}" not in combined:
            fail(f"missing dependency reference: #{issue}")

    common = git_common_dir()
    owner = owner_root()
    for issue, merge_sha in DEPENDENCIES.items():
        validation = run(
            [
                str(owner / ".adl" / "bin" / "csdlc-v2" / "csdlc-finish"),
                "--root",
                str(owner),
                "--validate-cached-issue",
                str(issue),
            ],
            cwd=owner,
        )
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

    record = read_json(ROOT / ".csdlc/issues/281/index.json")
    if record.get("issue") != 281:
        fail(f"wrong issue index: {record.get('issue')}")
    if record.get("phase") not in ("initialized", "ready", "bound", "implemented"):
        fail(f"unexpected #281 phase for preparation/bound/implemented validation: {record.get('phase')}")
    if record.get("phase") in ("bound", "implemented"):
        if record.get("branch") != EXPECTED_BRANCH:
            fail(f"unexpected #281 bound branch: {record.get('branch')}")
        if record.get("worktree") != EXPECTED_WORKTREE:
            fail(f"unexpected #281 bound worktree: {record.get('worktree')}")

    print(json.dumps({"issue": 281, "status": "pass", "dependencies_checked": sorted(DEPENDENCIES)}))


if __name__ == "__main__":
    main()
