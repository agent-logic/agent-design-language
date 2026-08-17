#!/usr/bin/env python3
"""Validate #282 production Polis interface qualification packet.

This validator is intentionally local/read-only. It checks that the final
qualification packet contains exact terminal evidence for #279/#280/#281,
operator-runbook commands, review outcome retention, and explicit non-claims.
"""

from __future__ import annotations

import json
import pathlib
import sys


REQUIRED_STRINGS = [
    "Integrated candidate revision: `716f0ff612997449f5c363571b105b670545a1c7`",
    "#279",
    "#280",
    "#281",
    "#393",
    "#394",
    "#395",
    "9d19b2b1175789658bde4f776508aff488060061",
    "6b8eb3435268fcb4618703df8158cee377fe3ad5",
    "716f0ff612997449f5c363571b105b670545a1c7",
    "e2bde4c2b28463e697b406531566b2a7d60b2d0e",
    "a8c3695750dd6037406c225a1b929d5a420a752c",
    "eb6e00399ee75a5208d9a11dff95f26308588732",
    "3dafe3710d57bf2cde222e612d8c9bb1e9c95261de586cc4b4db8c3bc417ad5a",
    "0c0515a24ace9bc1a02da30a2188ac328dfc9b8756d3e5dd82007066c79e59ee",
    "d75c7a1484931153ba29e13b36d8cd50b416f07df4fcfc927044e7d8c376e10a",
    "15b1f64fcdbb9d871174228d80cf9b1d79b7471133418e8e021278e45d444fab",
    "c7f9e4a23c6c9b03dca73b215846261f8fa71a0092065559da7d2d77a5874177",
    "ece3bd46f5e1f2fd1ec66b5bf46d047532c6d733ba66ebbbc83150e796ec70ed",
    "canonical_match=true",
    "Operator runbook",
    "Review outcomes retained",
    "Residual risks and non-claims",
    "does not claim public cloud deployment",
    "does not claim Unity native live proof",
    "does not change Runtime authority",
]

REQUIRED_EVIDENCE_REFERENCES = [
    "279-observatory-accessibility-responsive.log",
    "280-observatory-large-polis-performance-recovery.log",
    "large_polis_performance_recovery_metrics.json",
    "281-observatory-security-privacy-adversarial.log",
    "security_privacy_adversarial.json",
]


def main() -> int:
    packet = pathlib.Path(sys.argv[1]) if len(sys.argv) > 1 else pathlib.Path(
        ".csdlc/evidence/282/production-polis-interface-qualification.md"
    )
    text = packet.read_text(encoding="utf-8")
    missing = [needle for needle in REQUIRED_STRINGS if needle not in text]
    missing.extend(
        reference for reference in REQUIRED_EVIDENCE_REFERENCES if reference not in text
    )
    if missing:
        print(
            json.dumps(
                {
                    "schema": "adl.issue_282.qualification_validation.v1",
                    "status": "fail",
                    "packet": str(packet),
                    "missing": missing,
                },
                indent=2,
                sort_keys=True,
            )
        )
        return 1
    print(
        json.dumps(
            {
                "schema": "adl.issue_282.qualification_validation.v1",
                "status": "pass",
                "packet": str(packet),
                "integrated_candidate": "716f0ff612997449f5c363571b105b670545a1c7",
                "terminal_dependencies": [279, 280, 281],
            },
            indent=2,
            sort_keys=True,
        )
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
