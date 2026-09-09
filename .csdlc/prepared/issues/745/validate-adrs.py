#!/usr/bin/env python3
"""Issue 745 PVF docs lane: deterministic local evidence/contract checks.
Proof role: document structure, eight-topic coverage, source integrity, links.
Resource: small, offline, <=120 seconds. Release gate: issue-local, not runtime.
"""
from pathlib import Path
import hashlib
import json
import posixpath
import re
import subprocess

ROOT = Path(__file__).resolve().parents[4]
MANIFEST = ROOT / '.csdlc/prepared/issues/745/source-manifest.json'
SECTIONS = ['Status', 'Context', 'Decision', 'Consequences', 'Alternatives Considered',
            'Supersession Relationships', 'Source Evidence', 'Validation Notes', 'Approval Boundary']
TOPICS = ['C-SDLC v3 state, migration, compatibility authority',
          'Distributed Runtime continuity or qualification',
          'Hot-reload atomicity, validation, exclusions',
          'Observatory projection and authenticity', 'Runtime v4 rebaseline',
          'DEC-01 Runtime v2/v3 authority and compatibility',
          'Provider profiles and shadow comparison', 'GCP portability differences']

def require(ok, message):
    if not ok:
        raise SystemExit(message)

def main():
    manifest = json.loads(MANIFEST.read_text())
    require(manifest['issue'] == 745, 'wrong issue')
    baseline = manifest['baseline']
    require(re.fullmatch(r'[0-9a-f]{40}', baseline), 'invalid baseline')
    paths = manifest['documents']
    require(len(paths) == 7 and len(set(paths)) == 7, 'document denominator mismatch')
    links = 0
    for source in manifest['sources']:
        data = subprocess.check_output(['git', 'show', baseline + ':' + source['path']], cwd=ROOT)
        require(hashlib.sha256(data).hexdigest() == source['sha256'], 'source digest drift: ' + source['path'])
        require(len(data) == source['bytes'], 'source size drift: ' + source['path'])
        if source['path'] not in paths:
            require((ROOT/source['path']).read_bytes() == data, 'source refresh required: ' + source['path'])
    for rel in paths:
        text = (ROOT/rel).read_text()
        require(all(line.rstrip() == line for line in text.splitlines()), 'trailing whitespace: ' + rel)
        for target in re.findall(r'\]\(([^)]+)\)', text):
            if '://' in target:
                continue
            dest = posixpath.normpath(posixpath.join(posixpath.dirname(rel), target.split('#')[0]))
            require(not dest.startswith('../') and (ROOT/dest).exists(), 'broken link: ' + rel + ' -> ' + dest)
            links += 1
        if Path(rel).name.startswith(('0072-', '0073-', '0074-', '0075-')):
            for section in SECTIONS:
                require('\n## ' + section + '\n' in text, 'missing section: ' + rel + ': ' + section)
            require('Proposed' in text.split('## Context')[0], 'new ADR must remain Proposed')
    plan = (ROOT/'docs/milestones/v0.92.1/ADR_PLAN_v0.92.1.md').read_text()
    rows = [line for line in plan.splitlines() if line.startswith('| ')][2:]
    require([row.split('|')[1].strip() for row in rows] == TOPICS, 'eight-topic denominator drift')
    for number in ('0066-', '0070-'):
        require(number in rows[1], 'existing distributed owner omitted')
    rel = 'docs/architecture/adr/0069-observatory-governed-runtime-consumer-boundary.md'
    original = subprocess.check_output(['git', 'show', baseline + ':' + rel], cwd=ROOT).decode()
    require((ROOT/rel).read_text().startswith(original), 'original Deferred ADR 0069 changed')
    recovery = (ROOT/'.csdlc/prepared/issues/745/v2-transition-recovery.md').read_text()
    require('#744' in recovery and 'explicitly authorized' in recovery, 'missing recovery authority or defect')
    require('8dbec9dac7cebe523aff76b76932358788bbb6e3139a3dd2ef0d34c6dad0f6cf' in recovery, 'missing original intent')
    print(json.dumps({'schema':'adl.745.adr_validation.v1','status':'pass','issue':745,
                      'baseline':baseline,'sources':len(manifest['sources']),'documents':len(paths),
                      'topics':len(rows),'relative_links':links,'runtime_tests':'not_run_documentation_only'}))

if __name__ == '__main__':
    main()
