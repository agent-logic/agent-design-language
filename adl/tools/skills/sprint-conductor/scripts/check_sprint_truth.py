#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import os
import shlex
import subprocess
from pathlib import Path
from typing import Any


def first_json_payload(text: str) -> Any:
    decoder = json.JSONDecoder()
    for index, char in enumerate(text):
        if char not in '{[':
            continue
        try:
            payload, _ = decoder.raw_decode(text[index:])
            return payload
        except json.JSONDecodeError:
            continue
    raise ValueError('command did not emit a JSON payload')


def run_json(cmd: list[str], *, cwd: Path) -> Any:
    out = subprocess.check_output(cmd, cwd=cwd, text=True)
    return first_json_payload(out)


def default_issue_command(repo_root: Path, subcommand: str) -> list[str]:
    issue_binary = repo_root / 'adl' / 'target' / 'debug' / 'adl-issue'
    if issue_binary.is_file():
        return [str(issue_binary), subcommand]
    return ['bash', str(repo_root / 'adl' / 'tools' / 'pr.sh'), 'issue', subcommand]


def default_pr_validation_command(repo_root: Path) -> list[str]:
    validation_binary = repo_root / 'adl' / 'target' / 'debug' / 'adl-pr-validation'
    if validation_binary.is_file():
        return [str(validation_binary)]
    return ['bash', str(repo_root / 'adl' / 'tools' / 'pr.sh'), 'validation']


def command_with_override(env_var: str, default: list[str]) -> list[str]:
    raw = os.environ.get(env_var)
    if not raw:
        return default
    return shlex.split(raw)


def issue_view(repo_root: Path, issue_number: int) -> dict[str, Any]:
    cmd = command_with_override('ADL_SPRINT_ISSUE_VIEW_CMD', default_issue_command(repo_root, 'view'))
    return run_json(cmd + [str(issue_number), '--json'], cwd=repo_root)


def pr_validation(repo_root: Path, pr_url: str) -> dict[str, Any]:
    cmd = command_with_override('ADL_SPRINT_PR_VALIDATION_CMD', default_pr_validation_command(repo_root))
    payload = run_json(cmd + [pr_url, '--json'], cwd=repo_root)
    if 'pr_state' in payload:
        return {
            'state': payload.get('pr_state'),
            'isDraft': payload.get('is_draft'),
            'url': payload.get('url') or pr_url,
        }
    return payload


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument('--repo-root', required=True)
    parser.add_argument('--state', required=True)
    parser.add_argument('--print-json', action='store_true')
    parser.add_argument('--require-match', action='store_true')
    args = parser.parse_args()

    repo_root = Path(args.repo_root).resolve()
    state_path = Path(args.state)
    state = json.loads(state_path.read_text())
    issue_records = state.get('issue_records', [])
    issue_numbers = [record.get('issue_number') for record in issue_records if record.get('issue_number') is not None]
    pr_urls = [record.get('pr_url') for record in issue_records if record.get('pr_url')]

    notes: list[str] = []
    drift = False

    for issue_number in issue_numbers:
        issue = issue_view(repo_root, issue_number)
        record = next((r for r in issue_records if r.get('issue_number') == issue_number), None)
        if record is None:
            continue
        issue_state = str(issue.get('state', '')).upper()
        record['github_issue_state'] = issue_state
        if issue_state == 'CLOSED' and record.get('status') not in {'closed_out'}:
            local_status = record.get('status')
            drift = True
            notes.append(
                f'issue #{issue_number} is CLOSED on GitHub but local status is {local_status}; '
                'record_child_issue_closeout.py must run before sprint state can advance'
            )
        if issue_state == 'OPEN' and record.get('status') == 'closed_out':
            drift = True
            notes.append(f'issue #{issue_number} is OPEN on GitHub but local status is closed_out')

    for pr_url in pr_urls:
        pr = pr_validation(repo_root, pr_url)
        matching = next((r for r in issue_records if r.get('pr_url') == pr_url), None)
        if matching is None:
            continue
        live_url = pr.get('url') or pr_url
        if live_url != pr_url:
            drift = True
            notes.append(f'PR URL drift for {pr_url}; repo-native validation returned {live_url}')
        matching['github_pr_state'] = pr.get('state')
        matching['github_pr_is_draft'] = pr.get('isDraft')

    truth_check = {
        'status': 'drift_detected' if drift else 'matched',
        'source': 'github_live',
        'gate_passed': not drift,
        'checked_issue_numbers': issue_numbers,
        'checked_pr_urls': pr_urls,
        'notes': notes,
    }
    state['truth_check'] = truth_check
    state_path.write_text(json.dumps(state, indent=2, sort_keys=True) + '\n')

    if args.print_json:
        print(json.dumps(truth_check, indent=2, sort_keys=True))
    else:
        print(state_path)
    if args.require_match and drift:
        return 2
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
