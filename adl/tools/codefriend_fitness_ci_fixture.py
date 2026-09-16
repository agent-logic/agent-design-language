#!/usr/bin/env python3
"""Qualification preparation only: fixed inert fixtures or an exact candidate checkout."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import subprocess


def require(condition):
    if not condition:
        raise ValueError('fitness_ci_contract_rejected')


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--binary', type=Path, required=True)
    ap.add_argument('--output-root', type=Path, required=True)
    ap.add_argument('--case', choices=['pass', 'fail', 'error', 'candidate'], required=True)
    ap.add_argument('--checkout', type=Path)
    ap.add_argument('--candidate')
    args = ap.parse_args()
    repo = Path(__file__).resolve().parents[2]
    manifest = json.loads((repo / 'adl/tests/fixtures/codefriend/fitness-ci/manifest.json').read_text())
    root = args.output_root.resolve()
    root.mkdir(parents=True, exist_ok=False)
    binary = args.binary.resolve(strict=True)
    require('target' not in binary.parts and (binary.parent / '.provenance/adl.sha256').is_file())

    def git(source, *argv):
        env = dict(os.environ, GIT_AUTHOR_DATE=manifest['fixture_commit_time'], GIT_COMMITTER_DATE=manifest['fixture_commit_time'])
        return subprocess.run(['git', '-C', str(source), *argv], env=env, capture_output=True, check=True).stdout.decode().strip()

    if args.case == 'candidate':
        require(args.checkout is not None and args.candidate)
        source = args.checkout.resolve(strict=True)
        revision = git(source, 'rev-parse', 'HEAD')
        require(revision == args.candidate)
        repository = 'https://github.com/agent-logic/agent-design-language'
        pin = manifest['policies']['candidate']
    else:
        source = root / 'source'
        source.mkdir()
        git(source, 'init', '-b', 'main')
        git(source, 'config', 'core.autocrlf', 'false')
        repository = 'https://example.com/owner/fitness-ci-fixture'
        git(source, 'remote', 'add', 'origin', repository)
        (source / 'lib.rs').write_bytes((repo / f'adl/tests/fixtures/codefriend/fitness/{args.case}.rs').read_bytes())
        git(source, 'add', 'lib.rs')
        git(source, '-c', 'user.name=fixture', '-c', 'user.email=fixture@example.com', 'commit', '-m', 'fitness-ci fixture')
        revision = git(source, 'rev-parse', 'HEAD')
        pin = manifest['policies']['fixture']
    policy = repo / pin['policy']
    policy_value = json.loads(policy.read_text())
    before = git(source, 'status', '--porcelain', '--untracked-files=no')
    scope = root / 'scope.json'
    scope.write_text(json.dumps(dict(analysis=sorted({r['source_path'] for r in policy_value['rules']}), context=[], max_files=4, max_bytes=100000, max_file_bytes=100000)))
    admitted = subprocess.run([str(binary), 'codefriend', 'evidence', 'admit-local', '--checkout', str(source), '--repository', repository, '--revision', revision, '--scope', str(scope), '--store', str(root / 'store'), '--retention-seconds', '86400'], capture_output=True, check=True)
    packet = json.loads(admitted.stdout)['packet_id']
    local = subprocess.run([str(binary), 'codefriend', 'fitness', 'run', '--store', str(root / 'store'), '--packet-id', packet, '--policy', str(policy), '--out', str(root / 'local-report.json')], capture_output=True)
    require(local.returncode in (0, 1, 2))
    require((root / 'local-report.json').is_file())
    require(git(source, 'status', '--porcelain', '--untracked-files=no') == before)
    context = dict(schema='codefriend.fitness.ci.proof.v1', case=args.case, candidate=revision, packet_id=packet, policy_digest=pin['policy_digest'], local_exit=local.returncode, binary_sha256=hashlib.sha256(binary.read_bytes()).hexdigest(), source_unchanged=True, workflow_revision=os.environ.get('GITHUB_SHA'), workflow_run_id=os.environ.get('GITHUB_RUN_ID'))
    (root / 'context.json').write_text(json.dumps(context, indent=2) + '\n')
    invocation = ['--store', str(root / 'store'), '--packet-id', packet, '--policy', str(policy), '--candidate', revision, '--policy-digest', pin['policy_digest'], '--out', str(root / 'ci')]
    (root / 'invocation.json').write_text(json.dumps(invocation))
    print(json.dumps(context))


if __name__ == '__main__':
    main()
