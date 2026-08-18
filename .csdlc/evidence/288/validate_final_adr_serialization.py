#!/usr/bin/env python3
"""Validate #288 final ADR serialization against terminal child evidence."""

from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[3]
MANIFEST = ROOT / ".csdlc/evidence/288/final-adr-serialization-manifest.json"
REVIEW_EVIDENCE = ROOT / "docs/milestones/v0.92/review/first-birthday-review-evidence.v1.json"
INDEX = ROOT / "docs/architecture/adr/V092_ADR_INDEX_143.md"
PLAN = ROOT / "docs/milestones/v0.92/ADR_PLAN_v0.92.md"
ADR0065 = ROOT / "docs/architecture/adr/0065-acip-schema-catalog-and-governed-projection-boundary.md"
HANDOFF = ROOT / "docs/milestones/v0.92/review/V092_ADR_INTERNAL_REVIEW_HANDOFF.md"

EXPECTED = {
    "0065": {
        "status": "Proposed",
        "issue": 283,
        "merge": "2bca3a60243ffc03b8264659ca1d6dd2770dfb0a",
        "digest": "45d457db9ef5232896a358aa3ab3bf941e0cbaff3f04bc634a183c61853939cd",
        "classification": "replacement_terminal_authority",
    },
    "0066": {
        "status": "Deferred",
        "issue": 284,
        "merge": "52de626194d37e25b00e2536285d5e0de332d893",
        "digest": "642bb3f4524ced9efddd8c52a7ac9f96ae3c852695bbf04281e29b2db7b6027f",
        "classification": "residual_gap",
    },
    "0068": {
        "status": "Deferred",
        "issue": 285,
        "merge": "298a23476b3cdc55e952e60793689fc6b738491f",
        "digest": "8b695c8622e7443a6216788f4ece3f09dd4b2842978ac4d9871b5aae661da3e6",
        "classification": "residual_gap",
    },
    "0069": {
        "status": "Deferred",
        "issue": 286,
        "merge": "af49f8f674722bee671d65db5b6a49ea08eeb4b0",
        "digest": "1114e8843d92731a241e46de38c65623da5aa87c801247445684a54a12d6bc75",
        "classification": "residual_gap",
    },
    "0071": {
        "status": "Deferred",
        "issue": 287,
        "merge": "b4f4b1a9104e4d72c52b32b6b42451665a7cca97",
        "digest": "0d55b10c3e0d48d572e5cfc9a23cd850743a008ab1e3cd2118f448ee8e36f227",
        "classification": "residual_gap",
    },
}


def fail(message: str) -> None:
    print(f"#288 final ADR serialization FAIL: {message}", file=sys.stderr)
    sys.exit(1)


def load_json(path: Path) -> dict:
    if not path.exists():
        fail(f"missing {path.relative_to(ROOT)}")
    return json.loads(path.read_text())


def git_common_dir() -> Path:
    proc = subprocess.run(
        ["git", "rev-parse", "--git-common-dir"],
        cwd=ROOT,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=True,
    )
    path = Path(proc.stdout.strip())
    if not path.is_absolute():
        path = (ROOT / path).resolve()
    return path


def assert_ancestral(merge_sha: str) -> None:
    subprocess.run(
        ["git", "merge-base", "--is-ancestor", merge_sha, "HEAD"],
        cwd=ROOT,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=True,
    )


def terminal(issue: int) -> dict:
    path = git_common_dir() / "csdlc-v2/derived-terminal" / f"{issue}.json"
    if not path.exists():
        fail(f"missing terminal cache {path}")
    return json.loads(path.read_text())


def require_text(path: Path, *fragments: str) -> str:
    if not path.exists():
        fail(f"missing {path.relative_to(ROOT)}")
    text = path.read_text()
    for fragment in fragments:
        if fragment not in text:
            fail(f"{path.relative_to(ROOT)} missing fragment: {fragment}")
    return text


def main() -> None:
    manifest = load_json(MANIFEST)
    review = load_json(REVIEW_EVIDENCE)
    rows = {row["adr"]: row for row in manifest["status_matrix"]}
    review_rows = {
        row["adr"]: row for row in review["adr_serialization"]["status_matrix"]
    }
    if set(rows) != set(EXPECTED) or set(review_rows) != set(EXPECTED):
        fail("status matrix ADR set mismatch")

    for adr, expected in EXPECTED.items():
        row = rows[adr]
        review_row = review_rows[adr]
        for key, value in [
            ("status", expected["status"]),
            ("source_issue", expected["issue"]),
            ("merge_sha", expected["merge"]),
            ("canonical_digest", expected["digest"]),
            ("classification", expected["classification"]),
        ]:
            if row.get(key) != value or review_row.get(key) != value:
                fail(f"ADR {adr} {key} mismatch")
        cache = terminal(expected["issue"])
        if cache.get("issue") != expected["issue"]:
            fail(f"terminal cache issue mismatch for ADR {adr}")
        if cache.get("canonical_digest") != expected["digest"]:
            fail(f"terminal cache digest mismatch for ADR {adr}")
        terminal_state = cache.get("issue_state")
        pr_state = cache.get("pr_state")
        if terminal_state != "closed_by_merged_pr" or pr_state != "closed":
            fail(f"terminal cache state mismatch for ADR {adr}")
        assert_ancestral(expected["merge"])

    index = require_text(
        INDEX,
        "| ADR 0065 | Proposed |",
        "| ADR 0066 | Deferred |",
        "| ADR 0068 | Deferred |",
        "| ADR 0069 | Deferred |",
        "| ADR 0071 | Deferred |",
        "#288 promoted only ADR 0065 to Proposed",
    )
    if "| ADR 0065 | Accepted |" in index or "| ADR 0071 | Proposed |" in index:
        fail("ADR index overclaims status")

    plan = require_text(
        PLAN,
        "| ADR 0065 | ACIP Schema Catalog And Governed Projection Boundary | Proposed |",
        "| ADR 0066 | Distributed Guardian Membership, Authority, And Fencing Boundary | Deferred |",
        "| ADR 0068 | Birthday-To-Governance Handoff Boundary | Deferred |",
        "| ADR 0069 | Observatory Governed Runtime Consumer Boundary | Deferred |",
        "| ADR 0071 | Provider-Neutral Multi-Agent Proof Boundary | Deferred |",
        "ADR 0068 remains deferred until terminal birthday proof exists",
    )
    if "| ADR 0065 | ACIP Schema Catalog And Governed Projection Boundary | Accepted |" in plan:
        fail("ADR plan overclaims acceptance")

    require_text(
        ADR0065,
        "Status: **Proposed**",
        "does not itself accept the ADR",
        "#283",
        "#209",
        "remains outside accepted ADR",
    )
    require_text(
        HANDOFF,
        "It is not an approval record and it accepts no ADR.",
        "| ADR 0065 | Proposed | #283 |",
        "| ADR 0066 | Deferred | #284 |",
        "| ADR 0068 | Deferred | #285 |",
        "| ADR 0069 | Deferred | #286 |",
        "| ADR 0071 | Deferred | #287 |",
        "#207 is not closed by this handoff.",
    )

    non_claims = set(manifest["non_claims"])
    if "No ADR is Accepted by #288." not in non_claims:
        fail("manifest missing no-accepted non-claim")
    if "#207 remains coordination-only and is not closed by this manifest." not in non_claims:
        fail("manifest missing #207 non-claim")

    print("#288 final ADR serialization PASS")


if __name__ == "__main__":
    main()
