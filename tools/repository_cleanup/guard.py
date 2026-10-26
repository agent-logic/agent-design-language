"""Exact #1284 archival/cleanup transition; no directory-wide exemptions."""
import hashlib
import json
from pathlib import Path

TRANSITION_SHA256 = '762c27b2d844ba3f166ea5bf05a1eb65b2f752b7d4e4f341f107b7a9ebf15915'


def normalize_candidate(root, candidate, baseline):
    root = Path(root)
    raw = (root / 'tools/repository_cleanup/transition.json').read_bytes()
    if hashlib.sha256(raw).hexdigest() != TRANSITION_SHA256:
        raise ValueError('cleanup transition identity changed')
    transition = json.loads(raw)
    raw = (root / 'docs/archives/POST_SPLIT_PAYLOADS_1284.json').read_bytes()
    if hashlib.sha256(raw).hexdigest() != transition['archive_manifest_sha256']:
        raise ValueError('cleanup archive manifest identity changed')
    archive = json.loads(raw)
    if archive['source_commit'] != transition['source_commit']:
        raise ValueError('cleanup archive source changed')
    entries = {r['path']: r for r in archive['entries']}
    if len(entries) != len(archive['entries']):
        raise ValueError('duplicate archived path')
    result = dict(candidate)
    seen = set()
    removed = set()
    for row in transition['changes']:
        path = row['path']
        if (path in seen or row['before'] != baseline.get(path)
                or candidate.get(path) != row['after']):
            raise ValueError('cleanup transition mismatch: ' + path)
        seen.add(path)
        if row['after'] is None:
            entry = entries.get(path)
            if entry is None or [entry['mode'], entry['git_blob']] != row['before']:
                raise ValueError('cleanup removal lacks exact archive identity: ' + path)
            removed.add(path)
        result[path] = row['before']
    if removed != set(entries):
        raise ValueError('archive and cleanup removal sets differ')
    return result


# #1346 exact successor layer. Original #1284/#1325/#1285 manifests stay immutable.
LANGUAGE_TRANSITION_SHA256 = 'cd26e57fd78ae1e354673968d7457584d0642bc7588afde5a5071b84f727e3d6'

def normalize_language_cleanup(root, candidate, tree):
    import subprocess
    root = Path(root)
    raw = (root / 'docs/language-cleanup/ruby-transition.json').read_bytes()
    if hashlib.sha256(raw).hexdigest() != LANGUAGE_TRANSITION_SHA256:
        raise ValueError('language transition identity changed')
    transition = json.loads(raw)
    baseline = tree(transition['source_commit'])
    result = dict(candidate)
    seen = set()
    for row in transition['changes']:
        path = row['path']
        if (path in seen or row['before'] != baseline.get(path)
                or candidate.get(path) != row['after']):
            raise ValueError('language transition mismatch: ' + path)
        seen.add(path)
        raw_before = subprocess.check_output(['git', 'cat-file', 'blob', row['before'][1]], cwd=root)
        if hashlib.sha256(raw_before).hexdigest() != row['before_sha256']:
            raise ValueError('language preimage custody mismatch: ' + path)
        result[path] = row['before']
    if {r['path'] for r in transition['changes'] if r['after'] is None} != set(transition['removed']):
        raise ValueError('language removal set mismatch')
    return result
