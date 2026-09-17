#!/usr/bin/env python3
"""PVF docs_only: source/decision integrity, never architectural acceptance."""
import argparse
import copy
import hashlib
import json
from pathlib import Path
import re
import subprocess

PACKET = Path(__file__).resolve().parent
ROOT = PACKET.parents[4]

def require(value, message):
    if not value:
        raise ValueError(message)

def check(decisions, content, manifest):
    old = json.loads((PACKET.parent / 'issue-911/decision-inventory.json').read_text())
    expected = {c['candidate']: c for c in old['candidates']}
    candidates = decisions['candidates']
    require(len(candidates) == 12 and {c['candidate'] for c in candidates} == set(expected), 'candidate denominator/identity')
    require(decisions['task_mapping'] == old['task_mapping'] and len(decisions['task_mapping']) == 69, '69 task dispositions changed')
    require(decisions['acceptance_complete'] is False and decisions['supersessions'] == [], 'unapproved acceptance/supersession')
    require(decisions['separate_obligations'] == [848, 910], 'separate obligations lost')
    require(decisions['implementation_revision'] == manifest['revision'], 'source revision mismatch')
    sources = manifest['sources']
    require(len({s['path'] for s in sources}) == len(sources), 'duplicate source')
    source_paths = {s['path'] for s in sources}
    for source in sources:
        path = Path(source['path'])
        require(not path.is_absolute() and '..' not in path.parts, 'unsafe source path')
        data = subprocess.check_output(['git', '-C', str(ROOT), 'show', manifest['revision'] + ':' + source['path']])
        require(len(data) == source['bytes'] and hashlib.sha256(data).hexdigest() == source['sha256'], 'source digest mismatch')
    boundary = decisions['repository_boundary']
    require(boundary['website'] == 'agent-logic/codefriend.ai' and boundary['code'].startswith('separate private repository'), 'website/code conflation')
    supporting = content['supporting_files']
    require(len(supporting) == 1 and supporting[0]['path'] == 'docs/milestones/v0.92.2/repository-decomposition/PLAN.md', 'missing corrected plan')
    require(hashlib.sha256((ROOT / supporting[0]['path']).read_bytes()).hexdigest() == supporting[0]['sha256'], 'corrected plan digest mismatch')
    hashes = {x['path']: x['sha256'] for x in content['files']}
    require(len(content['files']) == len(hashes) == 12 and set(hashes) == {c['path'] for c in candidates}, 'content manifest denominator')
    for c in candidates:
        original = expected[c['candidate']]
        require(c['path'] == original['path'] and c['owner'] == original['accountable_scope_owner'], 'candidate path/owner drift')
        require(c['disposition'] == 'pending_operator_decision' and c['approval'] is None and c['numeric_id'] is None, 'unapproved decision')
        require(c['recommendation'] == 'accept_revised', 'unexpected recommendation')
        require(c['source_refs'] and set(c['source_refs']) <= source_paths, 'missing source references')
        require(all(c[k].strip() for k in ['observation', 'correction', 'gate_consequence']), 'missing reconciliation')
        data = (ROOT / c['path']).read_bytes()
        require(hashlib.sha256(data).hexdigest() == hashes[c['path']], 'candidate digest mismatch')
        text = data.decode()
        require('## Status\n\n**Proposed.**' in text and '## Implementation Reconciliation — #945' in text, 'status/reconciliation drift')
        require(manifest['revision'] in text, 'candidate revision absent')
    for f in list(PACKET.glob('*.md')) + [ROOT / c['path'] for c in candidates]:
        text = f.read_text()
        require('/Users/' not in text and '/Volumes/' not in text, 'host path')
        require(all(line == line.rstrip() for line in text.splitlines()), 'trailing whitespace')
        for href in re.findall(r'\]\(([^)]+)\)', text):
            if href.startswith(('http://', 'https://', '#')):
                continue
            dest = (f.parent / href.split('#')[0]).resolve()
            require(dest.is_relative_to(ROOT) and dest.is_file(), 'broken relative link: ' + href)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--self-test', action='store_true')
    args = parser.parse_args()
    values = [json.loads((PACKET / name).read_text()) for name in ['decisions.json', 'candidate-content.json', 'source-manifest.json']]
    check(*values)
    count = 0
    if args.self_test:
        mutations = [
            lambda d,c,m: d['repository_boundary'].update(code='agent-logic/codefriend.ai'),
            lambda d,c,m: c['supporting_files'][0].update(sha256='0'*64),
            lambda d,c,m: d['candidates'].pop(),
            lambda d,c,m: d['task_mapping'].pop(),
            lambda d,c,m: d.update(acceptance_complete=True),
            lambda d,c,m: d['candidates'][0].update(disposition='accepted'),
            lambda d,c,m: d['candidates'][0].update(approval={'actor':'invented'}),
            lambda d,c,m: d['candidates'][0].update(numeric_id='9999'),
            lambda d,c,m: d['candidates'][0].update(owner='unknown'),
            lambda d,c,m: d.update(separate_obligations=[]),
            lambda d,c,m: c['files'][0].update(sha256='0'*64),
            lambda d,c,m: m['sources'][0].update(sha256='0'*64),
            lambda d,c,m: d['candidates'][0].update(source_refs=['missing.rs']),
            lambda d,c,m: d.update(supersessions=[{'old':'0025','new':'CF-01'}]),
        ]
        for mutate in mutations:
            bad = copy.deepcopy(values)
            mutate(*bad)
            try:
                check(*bad)
            except ValueError:
                count += 1
            else:
                raise ValueError('negative fixture accepted')
    print(json.dumps({'status':'pass','candidates':12,'task_dispositions':69,'sources':len(values[2]['sources']),'negative_fixtures':count,'pvf':'docs_only','architectural_acceptance':False}))

if __name__ == '__main__':
    main()
