#!/usr/bin/env python3
"""PVF: deterministic local release-projection contract; small CPU/Git; required for #835.
No provider execution, release approval, or promotion of removals to proof.
"""
import argparse
import copy
from collections import Counter
from functools import lru_cache
import hashlib
import json
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[4]
BASE = 'docs/milestones/v0.92.1'
PACKET = BASE + '/evidence/release/current-status'
CANDIDATE = '64a99fd71b9770e15cb0dc393d669450d3a5f5b4'
AUTHORITY_REVISION = '0aa227619f5549bebcdfa802d95351c3079fb89e'
DENOM = BASE + '/evidence/release/tail-01/required-lane-denominator.json'
GAPS = BASE + '/evidence/release/tail-06/issue-764/retained-proof-gap-denominator.json'
PREPARATION = BASE + '/evidence/release/tail-06/issue-821/preparation.json'
PLANS = {818: ('retained-corporate-runtime', 832), 819: ('retained-v3', 827), 820: ('distributed-runtime', 828)}
DOC_TITLES = {'FEATURE_PROOF_COVERAGE': 'Feature Proof Coverage', 'DEMO_MATRIX': 'Demo Matrix', 'RELEASE_NOTES': 'Release Notes', 'MILESTONE_CHECKLIST': 'Milestone Checklist'}


def sha(data):
    return hashlib.sha256(data).hexdigest()


def encoded(data):
    return (json.dumps(data, indent=2, ensure_ascii=False) + '\n').encode()


@lru_cache(None)
def git_bytes(revision, path):
    return subprocess.check_output(['git', 'show', f'{revision}:{path}'], cwd=ROOT)


def load(revision, path):
    return json.loads(git_bytes(revision, path))


def ancestor(revision, candidate=CANDIDATE):
    return subprocess.run(['git', 'merge-base', '--is-ancestor', revision, candidate], cwd=ROOT).returncode == 0


def derive():
    # These frozen inputs are the trust boundary; editing current JSON cannot redefine them.
    assert ancestor(CANDIDATE, 'HEAD'), 'candidate is not ancestral to projection'
    authority_path = '.csdlc/evidence/835/pr-authority.json'
    authority = load(AUTHORITY_REVISION, authority_path)
    assert (ROOT / authority_path).read_bytes() == git_bytes(AUTHORITY_REVISION, authority_path), 'authority snapshot changed'
    prs = {p['number']: p for p in authority}
    old = load(CANDIDATE, PACKET + '/status.json')
    denom = load(CANDIDATE, DENOM)
    gaps = load(CANDIDATE, GAPS)
    prep = load(CANDIDATE, PREPARATION)
    sources = []

    def source(path, revision=CANDIDATE):
        item = {'path': path, 'revision': revision, 'sha256': sha(git_bytes(revision, path))}
        if item not in sources:
            sources.append(item)
        assert (ROOT / path).read_bytes() == git_bytes(revision, path), f'source drift: {path}'
        return item

    for path in [DENOM, GAPS, PREPARATION]:
        source(path)
    source(authority_path, AUTHORITY_REVISION)
    source('.csdlc/evidence/835/issue-observation.json', AUTHORITY_REVISION)
    gap_rows = {r['row_id']: r for r in gaps['remediation_rows']}
    resolutions = {}
    for issue, (family, pr_number) in PLANS.items():
        pr = prs[pr_number]
        assert pr['state'] == 'MERGED' and pr['mergedAt'] and ancestor(pr['mergeCommit']['oid']), 'unmerged or nonancestral removal authority'
        assert f'Closes #{issue}' in pr['body'] and 'Merging this PR' in pr['body'], 'missing exact merge-approval text'
        path = f'.csdlc/prepared/issues/{issue}/{family}-resolution-plan.json'
        receipt_path = f'.csdlc/evidence/{issue}/{family}/reconciliation.json'
        plan = load(pr['headRefOid'], path)
        receipt = load(pr['headRefOid'], receipt_path)
        source(path, pr['headRefOid'])
        source(receipt_path, pr['headRefOid'])
        assert {r['row_id'] for r in plan['rows']} == {r['row_id'] for r in receipt['rows']}
        for row in plan['rows']:
            ident = row['row_id']
            assert ident not in resolutions and ident in gap_rows
            assert row['criterion_text_digest'] == gap_rows[ident]['criterion_text_digest']
            resolution = row['resolution']
            result = {'source_issue': issue, 'criterion_digest': row['criterion_text_digest'], 'source_receipt': receipt_path, 'source_candidate': receipt['candidate']}
            if resolution['type'] == 'governed_disposition_proposal':
                assert resolution['behavioral_pass_claim'] is False
                assert resolution['proposed_disposition'] == 'remove_from_v0.92.1_retained_release_gate'
                result.update(disposition='approved_removal', behavioral_pass=False, proposal_digest=resolution['proposal_digest'], proposal=resolution,
                              authority={'pr': pr_number, 'url': pr['url'], 'head': pr['headRefOid'], 'merge': pr['mergeCommit']['oid'], 'body_sha256': sha(pr['body'].encode()), 'merged_at': pr['mergedAt'], 'plan_sha256': sha(git_bytes(pr['headRefOid'], path))})
            else:
                assert issue == 819 and resolution['type'] == 'candidate_bound_execution'
                changed = subprocess.check_output(['git', 'diff', '--name-only', receipt['candidate'], CANDIDATE, '--', *receipt['candidate_surface_paths']], cwd=ROOT, text=True).splitlines()
                # Execution at an older candidate is retained, never silently transported across changed producers.
                result.update(disposition='execution_refresh_required' if changed else 'retained_candidate_execution', behavioral_pass=not changed, changed_proof_paths=changed)
            resolutions[ident] = result
    pending = {r['row_id'] for r in prep['rows']}
    assert len(pending) == 4 and pending == set(gap_rows) - set(resolutions)
    assert prep['status'] == 'preparation_only' and prs[829]['state'] == 'MERGED'
    admission_path = BASE + '/evidence/integration/release-tail-admission.json'
    admission = load(CANDIDATE, admission_path)
    source(admission_path)
    observations = {r['number']:r for r in load(AUTHORITY_REVISION,'.csdlc/evidence/835/issue-observation.json')}
    stages = {}
    for stage in admission['release_tail_stages']:
        ident = stage['planned_id']
        source_issue = stage['issue']
        observed_state = observations.get(source_issue, {}).get('state', 'unobserved')
        owner = source_issue if observed_state == 'OPEN' else 522
        stages[ident] = {'id':ident, 'source_issue':source_issue, 'observed_issue_state':observed_state,
                         'status':'ceremony_not_run' if ident == 'TAIL-10' else 'awaiting_final_gate',
                         'owner':526 if ident == 'TAIL-10' else owner}
    rows = []
    seen = set()
    preserved = {r['row_id']: r for r in gaps['preserved_rows'] + gaps['accounted_non_764_eligible_rows']}
    for historic, identities in denom['lane_results'].items():
        for ident in identities:
            assert ident not in seen, 'duplicate denominator identity'
            seen.add(ident)
            row = {'id': ident, 'work_package': ident.split(':')[0], 'historical_result': historic, 'behavioral_pass': False}
            if ident in resolutions:
                row.update(resolutions[ident])
            elif ident in pending:
                row.update(disposition='final_gate_proof_required', owner=522, source_issue=821, evidence=PREPARATION)
            elif ident in preserved:
                row.update(disposition='preserved_review_disposition', source_disposition=preserved[ident]['reconciliation_class'], evidence=GAPS)
            elif historic == 'not_applicable':
                stage = stages[row['work_package']]
                row.update(disposition=stage['status'], owner=stage['owner'], source_issue=stage['source_issue'], observed_issue_state=stage['observed_issue_state'], evidence=DENOM)
            else:
                row.update(disposition='historical_evidence_retained', evidence=DENOM)
            rows.append(row)
    assert len(rows) == 393 and len(gap_rows) == 198 and set(gap_rows) <= seen
    assert Counter(r['disposition'] for r in rows)['approved_removal'] == 143
    features = []
    for feature in old['features']:
        members = [r for r in rows if r['work_package'] in feature['work_packages']]
        features.append({'id': feature['id'], 'name': feature['name'], 'work_packages': feature['work_packages'], 'delivery_status': feature['delivery_status'], 'delivered_scope':feature['delivered'], 'bounded_demo_status': feature['demo_status'], 'demo_scope': feature['demo_scope'], 'retained_gate_counts': dict(sorted(Counter(r['disposition'] for r in members).items())), 'release_status': 'awaiting_final_gate', 'owner': 522})
    blockers = [
        {'id': 'FINAL-GATE-821', 'owner': 522, 'rows': sorted(pending), 'status': 'unresolved', 'reason': 'PR829 merged preparation only. No final candidate proof exists for these four obligations.'},
        {'id': 'CURRENT-CANDIDATE-PROOF', 'owner': 835, 'rows': sorted(r['id'] for r in rows if r['disposition'] == 'execution_refresh_required'), 'status': 'unresolved', 'reason': 'The 51 #819 execution receipts target an older candidate; changed proof producers require explicit refresh.'},
        {'id': 'RELEASE-REVIEW', 'owner': 522, 'rows': [], 'status': 'unresolved', 'reason': 'Final required-lane recomputation and all current P1/P2 dispositions remain prerequisites. Historical accounting and merged remediation are not final review approval.'}]
    checklist = []
    for r in old['checklist']:
        checklist.append({'id': r['id'], 'obligation': r['obligation'], 'retained_status': r['status'], 'retained_evidence': r['evidence'], 'retained_disposition': r['disposition'], 'current_disposition': 'retained_bounded_evidence' if r['status'] in ('proved', 'not_applicable', 'deferred') else 'final_gate_review_required', 'owner': None if r['status'] in ('proved', 'not_applicable', 'deferred') else 522})
    return {'schema': 'adl.v0921.release_status_projection.v2', 'issue': 835, 'candidate': CANDIDATE, 'authority_snapshot_revision': subprocess.check_output(['git','rev-parse',AUTHORITY_REVISION],cwd=ROOT,text=True).strip(), 'release_decision': 'blocked', 'release_authorized': False, 'sources': sources, 'inventory_count': len(rows), 'original_required_count': denom['required_lane_count'], 'approved_removal_count': 143, 'remaining_required_count': denom['required_lane_count'] - 143, 'remediation_count': 198, 'preserved_and_separately_accounted_count': 32, 'excluded_accounting_count': 15, 'disposition_counts': dict(sorted(Counter(r['disposition'] for r in rows).items())), 'rows': sorted(rows,key=lambda r:r['id']), 'features': features, 'work_packages': [{'id': w['id'], 'issue': w['issue'], 'implementation_revision': w['implementation_revision'], 'implementation_evidence': [{'path':path, 'revision':next(x['revision'] for x in old['sources'] if x['path']==path)} for path in w['implementation_evidence']], 'retained_classifications':w['retained_classifications'], 'delivery_basis':w['delivery_basis'], 'release_status': 'awaiting_final_gate', 'owner': 522} for w in old['work_packages']], 'checklist': checklist, 'exclusions': old['exclusions'], 'blockers': blockers, 'release_tail':list(stages.values())}


def render(data):
    counts = data['disposition_counts']
    intro = f"""## Current release projection

**Release decision: {data['release_decision']}.** Frozen candidate: `{data['candidate']}`.

All {data['inventory_count']} inventoried rows have exactly one disposition. Of 366 historically required rows, 143 criteria were explicitly removed by merged operator-reviewed PRs, leaving 223 requirements in the denominator. Removal changes the release requirement; it is **not a behavioral pass**. The 198-row remediation partition is fully joined to its successor evidence; the 32 preserved/separately accounted rows and 15 excluded accounting rows remain visible.

PR #829 merged preparation only. Its four final #821 obligations remain unresolved. The 51 #819 execution rows retain their original candidate and require refresh where proof producers changed. No current execution or release approval is inferred from issue closure, merge, or source presence.

[Machine-readable status](evidence/release/current-status/status.json), [quality gate](evidence/release/current-status/quality-gate.json), [blocker register](evidence/release/current-status/blockers.json), and [full evidence map](evidence/release/current-status/EVIDENCE_MAP.md) are generated together. Historical #517 and #767 packets remain source-time evidence in Git; this projection does not rewrite them.

| Disposition | Rows |
|---|---:|
""" + ''.join(f'| {k} | {v} |\n' for k,v in counts.items())
    intro += '\n<!-- release-status:start -->\n| Feature | Delivery | Approved removals | Prior execution needing refresh | Release |\n|---|---|---:|---:|---|\n'
    for f in data['features']:
        c=f['retained_gate_counts'];intro+=f"| {f['id']} — {f['name']} | {f['delivery_status']} | {c.get('approved_removal',0)} | {c.get('execution_refresh_required',0)} | {f['release_status']} |\n"
    intro+='<!-- release-status:end -->\n\n### Release-tail stages\n\n' + ''.join(f"- {r['id']}: {r['status']} (#{r['owner']}).\n" for r in data['release_tail'])
    intro+='\n### Scope exclusions\n\n'+''.join(f"- {e['id']}: {e['status']} — {e.get('rationale',e.get('reason','Retain the governed exclusion; no product proof inferred.'))}\n" for e in data['exclusions'])
    intro+='\nValidate with `python3 .csdlc/prepared/issues/835/project_release.py --check`. `--require-ready` deliberately fails while release blockers remain.\n'
    outputs={}
    for key,title in DOC_TITLES.items():
        text=f'# {title} — v0.92.1\n\n'+intro
        if key=='MILESTONE_CHECKLIST':
            text+='\n## Complete checklist denominator\n\nRetained status describes its bounded historical evidence, not a new candidate pass.\n\n| ID | Obligation | Current disposition | Retained status | Owner | Evidence and retained rationale |\n|---|---|---|---|---|---|\n'
            for r in data['checklist']:
                text+=f"| {r['id']} | {r['obligation']} | {r['current_disposition']} | {r['retained_status']} | {r['owner'] or 'none'} | [Evidence]({r['retained_evidence']}) — {r['retained_disposition']} |\n"
        else:
            text+='\n## Bounded feature and demo evidence\n\n'
            for f in data['features']:
                text+=f"### {f['name']}\n\n{f['delivered_scope']}\n\nRetained demo status: {f['bounded_demo_status']}. {f['demo_scope'].replace('are linked below', 'are linked in the evidence map')} [Evidence map](evidence/release/current-status/EVIDENCE_MAP.md#{f['id'].lower()}).\n\n"
        outputs[f'{BASE}/{key}_v0.92.1.md']=(text.rstrip()+'\n').encode()
    evidence='# Complete release-gate evidence map\n\nCandidate: `'+data['candidate']+'`. No rows omitted or removals called passes.\n\n| ID | Current disposition | Historical result | Source issue |\n|---|---|---|---|\n'
    for r in data['rows']:
        evidence+=f"| {r['id']} | {r['disposition']} | {r['historical_result']} | {r.get('source_issue','retained inventory')} |\n"
    for feature in data['features']:
        evidence += f"\n## {feature['id']}\n\n**{feature['name']}** — {feature['delivered_scope']}\n\nThe links below retain their source-time revision and scope; they do not establish a new candidate pass.\n"
        for work in data['work_packages']:
            if work['id'] not in feature['work_packages']: continue
            evidence += f"\n### {work['id']} — [#{work['issue']}](https://github.com/agent-logic/agent-design-language/issues/{work['issue']})\n\n{work['delivery_basis']} Retained classifications: `{json.dumps(work['retained_classifications'], sort_keys=True)}`.\n\n"
            if work['implementation_revision']:
                evidence += f"Implementation revision: `{work['implementation_revision']}`.\n\n"
            for ref in work['implementation_evidence']:
                evidence += f"- [{ref['path']}](https://github.com/agent-logic/agent-design-language/blob/{ref['revision']}/{ref['path']})\n"
        if not feature['work_packages']:
            # Podcast is retained outside the numbered execution wave.
            old = load(CANDIDATE, PACKET + '/status.json')
            for ref in old['sources']:
                if '/262/' in ref['path'] or '/264/' in ref['path']:
                    evidence += f"- [{ref['path']}](https://github.com/agent-logic/agent-design-language/blob/{ref['revision']}/{ref['path']})\n"
    outputs[PACKET+'/EVIDENCE_MAP.md']=evidence.encode()
    outputs[PACKET+'/status.json']=encoded(data)
    outputs[PACKET+'/issue-observation.json']=encoded({'schema':'adl.release_issue_observation.v2','repository':'agent-logic/agent-design-language','candidate':CANDIDATE,'snapshot_revision':data['authority_snapshot_revision'],'issues':load(AUTHORITY_REVISION,'.csdlc/evidence/835/issue-observation.json')})
    outputs[PACKET+'/quality-gate.json']=encoded({'schema':'adl.v0921.current_quality_gate.v1','candidate':data['candidate'],'status_sha256':sha(encoded(data)),'decision':data['release_decision'],'release_authorized':False,'inventory':393,'approved_removals':143,'remaining_required':223,'unresolved_blockers':[r['id'] for r in data['blockers'] if r['status']=='unresolved']})
    outputs[PACKET+'/blockers.json']=encoded({'schema':'adl.v0921.current_release_blockers.v1','candidate':data['candidate'],'blockers':data['blockers']})
    return outputs


def validate(data, files):
    expected=derive()
    errors=[]
    if data!=expected: errors.append('projection differs from frozen source derivation')
    for path,content in render(expected).items():
        if files.get(path)!=content: errors.append('contradictory projection: '+path)
    return errors


def main():
    parser=argparse.ArgumentParser();parser.add_argument('--write',action='store_true');parser.add_argument('--check',action='store_true');parser.add_argument('--require-ready',action='store_true');args=parser.parse_args()
    data=derive();outputs=render(data)
    if args.write:
        for path,content in outputs.items(): (ROOT/path).write_bytes(content)
    files={p:(ROOT/p).read_bytes() for p in outputs}
    errors=validate(json.loads(files[PACKET+'/status.json']),files)
    if errors: raise SystemExit('\n'.join(errors))
    mutants=[]
    for field,value in [('candidate','0'*40),('release_decision','ready'),('release_authorized',True),('inventory_count',392)]:
        m=copy.deepcopy(data);m[field]=value;mutants.append(m)
    m=copy.deepcopy(data);m['rows'].pop();mutants.append(m)
    m=copy.deepcopy(data);m['rows'].append(m['rows'][0]);mutants.append(m)
    for field,value in [('proposal_digest','0'*64),('behavioral_pass',True),('disposition','behavioral_pass')]:
        m=copy.deepcopy(data);next(r for r in m['rows'] if r['disposition']=='approved_removal')[field]=value;mutants.append(m)
    m=copy.deepcopy(data);m['blockers']=[];mutants.append(m)
    m=copy.deepcopy(data);next(r for r in m['rows'] if r['disposition']=='approved_removal')['authority']['pr']=829;mutants.append(m)
    m=copy.deepcopy(data);next(r for r in m['rows'] if r['work_package']=='INT-01' and r.get('owner'))['owner']=526;mutants.append(m)
    for m in mutants:
        assert validate(m,render(m)), 'coordinated invalid projection accepted'
    changed=dict(files);changed[next(p for p in files if p.endswith('.md'))]+=b'\nContradictory extra projection\n';assert validate(data,changed)
    missing_links=dict(files);missing_links[PACKET+'/EVIDENCE_MAP.md']=b'# Evidence map without feature anchors or links\n';assert validate(data,missing_links)
    print(json.dumps({'status':'pass','candidate':CANDIDATE,'rows':len(data['rows']),'approved_removals':143,'negative_cases':len(mutants)+2,'release_decision':data['release_decision']}))
    if args.require_ready and (data['release_decision']!='ready' or any(b['status']=='unresolved' for b in data['blockers'])):
        raise SystemExit('release readiness refused: unresolved final proof and review blockers')

if __name__=='__main__':main()
