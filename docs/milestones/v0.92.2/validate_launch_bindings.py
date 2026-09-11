"""PVF: deterministic local launch identity/evidence contract; no delivery proof."""
import copy
import hashlib
import json
import re
from pathlib import Path
import subprocess
import sys

STARTUP_GATE = {'policy': 'all_69_created_and_reviewed_before_new_implementation', 'core_task_count': 69, 'adds_dependency_edges': False}
INDEPENDENT_REVIEWERS = {'/root/task_contract_review', '/root/docs_final_review', '/root/planning_docs', '/root/final_launch_audit'}
EXISTING = {'WP-01': 864, 'OBS-LIVE': 720, 'ARCH-SPLIT': 848, 'CSDLC-MERGE': 849, 'QUAL-RUNTIME': 852, 'RT-COST': 854, 'RT-PROVIDER': 855, 'CSDLC-MAN': 861, 'CSDLC-DECOMPOSE': 862}

SIDECARS = [{'issue': 671, 'milestone': 2, 'core_task': False, 'startup_gate_member': False, 'preserve_existing_approval_gates': True}]


def review_failures(post, expected, readback_bytes, authors=()):
    expected_review = {k: {'number': v, 'result': 'pass'} for k, v in expected.items()}
    if (post.get('schema') != 'adl.live-issue-review.v1' or post.get('result') != 'pass'
            or post.get('findings') != [] or post.get('reviewer') not in INDEPENDENT_REVIEWERS
            or post.get('reviewer') in authors
            or post.get('issues') != expected_review
            or post.get('readbacks_sha256') != hashlib.sha256(readback_bytes).hexdigest()):
        return ['independent final review denominator, result or readback hash mismatch']
    return []


def native_readback_failures(remote, native, number):
    """Check native receipt parity and separately reviewed bounded projection.

    Native remote/mod.rs hashes the entire authenticated REST Value using
    BLAKE3(serde_json bytes + NUL). The retained gh-view projection is not that
    REST object, so its digest cannot truthfully be compared to readback_digest.
    The final independent review binds these projection bytes through SHA256;
    receipt/reconciliation parity and operation marker bind native observation.
    """
    failures = []
    result = native['result']['outcome']['result']
    receipt, reconciliation = result['receipt'], result['reconciliation']
    if remote.get('number') != number or (remote.get('milestone') or {}).get('number') != 2 or 'version:v0.92.2' not in {v['name'] for v in remote.get('labels', [])}:
        failures.append(f'#{number}: final identity or milestone/label mismatch')
    for record in (receipt, reconciliation):
        if record.get('authenticated') is not True or record.get('issue') != number or record.get('repository') != 'agent-logic/agent-design-language':
            failures.append(f'#{number}: native authenticated identity mismatch')
    for field in ('operation_digest', 'readback_digest', 'expected_head_sha'):
        value = receipt.get(field, '')
        length = 40 if field == 'expected_head_sha' else 64
        if not re.fullmatch(f'[0-9a-f]{{{length}}}', value) or value != reconciliation.get(field):
            failures.append(f'#{number}: native reconciliation {field} mismatch')
    marker = f"<!-- csdlc-v3-operation:{receipt.get('operation_digest')} -->"
    if reconciliation.get('operation_marker') != marker or marker not in remote.get('body', ''):
        failures.append(f'#{number}: native operation marker not retained')
    return failures


def load_existing_proof(repo, prove=False):
    folder = repo / '.csdlc/evidence/864/existing-issue-launch-review'
    failures = []
    try:
        raw = (folder / 'final-readbacks.json').read_bytes()
        readbacks = json.loads(raw)
        review = json.loads((folder / 'postcreation-review.json').read_text())
        pointers = json.loads((folder / 'native-receipts.json').read_text())
        if set(readbacks) != set(EXISTING) or set(pointers) != set(EXISTING):
            failures.append('Existing final readback/native receipt denominator must be exactly nine')
        failures.extend(review_failures(review, EXISTING, raw))
        if review.get('reviewer') != '/root/docs_final_review':
            failures.append('Existing issue final review must be the assigned independent reviewer')
        for key, number in EXISTING.items():
            path = repo / pointers[key]
            if not path.resolve().is_relative_to(repo.resolve()):
                raise ValueError('Native evidence pointer escapes repository')
            failures.extend(native_readback_failures(readbacks[key], json.loads(path.read_text()), number))
            sprint = EXISTING_EXECUTION_SPRINTS[key]
            if f'## Execution sprint assignment\n\n**Sprint {sprint}**' not in readbacks[key].get('body', ''):
                failures.append(f'{key}: live execution sprint assignment differs')
        sidecar = repo / '.csdlc/evidence/864/sidecar-671'
        failures.extend(native_readback_failures(json.loads((sidecar / 'readback.json').read_text()), json.loads((sidecar / 'native-routing-update.json').read_text()), 671))
        sidecar_readback = json.loads((sidecar / 'readback.json').read_text())
        if 'backlog' in sidecar_readback.get('title', '').lower() or 'track:backlog' in {item['name'] for item in sidecar_readback.get('labels', [])}:
            failures.append('#671 current sidecar routing still says backlog')
        if prove:
            paths = [folder / name for name in ('final-readbacks.json', 'postcreation-review.json', 'native-receipts.json')]
            paths += [repo / value for value in pointers.values()]
            paths += [sidecar / name for name in ('readback.json', 'native-routing-update.json')]
            for path in paths:
                if subprocess.run(['git', 'ls-files', '--error-unmatch', str(path.relative_to(repo))], cwd=repo, capture_output=True).returncode:
                    failures.append(f'Untracked existing/sidecar evidence: {path.relative_to(repo)}')
    except (OSError, KeyError, TypeError, ValueError) as error:
        failures.append(f'Missing or invalid final existing issue evidence: {error}')
    return failures


def existing_review_negative_checks():
    raw = b'exact fixture readback bytes'
    review = {'schema': 'adl.live-issue-review.v1', 'reviewer': '/root/docs_final_review', 'result': 'pass', 'findings': [], 'readbacks_sha256': hashlib.sha256(raw).hexdigest(), 'issues': {k: {'number': n, 'result': 'pass'} for k, n in EXISTING.items()}}
    cases = []
    for field, value in [('result', 'fail'), ('reviewer', '/root'), ('reviewer', 'unassigned-reviewer'), ('findings', ['unresolved']), ('readbacks_sha256', '0' * 64), ('issues', {})]:
        broken = copy.deepcopy(review); broken[field] = value
        cases.append((f'existing review {field}={value}', broken))
    for key in EXISTING:
        for mode in ('missing', 'failed', 'wrong'):
            broken = copy.deepcopy(review)
            if mode == 'missing': del broken['issues'][key]
            elif mode == 'failed': broken['issues'][key]['result'] = 'fail'
            else: broken['issues'][key]['number'] = 999999
            cases.append((f'{key} {mode} final review', broken))
    missed = [name for name, broken in cases if not review_failures(broken, EXISTING, raw)]
    if not review_failures(review, EXISTING, raw, {'/root/docs_final_review'}):
        missed.append('postcreation reviewer also authored draft')
    return missed, len(cases) + 1


def load_births(repo, config, prove=False):
    """Read tracked birth evidence, validate independently reviewed immutable sources."""
    births, failures = {}, []
    groups = {'1': config['first_sprint'], **config['remaining_batches']}
    if set(groups) != {str(n) for n in range(1, 12)}:
        return {}, ['Expected exactly eleven creation groups']
    for n in range(1, 12):
        folder = repo / f'.csdlc/evidence/864/sprint{n:02}-launch'
        try:
            ledger = json.loads((folder / 'issues.json').read_text())
            post = json.loads((folder / 'postcreation-review.json').read_text())
            readback = folder / 'readbacks.json'
            if set(ledger) != set(groups[str(n)]):
                failures.append(f'Batch {n}: birth membership differs')
            pre = json.loads((folder / 'precreation-review.json').read_text())
            if n == 1:
                # S1 predates author fields; all authors are known from the
                # independently reviewed draft assignments. Fresh final review
                # must be independent of every one of those authors.
                authors = {'/root', '/root/task_contract_review', '/root/planning_docs', '/root/docs_final_review'}
            else:
                authors = {row.get('author') for row in pre['issues']}
                if None in authors or '' in authors:
                    failures.append(f'Batch {n}: missing precreation author identity')
            review_errors = review_failures(post, {k: v['number'] for k, v in ledger.items()}, readback.read_bytes(), authors)
            failures.extend(f'Batch {n}: {error}' for error in review_errors)
            for key, value in ledger.items():
                if key in births:
                    failures.append(f'Duplicate birth task {key}')
                births[key] = {'number': value['number'], 'batch': n}
            if prove:
                required = [folder / f for f in ['issues.json', 'precreation-review.json', 'postcreation-review.json', 'readbacks.json']]
                required += [folder / v['final_receipt'] for v in ledger.values()]
                for path in required:
                    result = subprocess.run(['git', 'ls-files', '--error-unmatch', str(path.relative_to(repo))], cwd=repo, capture_output=True)
                    if result.returncode:
                        failures.append(f'Untracked birth evidence: {path.relative_to(repo)}')
                command = [sys.executable, str(folder / 'validate_launch.py')] if n == 1 else [sys.executable, str(repo / '.csdlc/evidence/864/validate_issue_batches.py'), str(n)]
                proof = subprocess.run(command, cwd=repo, text=True, capture_output=True)
                if proof.returncode:
                    failures.append(f'Batch {n}: immutable draft/review/native evidence validator failed: {proof.stdout[-1000:]} {proof.stderr[-300:]}')
        except (OSError, KeyError, TypeError, ValueError) as error:
            failures.append(f'Batch {n}: missing or invalid reviewed birth evidence: {error}')
    numbers = [v['number'] for v in births.values()]
    if len(births) != 60 or len(set(numbers)) != 60 or any(type(v) is not int or v <= 0 for v in numbers):
        failures.append('Expected sixty unique positive native birth identities')
    return births, failures


def binding_failures(wave, specs, atomic, config, births):
    failures = []
    rows = {r['id']: r for r in wave['work_packages']}
    spec_rows = {r['id']: r for r in specs['specifications']}
    expected = atomic['preexisting_bindings'] | {k: v['number'] for k, v in births.items()}
    if len(expected) != 69 or len(set(expected.values())) != 69 or set(rows) != set(expected):
        failures.append('Final binding denominator must be exactly 69 unique core tasks')
    for key, issue in expected.items():
        if rows.get(key, {}).get('issue') != issue or spec_rows.get(key, {}).get('issue') != issue:
            failures.append(f'{key}: issue differs from native birth/existing identity')
        if key in births and rows.get(key, {}).get('creation_policy') != f"created_sprint{births[key]['batch']:02}":
            failures.append(f'{key}: wrong creation batch policy')
    if atomic.get('existing_bindings') != expected or atomic.get('created_bindings') != {k: v['number'] for k, v in births.items()} or atomic.get('prospective_count') != 0:
        failures.append('Final atomic binding projection differs')
    if config.get('startup_gate') != STARTUP_GATE:
        failures.append('All-69 startup requirement or existing admission boundary changed')
    if config.get('sidecars') != SIDECARS or 671 in expected.values():
        failures.append('#671 sidecar scope drift')
    return failures


def binding_negative_checks(wave, specs, atomic, config, births):
    cases = []
    for key in births:
        for value in (None, 999999):
            w = copy.deepcopy(wave)
            next(r for r in w['work_packages'] if r['id'] == key)['issue'] = value
            cases.append((f'{key} missing/wrong birth {value}', w, specs, atomic, config))
    for name, field, value in [('missing startup gate', 'startup_gate', None), ('sidecar admitted to core', 'sidecars', []), ('batch gates added', 'startup_gate', {**STARTUP_GATE, 'adds_dependency_edges': True}), ('partial startup denominator', 'startup_gate', {**STARTUP_GATE, 'core_task_count': 68})]:
        c = copy.deepcopy(config); c[field] = value
        cases.append((name, wave, specs, atomic, c))
    missed = [name for name, w, s, a, c in cases if not binding_failures(w, s, a, c, births)]
    return missed, len(cases)


EXISTING_EXECUTION_SPRINTS = {'WP-01': 1, 'CSDLC-MAN': 1, 'ARCH-SPLIT': 2, 'RT-COST': 2, 'RT-PROVIDER': 2, 'QUAL-RUNTIME': 5, 'CSDLC-MERGE': 7, 'CSDLC-DECOMPOSE': 7, 'OBS-LIVE': 8}


def execution_sprint_failures(config, wave):
    groups = config.get('execution_sprints', {})
    failures = []
    if set(groups) != {str(n) for n in range(1, 12)} or any(not isinstance(v, list) for v in groups.values()):
        return ['Expected eleven explicit execution sprint groups']
    members = [key for keys in groups.values() for key in keys]
    rows = {r['id']: r for r in wave['work_packages']}
    if len(members) != 69 or len(set(members)) != 69 or set(members) != set(rows):
        failures.append('Execution sprints must cover all69 core tasks exactly once, excluding sidecars')
    assigned = {key: int(n) for n, keys in groups.items() for key in keys}
    expected = dict(EXISTING_EXECUTION_SPRINTS)
    for n, keys in {'1': config['first_sprint'], **config['remaining_batches']}.items():
        for key in keys:
            if key in expected:
                failures.append(f'{key}: creation membership duplicates an existing task')
            expected[key] = int(n)
    if assigned != expected:
        failures.append('Execution sprint membership differs from adopted existing/new task assignments')
    for key, row in rows.items():
        for dependency in row.get('depends_on', []):
            if dependency not in assigned or key not in assigned or assigned[dependency] > assigned[key]:
                failures.append(f'{key}: execution sprint precedes prerequisite {dependency}')
    return failures


def execution_sprint_negative_checks(config, wave):
    cases = []
    for key, number in EXISTING_EXECUTION_SPRINTS.items():
        for mode in ('missing', 'duplicate', 'wrong_sprint'):
            c = copy.deepcopy(config)
            if mode != 'duplicate': c['execution_sprints'][str(number)].remove(key)
            if mode != 'missing': c['execution_sprints'][str(number + 1)].append(key)
            cases.append((f'{key} execution {mode}', c))
    c = copy.deepcopy(config); c['execution_sprints']['1'].append('SIDECAR-671')
    cases.append(('sidecar in core execution sprint', c))
    return [name for name, c in cases if not execution_sprint_failures(c, wave)], len(cases)


def execution_sprint_projection_failures(text, config, wave):
    match = re.search(r'^## Execution sprint assignments\n(.*?)(?=^## |\Z)', text, re.M | re.S)
    if not match:
        return ['Missing execution sprint Markdown projection']
    numbers = {r['id']: r.get('issue') for r in wave['work_packages']}
    observed, failures = {}, []
    for line in match.group(1).splitlines():
        columns = [part.strip() for part in line.strip().strip('|').split('|')]
        if not line.startswith('|') or not columns[0].isdigit():
            continue
        n = columns[0]
        if n in observed or len(columns) != 4:
            failures.append('Duplicate or malformed execution sprint table row')
            continue
        pairs = re.findall(r'([A-Z][A-Z0-9-]+) \(#(\d+)\)', columns[2] + ', ' + columns[3])
        observed[n] = [(key, int(number)) for key, number in pairs]
    expected = {n: [(key, numbers.get(key)) for key in keys] for n, keys in config.get('execution_sprints', {}).items()}
    if observed != expected:
        failures.append('Execution sprint Markdown differs from exact machine task/issue assignments')
    return failures
