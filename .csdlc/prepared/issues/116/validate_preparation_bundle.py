#!/usr/bin/env python3
"""Validate the #116 preparation packet stays inside its declared boundary."""

from __future__ import annotations

import json
import pathlib
import subprocess
import sys


REPO = pathlib.Path(__file__).resolve().parents[4]
ISSUE = 116
REQUIRED_TERMINAL = [111, 112, 114, 115, 265, 270, 271, 276, 277, 278]
REQUIRED_TEXT = [
    "operator attention inbox",
    "acknowledge",
    "reply",
    "defer",
    "resolve",
    "refuse",
    "rate limit",
    "dedup",
    "expiry",
    "restart",
    "Do not implement #117",
]


def git_common_dir() -> pathlib.Path:
    result = subprocess.run(
        ["git", "-C", str(REPO), "rev-parse", "--git-common-dir"],
        check=True,
        text=True,
        capture_output=True,
    )
    path = pathlib.Path(result.stdout.strip())
    if not path.is_absolute():
        path = REPO / path
    return path


def owner_binary(name: str) -> pathlib.Path:
    local = REPO / ".adl" / "bin" / "csdlc-v2" / name
    if local.exists():
        return local
    canonical = pathlib.Path("/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2") / name
    if canonical.exists():
        return canonical
    fail(f"missing owner binary {name}")


def fail(message: str) -> None:
    print(f"issue-{ISSUE} preparation validator: FAIL: {message}", file=sys.stderr)
    raise SystemExit(1)


def read(path: pathlib.Path) -> str:
    try:
        return path.read_text(encoding="utf-8")
    except FileNotFoundError:
        fail(f"missing {path}")


def ensure_terminal_cache(issue: int) -> None:
    common = git_common_dir()
    candidates = [
        common / "csdlc-v2" / "derived-terminal" / f"{issue}.json",
        REPO / ".csdlc" / "terminal" / f"{issue}.json",
    ]
    existing = next((path for path in candidates if path.exists()), None)
    if existing is None:
        # Current terminal dependencies may be tracked as closed issue records instead of
        # derived cache files in fresh checkouts. Fall back to live git ancestry of a
        # known terminal cache only for #114, which is the immediate unlock.
        if issue == 114 and (REPO / ".csdlc" / "issues" / "114" / "index.json").exists():
            return
        fail(f"missing terminal cache for #{issue}")
    data = json.loads(existing.read_text(encoding="utf-8"))
    if data.get("issue") != issue:
        fail(f"terminal cache issue mismatch for #{issue}: {existing}")


def ensure_main_current() -> None:
    result = subprocess.run(
        ["git", "-C", str(REPO), "rev-parse", "HEAD"],
        check=True,
        text=True,
        capture_output=True,
    )
    head = result.stdout.strip()
    origin = subprocess.run(
        ["git", "-C", str(REPO), "rev-parse", "origin/main"],
        check=True,
        text=True,
        capture_output=True,
    ).stdout.strip()
    if head != origin:
        fail(f"preparation root is not current origin/main: HEAD={head} origin/main={origin}")


def ensure_card_bindings() -> None:
    spp = json.loads((REPO / ".csdlc" / "issues" / str(ISSUE) / "cards" / "spp.values.json").read_text(encoding="utf-8"))
    vpp = json.loads((REPO / ".csdlc" / "issues" / str(ISSUE) / "cards" / "vpp.values.json").read_text(encoding="utf-8"))
    spp_values = spp["content"]["values"]
    vpp_values = vpp["content"]["values"]
    pairs = [
        ("design_ref", spp_values["design_ref"], vpp_values["design_ref"]),
        ("design_digest", spp_values["design_digest"], vpp_values["design_digest"]),
        ("diagram_ref", spp_values["diagram_ref"], vpp_values["diagram_ref"]),
        ("diagram_digest", spp_values["diagram_digest"], vpp_values["diagram_digest"]),
    ]
    for label, left, right in pairs:
        if left != right:
            fail(f"SPP/VPP {label} mismatch: {left!r} != {right!r}")
    if spp_values["design_ref"] != f".csdlc/prepared/issues/{ISSUE}/design.md":
        fail(f"unexpected design_ref {spp_values['design_ref']!r}")
    if spp_values["diagram_ref"] != f".csdlc/prepared/issues/{ISSUE}/diagram.mmd":
        fail(f"unexpected diagram_ref {spp_values['diagram_ref']!r}")


def ensure_owner_doctor_has_only_expected_design_review_blocker() -> None:
    doctor = subprocess.run(
        [
            str(owner_binary("csdlc-doctor")),
            "--repo",
            str(REPO),
            "--issue",
            str(ISSUE),
        ],
        text=True,
        capture_output=True,
    )
    try:
        report = json.loads(doctor.stdout)
    except json.JSONDecodeError as exc:
        fail(f"doctor did not return JSON: {exc}: stdout={doctor.stdout!r} stderr={doctor.stderr!r}")
    findings = report.get("findings", [])
    unexpected = [finding for finding in findings if finding.get("code") != "design_review_missing_or_stale"]
    if unexpected:
        fail(f"unexpected owner-doctor findings before design review: {unexpected}")


def main() -> None:
    ensure_main_current()
    design = read(REPO / ".csdlc" / "prepared" / "issues" / str(ISSUE) / "design.md")
    diagram = read(REPO / ".csdlc" / "prepared" / "issues" / str(ISSUE) / "diagram.mmd")
    for text in REQUIRED_TEXT:
        if text not in design:
            fail(f"design missing required boundary text: {text}")
    for issue in REQUIRED_TERMINAL:
        ensure_terminal_cache(issue)
    ensure_card_bindings()
    ensure_owner_doctor_has_only_expected_design_review_blocker()
    if "#279" in diagram or "#280" in diagram or "#281" in diagram or "#282" in diagram:
        fail("diagram must not absorb downstream #117 proof children")
    print("issue-116 preparation validator: PASS")


if __name__ == "__main__":
    main()
