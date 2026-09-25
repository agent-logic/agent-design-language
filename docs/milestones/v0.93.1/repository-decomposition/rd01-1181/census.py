#!/usr/bin/env python3
"""Issue-local RD01 evidence, not an extraction planner or product engine."""
import collections
import copy
import gzip
import hashlib
import json
from pathlib import Path
import subprocess
import sys

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[4]
BASELINE = 'f69019c24a9b61511e912c93f95442f96fa66d92'
FROZEN = '248f00e359e412f6bc0061ac88be9ff374e3a212'
OPENING = '08236f86026f2e317e7f405bb0381b782a1417b9'


def git(*args):
    return subprocess.check_output(['git', '-C', str(ROOT), *args])


def tree(revision):
    result = {}
    for entry in git('ls-tree', '-rz', '-l', '--full-tree', revision).split(b'\0'):
        if not entry:
            continue
        meta, path = entry.split(b'\t', 1)
        mode, kind, oid, size = meta.decode().split()
        path = path.decode('utf-8')
        result[path] = dict(path=path, mode=mode, type=kind, git_object=oid,
                            size_bytes=None if size == '-' else int(size))
    return result


def owner(path):
    # Only cohesive product surfaces receive candidates. Mixed surfaces remain
    # explicit decisions; documentation naming is not evidence of implementation ownership.
    roots = {'csdlc-v3': 'cognitive-sdlc', 'csdlc-v2': 'cognitive-sdlc',
             'adl-runtime': 'agent-logic-runtime', 'adl-runtime-kernel': 'agent-logic-runtime',
             'adl-provider-core': 'agent-logic-runtime', 'adl-uts': 'agent-design-language',
             'adl-spec': 'agent-design-language'}
    candidate = roots.get(path.split('/')[0])
    rule = 'cohesive-product-root'
    for prefix, product in [('adl/src/codefriend/', 'codefriend'),
                            ('docs/codefriend/', 'codefriend'),
                            ('docs/csdlc-v3/', 'cognitive-sdlc'),
                            ('docs/cognitive-sdlc/', 'cognitive-sdlc'),
                            ('docs/runtime-v3/', 'agent-logic-runtime'),
                            ('docs/runtime/', 'agent-logic-runtime'),
                            ('demos/html-observatory/', 'agent-logic-runtime')]:
        if path.startswith(prefix):
            candidate, rule = product, 'cohesive-product-surface'
    if candidate:
        return dict(owner=candidate, ownership_status='candidate', rule=rule,
                    decision_id=None)
    if path.startswith('.csdlc/') or path.startswith('docs/milestones/'):
        decision = 'D-HISTORY'
    elif path.startswith(('infra/', '.github/', 'tools/')):
        decision = 'D-DELIVERY'
    elif path.startswith('demos/'):
        decision = 'D-DEMOS'
    elif path.startswith('docs/'):
        decision = 'D-DOCS'
    elif path.startswith(('adl-resilience/', 'adl-v2/', 'adl-characterization/')):
        decision = 'D-GENERATION'
    elif path.startswith('adl/'):
        decision = 'D-MIXED-ADL'
    else:
        decision = 'D-ROOT'
    return dict(owner=None, ownership_status='blocked_decision', decision_id=decision,
                decision_owner='RD-02 #1182', decision_deadline='before extraction',
                rule='mixed-or-retained-surface')


def delta(before, after):
    result = []
    for path in sorted(before.keys() | after.keys()):
        a, b = before.get(path), after.get(path)
        if a == b:
            continue
        result.append(dict(path=path, change='added' if a is None else 'deleted' if b is None else 'modified',
                           before=a, after=b))
    return result


def write_gzip(name, rows):
    payload = ''.join(json.dumps(r, sort_keys=True, ensure_ascii=False) + '\n' for r in rows).encode()
    (HERE / name).write_bytes(gzip.compress(payload, mtime=0))


def read_gzip(name):
    return [json.loads(line) for line in gzip.decompress((HERE / name).read_bytes()).decode().splitlines()]


def check_rows(expected, rows, decisions):
    indexed = {r['path']: r for r in rows}
    if len(indexed) != len(rows) or indexed.keys() != expected.keys():
        raise ValueError('path denominator mismatch')
    for path, meta in expected.items():
        row = indexed[path]
        if any(row.get(k) != v for k, v in meta.items()):
            raise ValueError('Git metadata mismatch')
        if row.get('owner'):
            if row['ownership_status'] != 'candidate' or row.get('decision_id') is not None:
                raise ValueError('ambiguous owner')
        elif (row.get('ownership_status') != 'blocked_decision'
              or row.get('decision_id') not in decisions
              or row.get('decision_owner') != 'RD-02 #1182'
              or row.get('decision_deadline') != 'before extraction'):
            raise ValueError('hidden unresolved owner')


def check_delta(before, after, rows):
    if rows != delta(before, after):
        raise ValueError('incomplete or corrupted delta')


def generate():
    before, frozen, opening = tree(BASELINE), tree(FROZEN), tree(OPENING)
    old_bytes = git('show', FROZEN + ':docs/milestones/v0.92.2/evidence/rd01-977/inventory.jsonl.gz')
    old = {r['path']: r for r in map(json.loads, gzip.decompress(old_bytes).decode().splitlines())}
    if old.keys() != before.keys():
        raise ValueError('historical inventory differs from pinned baseline')
    rows = [dict(**meta, **owner(path), previous_977_owner=old.get(path, {}).get('owner'),
                 previous_977_status=old.get(path, {}).get('ownership_status'))
            for path, meta in frozen.items()]
    write_gzip('census.jsonl.gz', rows)
    write_gzip('delta-977.jsonl.gz', delta(before, frozen))
    write_gzip('delta-opening.jsonl.gz', [dict(**r, disposition=owner(r['path'])) for r in delta(frozen, opening)])
    names = ['census.jsonl.gz', 'delta-977.jsonl.gz', 'delta-opening.jsonl.gz']
    summary = dict(schema='adl.rd01.refresh.v1', baseline_977=BASELINE, frozen_predecessor=FROZEN,
                   opening_integration=OPENING, baseline_paths=len(before), frozen_paths=len(frozen),
                   opening_paths=len(opening), owners=dict(collections.Counter(r['owner'] or 'blocked_decision' for r in rows)),
                   decisions=dict(collections.Counter(r['decision_id'] for r in rows if r['decision_id'])),
                   delta_977=dict(collections.Counter(r['change'] for r in delta(before, frozen))),
                   delta_opening=dict(collections.Counter(r['change'] for r in delta(frozen, opening))),
                   reports={n: hashlib.sha256((HERE/n).read_bytes()).hexdigest() for n in names})
    (HERE/'summary.json').write_text(json.dumps(summary, indent=2)+'\n')


def verify():
    summary = json.loads((HERE/'summary.json').read_text())
    decisions = json.loads((HERE/'decisions.json').read_text())
    before, frozen, opening = tree(BASELINE), tree(FROZEN), tree(OPENING)
    if (summary['baseline_977'], summary['frozen_predecessor'], summary['opening_integration']) != (BASELINE, FROZEN, OPENING):
        raise ValueError('revision drift')
    for name in ['census.jsonl.gz', 'delta-977.jsonl.gz', 'delta-opening.jsonl.gz']:
        if hashlib.sha256((HERE/name).read_bytes()).hexdigest() != summary['reports'][name]:
            raise ValueError('report hash mismatch')
    rows = read_gzip('census.jsonl.gz')
    check_rows(frozen, rows, decisions)
    check_delta(before, frozen, read_gzip('delta-977.jsonl.gz'))
    expected = [dict(**r, disposition=owner(r['path'])) for r in delta(frozen, opening)]
    if read_gzip('delta-opening.jsonl.gz') != expected:
        raise ValueError('incomplete opening delta')
    if [summary[k] for k in ['baseline_paths','frozen_paths','opening_paths']] != [len(before),len(frozen),len(opening)]:
        raise ValueError('summary denominator mismatch')
    if summary['owners'] != dict(collections.Counter(r['owner'] or 'blocked_decision' for r in rows)):
        raise ValueError('owner denominator mismatch')
    if summary['decisions'] != dict(collections.Counter(r['decision_id'] for r in rows if r['decision_id'])):
        raise ValueError('decision denominator mismatch')
    for label, a, b in [('delta_977', before, frozen), ('delta_opening', frozen, opening)]:
        if summary[label] != dict(collections.Counter(r['change'] for r in delta(a, b))):
            raise ValueError('delta denominator mismatch')
    old_bytes = git('show', FROZEN + ':docs/milestones/v0.92.2/evidence/rd01-977/inventory.jsonl.gz')
    old = {r['path']: r for r in map(json.loads, gzip.decompress(old_bytes).decode().splitlines())}
    if old.keys() != before.keys():
        raise ValueError('historical inventory mismatch')
    for row in rows:
        prior = old.get(row['path'], {})
        if (row['previous_977_owner'], row['previous_977_status']) != (prior.get('owner'), prior.get('ownership_status')):
            raise ValueError('historical ownership drift')
    for decision in decisions.values():
        if not decision.get('question') or decision.get('owner') != 'RD-02 #1182' or not decision.get('evidence'):
            raise ValueError('unaccountable decision')
    negatives = 0
    for mutate in [lambda r:r.pop(), lambda r:r.append(copy.deepcopy(r[0])),
                   lambda r:r[0].update(git_object='0'*40),
                   lambda r:next(x for x in r if not x['owner']).pop('decision_owner')]:
        bad = copy.deepcopy(rows); mutate(bad)
        try:
            check_rows(frozen, bad, decisions)
        except ValueError:
            negatives += 1
        else:
            raise AssertionError('negative census accepted')
    # Exact ordered deltas include deleted paths and reject dropped entries.
    for a, b in [(before, frozen), (frozen, opening)]:
        actual = delta(a, b)
        assert actual
        try:
            check_delta(a, b, actual[:-1])
        except ValueError:
            negatives += 1
        else:
            raise AssertionError('omitted delta row accepted')
    print(json.dumps(dict(status='pass', paths=len(rows), negative_fixtures=negatives)))


if __name__ == '__main__':
    if sys.argv[1:] == ['--generate']:
        generate()
    elif sys.argv[1:]:
        raise SystemExit('usage: census.py [--generate]')
    verify()
