#!/usr/bin/env python3
"""Run the v3 Sprint 8/9 readiness canary against real typed issue readbacks."""

from __future__ import annotations

import json
import pathlib
import re
import subprocess
import sys


REPO = "agent-logic/agent-design-language"
ROOT = pathlib.Path.cwd()
EVIDENCE_DIR = pathlib.Path(".csdlc/evidence/sprints-8-9-v3-readiness")
REQUEST_DIR = pathlib.Path(".csdlc/prepared/issues/505/sprints-8-9-readiness")
V2_ISSUE = pathlib.Path(".adl/bin/csdlc-v2/csdlc-github-issue")
V3 = [
    "cargo",
    "run",
    "--quiet",
    "--manifest-path",
    "csdlc-v3/Cargo.toml",
    "--bin",
    "csdlc",
    "--",
]

SPRINTS = [
    {
        "sprint": 8,
        "umbrella_issue": 536,
        "title": "Sprint 8",
        "execution_mode": "hybrid",
        "serial_gates": [
            "#512 OBS-B waits for #511 OBS-A",
            "#84 Unity authority remains independent and outside the current Sprint 8 membership denominator",
            "Sprint closeout waits for independently reviewed child PRs and terminal issue truth",
        ],
    },
    {
        "sprint": 9,
        "umbrella_issue": 537,
        "title": "Sprint 9",
        "execution_mode": "sequential",
        "serial_gates": [
            "#516 PROV-B follows #515 PROV-A",
            "#517 INT-01 waits for every declared root dependency",
            "#518 and #519 tail work waits for provider-comparison/convergence truth",
        ],
    },
]


def write_issue_read_request(issue: int) -> pathlib.Path:
    REQUEST_DIR.mkdir(parents=True, exist_ok=True)
    target = REQUEST_DIR / f"read-live-issue-{issue}.json"
    payload = {
        "action": "issue_read",
        "repository": REPO,
        "operation_key": f"worker-6-v3-sprint-readiness-live-issue-{issue}-20260901",
        "token_file": None,
        "issue": issue,
        "pull_request": None,
        "title": None,
        "body": None,
        "labels": [],
        "assignees": [],
        "milestone": None,
        "state": None,
        "comment_body": None,
        "required_checks": [],
        "require_review": False,
        "linked_issue": None,
    }
    target.write_text(json.dumps(payload, indent=2) + "\n", encoding="utf-8")
    return target


def read_issue(issue: int) -> dict:
    request = write_issue_read_request(issue)
    target = EVIDENCE_DIR / f"issue-{issue}-readback.json"
    result = subprocess.run(
        [str(V2_ISSUE), "run", "--request", str(request)],
        cwd=ROOT,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )
    if result.returncode != 0:
        sys.stderr.write(result.stderr)
        raise SystemExit(result.returncode)
    target.write_text(result.stdout, encoding="utf-8")
    return json.loads(result.stdout)


def parse_children(body: str) -> list[int]:
    children: list[int] = []
    in_membership = False
    for line in body.splitlines():
        stripped = line.strip()
        if stripped.startswith("## "):
            in_membership = "child membership" in stripped
            continue
        if not in_membership:
            continue
        match = re.match(r"- #([0-9]+)\b", stripped)
        if match:
            children.append(int(match.group(1)))
    return children


def main() -> None:
    EVIDENCE_DIR.mkdir(parents=True, exist_ok=True)
    sprint_payloads = []
    readbacks: dict[int, dict] = {}
    for sprint in SPRINTS:
        umbrella_issue = sprint["umbrella_issue"]
        umbrella = read_issue(umbrella_issue)
        readbacks[umbrella_issue] = umbrella
        children = parse_children(umbrella["issue"]["body"])
        for child in children:
            readbacks[child] = read_issue(child)
        sprint_payload = dict(sprint)
        sprint_payload["umbrella_readback_ref"] = (
            f".csdlc/evidence/sprints-8-9-v3-readiness/issue-{umbrella_issue}-readback.json"
        )
        sprint_payload["child_readback_refs"] = {
            str(child): f".csdlc/evidence/sprints-8-9-v3-readiness/issue-{child}-readback.json"
            for child in children
        }
        sprint_payloads.append(sprint_payload)
    request = {"repository": REPO, "version": "v0.92.1", "sprints": sprint_payloads}
    request_path = REQUEST_DIR / "sprint-readiness-request.json"
    request_path.write_text(json.dumps(request, indent=2) + "\n", encoding="utf-8")
    report_path = EVIDENCE_DIR / "sprint-8-9-readiness-report.json"
    result = subprocess.run(
        V3 + ["sprint", "--repo-root", ".", "--request", str(request_path)],
        cwd=ROOT,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )
    if result.returncode != 0:
        sys.stderr.write(result.stderr)
        raise SystemExit(result.returncode)
    report_path.write_text(result.stdout, encoding="utf-8")
    run_issue_local_canaries(readbacks)
    print(result.stdout, end="")


def run_issue_local_canaries(readbacks: dict[int, dict]) -> None:
    registrations_path = REQUEST_DIR / "issue-local-registrations.json"
    registrations = []
    for issue in [511, 515]:
        registrations.append(
            {
                "branch": f"codex/{issue}-v3-readiness-canary",
                "worktree": f"adl-worktrees/adl-issue-{issue}-v3-readiness-canary",
                "primary": False,
            }
        )
    registrations_path.write_text(json.dumps(registrations, indent=2) + "\n", encoding="utf-8")
    for issue in [511, 515]:
        issue_readback = readbacks[issue]["issue"]
        request = {
            "issue": issue,
            "title": issue_readback["title"],
            "repository": REPO,
            "branch": f"codex/{issue}-v3-readiness-canary",
            "worktree": f"adl-worktrees/adl-issue-{issue}-v3-readiness-canary",
            "registry_version": "1.0.3",
            "commands": ["prepare_issue", "bind_worktree", "plan_pvf", "doctor"],
        }
        request_path = REQUEST_DIR / f"issue-{issue}-local-request.json"
        request_path.write_text(json.dumps(request, indent=2) + "\n", encoding="utf-8")
        report_path = EVIDENCE_DIR / f"issue-{issue}-local-readiness-report.json"
        result = subprocess.run(
            V3
            + [
                "local",
                "--request",
                str(request_path),
                "--registry",
                "docs/templates/prompts/current.json",
                "--registrations",
                str(registrations_path),
            ],
            cwd=ROOT,
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            check=False,
        )
        if result.returncode != 0:
            sys.stderr.write(result.stderr)
            raise SystemExit(result.returncode)
        report_path.write_text(result.stdout, encoding="utf-8")


if __name__ == "__main__":
    main()
