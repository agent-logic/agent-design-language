#!/usr/bin/env python3
"""Validate the bounded predecessor reconciliation; --live refreshes state only.

PVF: release-evidence, deterministic local Git/JSON contract, small CPU, required
for this packet. --live is a separate network observation, not product proof.
"""
import argparse
import datetime
import hashlib
import json
from pathlib import Path
import re
import subprocess

HERE = Path(__file__).resolve().parent
ROOT = next(p for p in HERE.parents if (p / '.git').exists())
REPO = 'agent-logic/agent-design-language'
SOURCE = 'docs/milestones/v0.92.1/evidence/release/tail-04/findings.json'
PREDECESSOR_MERGE = 'e058412611eff0975148ba9ed8e40bbd20ecb985'
OWNER_PRS = {814: 822, 815: 825, 816: 823, 817: 826,
             818: 832, 819: 827, 820: 828, 821: 829}
PROOF_PATHS = {
    814: ['adl-runtime-kernel/src/control.rs', 'adl-runtime/tests/shepherd_local_model.rs'],
    815: ['.csdlc/prepared/issues/815/test_cloud_authorization.py'],
    816: ['.csdlc/prepared/issues/816/validate-publication-manifest.py', 'adl-runtime/tests/config_reload.rs'],
    817: ['.csdlc/prepared/issues/817/validate-release-truth.py'],
    818: ['.csdlc/evidence/818/retained-corporate-runtime/reconciliation.json'],
    819: ['.csdlc/evidence/819/retained-v3/reconciliation.json'],
    820: ['.csdlc/evidence/820/distributed-runtime/reconciliation.json'],
    821: ['docs/milestones/v0.92.1/evidence/release/tail-06/issue-821/preparation.json'],
}


def git(*args):
    return subprocess.check_output(['git', '-C', str(ROOT), *args], stderr=subprocess.PIPE)


def require(condition, message):
    if not condition:
        raise ValueError(message)


def ancestor(revision, candidate):
    require(re.fullmatch('[0-9a-f]{40}', revision) is not None, 'invalid revision')
    require(subprocess.run(['git', '-C', str(ROOT), 'merge-base', '--is-ancestor',
                            revision, candidate], capture_output=True).returncode == 0,
            'non-ancestral merge or candidate')


def artifact(item):
    path = item['path']
    require(not Path(path).is_absolute() and '..' not in Path(path).parts, 'unsafe path')
    blob = git('show', item['revision'] + ':' + path)
    require(hashlib.sha256(blob).hexdigest() == item['sha256'], 'evidence digest mismatch')
    return blob


def validate(packet, observed, candidate='HEAD'):
    require(packet['schema'] == 'adl.v0921.predecessor_reconciliation.v1', 'schema')
    require(packet['issue'] == 834 and packet['predecessor_issue'] == 520
            and packet['closing_pr'] == 831, 'wrong predecessor or closing PR')
    require(packet['release_ready'] is False, 'reconciliation is not release approval')
    require(observed['repository'] == REPO, 'wrong repository')
    timestamp = datetime.datetime.fromisoformat(observed['observed_at'])
    require(timestamp.tzinfo is not None, 'observation must have timezone')
    ancestor(packet['source_candidate'], candidate)
    issues = {i['number']: i for i in observed['issues']}
    prs = {p['number']: p for p in observed['pull_requests']}
    require(len(issues) == len(observed['issues']) and len(prs) == len(observed['pull_requests']),
            'duplicate remote identity')
    for issue, pr in {520: 831, **OWNER_PRS}.items():
        i, p = issues[issue], prs[pr]
        require(i['url'] == f'https://github.com/{REPO}/issues/{issue}'
                and p['url'] == f'https://github.com/{REPO}/pull/{pr}', 'wrong remote URL')
        require(i['state'] == 'CLOSED' and i['closedAt'], 'stale open issue')
        require(p['state'] == 'MERGED' and p['mergedAt'], 'unmerged closing PR')
        require(timestamp >= datetime.datetime.fromisoformat(p['mergedAt'].replace('Z', '+00:00')),
                'observation predates merge')
        require(any(r['number'] == pr and r['url'] == p['url']
                    for r in i['closedByPullRequestsReferences']), 'wrong closing PR edge')
        require(any(r['number'] == issue and r['url'] == i['url']
                    for r in p['closingIssuesReferences']), 'missing reciprocal closing edge')
        ancestor(p['mergeCommit']['oid'], packet['source_candidate'])
    require(prs[831]['mergeCommit']['oid'] == PREDECESSOR_MERGE, 'wrong predecessor merge')
    historical = packet['historical_sources']
    require([i['path'] for i in historical] == [SOURCE,
            'docs/milestones/v0.92.1/evidence/release/tail-04/SECOND_REVIEW_SUMMARY.md'],
            'dropped historical source')
    for item in historical:
        require(item['revision'] == PREDECESSOR_MERGE, 'historical revision drift')
        require((ROOT / item['path']).read_bytes() == artifact(item), 'historical report changed')
    source_rows = json.loads(git('show', PREDECESSOR_MERGE + ':' + SOURCE))['findings']
    expected = {f['id']: [int(n) for n in re.findall(r'#(\d+)', f['owner']) if n != '522']
                for f in source_rows}
    rows = packet['findings']
    require(len(rows) == 14 and len({r['id'] for r in rows}) == 14
            and {r['id'] for r in rows} == set(expected), 'dropped or duplicate finding ID')
    for row in rows:
        require(row['owners'] == expected[row['id']], 'wrong remediation owner')
        corrections = [835] if row['id'] in ['D520-RET-001', 'D520-V3F-001', 'D520-REL-001'] else []
        require(row['correction_owners'] == corrections, 'dropped correcting follow-on')
        disposition = ('merged_retained_packets_and_preparation; final_gate_correction_owned_by_835'
                       if row['id'] == 'D520-RET-001' else 'merged_remediation_evidence')
        require(row['disposition'] == disposition, 'overstated disposition')
    owners = packet['owners']
    require(len(owners) == 8 and {o['issue'] for o in owners} == set(OWNER_PRS), 'owner denominator')
    for owner in owners:
        pr = OWNER_PRS[owner['issue']]
        require(owner['closing_pr'] == pr, 'owner closing PR mismatch')
        require([e['path'] for e in owner['evidence']] ==
                [f".csdlc/issues/{owner['issue']}/cards/sor.values.json", *PROOF_PATHS[owner['issue']]],
                'missing or substituted semantic evidence')
        for item in owner['evidence']:
            require(item['revision'] == prs[pr]['mergeCommit']['oid'], 'evidence not at owner merge')
            artifact(item)
    require(packet['historical_report_retention_owner'] == 833, 'external report retention owner')
    require(packet['external_finding'] == 'TPR-002', 'wrong external finding')
    require(packet['historical_observation_source'] == f'https://github.com/{REPO}/issues/834'
            and packet['historical_observation'] ==
            'Required predecessor #520 was not finalized and had returned changes required when the external review ran.',
            'historical observation changed')
    return {'result': 'pass', 'findings': 14, 'merged_owners': 8,
            'observation_time': observed['observed_at'], 'release_ready': False}


def live_check(observed):
    # Read fresh closure topology. Retained observations stay immutable.
    for kind, entries, fields in [('issue', observed['issues'], 'number,url,state,closedAt,closedByPullRequestsReferences'),
                                  ('pr', observed['pull_requests'], 'number,url,state,mergedAt,mergeCommit,closingIssuesReferences')]:
        for saved in entries:
            if kind == 'issue' and saved['number'] in (833, 835):
                continue  # Independently owned work can progress after this observation.
            current = json.loads(subprocess.check_output(['gh', kind, 'view', str(saved['number']),
                                 '--repo', REPO, '--json', fields]))
            require(all(current[k] == saved[k] for k in current), 'live remote state drift')


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--live', action='store_true')
    args = parser.parse_args()
    try:
        packet = json.loads((HERE / 'reconciliation.json').read_text())
        observed = json.loads((HERE / 'github-readback.json').read_text())
        result = validate(packet, observed)
        if args.live:
            live_check(observed)
            result['live_state'] = 'matched'
        print(json.dumps(result, sort_keys=True))
    except (ValueError, KeyError, TypeError, OSError, subprocess.CalledProcessError) as error:
        print(json.dumps({'result': 'fail', 'reason': str(error)}))
        raise SystemExit(1)
