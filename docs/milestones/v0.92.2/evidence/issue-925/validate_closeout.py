"""Local closeout identity proof only; never executes release or remote writes."""
import copy
import hashlib
import json
from pathlib import Path
ROOT = Path(__file__).resolve().parents[5]
PACKET = Path(__file__).resolve().parent

def validate(manifest):
    assert manifest['software_release'] == 'not_performed'
    assert manifest['external_review'] == 'failed_attempt'
    assert manifest['operator_instruction'].strip()
    for row in manifest['files']:
        assert hashlib.sha256((ROOT / row['path']).read_bytes()).hexdigest() == row['sha256']
    names = {x['path'] for x in manifest['files']}
    assert '.csdlc/evidence/910/browser-proof.json' in names
    assert 'docs/milestones/v0.92.2/adr/issue-945/README.md' in names
    ledger = json.loads((PACKET / 'ISSUE_DISPOSITIONS.json').read_text())
    assert ledger['core_denominator'] == len(ledger['tasks']) == 69
    assert len({x['issue'] for x in ledger['tasks']}) == 69
    assert next(x for x in ledger['tasks'] if x['issue'] == 915)['final_disposition'] == 'deferred_qualification_to_1148_1149_1150'
    reviewed = json.loads((ROOT / 'docs/milestones/v0.92.2/evidence/issue-924/REVIEWED_MANIFEST.json').read_text())
    assert len(reviewed['files']) == 60
    for row in reviewed['files']:
        assert hashlib.sha256((ROOT / row['path']).read_bytes()).hexdigest() == row['sha256']

if __name__ == '__main__':
    manifest = json.loads((PACKET / 'CLOSEOUT_MANIFEST.json').read_text())
    validate(manifest)
    cases=[]
    for key,val in [('software_release','released'),('external_review','pass'),('operator_instruction','')]:
        j=copy.deepcopy(manifest);j[key]=val;cases.append(j)
    j=copy.deepcopy(manifest);j['files'][0]['sha256']='0'*64;cases.append(j)
    for path in ['.csdlc/evidence/910/browser-proof.json','docs/milestones/v0.92.2/adr/issue-945/README.md']:
        j=copy.deepcopy(manifest);j['files']=[x for x in j['files'] if x['path']!=path];cases.append(j)
    for case in cases:
        try: validate(case)
        except (AssertionError,KeyError,FileNotFoundError): pass
        else: raise AssertionError('invalid closeout accepted')
    print(json.dumps({'status':'pass','core_tasks':69,'planning_manifest_files':60,'negative_cases':len(cases),'remote_effects':False}))
