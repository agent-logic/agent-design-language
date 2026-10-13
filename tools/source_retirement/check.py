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
# Captured-current identities (4134 originals + 22 additions) in immutable
# COMBINED_REMAINING_DELTA at 666818f2aafdaf4839d1c3de4119b513c4236d3d.
CAPTURED_IDENTITIES_SHA256 = 'e306e5a70ae7792fd2fef91449a0285d665126375a28c0dd05914724fe1bdebe'
ARCHIVE_SHA256 = '12205043566913435c25ea414b1fd9148b46727821f7bfefd5d452cfe47a1c17'
# Independently reviewed existing execution overlay; changing this pin requires review.
EXECUTION_APPROVAL_SHA256 = '0921865acb9b361e15143f4815faec18951049519ae56799ff556d7d1359428f'
COMPONENTS = ('infra/ddns', 'infra/aws/runtime/log-archive', 'infra/aws/runtime/agent-checkpoint-archive')
PROTECTED = ('csdlc-v3/operator/authority-selector.json', 'csdlc-v3/operator/native-authority-receipt.json',
             'docs/templates/prompts/current.json', 'adl/tools/install_owner_binaries.sh',
             'adl-schema/Cargo.toml', 'adl-uts/Cargo.toml', 'adl-v2/crates/adl-language/Cargo.toml',
             'adl-v2/crates/adl-compiler/Cargo.toml', 'adl-legacy-contracts/Cargo.toml')
INFRA_RUNBOOKS = (
    'docs/operations/cloud/gcp/terraform-bootstrap/README.md',
    'docs/operations/cloud/gcp/platform-foundation/README.md',
    'docs/operations/cloud/gcp/organization-billing/README.md',
    'docs/operations/cloud/aws/terraform-bootstrap/AWS_TERRAFORM_BOOTSTRAP_RUNBOOK.md',
    'docs/operations/cloud/aws/audit-security/AWS_AUDIT_SECURITY_BASELINE.md',
)


def consumer_reference_repair(path, before):
    if path == 'adl/CONTRIBUTING.md':
        require(hashlib.sha256(before).hexdigest() ==
                '05b1ec567794c0c07ac829ef8122fbc8b2a7ca3f5b3133b8f7898f9defdfea67',
                'public contributor baseline changed')
        after = (ROOT / path).read_bytes()
        require(hashlib.sha256(after).hexdigest() ==
                '9f30372ea3cffb1b608b31f22f2e7f22fb218101d769c9a4c059db5954b483a6',
                'public contributor repair exceeds reviewed bytes')
        return after
    if path == 'docs/runtime/RESIDENT_HEALTH_SUPERVISION.md':
        old = b'`infra/aws/csm-runtime-health/shepherd.tf` declares the separate role: scoped'
        require(before.count(old) == 1, 'consumer reference baseline changed')
        return before.replace(old, b"The agent-logic-infrastructure repository's " + old)
    require(path in INFRA_RUNBOOKS, 'unapproved consumer repair path: ' + path)
    title, remainder = before.split(b'\n', 1)
    notice = ('\nThe maintained [runbook and Terraform source]('
              'https://github.com/agent-logic/agent-logic-infrastructure/blob/main/' + path +
              ') belong to `agent-logic-infrastructure`. Run the infrastructure commands below '
              'from that repository. This ADL copy preserves the migration context; '
              'infrastructure changes belong in the successor repository.\n')
    return title + b'\n' + notice.encode() + remainder


def require(condition, reason):
    if not condition:
        raise ValueError(reason)


def git(*args):
    return subprocess.check_output(['git', '--no-optional-locks', *args], cwd=ROOT, text=True).strip()


def committed_json(path):
    return json.loads(git('show', 'HEAD:' + str(path)))


def committed_bytes(path):
    return subprocess.check_output(['git', '--no-optional-locks', 'show', 'HEAD:' + str(path)], cwd=ROOT)


def check_combined(original, combined, groups, frozen, observed):
    require(original['new_removals_authorized'] is True and original['all_holds_resolved'] is False,
            'original scoped approval flags changed')
    captured = [[r.get('id'), r['path'], r['current_identity']] for r in combined['rows'] + combined['additional_paths']]
    require(hashlib.sha256(json.dumps(captured, sort_keys=True, separators=(',', ':')).encode()).hexdigest() ==
            CAPTURED_IDENTITIES_SHA256, 'captured current identities changed')
    rows = original['rows']
    tuples = [[r['id'], r['path'], r['original_mode'], r['original_blob']] for r in rows]
    require(len(rows) == len({r['path'] for r in rows}) == 4134 and
            hashlib.sha256(json.dumps(tuples, separators=(',', ':')).encode()).hexdigest() == INVENTORY_SHA256,
            'original inventory identity changed')
    require(original['original_revision'] == '248f00e359e412f6bc0061ac88be9ff374e3a212' and
            original['observed_main'] == '9cfd035aac2a44637b3c8026bd40b669579a8c7f', 'reviewed baseline changed')
    for row in rows:
        require(frozen.get(row['path']) == [row['original_mode'], row['original_blob']], 'frozen identity mismatch')
        require(observed.get(row['path']) == row['observed_main'], 'observed identity mismatch')
    require([(r['id'], r['path']) for r in combined['rows']] == [(r['id'], r['path']) for r in rows],
            'combined ordered inventory changed')
    additional = combined['additional_paths']
    paths = [r['path'] for r in additional]
    require(len(paths) == len(set(paths)) == combined['additional_current_source_paths'] == 22 and
            not set(paths).intersection(r['path'] for r in rows) and
            all(r['original_inventory_member'] is False for r in additional), 'additional inventory overlap or count drift')
    counts = {}
    for row in combined['rows']:
        action = row['proposed_action']
        counts[action] = counts.get(action, 0) + 1
    require(counts == combined['counts'] and combined['original_count'] == 4134, 'combined counts drift')
    held = {r['id']: r for r in combined['rows'] if r['proposed_action'].startswith('hold_')}
    grouped = [r for g in groups['groups'] for r in g['rows']]
    require(len(grouped) == len({r['id'] for r in grouped}) == groups['held_count'] == len(held) and
            all(held.get(r['id']) == r for r in grouped) and
            all(g['count'] == len(g['rows']) for g in groups['groups']) and
            groups['original_count'] == 4134 and groups['additional_paths_separate'] == 22,
            'held groups are not complete and disjoint')


def check_execution_overlay(manifest, baseline, candidate, approved_removed, scoped_paths):
    """Pure exact-identity check; proposals never confer execution authority."""
    require(manifest['schema'] == 'adl.source_retirement.execution_overlay.v1' and manifest['issue'] == 1223,
            'invalid execution manifest')
    require(manifest['status'] == 'complete' and not manifest['missing_entries'],
            'execution manifest incomplete: ' + '; '.join(manifest['missing_entries']))
    expected = dict(baseline)
    seen = set()
    removals = set(approved_removed)
    for entry in manifest['entries']:
        path = entry['path']
        require(path not in seen, 'duplicate execution path: ' + path)
        seen.add(path)
        require(entry['review_reference'] and entry['implementation_commit'] and entry['approval_reference'],
                'execution provenance missing: ' + path)
        require(entry['before'] == baseline.get(path), 'execution before identity mismatch: ' + path)
        after = entry['after']
        action = entry['action']
        require(action in ('add', 'modify', 'remove'), 'unknown execution action')
        require((action == 'add' and entry['before'] is None and after is not None) or
                (action == 'modify' and entry['before'] is not None and after is not None) or
                (action == 'remove' and entry['before'] is not None and after is None), 'invalid execution transition')
        if after is not None:
            require(len(after) == 2 and after[0] in ('100644', '100755', '120000') and
                    len(after[1]) == 40 and all(c in '0123456789abcdef' for c in after[1]), 'invalid after identity')
            expected[path] = after
        else:
            require(path in removals and entry.get('rollback_custody'), 'unapproved removal or missing custody: ' + path)
            expected.pop(path, None)
    require(set(baseline) - set(candidate) == removals, 'new deletion outside executed approval set')
    for path in scoped_paths | seen:
        if path in removals:
            require(path not in candidate, 'approved removal still present: ' + path)
        else:
            require(candidate.get(path) == expected.get(path), 'retained source exceeds execution identity: ' + path)
    return sorted(removals)


def tree(ref):
    return {path: fields.split()[::2] for line in git('ls-tree', '-r', ref).splitlines()
            for fields, path in [line.split('\t', 1)]}


def approved_component_delta():
    data = committed_bytes(PACKET / 'APPROVED_COMPONENT_DELTA.json')
    require(hashlib.sha256(data).hexdigest() == '4b752d6ccb95bb7c2857540126f3d1c145c2220838a1a1df9e9c59bc0d94198c',
            'approved component delta changed')
    return json.loads(data)


def approved_consumer_identity(path, identity):
    # #1223 public qualification adds one integration target without changing
    # the immutable component approval or authorizing another source removal.
    if path != 'adl/config/validation_lane_selector.v0.91.6.json':
        return identity
    before = subprocess.check_output(['git', '--no-optional-locks', 'cat-file', 'blob', identity[1]], cwd=ROOT)
    old = b'        "adl/tests/public_process.rs",\n'
    require(before.count(old) == 2, 'public observation selector baseline changed')
    after = before.replace(old, old + b'        "adl/tests/public_observation.rs",\n')
    digest = hashlib.sha1(b'blob ' + str(len(after)).encode() + b'\0' + after).hexdigest()
    return [identity[0], digest]


def check_inventory(ledger, frozen, observed, candidate):
    require(ledger['original_revision'] == '248f00e359e412f6bc0061ac88be9ff374e3a212' and
            ledger['observed_main'] == '9cfd035aac2a44637b3c8026bd40b669579a8c7f', 'reviewed baseline changed')
    rows = ledger['rows']
    tuples = [[r['id'], r['path'], r['original_mode'], r['original_blob']] for r in rows]
    require(len(rows) == len({r['path'] for r in rows}) == 4134, 'original denominator missing or duplicated')
    require(hashlib.sha256(json.dumps(tuples, separators=(',', ':')).encode()).hexdigest() == INVENTORY_SHA256,
            'original inventory identity changed')
    require(ledger['new_removals_authorized'] is True and ledger['all_holds_resolved'] is False,
            'scoped approval does not resolve the full inventory')
    approved = approved_component_delta()
    removed = []
    for r in rows:
        p = r['path']
        require(frozen.get(p) == [r['original_mode'], r['original_blob']], 'frozen identity mismatch: ' + p)
        require(observed.get(p) == r['observed_main'], 'observed source identity mismatch: ' + p)
        if r['action'] == 'existing_branch_removal':
            require(any(p.startswith(c + '/') for c in COMPONENTS), 'out-of-scope removal: ' + p)
            require(p not in candidate, 'declared prior removal still present: ' + p)
            removed.append(p)
        elif r['action'] == 'approved_component_removal':
            require(p in approved['removed'] and p not in candidate, 'unapproved component removal: ' + p)
            removed.append(p)
        elif r['action'] == 'approved_component_consumer_repair':
            require(p in approved['repairs'] and candidate.get(p) == approved_consumer_identity(p, approved['repairs'][p]),
                    'component consumer repair exceeds approved bytes: ' + p)
        elif r['action'] == 'existing_branch_surgical':
            require(p == 'infra/runtime-v3/runtime-init.toml', 'unapproved surgical path')
            before = subprocess.check_output(['git', '--no-optional-locks', 'show', ledger['observed_main'] + ':' + p], cwd=ROOT)
            after = before.replace(b'# S3 archive values come from infra/aws/runtime/agent-checkpoint-archive outputs.',
                                   b"# S3 archive values come from the agent-logic-infrastructure repository's\n# infra/aws/runtime/agent-checkpoint-archive outputs.")
            expected = hashlib.sha1(b'blob ' + str(len(after)).encode() + b'\0' + after).hexdigest()
            require(after != before and candidate.get(p) == [observed[p][0], expected], 'surgical change exceeds reviewed comment')
        elif r['action'] == 'consumer_reference_repair':
            before = subprocess.check_output(['git', '--no-optional-locks', 'show', ledger['observed_main'] + ':' + p], cwd=ROOT)
            after = consumer_reference_repair(p, before)
            expected = hashlib.sha1(b'blob ' + str(len(after)).encode() + b'\0' + after).hexdigest()
            require(candidate.get(p) == [observed[p][0], expected], 'consumer repair exceeds exact ownership notice: ' + p)
        else:
            require(r['action'] == 'retain', 'unknown action: ' + p)
            require(candidate.get(p) == observed.get(p), 'held source removed or altered: ' + p)
    for path, identity in approved['repairs'].items():
        require(candidate.get(path) == approved_consumer_identity(path, identity), 'component consumer repair exceeds approved bytes: ' + path)
    require(len(removed) == 70 and set(approved['removed']).issubset(removed),
            'retirement delta must preserve prior 34 and approved 36 removals')
    require(set(observed) - set(candidate) == set(removed), 'new deletion outside reviewed 70-path set')
    for p in PROTECTED:
        expected = approved['repairs'][p] if p == 'adl/tools/install_owner_binaries.sh' else observed.get(p)
        require(candidate.get(p) == expected and p in observed, 'required evidence/contract missing or changed: ' + p)
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
    combined = committed_json(PACKET / 'COMBINED_REMAINING_DELTA.json')
    groups = committed_json(PACKET / 'GROUPED_REMAINING_HOLDS.json')
    check_combined(ledger, combined, groups, tree(ledger['original_revision']), observed)
    require(groups['ledger_sha256'] == hashlib.sha256(committed_bytes(PACKET / 'COMBINED_REMAINING_DELTA.json')).hexdigest(),
            'grouped ledger digest mismatch')
    approved = approved_component_delta()
    removed = {r['path'] for r in ledger['rows'] if r['action'] == 'existing_branch_removal'}
    require(len(removed) == 34 and all(any(p.startswith(c + '/') for c in COMPONENTS) for p in removed), 'original removal scope changed')
    require(len(approved['removed']) == 36 and not removed.intersection(approved['removed']), 'component removal scope changed')
    removed.update(approved['removed'])
    require(hashlib.sha256(committed_bytes(PACKET / 'PUBLIC_CUTOVER_APPLIED.json')).hexdigest() ==
            'b4c8926946c54a5bca644b8c171b03995a404b8bcea55102ef45c5ec6ff12dcd', 'public cutover approval changed')
    cutover = committed_json(PACKET / 'PUBLIC_CUTOVER_APPLIED.json')
    extra = 'adl/tools/test_select_validation_lanes.sh'
    require([r['path'] for r in cutover['paths'] if r['action'] == 'delete'] == [extra], 'public cutover deletion scope changed')
    require(extra not in tree(cutover['implementation_commit']) and extra in tree(cutover['base_head']), 'cutover removal custody mismatch')
    removed.add(extra)
    overlay_bytes = committed_bytes(PACKET / 'EXECUTION_OVERLAY.json')
    overlay = json.loads(overlay_bytes)
    require(overlay['status'] == 'complete' and not overlay['missing_entries'],
            'execution manifest incomplete: ' + '; '.join(overlay['missing_entries']))
    require(EXECUTION_APPROVAL_SHA256 is not None and
            hashlib.sha256(overlay_bytes).hexdigest() == EXECUTION_APPROVAL_SHA256,
            'complete execution overlay lacks immutable reviewed approval')
    require(overlay['baseline'] == ledger['observed_main'], 'execution baseline changed')
    # Inventory members, additional paths, protected contracts and every approved
    # cutover path remain exact. Evidence records are not permission to change code.
    scoped = {r['path'] for r in combined['rows'] + combined['additional_paths']} | set(PROTECTED) | {r['path'] for r in cutover['paths']}
    removed = check_execution_overlay(overlay, observed, candidate, removed, scoped)
    for path, identity in observed.items():
        if path.startswith('.csdlc/') and not path.startswith(('.csdlc/issues/1223/', '.csdlc/v3/issues/1223/', '.csdlc/evidence/1223/', '.csdlc/transactions/')):
            require(candidate.get(path) == identity, 'historical lifecycle evidence changed: ' + path)
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
                        str(PACKET / 'INDEPENDENT_REPOSITORY_BUILDS.json'),
                        str(PACKET / 'REMAINING_COMPONENT_DECISIONS.json'),
                        str(PACKET / 'COMBINED_REMAINING_DELTA.json'),
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
            'candidate_rows': 4134, 'existing_branch_removals': 34, 'approved_component_removals': 36, 'public_cutover_removals': 1, 'additional_inventory_paths': 22, 'total_removals': len(removed), 'direct_reference_files': scanned,
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
