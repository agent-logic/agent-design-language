#!/usr/bin/env python3
"""PVF docs_only: source/approval recording integrity, never product qualification."""
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
    require(decisions['acceptance_complete'] is True and decisions['supersessions'] == [], 'acceptance/supersession drift')
    history = json.loads((PACKET / 'operator-decisions.json').read_text())['decisions']
    require(len({c['numeric_id'] for c in candidates}) == 12, 'duplicate accepted number')
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
    require('docs/milestones/v0.92.2/repository-decomposition/PLAN.md' in {x['path'] for x in supporting}, 'missing corrected plan')
    for entry in supporting + content['accepted_files']:
        require(hashlib.sha256((ROOT / entry['path']).read_bytes()).hexdigest() == entry['sha256'], 'current content digest mismatch')
    require({x['path'] for x in content['accepted_files']} == {c['accepted_path'] for c in candidates}, 'accepted content denominator')
    hashes = {x['path']: x['sha256'] for x in content['files']}
    require(len(content['files']) == len(hashes) == 12 and set(hashes) == {c['path'] for c in candidates}, 'content manifest denominator')
    for c in candidates:
        original = expected[c['candidate']]
        require(c['path'] == original['path'] and c['owner'] == original['accountable_scope_owner'], 'candidate path/owner drift')
        require(c['disposition'] == 'accepted' and c['approval'] in history, 'unrecorded decision')
        approval = c['approval']
        require(approval['actor'] == 'operator' and approval['candidate'] == c['candidate'] and approval['disposition'] == 'accepted', 'approval identity mismatch')
        require(c['numeric_id'] == str(76 + candidates.index(c)).zfill(4), 'accepted number drift')
        accepted = ROOT / c['accepted_path']
        require(accepted.parent == ROOT / 'docs/adr' and accepted.name.startswith(c['numeric_id'] + '-'), 'accepted path drift')
        accepted_text = accepted.read_text()
        require('**Accepted.**' in accepted_text, 'accepted status absent')
        reviewed = subprocess.check_output(['git', '-C', str(ROOT), 'show', approval['reviewed_head'] + ':' + c['path']])
        if c['candidate'] == 'ADR-CF-09':
            statement = approval['approved_statement']
            require(hashlib.sha256(statement.encode()).hexdigest() == approval['approved_statement_sha256'] and statement in accepted_text, 'revised approval statement drift')
            require(hashlib.sha256(reviewed).hexdigest() == approval['superseded_proposal_sha256'], 'superseded proposal mismatch')
            for term in ['installed CodeFriend agent', 'invitation-only', 'GitHub sign-in', 'server-hosted', 'Beta 1', 'CLI-only proof is insufficient']:
                require(term in accepted_text, 'required product boundary missing: ' + term)
        else:
            require(hashlib.sha256(reviewed).hexdigest() == approval['candidate_sha256'], 'approved proposal digest mismatch')
            # The accepted decision itself must match approved text; only relative link locations change.
            def decision_text(text):
                block = text.split('## Decision\n\n', 1)[1].split('\n## ', 1)[0]
                return re.sub(r'\[([^\]]+)\]\([^)]+\)', r'\1', block)
            require(decision_text(reviewed.decode()) == decision_text(accepted_text), 'approved decision changed')
        require(c['recommendation'] == 'accept_revised', 'unexpected recommendation')
        require(c['source_refs'] and set(c['source_refs']) <= source_paths, 'missing source references')
        require(all(c[k].strip() for k in ['observation', 'correction', 'gate_consequence']), 'missing reconciliation')
        data = (ROOT / c['path']).read_bytes()
        require(hashlib.sha256(data).hexdigest() == hashes[c['path']], 'candidate digest mismatch')
        text = data.decode()
        require('## Status\n\n**Proposed.**' in text and '## Implementation Reconciliation — #945' in text, 'status/reconciliation drift')
        require(manifest['revision'] in text, 'candidate revision absent')
    relations = json.loads((PACKET / 'relationships.json').read_text())
    prior = json.loads((PACKET.parent / 'issue-911/relationships.json').read_text())
    require(relations['supersessions'] == [], 'unexpected supersession')
    require(len(relations['relationships']) == len(prior['relationships']), 'refinement denominator')
    for relation, original in zip(relations['relationships'], prior['relationships']):
        require(all(relation[k] == original[k] for k in ['candidate', 'source_record', 'source_path', 'source_status', 'supersession_enacted']), 'historical relationship drift')
        candidate = next(c for c in candidates if c['candidate'] == relation['candidate'])
        require(relation['relation'] == 'refinement' and relation['accepted_path'] == candidate['accepted_path'] and relation['numeric_id'] == candidate['numeric_id'], 'refinement identity drift')
        require(Path(candidate['accepted_path']).name in (ROOT / relation['source_path']).read_text(), 'reciprocal refinement absent')
    delivery = (PACKET / 'BETA1_DELIVERY.md').read_text()
    require(all(t in delivery for t in ['server-hosted', 'installed local', 'invitation-only', 'GitHub sign-in', '#914', '#915', 'CLI-only']), 'delivery gates incomplete')
    for f in list(PACKET.glob('*.md')) + [ROOT / c[k] for c in candidates for k in ['path', 'accepted_path']]:
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
            lambda d,c,m: d.update(acceptance_complete=False),
            lambda d,c,m: d['candidates'][0].update(disposition='pending_operator_decision'),
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
    print(json.dumps({'status':'pass','candidates':12,'task_dispositions':69,'sources':len(values[2]['sources']),'negative_fixtures':count,'pvf':'docs_only','approval_recording_integrity':True,'product_qualification':False}))

if __name__ == '__main__':
    main()
