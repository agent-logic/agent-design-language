#!/usr/bin/env python3
"""Validate #282 production Polis interface qualification packet.

This validator is intentionally local/read-only. It checks that the final
qualification packet contains exact terminal evidence for #279/#280/#281,
operator-runbook commands, review outcome retention, and explicit non-claims.
"""

from __future__ import annotations

import json
import pathlib
import re
import subprocess
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

REVIEWED_HEAD = "0befd94f4aceb186840c92e51533b555d2aa992e"
INTEGRATED_CANDIDATE = "716f0ff612997449f5c363571b105b670545a1c7"
OWNER_ROOT = pathlib.Path("/Users/daniel/git/agent-design-language")
FINISH_BIN = OWNER_ROOT / ".adl" / "bin" / "csdlc-v2" / "csdlc-finish"
ISSUE_ROOTS = {
    279: pathlib.Path("/Volumes/FastWork/adl-worktrees/adl-issue-279-observatory-accessibility-responsive-ux-proof"),
    280: pathlib.Path("/Volumes/FastWork/adl-worktrees/adl-issue-280-large-polis-performance-recovery"),
    281: pathlib.Path("/Volumes/FastWork/adl-worktrees/adl-issue-281-observatory-security-privacy-adversarial-proof-bound"),
}
EXPECTED_TERMINAL = {
    279: {
        "pull_request": 393,
        "merge_sha": "9d19b2b1175789658bde4f776508aff488060061",
        "head_sha": "e2bde4c2b28463e697b406531566b2a7d60b2d0e",
        "canonical_generation": 14,
        "canonical_digest": "3dafe3710d57bf2cde222e612d8c9bb1e9c95261de586cc4b4db8c3bc417ad5a",
        "terminal_digest": "15b1f64fcdbb9d871174228d80cf9b1d79b7471133418e8e021278e45d444fab",
    },
    280: {
        "pull_request": 394,
        "merge_sha": "6b8eb3435268fcb4618703df8158cee377fe3ad5",
        "head_sha": "a8c3695750dd6037406c225a1b929d5a420a752c",
        "canonical_generation": 15,
        "canonical_digest": "0c0515a24ace9bc1a02da30a2188ac328dfc9b8756d3e5dd82007066c79e59ee",
        "terminal_digest": "c7f9e4a23c6c9b03dca73b215846261f8fa71a0092065559da7d2d77a5874177",
    },
    281: {
        "pull_request": 395,
        "merge_sha": "716f0ff612997449f5c363571b105b670545a1c7",
        "head_sha": "eb6e00399ee75a5208d9a11dff95f26308588732",
        "canonical_generation": 16,
        "canonical_digest": "d75c7a1484931153ba29e13b36d8cd50b416f07df4fcfc927044e7d8c376e10a",
        "terminal_digest": "ece3bd46f5e1f2fd1ec66b5bf46d047532c6d733ba66ebbbc83150e796ec70ed",
    },
}
EXPECTED_TABLE_SCOPE = {
    279: "Observatory accessibility and responsive UX proof",
    280: "Large-Polis performance and recovery behavior proof",
    281: "Observatory security, privacy, and adversarial behavior proof",
}
REQUIRED_EVIDENCE_PATHS = [
    pathlib.Path("/Volumes/FastWork/adl-worktrees/adl-issue-279-observatory-accessibility-responsive-ux-proof/.csdlc/evidence/279/279-observatory-accessibility-responsive.log"),
    pathlib.Path("/Volumes/FastWork/adl-worktrees/adl-issue-280-large-polis-performance-recovery/.csdlc/evidence/280/280-observatory-large-polis-performance-recovery.log"),
    pathlib.Path("/Volumes/FastWork/adl-worktrees/adl-issue-280-large-polis-performance-recovery/.csdlc/evidence/280/large_polis_performance_recovery_metrics.json"),
    pathlib.Path("/Volumes/FastWork/adl-worktrees/adl-issue-281-observatory-security-privacy-adversarial-proof-bound/.csdlc/evidence/281/281-observatory-security-privacy-adversarial.log"),
    pathlib.Path("/Volumes/FastWork/adl-worktrees/adl-issue-281-observatory-security-privacy-adversarial-proof-bound/.csdlc/evidence/281/security_privacy_adversarial.json"),
]


def run_json(argv: list[str], cwd: pathlib.Path) -> dict:
    completed = subprocess.run(
        argv,
        cwd=cwd,
        check=False,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )
    if completed.returncode != 0:
        raise AssertionError(
            f"command failed ({completed.returncode}): {' '.join(argv)}\n"
            f"stdout:\n{completed.stdout}\nstderr:\n{completed.stderr}"
        )
    return json.loads(completed.stdout)


def run_ok(argv: list[str], cwd: pathlib.Path) -> None:
    completed = subprocess.run(
        argv,
        cwd=cwd,
        check=False,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )
    if completed.returncode != 0:
        raise AssertionError(
            f"command failed ({completed.returncode}): {' '.join(argv)}\n"
            f"stdout:\n{completed.stdout}\nstderr:\n{completed.stderr}"
        )


def current_head(cwd: pathlib.Path) -> str:
    completed = subprocess.run(
        ["git", "rev-parse", "HEAD"],
        cwd=cwd,
        check=False,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )
    if completed.returncode != 0:
        raise AssertionError(
            f"failed to resolve HEAD\nstdout:\n{completed.stdout}\nstderr:\n{completed.stderr}"
        )
    return completed.stdout.strip()


def parse_terminal_table(text: str) -> dict[int, dict[str, str]]:
    rows: dict[int, dict[str, str]] = {}
    for line in text.splitlines():
        match = re.match(r"^\|\s*#(279|280|281)\s*\|(.+)\|$", line)
        if not match:
            continue
        cells = [cell.strip().strip("`") for cell in line.strip().strip("|").split("|")]
        if len(cells) != 9:
            raise AssertionError(f"unexpected terminal dependency table shape: {line}")
        issue = int(cells[0].lstrip("#"))
        rows[issue] = {
            "scope": cells[1],
            "pull_request": int(cells[2].lstrip("#")),
            "merge_sha": cells[3],
            "head_sha": cells[4],
            "canonical_generation": int(cells[5]),
            "canonical_digest": cells[6],
            "terminal_digest": cells[7],
            "canonical_cache": cells[8],
        }
    return rows


def main() -> int:
    packet = pathlib.Path(sys.argv[1]) if len(sys.argv) > 1 else pathlib.Path(
        ".csdlc/evidence/282/production-polis-interface-qualification.md"
    )
    text = packet.read_text(encoding="utf-8")
    missing = [needle for needle in REQUIRED_STRINGS if needle not in text]
    missing.extend(
        reference for reference in REQUIRED_EVIDENCE_REFERENCES if reference not in text
    )
    missing.extend(str(path) for path in REQUIRED_EVIDENCE_PATHS if not path.is_file())
    if current_head(pathlib.Path.cwd()) != REVIEWED_HEAD:
        missing.append(
            f"current worktree HEAD did not match reviewed qualification head {REVIEWED_HEAD}"
        )
    table_rows = parse_terminal_table(text)
    if sorted(table_rows) != [279, 280, 281]:
        missing.append(f"terminal dependency table rows were {sorted(table_rows)}")
    observed = {}
    for issue, expected in EXPECTED_TERMINAL.items():
        result = run_json(
            [str(FINISH_BIN), "--root", str(ISSUE_ROOTS[issue]), "--validate-cached-issue", str(issue)],
            cwd=OWNER_ROOT,
        )
        terminal = result.get("terminal") or {}
        checks = {
            "canonical_match": result.get("canonical_match"),
            "pull_request": terminal.get("pull_request"),
            "merge_sha": terminal.get("merge_sha"),
            "head_sha": terminal.get("head_sha"),
            "canonical_generation": terminal.get("canonical_generation"),
            "canonical_digest": terminal.get("canonical_digest"),
            "terminal_digest": terminal.get("digest"),
            "disposition": terminal.get("disposition"),
            "issue_state": terminal.get("issue_state"),
        }
        observed[issue] = checks
        row = table_rows.get(issue) or {}
        if row.get("scope") != EXPECTED_TABLE_SCOPE[issue]:
            missing.append(
                f"issue {issue} table scope: expected {EXPECTED_TABLE_SCOPE[issue]}, observed {row.get('scope')}"
            )
        if row.get("canonical_cache") != "canonical_match=true":
            missing.append(
                f"issue {issue} table canonical cache was {row.get('canonical_cache')}"
            )
        for key, value in expected.items():
            if checks.get(key) != value:
                missing.append(f"issue {issue} terminal {key}: expected {value}, observed {checks.get(key)}")
            if row.get(key) != value:
                missing.append(f"issue {issue} table {key}: expected {value}, observed {row.get(key)}")
        if checks["canonical_match"] is not True:
            missing.append(f"issue {issue} canonical_match was not true")
        if checks["disposition"] != "merged":
            missing.append(f"issue {issue} disposition was not merged")
        if checks["issue_state"] != "closed_by_merged_pr":
            missing.append(f"issue {issue} issue_state was not closed_by_merged_pr")
    run_ok(["git", "merge-base", "--is-ancestor", INTEGRATED_CANDIDATE, REVIEWED_HEAD], cwd=pathlib.Path.cwd())
    if missing:
        print(
            json.dumps(
                {
                    "schema": "adl.issue_282.qualification_validation.v1",
                    "status": "fail",
                "packet": str(packet),
                "missing": missing,
                "observed_terminal": observed,
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
                "reviewed_head": "0befd94f4aceb186840c92e51533b555d2aa992e",
                "terminal_dependencies": [279, 280, 281],
            },
            indent=2,
            sort_keys=True,
        )
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
