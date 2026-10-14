#!/usr/bin/env python3
"""Public ADL CI selection after the coherent private-owner source cutover."""
import argparse
import os
from pathlib import Path
import subprocess

PUBLIC = ('adl-schema/', 'adl-uts/', 'adl-v2/', 'adl-legacy-contracts/', 'adl/', 'tools/public_adl/')
TRANSPILE = ('demos/transpiler_demo/', 'demos/rust-transpiler/', 'tools/transpiler_distribution/')
POLICY = ('.github/', 'tools/source_retirement/', 'tools/godel_contract_distribution/')


def select(event, paths):
    # Unknown, empty or non-PR inputs take the conservative public baseline.
    full = event != 'pull_request' or not paths
    public = full
    transpiler = full
    reason = 'manual_or_empty_public_baseline' if full else 'documentation_only'
    for path in paths:
        if path in ('Cargo.toml', 'Cargo.lock', 'rust-toolchain.toml', 'LICENSE') or path.startswith(POLICY):
            public = transpiler = True
            reason = 'repository_policy'
        elif path.startswith(TRANSPILE):
            transpiler = True
            reason = 'transpiler_distribution'
        elif path.startswith(PUBLIC):
            public = True
            reason = 'public_source_or_contract'
        elif path.endswith('.md') and (path.startswith('docs/') or '/' not in path):
            pass
        else:
            public = transpiler = True
            reason = 'unknown_or_retired_path_public_baseline'
    return {
        'public_adl_validation_required': str(public).lower(),
        'transpiler_distribution_required': str(transpiler).lower(),
        'ci_contracts_required': 'true',
        'reason': reason,
        'automatic_pr_entrypoint': 'ci',
        'optional_workflows_status': 'deferred',
        'optional_workflows_reason': 'explicit_dispatch_required',
        'soak_workflows_status': 'deferred',
        'duplicate_head_status': 'canceled',
        'duplicate_head_reason': 'source_branch_concurrency_cancel_in_progress',
    }


def validate_result(selected, result):
    if (selected, result) not in (('true', 'success'), ('false', 'skipped')):
        raise ValueError('selected CI lane must succeed; unselected lane must be skipped')


def aggregate(kind, env):
    if env.get('PATH_POLICY_RESULT') != 'success':
        raise ValueError('path policy must succeed')
    validate_result(env.get('PUBLIC_REQUIRED'), env.get('PUBLIC_RESULT'))
    if kind == 'ci':
        if env.get('CONTRACT_RESULT') != 'success':
            raise ValueError('repository contracts must succeed')
        validate_result(env.get('TRANSPILE_REQUIRED'), env.get('TRANSPILE_RESULT'))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--event-name', default='workflow_dispatch')
    parser.add_argument('--base', default='')
    parser.add_argument('--head', default='HEAD')
    parser.add_argument('--ref', default='')
    parser.add_argument('--github-output', type=Path)
    parser.add_argument('--aggregate', choices=('ci', 'coverage'))
    args = parser.parse_args()
    if args.aggregate:
        aggregate(args.aggregate, os.environ)
        print('PASS public ' + args.aggregate + ' selected-result contract')
        return
    paths = []
    if args.event_name == 'pull_request':
        if not args.base or not args.head:
            raise ValueError('pull request revision pair required')
        revisions = [subprocess.check_output(
            ['git', 'rev-parse', '--verify', '--end-of-options', ref + '^{commit}']
        ).decode('ascii').strip() for ref in (args.base, args.head)]
        # No rename elision: both deletion and new destination affect selection.
        data = subprocess.check_output(['git', 'diff', '--name-only', '-z', '--no-renames',
                                        *revisions, '--'])
        paths = [p.decode('utf-8', 'strict') for p in data.split(b'\0') if p]
    result = select(args.event_name, paths)
    text = ''.join(key + '=' + value + '\n' for key, value in result.items())
    print(text, end='')
    if args.github_output:
        with args.github_output.open('a') as stream:
            stream.write(text)


if __name__ == '__main__':
    main()
