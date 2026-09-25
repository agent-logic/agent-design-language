"""Local closeout identity proof only; never executes release or remote writes."""
import copy
import hashlib
import json
import re
import subprocess
from functools import lru_cache
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
    validate_reviewed(reviewed)

@lru_cache(maxsize=128)
def historical_blob(revision, path):
    # Pin a commit object, never a moving ref or working-copy successor document.
    assert re.fullmatch(r'[0-9a-f]{40}', revision), 'full reviewed revision required'
    relative = Path(path)
    assert not relative.is_absolute() and '..' not in relative.parts
    assert path.startswith('docs/milestones/')
    commit = subprocess.run(['git', '-C', str(ROOT), 'rev-parse', '--verify', revision + '^{commit}'],
                            capture_output=True, check=True).stdout.decode().strip()
    assert commit == revision
    return subprocess.run(['git', '-C', str(ROOT), 'show', revision + ':' + path],
                          capture_output=True, check=True).stdout

def validate_reviewed(reviewed):
    assert len(reviewed['files']) == 60
    assert len({row['path'] for row in reviewed['files']}) == 60
    for row in reviewed['files']:
        digest = hashlib.sha256(historical_blob(reviewed['reviewed_revision'], row['path'])).hexdigest()
        assert digest == row['sha256'], row['path']
        # Closed-milestone evidence remains immutable in the current checkout too.
        # Only the successor plans can evolve independently of their historical review.
        if row['path'].startswith('docs/milestones/v0.92.2/'):
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
    reviewed = json.loads((ROOT / 'docs/milestones/v0.92.2/evidence/issue-924/REVIEWED_MANIFEST.json').read_text())
    historical_cases=[]
    for key,value in [('reviewed_revision','HEAD'),('reviewed_revision','0'*40)]:
        candidate=copy.deepcopy(reviewed);candidate[key]=value;historical_cases.append(candidate)
    candidate=copy.deepcopy(reviewed);candidate['files'][1]['sha256']='0'*64;historical_cases.append(candidate)
    candidate=copy.deepcopy(reviewed);candidate['files'][1]['path']='docs/milestones/missing.md';historical_cases.append(candidate)
    candidate=copy.deepcopy(reviewed);candidate['files'][1]=candidate['files'][0];historical_cases.append(candidate)
    for candidate in historical_cases:
        try: validate_reviewed(candidate)
        except (AssertionError,KeyError,FileNotFoundError,subprocess.CalledProcessError): pass
        else: raise AssertionError('invalid historical review accepted')
    print(json.dumps({'status':'pass','core_tasks':69,'planning_manifest_files':60,'negative_cases':len(cases),'historical_negative_cases':len(historical_cases),'remote_effects':False}))
