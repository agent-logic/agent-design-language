#!/usr/bin/env python3
"""Local docs/manifest audit; no network, provider, deployment or release effects."""
import argparse
import copy
import hashlib
import json
import re
from pathlib import Path
from urllib.parse import unquote

ROOT = Path(__file__).resolve().parents[5]
PACKET = Path(__file__).resolve().parent


def check(data):
    failures = []
    if data.get('schema') != 'adl.v0922.documentation_handoff.v1':
        failures.append('schema_invalid')
    if data.get('acceptance') != 'pending_916':
        failures.append('unsupported_acceptance')
    tasks = data.get('tasks', [])
    if len(tasks) != 69 or len({x['issue'] for x in tasks}) != 69 or len({x['task'] for x in tasks}) != 69:
        failures.append('task_denominator')
    actual = json.loads((ROOT / 'docs/milestones/v0.92.2/evidence/issue-916/TASK_LEDGER.json').read_text())
    if sorted((x['task'], x['issue']) for x in tasks) != sorted((x['task'], x['issue']) for x in actual['tasks']):
        failures.append('task_identity_mismatch')
    expected = sorted(x['issue'] for x in actual['tasks'] if x['is_916_prerequisite'])
    if data.get('prerequisites') != expected or len(expected) != 24:
        failures.append('prerequisite_denominator')
    inventory = ROOT / 'docs/milestones/v0.92.2/CANONICAL_DOC_INVENTORY_v0.92.2.md'
    required = {(inventory.parent / unquote(x.split('#')[0])).resolve()
                for x in re.findall(r'\]\(([^)]+)\)', inventory.read_text())
                if not x.startswith(('http:', 'https:'))}
    required.discard(PACKET / 'HANDOFF_MANIFEST.json')
    required.update([inventory, PACKET / 'HANDOFF.md', Path(__file__).resolve()])
    for name in ['QUALITY_DECISION.json', 'TASK_LEDGER.json',
                 'PREREQUISITE_ACCEPTANCE.json', 'POSTMERGE_VERIFICATION.json',
                 'PAIR_HEALTH_RECHECK.json']:
        required.add(PACKET.parent / 'issue-916' / name)
    documents = data.get('documents', [])
    if not documents or len({x['path'] for x in documents}) != len(documents):
        failures.append('document_denominator')
    if {(ROOT / row['path']).resolve() for row in documents} != required:
        failures.append('document_inventory_mismatch')
    for row in documents:
        path = (ROOT / row['path']).resolve()
        if not path.is_relative_to(ROOT) or not path.is_file():
            failures.append('document_missing:' + row['path'])
            continue
        if hashlib.sha256(path.read_bytes()).hexdigest() != row['sha256']:
            failures.append('document_digest:' + row['path'])
        if path.suffix == '.md':
            for target in re.findall(r'\]\(([^)]+)\)', path.read_text()):
                if target.startswith(('https:', 'http:', 'mailto:', '#')):
                    continue
                linked = (path.parent / unquote(target.split('#')[0])).resolve()
                if not linked.is_relative_to(ROOT) or not linked.exists():
                    failures.append('link_missing:' + row['path'] + ':' + target)
    for name in ['native_main', 'website_main', 'inherited_916_checkpoint']:
        if not re.fullmatch('[0-9a-f]{40}', data.get('sources', {}).get(name, '')):
            failures.append('source_identity:' + name)
    return failures


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--self-test', action='store_true')
    args = parser.parse_args()
    data = json.loads((PACKET / 'HANDOFF_MANIFEST.json').read_text())
    failures = check(data)
    rejected = 0
    if args.self_test:
        mutations = [lambda x: x.update(schema='wrong'),
                     lambda x: x['documents'][0].update(sha256='0' * 64),
                     lambda x: x['tasks'].append(x['tasks'][0]),
                     lambda x: x['prerequisites'].pop(),
                     lambda x: x.update(acceptance='accepted'),
                     lambda x: x['documents'].pop()]
        for mutation in mutations:
            broken = copy.deepcopy(data)
            mutation(broken)
            if check(broken):
                rejected += 1
            else:
                failures.append('negative_case_not_rejected')
    print(json.dumps({'status': 'pass' if not failures else 'fail',
                      'documents': len(data['documents']), 'tasks': len(data['tasks']),
                      'prerequisites': len(data['prerequisites']),
                      'negative_fixtures': rejected, 'failures': failures,
                      'handoff_accepted': False, 'release_authorized': False,
                      'scope': 'Local path/digest/identity checks; external URLs, fragments, private raw replay and human review not established.'}, indent=2))
    raise SystemExit(bool(failures))


if __name__ == '__main__':
    main()
