"""Exact #1285 successor transition for the immutable #1223 source-retirement guard."""
import hashlib
import json
from pathlib import Path

# Filled from the reviewed transition record, not inferred from mutable HEAD.
TRANSITION_SHA256 = 'a7026948d2934bb4cef4071034a168d61d51df3c96c7a6131ffd56d8ea80fd3f'

def normalize_candidate(root, candidate, baseline):
    raw=(Path(root)/'tools/podcast_migration/transition.json').read_bytes()
    if hashlib.sha256(raw).hexdigest()!=TRANSITION_SHA256:
        raise ValueError('podcast transition identity changed')
    transition=json.loads(raw)
    manifest=(Path(root)/'tools/podcast_migration/source-manifest.json').read_bytes()
    if hashlib.sha256(manifest).hexdigest()!=transition['manifest_sha256']:
        raise ValueError('podcast source manifest identity changed')
    imported={e['source_path']:e for e in json.loads(manifest)['entries'] if e['source_kind']=='adl_git'}
    result=dict(candidate)
    removed=set(transition['removed'])
    if len(removed)!=len(transition['removed']):
        raise ValueError('duplicate podcast removal')
    seen=set()
    for row in transition['changes']:
        path=row['path']
        if path in seen or row['before']!=baseline.get(path) or candidate.get(path)!=row['after']:
            raise ValueError('podcast transition mismatch: '+path)
        seen.add(path)
        if row['after'] is None:
            if path not in removed or path not in imported:
                raise ValueError('podcast removal lacks destination provenance: '+path)
        elif path in removed:
            raise ValueError('podcast removal still present: '+path)
        result[path]=row['before']
    if removed!={r['path'] for r in transition['changes'] if r['after'] is None}:
        raise ValueError('podcast removal set mismatch')
    return result
