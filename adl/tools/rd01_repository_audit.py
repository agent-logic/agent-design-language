#!/usr/bin/env python3
"""RD-01 static Git-object inventory. Ownership and category rules are candidates,
not extraction decisions or proof of independent operation. Never emits source lines.
"""
import argparse
import collections
import hashlib
import gzip
import json
import re
import subprocess
import tomllib
from pathlib import Path

BASELINE = 'f69019c24a9b61511e912c93f95442f96fa66d92'
RULES = {
    'csdlc-v3': ('csdlc', 'native lifecycle implementation'),
    'csdlc-v2': ('csdlc', 'retained rollback owner; retain resilience dependency'),
    'adl-runtime': ('runtime', 'runtime contracts and orchestration'),
    'adl-runtime-kernel': ('runtime', 'runtime execution kernel'),
    'adl-provider-core': ('runtime', 'provider execution substrate; distributable shared interface required'),
    'adl-uts': ('public-adl', 'versioned tool schema contract; execution remains with consumer'),
    'adl-resilience': (None, 'shared resilience package requires consumer/distribution decision'),
    'adl-v2': (None, 'public language/engine and Runtime adapter mixed workspace'),
    'adl': (None, 'mixed product implementation, binaries, tests and tooling'),
    'docs': (None, 'cross-product documentation and historical records'),
    '.github': (None, 'shared validation and delivery infrastructure'),
    'infra': (None, 'deployment owner requires service-level decision'),
    '.csdlc': ('historical-evidence', 'retained lifecycle evidence; preserve provenance'),
}
REFERENCE_RULES = {
    'ci': r'github/workflows|\bworkflow_dispatch\b|\bcargo test\b|\brun_owner_validation_lane',
    'cli': r'\[\[bin\]\]|\bCommand::new\(|\bsubcommand\b',
    'checkout': r'git worktree|CARGO_MANIFEST_DIR|git_common|/Volumes/|/Users/|\.\./',
    'generated': r'generated|DO NOT EDIT|auto-generated',
    'credential_reference': r'(?i)credential|secret|token|api[_-]?key|AWS_PROFILE|authorization',
}
PATTERNS = {k: re.compile(v) for k, v in REFERENCE_RULES.items()}


def git(*args):
    return subprocess.check_output(['git', *args])


def classify(path):
    parts = Path(path).parts
    owner, reason = RULES.get(parts[0], (None, 'no reviewed product-level ownership rule'))
    if path.startswith('adl/src/codefriend/'):
        owner, reason = 'codefriend', 'CodeFriend product acquisition/evidence implementation'
    lower = path.lower()
    if any(x in parts for x in ('evidence', 'artifacts', 'rollout_summaries')) or parts[0] == '.csdlc':
        category = 'evidence'
    elif any(x in parts for x in ('generated', 'vendor', 'node_modules', 'target')) or lower.endswith(('.lock', '.min.js', '.min.css')):
        category = 'generated'
    elif any(x in parts for x in ('tests', 'fixtures', 'testdata')) or Path(path).stem.startswith('test_') or Path(path).stem.endswith('_tests'):
        category = 'test'
    else:
        category = 'maintained'
    result = {'owner': owner, 'ownership_status': 'candidate' if owner else 'unresolved',
              'rule_rationale': reason, 'category': category}
    if owner is None:
        result.update(decision_owner='RD-02', decision_deadline='before extraction')
    return result


def validate_coverage(tree, rows):
    expected = [r['path'] for r in tree]
    actual = [r['path'] for r in rows]
    if len(actual) != len(set(actual)):
        raise ValueError('duplicate inventory path')
    if set(actual) != set(expected):
        raise ValueError('missing or extra inventory coverage')
    for row in rows:
        if not row.get('owner') and (row.get('ownership_status') != 'unresolved' or row.get('decision_owner') != 'RD-02' or row.get('decision_deadline') != 'before extraction'):
            raise ValueError('unresolved path lacks decision accountability')


def manifest_family(path):
    name = Path(path).name
    if name == 'Cargo.toml':
        return 'cargo'
    if name in ('package.json', 'package-lock.json', 'yarn.lock', 'pnpm-lock.yaml'):
        return 'node'
    if name in ('pyproject.toml', 'Pipfile', 'Pipfile.lock', 'poetry.lock', 'uv.lock', 'setup.py', 'setup.cfg') or (name.startswith('requirements') and name.endswith(('.txt', '.in'))):
        return 'python'
    if name.endswith(('.tf', '.tf.json')) or name == '.terraform.lock.hcl':
        return 'terraform'
    return None


def dependency_edges(path, data):
    edges = []
    def walk(value, scope):
        if not isinstance(value, dict):
            return
        for key, child in value.items():
            if key in ('dependencies', 'dev-dependencies', 'build-dependencies', 'patch', 'replace') and isinstance(child, dict):
                if key == 'patch':
                    for registry, declarations in child.items():
                        collect(declarations, scope + [key, registry])
                else:
                    collect(child, scope + [key])
            else:
                walk(child, scope + [key])
    def collect(declarations, scope):
        if not isinstance(declarations, dict):
            return
        for name, declaration in declarations.items():
            d = declaration if isinstance(declaration, dict) else {'version': declaration}
            # URLs may embed credentials. Record source class, never URL contents.
            edge = {'manifest': path, 'scope': '.'.join(scope), 'dependency': name,
                    'package': d.get('package', name), 'source': 'path' if 'path' in d else 'git' if 'git' in d else 'workspace' if d.get('workspace') else 'registry'}
            if 'path' in d:
                edge['declared_path'] = d['path']
            if 'version' in d:
                edge['version'] = d['version']
            edge['optional'] = d.get('optional', False)
            edges.append(edge)
    walk(data, [])
    return edges


def self_test():
    rows = [{'path': 'adl/src/lib.rs', **classify('adl/src/lib.rs')}]
    validate_coverage(rows, rows)
    checks = 1
    for bad in (rows + rows, [], [{'path': 'extra', **classify('extra')}], [{'path': rows[0]['path'], 'owner': None}]):
        try:
            validate_coverage(rows, bad)
        except ValueError:
            checks += 1
        else:
            raise AssertionError('negative coverage fixture accepted')
    assert classify('adl/src/codefriend/evidence/store.rs')['owner'] == 'codefriend'
    assert classify('csdlc-v2/Cargo.toml')['owner'] == 'csdlc'
    edges = dependency_edges('Cargo.toml', tomllib.loads('[dependencies]\na={path="../a"}\n[target.x.dev-dependencies]\nb="1"\n[workspace.dependencies]\nc="2"\n[patch.crates-io]\nd={git="https://example.invalid"}'))
    assert len(edges) == 4 and {e['source'] for e in edges} == {'path', 'git', 'registry'}
    assert not any('https://' in json.dumps(e) for e in edges)
    assert manifest_family('x/package.json') == 'node'
    assert manifest_family('x/requirements-dev.txt') == 'python'
    assert manifest_family('infra/.terraform.lock.hcl') == 'terraform'
    checks += 7
    print(json.dumps({'self_test': 'passed', 'assertions': checks, 'negative_fixtures': 4}))


def audit(baseline, output):
    if baseline != BASELINE or git('rev-parse', baseline).decode().strip() != BASELINE:
        raise ValueError('RD-01 requires the exact approved baseline')
    raw = git('ls-tree', '-r', '-z', '-l', '--full-tree', baseline)
    tree = []
    for entry in raw.split(b'\0'):
        if not entry:
            continue
        meta, path = entry.split(b'\t', 1)
        mode, kind, oid, size = meta.decode().split()
        tree.append({'path': path.decode('utf-8'), 'git_object': oid, 'mode': mode,
                     'type': kind, 'size_bytes': None if size == '-' else int(size)})
    rows, edges, manifests, references = [], [], [], []
    # One batch reader avoids one process per tracked blob. Submodule commits have
    # no in-repository contents and remain explicit records rather than disappearing.
    batch = subprocess.Popen(['git', 'cat-file', '--batch'], stdin=subprocess.PIPE, stdout=subprocess.PIPE)
    try:
        for entry in tree:
            row = {**entry, **classify(entry['path'])}
            rows.append(row)
            if entry['type'] != 'blob':
                row['scan_status'] = 'non_blob_not_scanned'
                continue
            batch.stdin.write((entry['git_object'] + '\n').encode()); batch.stdin.flush()
            header = batch.stdout.readline().split(); length = int(header[2])
            content = batch.stdout.read(length); assert batch.stdout.read(1) == b'\n'
            if b'\0' in content:
                row['scan_status'] = 'binary'
                continue
            try:
                text = content.decode('utf-8')
            except UnicodeDecodeError:
                row['scan_status'] = 'non_utf8'
                continue
            row['scan_status'] = 'text'
            row['lines'] = len(text.splitlines())
            hits = {kind: [i for i, line in enumerate(text.splitlines(), 1) if pattern.search(line)] for kind, pattern in PATTERNS.items()}
            hits = {kind: lines for kind, lines in hits.items() if lines}
            if hits:
                references.append({'path': entry['path'], 'references': hits})
            family = manifest_family(entry['path'])
            if family and family != 'cargo':
                manifests.append({'path': entry['path'], 'family': family, 'status': 'recognized_unparsed', 'edge_status': 'unresolved', 'decision_owner': 'RD-02', 'decision_deadline': 'before extraction', 'rationale': 'Static recognition only; dependency resolution and module/lock semantics require owner audit'})
            if family == 'cargo':
                try:
                    data = tomllib.loads(text)
                    edges.extend(dependency_edges(entry['path'], data))
                    manifests.append({'path': entry['path'], 'status': 'parsed', 'package': data.get('package', {}).get('name'), 'workspace_members': data.get('workspace', {}).get('members', [])})
                except tomllib.TOMLDecodeError:
                    manifests.append({'path': entry['path'], 'status': 'parse_error'})
    finally:
        batch.stdin.close(); batch.wait()
    validate_coverage(tree, rows)
    output.mkdir(parents=True, exist_ok=True)
    reports = {'inventory.jsonl.gz': rows, 'references.jsonl.gz': references}
    for name, records in reports.items():
        payload = ''.join(json.dumps(r, sort_keys=True, ensure_ascii=False) + '\n' for r in records).encode('utf-8')
        with (output / name).open('wb') as stream:
            with gzip.GzipFile(filename='', mode='wb', fileobj=stream, mtime=0) as compressed:
                compressed.write(payload)
    (output / 'dependencies.json').write_text(json.dumps({'baseline': baseline, 'scope': 'all tracked Cargo.toml declarations, including target/workspace/patch; no Cargo resolution or independence proof', 'manifests': manifests, 'edges': edges}, indent=2) + '\n')
    summary = {'schema': 'adl.rd01.inventory.v1', 'baseline': baseline, 'tracked_paths': len(rows), 'source_path': 'adl/tools/rd01_repository_audit.py',
               'git_modes': dict(collections.Counter(r['mode'] for r in rows)),
               'symlinks': sum(r['mode'] == '120000' for r in rows),
               'submodule_commits': sum(r['type'] == 'commit' for r in rows),
               'ownership': dict(collections.Counter(r['owner'] or 'unresolved' for r in rows)),
               'categories': {c: {'paths': sum(r['category'] == c for r in rows), 'bytes': sum((r['size_bytes'] or 0) for r in rows if r['category'] == c), 'text_lines': sum(r.get('lines', 0) for r in rows if r['category'] == c)} for c in ('maintained', 'test', 'generated', 'evidence')},
               'manifests': len(manifests), 'manifest_statuses': dict(collections.Counter(m['status'] for m in manifests)), 'dependency_edges': len(edges),
               'reference_policy': 'paths and matching line numbers only; no source text or credential values',
               'limitations': ['Ownership and size categories are heuristic candidates, not RD-02 decisions.', 'Generated content outside named directories/suffixes can be classified maintained; test modules embedded in production files are not split.', 'References are lexical candidates, not proof of functional coupling.', 'Cargo dependency declarations are parsed; recognized Node/Python/Terraform manifests are explicitly unresolved, not zero-dependency claims. Workflow/shell dependency semantics require separate audit.', 'No builds, release, deployment or operational independence established.']}
    summary['reports'] = {p.name: {'bytes': p.stat().st_size, 'sha256': hashlib.sha256(p.read_bytes()).hexdigest()} for p in sorted(output.iterdir()) if p.name in ('inventory.jsonl.gz', 'references.jsonl.gz', 'dependencies.json')}
    (output / 'summary.json').write_text(json.dumps(summary, indent=2) + '\n')
    verify(output)
    # Remove only the two superseded products of this generator after verification.
    for name in ('inventory.jsonl', 'references.jsonl'):
        (output / name).unlink(missing_ok=True)
    print(json.dumps(summary))


def verify(output):
    summary = json.loads((output / 'summary.json').read_text())
    if summary['baseline'] != BASELINE:
        raise ValueError('verification baseline mismatch')
    for name, record in summary['reports'].items():
        if name not in ('inventory.jsonl.gz', 'references.jsonl.gz', 'dependencies.json'):
            raise ValueError('unexpected report path')
        raw = (output / name).read_bytes()
        if len(raw) != record['bytes'] or hashlib.sha256(raw).hexdigest() != record['sha256']:
            raise ValueError('report hash or size mismatch')
    with gzip.open(output / 'inventory.jsonl.gz', 'rt', encoding='utf-8') as stream:
        rows = [json.loads(line) for line in stream]
    entries = []
    for raw in git('ls-tree', '-r', '-z', '-l', '--full-tree', BASELINE).split(b'\0'):
        if raw:
            meta, path = raw.split(b'\t', 1)
            mode, kind, oid, size = meta.decode().split()
            entries.append({'path': path.decode('utf-8'), 'git_object': oid, 'mode': mode,
                            'type': kind, 'size_bytes': None if size == '-' else int(size)})
    validate_coverage(entries, rows)
    indexed = {r['path']: r for r in rows}
    for entry in entries:
        if any(indexed[entry['path']][key] != value for key, value in entry.items()):
            raise ValueError('Git object metadata mismatch')
    if len(rows) != summary['tracked_paths']:
        raise ValueError('summary count mismatch')
    dependencies = json.loads((output / 'dependencies.json').read_text())
    if len(dependencies['manifests']) != summary['manifests'] or len(dependencies['edges']) != summary['dependency_edges']:
        raise ValueError('dependency denominator mismatch')
    if dict(collections.Counter(r['mode'] for r in rows)) != summary['git_modes']:
        raise ValueError('Git mode denominator mismatch')
    with gzip.open(output / 'references.jsonl.gz', 'rt', encoding='utf-8') as stream:
        refs = [json.loads(line) for line in stream]
    if len({r['path'] for r in refs}) != len(refs):
        raise ValueError('duplicate reference path')
    for record in refs:
        if record['path'] not in indexed or any(kind not in PATTERNS or any(not isinstance(n, int) or n < 1 or n > indexed[record['path']].get('lines', 0) for n in lines) for kind, lines in record['references'].items()):
            raise ValueError('invalid reference location')
    print(json.dumps({'verification': 'passed', 'tracked_paths': len(rows), 'reference_paths': len(refs)}))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--baseline', required=False)
    parser.add_argument('--output', type=Path, default=Path('docs/milestones/v0.92.2/evidence/rd01-977'))
    parser.add_argument('--self-test', action='store_true')
    parser.add_argument('--verify', action='store_true')
    args = parser.parse_args()
    if args.self_test:
        self_test()
    if args.baseline:
        audit(args.baseline, args.output)
    elif args.verify:
        verify(args.output)
    elif not args.self_test:
        parser.error('--baseline is required for inventory generation')


if __name__ == '__main__':
    main()
