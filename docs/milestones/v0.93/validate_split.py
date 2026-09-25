#!/usr/bin/env python3
"""PVF: deterministic docs-only scope/graph proof; no product or release acceptance."""
import copy
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
VERSIONS = ('v0.93.1', 'v0.93.2')

def read(version, name):
    return json.loads((ROOT / version / name).read_text())

def validate(plans, mapping, inspect_files=True):
    all_rows = {}
    for version, plan in plans.items():
        assert plan['status'] == ('sprint_1_preparation_open_execution_not_authorized' if version == 'v0.93.1' else 'approved_scope_draft_not_open')
        rows = {w['id']: w for w in plan['work_packages']}
        assert len(rows) == len(plan['work_packages']) == plan['counts']['core_work_packages']
        assert plan['repositories']['codefriend'] == plan['repositories']['codefriend.ai'] == 'private'
        assert [r for r, v in plan['repositories'].items() if v == 'public'] == ['agent-design-language']
        assignment = {}
        for n, sprint in enumerate(plan['sprint_plan']['sprints'], 1):
            assert sprint['number'] == n
            for item in sprint['work_packages']:
                assert item in rows and item not in assignment
                assignment[item] = n
        assert set(assignment) == set(rows)
        for key, row in rows.items():
            assert row['sprint'] == assignment[key]
            assert row['result'] and row['acceptance'] and row['negative_cases'] and row['task_boundary']['single_result']
            if inspect_files:
                assert (ROOT / version / row['source']).is_file(), (version, key, row['source'])
            for dep in row['depends_on']:
                if '/' in dep:
                    assert dep in plan['external_dependencies'], (key, dep)
                    assert dep.startswith('v0.93.1/') and version == 'v0.93.2'
                else:
                    assert dep in rows and assignment[dep] <= assignment[key], (key, dep)
            all_rows[version + '/' + key] = row
        for n in range(2, 11):
            assert rows[f'TAIL-{n:02}']['depends_on'] == [f'TAIL-{n-1:02}']
        if inspect_files:
            spec = read(version, 'WP_EXECUTION_SPECIFICATIONS_' + version + '.yaml')
            wave = read(version, 'WP_ISSUE_WAVE_' + version + '.yaml')
            assert spec['work_packages'] == plan['work_packages']
            assert spec['sprint_plan'] == wave['sprint_plan'] == plan['sprint_plan']
            assert len(wave['work_packages']) == len(rows)
            for row, item in zip(plan['work_packages'], wave['work_packages']):
                for a, b in [('id','wp'),('result','outcome'),('depends_on','depends_on'),('acceptance','acceptance'),('sprint','sprint'),('pvf','pvf'),('negative_cases','negative_cases')]:
                    assert row[a] == item[b], (version, row['id'], a)
                for field in ['existing_issue','issue_repository','dependency_gates']:
                    assert row.get(field) == item.get(field), (version,row['id'],field)
    def ancestors(key, active=None):
        active = set() if active is None else active
        assert key not in active, ('cycle', key)
        assert key in all_rows, ('missing', key)
        active.add(key)
        version = key.split('/')[0]
        found = set()
        for dep in all_rows[key]['depends_on']:
            target = dep if '/' in dep else version + '/' + dep
            found.add(target)
            found.update(ancestors(target, active))
        active.remove(key)
        return found
    for key in all_rows:
        ancestors(key)
    first = plans['v0.93.1']
    first_rows = {r['id']: r for r in first['work_packages']}
    assert not any(key.startswith('RV-') for key in first_rows)
    assert first_rows['CF-05']['existing_issue'] == 1150
    assert not any('closure of the #915 successors #1148, #1149 and #1150' in a for a in first_rows['CF-05']['acceptance'])
    assert 'Missing Beta 1 behavior remains a predecessor blocker.' not in first_rows['CF-01']['result']
    assert {r['issue'] for r in first['existing_issue_routing']} == {875,1145,1148,1149,1150}
    assert first['counts']['distinct_planned_issue_identities'] == 47
    prep = first['preparation']
    expected = {'WP-01':1178,'RD-01':1181,'RD-02':1182,'RD-12':1183,'RD-13':1184,'RD-03':1185,'RD-04':1186,'RD-05':1187,'RD-06':1188,'RD-09':1189,'RD-10':1190,'RD-08':1191,'RD-07':1192,'RD-11':1193}
    assert prep['issue_map'] == expected
    assert prep['umbrella_issue'] == 1180 and prep['sprint'] == 1
    assert prep['execution_authorized'] is False and prep['closeout_issue_creation_authorized'] is False
    assert prep['later_sprint_creation'] == 'after_split_acceptance'
    assert prep['checkpoint_status'] == 'pending_independent_acceptance'
    assert prep['wp01_terminal_closure_required'] is False
    assert set(expected) == set(first['sprint_plan']['sprints'][0]['work_packages'])
    for key, issue in expected.items():
        assert first_rows[key]['existing_issue'] == issue
        assert first_rows[key]['issue_repository'] == 'agent-logic/agent-design-language'
    assert first_rows['RD-01']['dependency_gates'] == {'WP-01':'accepted_sprint_1_preparation_checkpoint_not_terminal_closure'}
    for key in ['WP-01','RD-01','RD-02']:
        assert first_rows[key]['pvf'] == 'planning_contract'
        assert not any('Installed consumer evidence' in a for a in first_rows[key]['acceptance'])
    assert 'remains OPEN' in ' '.join(first_rows['WP-01']['acceptance'])
    assert 'transfer' in first_rows['RD-13']['result'] and 'existing' in first_rows['RD-13']['result']
    assert first_rows['RD-03']['depends_on'] == first_rows['RD-04']['depends_on'] == ['RD-13']
    assert first['counts']['new_core_issues_to_seed_at_most'] == 28
    assert first['counts']['existing_additional_issues'] == 4
    assert plans['v0.93.2']['existing_issue_routing'] == []
    assert plans['v0.93.2']['counts']['distinct_planned_issue_identities'] == 53
    assert mapping['existing_issue_routing'] == {'v0.93.1':[1148,1149,1150,875,1145],'v0.93.2':[]}
    if inspect_files:
        overlay = ROOT / 'v0.93' / mapping['operator_amendment']
        assert overlay.is_file() and '#923' in overlay.read_text()
    for key, row in first_rows.items():
        if key != 'WP-01' and not key.startswith('RD-'):
            assert 'v0.93.1/RD-11' in ancestors('v0.93.1/' + key)
        if key.startswith('RD-'):
            assert not any('/CF-' in a or '/CT-' in a or '/RV-' in a for a in ancestors('v0.93.1/' + key))
    assert not any(a.startswith('v0.93.2/') for a in ancestors('v0.93.1/CF-07'))
    for v, prefixes in [('v0.93.1', [('CF',7),('CT',10)]),('v0.93.2',[('RV',11),('GOV',16),('WP-S',6),('CM',4)])]:
        release = ancestors(v+'/TAIL-10')
        for prefix, count in prefixes:
            for n in range(1,count+1):
                key = f'{prefix}{n}' if prefix == 'WP-S' else f'{prefix}-{n:02}'
                assert v+'/'+key in release, ('release omits',v,key)
    old = read('v0.93','EXECUTION_PLAN_v0.93.json')
    original = {r['id'] for r in old['work_packages']}
    mapped = mapping['source_disposition']
    assert len(mapped) == len({r['source_id'] for r in mapped}) == 83
    assert {r['source_id'] for r in mapped} == original
    for row in mapped:
        assert row['destination']+'/'+row['destination_id'] in all_rows
    extras = {(r['milestone'],r['id']) for r in mapping['additional_version_specific_gates']}
    assert len(extras) == 13 and len(all_rows) == 83 + len(extras)

if __name__ == '__main__':
    plans = {v:read(v,'EXECUTION_PLAN_'+v+'.json') for v in VERSIONS}
    mapping = read('v0.93','MILESTONE_SPLIT_v0.93.json')
    validate(plans,mapping)
    probes=[]
    def reject(label, mutate):
        candidate=copy.deepcopy(plans); census=copy.deepcopy(mapping);mutate(candidate,census)
        try:validate(candidate,census,False)
        except (AssertionError,KeyError):probes.append(label)
        else:raise AssertionError('negative accepted: '+label)
    reject('lost source work',lambda p,m:m['source_disposition'].pop())
    reject('duplicate source work',lambda p,m:m['source_disposition'].__setitem__(0,m['source_disposition'][1]))
    reject('missing dependency',lambda p,m:p['v0.93.1']['work_packages'][1]['depends_on'].append('MISSING'))
    reject('cycle',lambda p,m:p['v0.93.1']['work_packages'][0]['depends_on'].append('RD-11'))
    reject('v4 reintroduced into launch',lambda p,m:next(w for w in p['v0.93.1']['work_packages'] if w['id']=='CF-02')['depends_on'].append('v0.93.2/RV-08'))
    reject('split blocked by launch',lambda p,m:next(w for w in p['v0.93.1']['work_packages'] if w['id']=='RD-10')['depends_on'].append('CF-05'))
    reject('sprint before dependency',lambda p,m:next(w for w in p['v0.93.1']['work_packages'] if w['id']=='CF-05')['depends_on'].append('TAIL-10'))
    reject('duplicate qualification issue',lambda p,m:next(w for w in p['v0.93.1']['work_packages'] if w['id']=='CF-05').pop('existing_issue'))
    reject('template release omitted',lambda p,m:next(w for w in p['v0.93.1']['work_packages'] if w['id']=='CT-03')['depends_on'].remove('CT-09'))
    reject('public private product',lambda p,m:p['v0.93.1']['repositories'].__setitem__('codefriend','public'))
    reject('tail skipped',lambda p,m:next(w for w in p['v0.93.2']['work_packages'] if w['id']=='TAIL-10')['depends_on'].clear())
    reject('opening falsely asserted',lambda p,m:p['v0.93.2'].__setitem__('status','open'))
    reject('duplicate Sprint 1 identity',lambda p,m:p['v0.93.1']['preparation']['issue_map'].__setitem__('RD-01',1178))
    reject('WP01 closure dependency',lambda p,m:next(w for w in p['v0.93.1']['work_packages'] if w['id']=='RD-01')['dependency_gates'].__setitem__('WP-01','terminal_closure'))
    reject('premature execution',lambda p,m:p['v0.93.1']['preparation'].__setitem__('execution_authorized',True))
    reject('premature later issue creation',lambda p,m:p['v0.93.1']['preparation'].__setitem__('later_sprint_creation','now'))
    reject('premature closeout creation',lambda p,m:p['v0.93.1']['preparation'].__setitem__('closeout_issue_creation_authorized',True))
    reject('downstream proof blocks audit',lambda p,m:next(w for w in p['v0.93.1']['work_packages'] if w['id']=='RD-01')['acceptance'].append('Installed consumer evidence'))
    reject('lost existing transfer identities',lambda p,m:next(w for w in p['v0.93.1']['work_packages'] if w['id']=='RD-13').__setitem__('result','Create new issues'))
    reject('invented serial acceptance edge',lambda p,m:next(w for w in p['v0.93.1']['work_packages'] if w['id']=='RD-04')['depends_on'].append('RD-03'))
    reject('stale sidecar routing',lambda p,m:p['v0.93.2']['existing_issue_routing'].append({'issue':875}))
    reject('core count inflated by sidecar',lambda p,m:p['v0.93.1']['counts'].__setitem__('core_work_packages',45))
    print(json.dumps({'status':'pass','source_tasks':83,'successor_tasks':[43,53],'version_specific_gates':13,'negative_fixtures':len(probes),'execution_opened':False},indent=2))
