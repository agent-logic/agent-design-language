#!/usr/bin/env python3
"""Bounded #1223 guard: inventory, retained source/evidence, rollback and direct references."""
import argparse
import hashlib
import json
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[2]
PACKET = Path('docs/milestones/v0.93.1/repository-decomposition/rd07-1223')
# Original 4134-row inventory, ordered id/path/mode/blob tuple digest.
INVENTORY_SHA256 = 'a22fa6306245ddb3916cf4cb3021286a2d91ebfe77ca719eb7b1039d9f6d3cd3'
ARCHIVE_SHA256 = '12205043566913435c25ea414b1fd9148b46727821f7bfefd5d452cfe47a1c17'
COMPONENTS = ('infra/ddns', 'infra/aws/runtime/log-archive', 'infra/aws/runtime/agent-checkpoint-archive')
PROTECTED = ('csdlc-v3/operator/authority-selector.json', 'csdlc-v3/operator/native-authority-receipt.json',
             'docs/templates/prompts/current.json', 'adl/tools/install_owner_binaries.sh',
             'adl-schema/Cargo.toml', 'adl-uts/Cargo.toml', 'adl-v2/crates/adl-language/Cargo.toml',
             'adl-v2/crates/adl-compiler/Cargo.toml', 'adl-legacy-contracts/Cargo.toml')


def require(condition, reason):
    if not condition:
        raise ValueError(reason)


def git(*args):
    return subprocess.check_output(['git', '--no-optional-locks', *args], cwd=ROOT, text=True).strip()


def committed_json(path):
    return json.loads(git('show', 'HEAD:' + str(path)))


def tree(ref):
    return {path: fields.split()[::2] for line in git('ls-tree', '-r', ref).splitlines()
            for fields, path in [line.split('\t', 1)]}


def check_inventory(ledger, frozen, observed, candidate):
    require(ledger['original_revision'] == '248f00e359e412f6bc0061ac88be9ff374e3a212' and
            ledger['observed_main'] == '9cfd035aac2a44637b3c8026bd40b669579a8c7f', 'reviewed baseline changed')
    rows = ledger['rows']
    tuples = [[r['id'], r['path'], r['original_mode'], r['original_blob']] for r in rows]
    require(len(rows) == len({r['path'] for r in rows}) == 4134, 'original denominator missing or duplicated')
    require(hashlib.sha256(json.dumps(tuples, separators=(',', ':')).encode()).hexdigest() == INVENTORY_SHA256,
            'original inventory identity changed')
    require(ledger['new_removals_authorized'] is False and ledger['all_holds_resolved'] is False,
            'preparation cannot grant retirement approval')
    removed = []
    for r in rows:
        p = r['path']
        require(frozen.get(p) == [r['original_mode'], r['original_blob']], 'frozen identity mismatch: ' + p)
        require(observed.get(p) == r['observed_main'], 'observed source identity mismatch: ' + p)
        if r['action'] == 'existing_branch_removal':
            require(any(p.startswith(c + '/') for c in COMPONENTS), 'out-of-scope removal: ' + p)
            require(p not in candidate, 'declared prior removal still present: ' + p)
            removed.append(p)
        elif r['action'] == 'existing_branch_surgical':
            require(p == 'infra/runtime-v3/runtime-init.toml', 'unapproved surgical path')
            before = subprocess.check_output(['git', '--no-optional-locks', 'show', ledger['observed_main'] + ':' + p], cwd=ROOT)
            after = before.replace(b'# S3 archive values come from infra/aws/runtime/agent-checkpoint-archive outputs.',
                                   b"# S3 archive values come from the agent-logic-infrastructure repository's\n# infra/aws/runtime/agent-checkpoint-archive outputs.")
            expected = hashlib.sha1(b'blob ' + str(len(after)).encode() + b'\0' + after).hexdigest()
            require(after != before and candidate.get(p) == [observed[p][0], expected], 'surgical change exceeds reviewed comment')
        else:
            require(r['action'] == 'retain', 'unknown action: ' + p)
            require(candidate.get(p) == observed.get(p), 'held source removed or altered: ' + p)
    require(len(removed) == 34, 'existing removal denominator must remain 34')
    require(set(observed) - set(candidate) == set(removed), 'new deletion outside reviewed 34-path set')
    for p in PROTECTED:
        require(candidate.get(p) == observed.get(p) and p in observed, 'required evidence/contract missing or changed: ' + p)
    for p, identity in observed.items():
        if p.startswith('.csdlc/') and not p.startswith(('.csdlc/issues/1223/', '.csdlc/v3/issues/1223/', '.csdlc/evidence/1223/', '.csdlc/transactions/')):
            require(candidate.get(p) == identity, 'historical lifecycle evidence changed: ' + p)
    return removed


def check_rollback(path):
    require(path.is_file() and not path.is_symlink(), 'rollback archive missing')
    require(hashlib.sha256(path.read_bytes()).hexdigest() == ARCHIVE_SHA256, 'rollback archive digest mismatch')


def check_references(actual, allowed):
    require(not (set(actual) - set(allowed)), 'unclassified retained reference: ' + ', '.join(sorted(set(actual) - set(allowed))))


def run():
    ledger = committed_json(PACKET / 'DISPOSITION_REGISTER.json')
    observed = tree(ledger['observed_main'])
    candidate = tree('HEAD')
    removed = check_inventory(ledger, tree(ledger['original_revision']), observed, candidate)
    metadata = Path(git('rev-parse', '--path-format=absolute', '--git-common-dir'))
    check_rollback(metadata / 'csdlc-v3/local/evidence/1223/retirement/baseline.tar.gz')
    scanned = {}
    for component in COMPONENTS:
        names = ('RETIREMENT_BATCH_01_DDNS', 'RETIREMENT_BATCH_02_LOG_ARCHIVE',
                 'RETIREMENT_BATCH_03_AGENT_CHECKPOINT_ARCHIVE')
        manifests = [committed_json(PACKET / (name + '.json')) for name in names]
        manifest = next(m for m in manifests if m['component'] == component)
        allowed = list(manifest['reference_scan']['retained_historical_reference_files'])
        require(all(p.startswith(('.csdlc/', 'docs/milestones/', 'docs/reviews/')) or
                    p == 'adl/tools/skills/repo-packet-builder/tests/fixtures/issue-520-changed-paths.txt' for p in allowed),
                'historical classification contains operational path')
        # These new artifacts describe the retirement itself; they do not invoke removed code.
        allowed.extend(str(PACKET / (name + suffix)) for name in names for suffix in ('.json', '_RESULT.json'))
        allowed.extend([str(PACKET / 'DISPOSITION_REGISTER.json'),
                        str(PACKET / 'CURRENT_SUCCESSOR_ROWS.jsonl'),
                        'tools/source_retirement/check.py', 'tools/source_retirement/README.md'])
        if component == COMPONENTS[2]:
            p = 'infra/runtime-v3/runtime-init.toml'
            require('agent-logic-infrastructure' in git('show', 'HEAD:' + p), 'successor comment missing')
            allowed.append(p)
        result = subprocess.run(['git', '--no-optional-locks', 'grep', '-I', '-l', '-F', '-e', component, 'HEAD', '--'],
                                cwd=ROOT, text=True, capture_output=True)
        require(result.returncode in (0, 1), 'reference inspection failed')
        refs = [p.removeprefix('HEAD:') for p in result.stdout.splitlines()]
        check_references(refs, allowed)
        scanned[component] = len(refs)
    return {'schema': 'adl.source_retirement.guard.v1', 'status': 'passed', 'head': git('rev-parse', 'HEAD'),
            'candidate_rows': 4134, 'existing_branch_removals': len(removed), 'direct_reference_files': scanned,
            'limits': ['Preparation guard only; no new deletion approval.',
                       'Direct component-path references only; dynamic and indirect consumers still require review.',
                       'Rollback digest is checked separately from functional restore and installed product proof.',
                       'Current remote heads are not accepted replacement artifacts.']}


if __name__ == '__main__':
    argparse.ArgumentParser(description=__doc__).parse_args()
    try:
        print(json.dumps(run(), sort_keys=True))
    except (ValueError, OSError, KeyError, subprocess.CalledProcessError) as error:
        print(json.dumps({'status': 'failed', 'error': str(error)}))
        raise SystemExit(1)
