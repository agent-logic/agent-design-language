#!/usr/bin/env python3
"""Validate the #286 ADR 0069 preparation packet."""

from pathlib import Path
import re
import sys

ROOT = Path(__file__).resolve().parents[4]
BASE = ROOT / ".csdlc" / "prepared" / "issues" / "286"

REQUIRED_TEXT = {
    BASE / "design.md": [
        "#286 is an ADR evidence-reconciliation issue for #207.d",
        "exact landed WP-18A and WP-18C revision identities",
        "retained human-review reference and",
        "retained machine-readable outcome reference",
        "Residual gaps are allowed",
        "does not move ADR 0069 to Accepted",
        "#288 / #207.f owns final serialized ADR",
    ],
    BASE / "diagram.mmd": [
        "Live issue #286 contract",
        "#207 ADR coordination",
        "#288 final ADR index/manifest serialization",
        "terminal only when proven",
    ],
}


def fail(message: str) -> None:
    print(f"FAIL: {message}", file=sys.stderr)
    raise SystemExit(1)


def main() -> None:
    for path, needles in REQUIRED_TEXT.items():
        if not path.is_file():
            fail(f"missing required artifact: {path.relative_to(ROOT)}")
        text = path.read_text(encoding="utf-8")
        for needle in needles:
            if needle not in text:
                fail(f"{path.relative_to(ROOT)} missing expected boundary text: {needle}")

    forbidden_claims = [
        "ADR 0069 accepted",
        "terminal WP-18C proof complete",
        "terminal WP-18C proof",
        "terminal WP-18C Runtime/Observatory evidence",
        "terminal WP-18C Runtime/Observatory integration evidence",
        "WP-18C terminal proof complete",
        "WP-18C terminal closeout",
        "#207 terminal",
        "#207 complete",
        "#207 closeout",
        "#288 terminal",
        "#288 complete",
        "#288 closeout",
        "terminal #207",
        "terminal #288",
        "complete #207",
        "complete #288",
        "implements Runtime",
        "implements UI",
        "updates shared ADR index",
    ]
    combined = "\n".join(path.read_text(encoding="utf-8") for path in REQUIRED_TEXT)
    for claim in forbidden_claims:
        if claim.lower() in combined.lower():
            fail(f"forbidden overclaim present: {claim}")

    forbidden_patterns = [
        (re.compile(r"\bWP-18C\b.*\bcomplete\b", re.IGNORECASE), "WP-18C complete"),
        (re.compile(r"\bcomplete\b.*\bWP-18C\b", re.IGNORECASE), "complete WP-18C"),
        (re.compile(r"\bWP-18C\b.*\bcloseout\b", re.IGNORECASE), "WP-18C closeout"),
        (re.compile(r"\bcloseout\b.*\bWP-18C\b", re.IGNORECASE), "closeout WP-18C"),
        (re.compile(r"\bWP-18C\b.*\bterminal\b", re.IGNORECASE), "WP-18C terminal"),
        (re.compile(r"\bterminal\b.*\bWP-18C\b", re.IGNORECASE), "terminal WP-18C"),
        (re.compile(r"#207\b.*\b(is\s+)?terminal\b", re.IGNORECASE), "#207 terminal"),
        (re.compile(r"\bterminal\b.*#207\b", re.IGNORECASE), "terminal #207"),
        (re.compile(r"#207\b.*\b(is\s+)?complete\b", re.IGNORECASE), "#207 complete"),
        (re.compile(r"\bcomplete\b.*#207\b", re.IGNORECASE), "complete #207"),
        (re.compile(r"#207\b.*\bcloseout\b", re.IGNORECASE), "#207 closeout"),
        (re.compile(r"\bcloseout\b.*#207\b", re.IGNORECASE), "closeout #207"),
        (re.compile(r"#288\b.*\b(is\s+)?terminal\b", re.IGNORECASE), "#288 terminal"),
        (re.compile(r"\bterminal\b.*#288\b", re.IGNORECASE), "terminal #288"),
        (re.compile(r"#288\b.*\b(is\s+)?complete\b", re.IGNORECASE), "#288 complete"),
        (re.compile(r"\bcomplete\b.*#288\b", re.IGNORECASE), "complete #288"),
        (re.compile(r"#288\b.*\bcloseout\b", re.IGNORECASE), "#288 closeout"),
        (re.compile(r"\bcloseout\b.*#288\b", re.IGNORECASE), "closeout #288"),
    ]
    allowed_terminal_qualifiers = [
        "terminal only when proven",
        "terminal-proving:",
        "terminal/current",
    ]
    for path in REQUIRED_TEXT:
        for line_number, line in enumerate(path.read_text(encoding="utf-8").splitlines(), start=1):
            line_lower = line.lower()
            if any(qualifier in line_lower for qualifier in allowed_terminal_qualifiers):
                continue
            for pattern, label in forbidden_patterns:
                if pattern.search(line):
                    fail(
                        f"{path.relative_to(ROOT)}:{line_number} forbidden overclaim pattern present: {label}"
                    )

    print("PASS #286 preparation packet: ADR 0069 evidence reconciliation boundary is explicit")


if __name__ == "__main__":
    main()
