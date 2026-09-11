#!/usr/bin/env python3
"""PVF local_contract: deterministic Git source accounting; small CPU/disk.
Required for #836 evidence; not a runtime or behavior release gate.
"""
import argparse
from collections import defaultdict
import difflib
import hashlib
import json
from pathlib import Path
import re
import subprocess

SCOPE = ['adl/src/resilience.rs', 'adl/src/resilience', 'adl/tests']
SCHEMA = 'adl.rust_recursive_size.v1'


def git(root, *args):
    return subprocess.check_output(['git', '-C', str(root), *args])


def revision(root, value):
    if not re.fullmatch(r'[0-9a-f]{40}', value):
        raise ValueError('full immutable 40-character Git revision required')
    if git(root, 'rev-parse', '--verify', value + '^{commit}').decode().strip() != value:
        raise ValueError('revision must name a commit')
    return value


def inventory(root, rev):
    # Enumerate recursively before applying literal file/directory filters.
    entries = git(root, 'ls-tree', '-r', '-z', rev).split(b'\0')
    result = {}
    for entry in entries:
        if not entry:
            continue
        meta, raw_path = entry.split(b'\t', 1)
        mode, kind, blob = meta.decode().split()
        path = raw_path.decode('utf-8')
        if not path.endswith('.rs') or not any(path == s or path.startswith(s + '/') for s in SCOPE):
            continue
        if kind != 'blob' or mode not in ('100644', '100755'):
            raise ValueError('non-regular Rust source in scope: ' + path)
        data = git(root, 'cat-file', 'blob', blob)
        if b'\0' in data:
            raise ValueError('binary Rust source: ' + path)
        # LF-delimited physical lines; include a nonempty unterminated final line.
        lines = data.split(b'\n')
        if lines[-1] == b'':
            lines.pop()
        result[path] = {'blob': blob, 'data': data, 'lines': lines}
    if not result:
        raise ValueError('empty declared Rust scope at ' + rev)
    return result


def measure(root, baseline, candidate):
    revision(root, baseline)
    revision(root, candidate)
    before, after = inventory(root, baseline), inventory(root, candidate)
    files, removed, added = [], defaultdict(list), defaultdict(list)
    totals = dict(baseline=0, candidate=0, added=0, deleted=0, unchanged=0)
    for path in sorted(before.keys() | after.keys()):
        old, new = before.get(path), after.get(path)
        a, b = old['lines'] if old else [], new['lines'] if new else []
        plus = minus = same = 0
        for tag, i, j, k, l in difflib.SequenceMatcher(None, a, b, autojunk=False).get_opcodes():
            if tag == 'equal':
                same += j-i
            else:
                minus += j-i
                plus += l-k
                for n in range(i, j):
                    if a[n].strip():
                        removed[a[n]].append((path, n+1))
                for n in range(k, l):
                    if b[n].strip():
                        added[b[n]].append((path, n+1))
        status = 'added' if not old else 'deleted' if not new else 'unchanged' if old['blob'] == new['blob'] else 'modified'
        files.append({'path': path, 'status': status, 'baseline_blob': old['blob'] if old else None,
                      'candidate_blob': new['blob'] if new else None, 'baseline_lines': len(a),
                      'candidate_lines': len(b), 'added_lines': plus, 'deleted_lines': minus,
                      'unchanged_lines': same})
        for key, value in [('baseline',len(a)),('candidate',len(b)),('added',plus),('deleted',minus),('unchanged',same)]:
            totals[key] += value
    # Evidence of matching content across paths, not an assertion of author intent.
    relocations = []
    for content in sorted(removed.keys() & added.keys()):
        targets = list(added[content])
        for source in removed[content]:
            index = next((i for i, t in enumerate(targets) if t[0] != source[0]), None)
            if index is not None:
                target = targets.pop(index)
                relocations.append({'from': list(source), 'to': list(target),
                                    'line_sha256': hashlib.sha256(content).hexdigest()})
    runs = []
    for pair in sorted(relocations, key=lambda r: (r['from'][0], r['to'][0], r['from'][1], r['to'][1])):
        old_path, old_line = pair['from']
        new_path, new_line = pair['to']
        if (runs and runs[-1]['from_path'] == old_path and runs[-1]['to_path'] == new_path
                and runs[-1]['from_start'] + runs[-1]['line_count'] == old_line
                and runs[-1]['to_start'] + runs[-1]['line_count'] == new_line):
            runs[-1]['line_count'] += 1
            runs[-1]['hashes'].append(pair['line_sha256'])
        else:
            runs.append({'from_path': old_path, 'from_start': old_line, 'to_path': new_path,
                         'to_start': new_line, 'line_count': 1, 'hashes': [pair['line_sha256']]})
    for run in runs:
        run['line_hash_sequence_sha256'] = hashlib.sha256(
            ''.join(run.pop('hashes')).encode('ascii')).hexdigest()
    raw = git(root, '-c', 'diff.renameLimit=0', 'diff', '--no-ext-diff', '--no-textconv',
              '--name-status', '-z', '--find-renames=50%', baseline, candidate, '--', *SCOPE).split(b'\0')
    rename_evidence = []
    i = 0
    while i < len(raw) and raw[i]:
        status = raw[i].decode(); i += 1
        path = raw[i].decode(); i += 1
        if status.startswith(('R', 'C')):
            dest = raw[i].decode(); i += 1
            if path.endswith('.rs') and dest.endswith('.rs'):
                rename_evidence.append({'status': status, 'from': path, 'to': dest})
    totals['net_change'] = totals['candidate'] - totals['baseline']
    totals['cross_path_identical_nonblank_line_pairs'] = len(relocations)
    return {'schema': SCHEMA, 'baseline': baseline, 'candidate': candidate,
            'scope': SCOPE, 'recursive': True, 'suffix': '.rs',
            'method': 'Git blobs; LF physical lines including blanks/comments and unterminated final line; per-path SequenceMatcher autojunk=false',
            'relocation_method': 'Lexical exact nonblank line pairs in deleted/added diff ranges, different paths, sorted greedy one-to-one; ambiguous repeated content is only a relocation candidate; pairs remain in gross added/deleted totals; consecutive pairs encoded as runs with SHA-256 of concatenated hexadecimal line hashes',
            'rename_method': 'Git diff --find-renames=50% --name-status -z; heuristic evidence, not semantic proof',
            'git_version': git(root, '--version').decode().strip(),
            'totals': totals, 'files': files, 'rename_evidence': rename_evidence,
            'relocation_candidates': runs, 'behavioral_pass_claim': False}


def markdown(report):
    t = report['totals']
    family = [f for f in report['files'] if f['path'].startswith('adl/src/resilience')]
    return ('# Recursive Rust source-size evidence\n\n'
            f"Baseline: `{report['baseline']}`\n\nCandidate: `{report['candidate']}`\n\n"
            'Scope: `adl/src/resilience.rs`, recursive `adl/src/resilience/`, recursive `adl/tests/`; tracked `.rs` files only.\n\n'
            '| Measure | Lines |\n|---|---:|\n' + ''.join(f'| {k} | {v} |\n' for k,v in t.items()) +
            f"\nResilience family: {sum(f['baseline_lines'] for f in family):,} → {sum(f['candidate_lines'] for f in family):,} lines. "
            f"Recursive family net change: {sum(f['candidate_lines']-f['baseline_lines'] for f in family):+d} lines. A smaller facade alone is not evidence of overall code reduction.\n\n"
            + report['method'] + '.\n\n' + report['relocation_method'] + '.\n\n' + report['rename_method'] + '.\n\n'
            'File-by-file inventories, blob identities and relocation locators are retained in `measurement.json`. '
            'Size, matching lines and rename detection do not prove behavior preservation or narrower validation impact. No LoC quota applies.\n')


def encode(report):
    # Keep evidence rows on one line each so review does not drown in indentation.
    rows = []
    for key, value in sorted(report.items()):
        if key in ('files', 'relocation_candidates'):
            body = '[\n' + ',\n'.join('    ' + json.dumps(row, sort_keys=True) for row in value) + '\n  ]'
        else:
            body = json.dumps(value, sort_keys=True)
        rows.append('  ' + json.dumps(key) + ': ' + body)
    return '{\n' + ',\n'.join(rows) + '\n}\n'


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--repo', type=Path, default=Path.cwd())
    parser.add_argument('--baseline')
    parser.add_argument('--candidate')
    parser.add_argument('--out', type=Path)
    parser.add_argument('--check', type=Path)
    args = parser.parse_args()
    if args.check:
        if args.baseline or args.candidate or args.out:
            parser.error('--check cannot be combined with generation options')
        saved = json.loads(args.check.read_text())
        if saved.get('scope') != SCOPE or saved.get('recursive') is not True:
            raise ValueError('incomplete or top-level-only scope')
        actual = measure(args.repo, saved['baseline'], saved['candidate'])
        if saved != actual:
            raise ValueError('measurement differs from recursive exact-revision recomputation')
        if args.check.with_suffix('.md').read_text() != markdown(actual):
            raise ValueError('Markdown claim differs from measured evidence')
        print('PASS recursive inventory, revision identity, all totals and Markdown claims')
    else:
        if not args.baseline or not args.candidate or not args.out:
            parser.error('--baseline, --candidate and --out required')
        report = measure(args.repo, args.baseline, args.candidate)
        args.out.write_text(encode(report))
        args.out.with_suffix('.md').write_text(markdown(report))

if __name__ == '__main__':
    main()
