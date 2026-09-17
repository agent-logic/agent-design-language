#!/usr/bin/env python3
"""Construct and invoke existing native adoption/recovery; never convert state here.

PVF: tooling; deterministic request/identity checks, local CPU/files, not live
transition qualification. Native owners remain responsible for admission/effects.
"""
import argparse
import hashlib
import json
from pathlib import Path
import subprocess
import sys

CANDIDATE_SHA256 = '6d30fcc7aa17c417c444145809968d0c286ad15b3338303963c42761a0620fa9'
CARDS = ('sip', 'stp', 'spp', 'vpp', 'srp', 'sor')
REPOSITORY = 'agent-logic/agent-design-language'


def read_json(path):
    return json.loads(Path(path).read_text())


def git(root, *args):
    return subprocess.check_output(['git', '-C', str(root), *args], text=True).strip()


def candidate(path):
    path = Path(path).resolve(strict=True)
    if hashlib.sha256(path.read_bytes()).hexdigest() != CANDIDATE_SHA256:
        raise ValueError('candidate checksum differs from SIM07 qualification')
    return path


def construct_plan(index, cards, retained_plan, issue, checkout):
    """Preserve real values; never infer validator, publication or receipt facts."""
    if index.get('issue') != issue or index.get('repository') != REPOSITORY:
        raise ValueError('native issue identity mismatch')
    phase = index.get('phase')
    if phase not in ('ready', 'bound'):
        raise ValueError('unsupported adoption phase; native recovery/conversion decision required')
    if phase == 'bound' and Path(index['worktree']).resolve() != checkout.resolve():
        raise ValueError('bound adoption must run in its registered worktree')
    if set(cards) != set(CARDS):
        raise ValueError('six current cards required')
    slug = retained_plan.get('slug')
    if not isinstance(slug, str) or not slug or index.get('branch') != f'codex/{issue}-{slug}':
        raise ValueError('retained plan slug does not match native branch')
    if retained_plan.get('schema') != 'csdlc.v3.intent_plan.v1':
        raise ValueError('unsupported retained plan schema')
    if not isinstance(retained_plan.get('validators'), list) or not isinstance(retained_plan.get('publication'), dict):
        raise ValueError('retained validator and publication declarations required')
    for kind, card in cards.items():
        values = card.get('content', {}).get('values', card)
        if (values.get('issue') != issue or values.get('repository') != REPOSITORY
                or values.get('slug') != slug or values.get('card') != kind):
            raise ValueError(f'{kind} card identity mismatch')
    if retained_plan.get('cards') != cards:
        raise ValueError('retained plan differs from current cards; use typed edit, not implicit amendment')
    return {k: retained_plan[k] for k in ('schema', 'slug', 'cards', 'validators', 'publication')}


def native_argv(binary, issue, checkout, action, plan=None, preview=None):
    if not isinstance(issue, int) or issue <= 0:
        raise ValueError('positive issue required')
    argv = [str(binary), action, str(issue)]
    if action == 'prepare':
        if plan is None:
            raise ValueError('prepare requires constructed plan')
        argv += ['--plan', str(plan)]
    elif action == 'recover':
        if preview is not None:
            if not isinstance(preview, str) or not preview.strip():
                raise ValueError('fresh native recovery preview required')
            argv += ['--execute', '--preview', preview]
    elif action not in ('status', 'validate'):
        raise ValueError('unsupported action; no inferred live cutover or snapshot restore')
    return argv + ['--repo-root', str(checkout), '--json']


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('action', choices=('status', 'prepare', 'validate', 'recover'))
    parser.add_argument('--issue', type=int, required=True)
    parser.add_argument('--checkout', type=Path, required=True)
    parser.add_argument('--candidate', type=Path, required=True)
    parser.add_argument('--retained-plan', type=Path)
    parser.add_argument('--output-plan', type=Path)
    parser.add_argument('--preview', help='Fresh digest returned by native recover inspection')
    parser.add_argument('--execute', action='store_true', help='Invoke argv; default prints exact argv without lifecycle effects')
    args = parser.parse_args()
    root = args.checkout.resolve(strict=True)
    binary = candidate(args.candidate)
    if git(root, 'rev-parse', '--show-toplevel') != str(root):
        raise ValueError('checkout must be exact Git root')
    if args.preview and args.action != 'recover':
        raise ValueError('--preview belongs only to recover')
    if args.action == 'prepare':
        if args.retained_plan is None or args.output_plan is None:
            raise ValueError('prepare requires --retained-plan and a new --output-plan')
        common = Path(git(root, 'rev-parse', '--path-format=absolute', '--git-common-dir'))
        native = common / 'csdlc-v3/local/issues' / str(args.issue)
        index = read_json(native / 'index.json')
        card_root = native / 'cards' if index.get('phase') == 'ready' else root / '.csdlc/issues' / str(args.issue) / 'cards'
        cards = {k: read_json(card_root / f'{k}.values.json') for k in CARDS}
        if index.get('phase') == 'bound' and git(root, 'symbolic-ref', '--quiet', '--short', 'HEAD') != index.get('branch'):
            raise ValueError('bound branch mismatch')
        plan = construct_plan(index, cards, read_json(args.retained_plan), args.issue, root)
        # Only a new request file is written; never edit native records or cards.
        with args.output_plan.open('x') as stream:
            json.dump(plan, stream, indent=2)
            stream.write('\n')
    argv = native_argv(binary, args.issue, root, args.action, args.output_plan, args.preview)
    if not args.execute:
        print(json.dumps({'argv': argv, 'lifecycle_executed': False, 'admission': 'not_checked'}))
        return 0
    # No shell, fallback, retry, receipt synthesis or manual lifecycle writes.
    return subprocess.run(argv, check=False).returncode


if __name__ == '__main__':
    try:
        sys.exit(main())
    except (ValueError, OSError, KeyError, subprocess.CalledProcessError) as error:
        print(json.dumps({'status': 'refused', 'reason': str(error)}), file=sys.stderr)
        sys.exit(2)
