#!/usr/bin/env python3
"""Bounded #1223 guard: inventory, retained source/evidence, rollback and direct references."""
import argparse
import hashlib
import json
from pathlib import Path
import subprocess
import tarfile

ROOT = Path(__file__).resolve().parents[2]
PACKET = Path('docs/milestones/v0.93.1/repository-decomposition/rd07-1223')
# Original 4134-row inventory, ordered id/path/mode/blob tuple digest.
INVENTORY_SHA256 = 'a22fa6306245ddb3916cf4cb3021286a2d91ebfe77ca719eb7b1039d9f6d3cd3'
# Captured-current identities (4134 originals + 22 additions) in immutable
# COMBINED_REMAINING_DELTA at 666818f2aafdaf4839d1c3de4119b513c4236d3d.
CAPTURED_IDENTITIES_SHA256 = 'e306e5a70ae7792fd2fef91449a0285d665126375a28c0dd05914724fe1bdebe'
ARCHIVE_SHA256 = '12205043566913435c25ea414b1fd9148b46727821f7bfefd5d452cfe47a1c17'
# Independently reviewed existing execution overlay; changing this pin requires review.
EXECUTION_APPROVAL_SHA256 = 'be9f5407e852567e9f9cbf978b974d2eab138b2e90e8e5d6b6c2065ec431f1dc'
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
    transient = set()
    for entry in manifest['entries']:
        path = entry['path']
        require(path not in seen, 'duplicate execution path: ' + path)
        seen.add(path)
        require(entry['review_reference'] and entry['implementation_commit'] and entry['approval_reference'],
                'execution provenance missing: ' + path)
        require(entry['before'] == baseline.get(path), 'execution before identity mismatch: ' + path)
        after = entry['after']
        action = entry['action']
        require(action in ('add', 'modify', 'remove', 'remove_added'), 'unknown execution action')
        require((action == 'add' and entry['before'] is None and after is not None) or
                (action == 'modify' and entry['before'] is not None and after is not None) or
                (action == 'remove' and entry['before'] is not None and after is None) or
                (action == 'remove_added' and entry['before'] is None and after is None), 'invalid execution transition')
        if action == 'remove_added':
            require(path == TRANSIENT_CONVERSION_BRIDGE and entry.get('rollback_custody'),
                    'unapproved transient removal')
            chain = entry.get('source_chain', []) + entry.get('supplemental_source_chain', [])
            add = [c for c in chain if c['commit'] == 'f598c0711d9b270a5b304c7cde64dcdf3068619e']
            delete = [c for c in chain if c['commit'] == 'd293b17fe05f70633826017eb581f5315359f80f']
            require(len(add) == len(delete) == 1 and add[0]['before'] is None and
                    add[0]['after'] == delete[0]['before'] == TRANSIENT_BRIDGE_IDENTITY and
                    delete[0]['after'] is None and
                    [entry['rollback_custody']['mode'], entry['rollback_custody']['blob']] == TRANSIENT_BRIDGE_IDENTITY,
                    'transient add/removal chain mismatch')
            transient.add(path)
        if after is not None:
            require(len(after) == 2 and after[0] in ('100644', '100755', '120000') and
                    len(after[1]) == 40 and all(c in '0123456789abcdef' for c in after[1]), 'invalid after identity')
            expected[path] = after
        else:
            require(path in removals and entry.get('rollback_custody'), 'unapproved removal or missing custody: ' + path)
            expected.pop(path, None)
    require(set(baseline) - set(candidate) == removals - transient, 'new deletion outside executed approval set')
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



CODEFRIEND_BATCH_SHA256 = '15b0314612954fff556a87c957baf60a73873d1535da7bc925b9abaca98dec1f'
CODEFRIEND_IMPLEMENTATION = '40dd5b4ce90d6f5fc89db77501788ad30505a292'
CODEFRIEND_REMOVALS = {
    'RD07-1281': 'adl/tools/codefriend_drift_installed_proof.py',
    'RD07-1286': 'adl/tools/codefriend_fitness_installed_proof.py',
    'RD07-1287': 'adl/tools/codefriend_impact_installed_proof.py',
    'RD07-1289': 'adl/tools/codefriend_palace_installed_proof.py',
    'RD07-1290': 'adl/tools/codefriend_rationale_installed_proof.py',
    'RD07-1291': 'adl/tools/codefriend_structure_installed_proof.py',
}


def check_codefriend_batch(batch, before_tree, after_tree, prior_removed):
    rows = batch['removed']
    require(len(rows) == 6 and {r['id']: r['path'] for r in rows} == CODEFRIEND_REMOVALS,
            'CodeFriend removal scope changed')
    paths = {r['path'] for r in rows}
    require(not paths.intersection(prior_removed), 'CodeFriend batch overlaps prior removals')
    require(batch['rollback_archive']['sha256'] == ARCHIVE_SHA256, 'CodeFriend archive changed')
    for row in rows:
        path, before = row['path'], row['before']
        require(before_tree.get(path) == [before['mode'], before['blob']] and
                row['after'] is None and path not in after_tree, 'CodeFriend applied identity mismatch')
        custody = row['rollback']
        require(custody['archive_member'] == 'source/' + path and
                custody['archive_member_sha256'] == before['sha256'] and
                custody['current_bytes_match_archive'] is True and not custody['supplement_required'],
                'CodeFriend rollback identity mismatch')
    return paths



CODEFRIEND_FAMILY_SHA256 = '06a4d80aa527033ef7528a19c714b3c884bf67bbaacbf5d5f21070821f8b4ed4'
CODEFRIEND_FAMILY_IMPLEMENTATION = '5fa701e8e12f87dce7c8e100eacfa04dc1828c22'


def check_codefriend_family(batch, before_tree, after_tree, prior_removed):
    rows = [r for r in batch['files'] if r['action'] == 'remove']
    paths = {r['path'] for r in rows}
    require(len(rows) == len(paths) == 198 and not paths.intersection(prior_removed),
            'CodeFriend family removal scope changed')
    require(batch['counts']['product_removals'] == 196 and batch['counts']['workflow_removals'] == 2,
            'CodeFriend family denominator changed')
    require('adl/src/bin/adl_review.rs' not in paths and 'adl/src/cli/codefriend_owner.rs' not in paths,
            'retained review or public selector removed')
    custody = {r['path']: r for r in batch['rollback_custody']['rows']}
    require(set(custody) == paths, 'CodeFriend family custody scope changed')
    for row in rows:
        path, before = row['path'], row['before']
        require(before_tree.get(path) == [before['mode'], before['blob']] and
                row['after'] is None and path not in after_tree, 'CodeFriend family applied identity mismatch')
        require(custody[path]['preimage_sha256'] == before['sha256'], 'CodeFriend family custody identity mismatch')
    return paths



CSDLC_FAMILY_SHA256 = '1e462af26effbb5ab0f9318152105a3f270d499386fa799c0622a36df014aea2'
CSDLC_FAMILY_IMPLEMENTATION = 'f598c0711d9b270a5b304c7cde64dcdf3068619e'
CSDLC_REMOVAL_SCOPE_SHA256 = 'cc9e93053181b4d0b2ef0379dc435250c9b34c82ad66a2647b3db7eeebba9ffb'


def check_csdlc_family(batch, before_tree, after_tree, prior_removed):
    rows = [r for r in batch['files'] if r['action'] == 'remove']
    paths = {r['path'] for r in rows}
    require(not any(p.startswith(('csdlc-v3/operator/', 'csdlc-v2/operator/',
                                  '.csdlc/', 'docs/templates/')) for p in paths),
            'C-SDLC protected path removal')
    require(len(rows) == len(paths) == 815 and not paths.intersection(prior_removed),
            'C-SDLC family removal scope changed')
    require(batch['counts'] == {'source_removals': 815, 'caller_modifications': 12, 'additions': 3},
            'C-SDLC family denominator changed')
    custody_rows = batch['rollback']['rows']
    custody = {r['path']: r for r in custody_rows}
    scope = hashlib.sha256(json.dumps(sorted((r['id'], r['path']) for r in custody_rows),
                                     separators=(',', ':')).encode()).hexdigest()
    require(len(custody_rows) == 815 and set(custody) == paths and
            scope == CSDLC_REMOVAL_SCOPE_SHA256, 'C-SDLC family custody scope changed')
    for row in rows:
        path, before = row['path'], row['before']
        require(before_tree.get(path) == [before['mode'], before['blob']] and
                row['after'] is None and path not in after_tree, 'C-SDLC family applied identity mismatch')
        c = custody[path]
        require(c['sha256'] == before['sha256'] and c['member'] == 'source/' + path and
                c['mode'] == int(before['mode'][-3:], 8), 'C-SDLC family custody identity mismatch')
    return paths


def check_csdlc_custody(batch, original_path):
    supplement = batch['rollback']['supplement']
    supplement_path = Path(supplement['path'])
    require(hashlib.sha256(supplement_path.read_bytes()).hexdigest() == supplement['sha256'],
            'C-SDLC family supplement changed')
    with tarfile.open(original_path, 'r:gz') as original, tarfile.open(supplement_path) as extra:
        expected_extra = {'source/' + c['path'] for c in batch['rollback']['rows']
                          if not c['original_archive_matches']}
        require({m.name for m in extra.getmembers()} == expected_extra,
                'C-SDLC family supplemental member scope changed')
        for c in batch['rollback']['rows']:
            archive = original if c['original_archive_matches'] else extra
            member = archive.getmember(c['member'])
            require(member.isfile() and member.mode == c['mode'], 'C-SDLC family archive mode changed')
            require(hashlib.sha256(archive.extractfile(member).read()).hexdigest() == c['sha256'],
                    'C-SDLC family archive bytes changed')


RUNTIME_FAMILY_SHA256 = '9600a1fb0ee1a58508069b5847f5b8cf475805cf9b0059bc3f3f5ee3b51fd486'
RUNTIME_FAMILY_IMPLEMENTATION = '3383f7f691ec248d6fe87cd18e4e13c5d1e8d900'
RUNTIME_REMOVAL_SCOPE_SHA256 = '761a90bcfbc11496b1a55dab3c497ca86af17bd83e73f01b9b6c8d370bd022d9'
RUNTIME_PUBLIC_RETAINS = {
    'adl/src/bin/adl_process.rs', 'adl/src/cli/observability.rs',
    'adl/src/cli/process_cmd.rs', 'adl/src/cli/process_cmd/args.rs',
    'adl/src/control_plane.rs', 'adl/tests/cli_smoke/process_status.rs',
    'adl/tests/fixtures/cli_smoke.adl.yaml',
    'adl/tests/fixtures/scheduler/economics_inputs_v1.json',
}


def check_runtime_family(batch, before_tree, after_tree, prior_removed):
    rows = [r for r in batch['files'] if r['action'] == 'remove']
    paths = {r['path'] for r in rows}
    require(not paths.intersection(RUNTIME_PUBLIC_RETAINS) and
            not any(p.startswith(('.csdlc/', 'docs/templates/', 'csdlc-v3/operator/',
                                  'csdlc-v2/operator/')) for p in paths),
            'Runtime protected path removal')
    require(len(rows) == len(paths) == 896 and not paths.intersection(prior_removed),
            'Runtime family removal scope changed')
    require(batch['counts'] == {'source_removals': 896, 'caller_changes': 54, 'public_retains': 8},
            'Runtime family denominator changed')
    custody_rows = batch['prepared_patch']['deletions']
    custody = {r['path']: r for r in custody_rows}
    scope = hashlib.sha256(json.dumps(sorted((r['id'], r['path']) for r in custody_rows),
                                     separators=(',', ':')).encode()).hexdigest()
    require(len(custody_rows) == 896 and set(custody) == paths and
            scope == RUNTIME_REMOVAL_SCOPE_SHA256, 'Runtime family custody scope changed')
    retained = batch['prepared_patch']['retains']
    require(len(retained) == 8 and {r['path'] for r in retained} == RUNTIME_PUBLIC_RETAINS,
            'Runtime public retain scope changed')
    for r in retained:
        require(before_tree.get(r['path']) == r['before'] == after_tree.get(r['path']),
                'Runtime public retained identity changed')
    for row in rows:
        path, before = row['path'], row['before']
        require(before_tree.get(path) == [before['mode'], before['blob']] and
                row['after'] is None and path not in after_tree,
                'Runtime family applied identity mismatch')
        c = custody[path]
        require(c['id'] == row['id'] and c['before'] == [before['mode'], before['blob']] and
                c['current_sha256'] == before['sha256'] and
                c['rollback']['member'] == 'source/' + path,
                'Runtime family custody identity mismatch')
    return paths


def check_runtime_custody(batch, original_path):
    rows = {r['path']: r for r in batch['files'] if r['action'] == 'remove'}
    expected = {r['path'] for r in batch['prepared_patch']['deletions']
                if not r['rollback']['matches_current']}
    archives = batch['prepared_patch']['custody']
    supplements = [a for a in archives if {r['path'] for r in a['entries']} == expected]
    require(len(supplements) == 1, 'Runtime supplement scope changed')
    supplement = supplements[0]
    supplement_path = Path(supplement['path'])
    require(hashlib.sha256(supplement_path.read_bytes()).hexdigest() == supplement['sha256'],
            'Runtime supplement changed')
    with tarfile.open(original_path, 'r:gz') as original, tarfile.open(supplement_path) as extra:
        members = extra.getmembers()
        require(len(members) == len(expected) and {m.name for m in members} == expected,
                'Runtime supplemental member scope changed')
        for path, row in rows.items():
            archive = extra if path in expected else original
            member = archive.getmember(path if path in expected else 'source/' + path)
            before = row['before']
            require(member.isfile() and member.mode == int(before['mode'][-3:], 8),
                    'Runtime archive mode changed')
            require(hashlib.sha256(archive.extractfile(member).read()).hexdigest() == before['sha256'],
                    'Runtime archive bytes changed')



INFRASTRUCTURE_FAMILY_SHA256 = '383c339ebd32c184177ebb6c01e7ce5dd18ea0bbbed38d86f781d021f4b4ac8a'
INFRASTRUCTURE_FAMILY_IMPLEMENTATION = 'ddfe457ebbc2107ee349abf61ba27d57c35b5c37'
INFRASTRUCTURE_REMOVAL_SCOPE_SHA256 = 'b7ec956e2847c0d0345bfcf9cbef3c34b6952ae64bb0618bd77fe342e4d379c4'


def check_infrastructure_family(batch, before_tree, after_tree, prior_removed):
    rows = [r for r in batch['files'] if r['action'] == 'remove']
    paths = {r['path'] for r in rows}
    scope = hashlib.sha256(json.dumps(sorted((r['id'], r['path']) for r in rows),
                                     separators=(',', ':')).encode()).hexdigest()
    require(len(rows) == len(paths) == 212 and not paths.intersection(prior_removed) and
            scope == INFRASTRUCTURE_REMOVAL_SCOPE_SHA256 and
            all(p.startswith('infra/') for p in paths), 'Infrastructure removal scope changed')
    require(batch['counts'] == {'source_removals': 212, 'caller_changes': 20},
            'Infrastructure family denominator changed')
    require(len(batch['files']) == 232 and len({r['path'] for r in batch['files']}) == 232,
            'Infrastructure caller scope changed')
    for row in batch['files']:
        path, before, after = row['path'], row['before'], row['after']
        expected_after = [after['mode'], after['blob']] if after else None
        require(before_tree.get(path) == [before['mode'], before['blob']] and
                after_tree.get(path) == expected_after, 'Infrastructure applied identity mismatch')
        custody = row['rollback']
        require(custody['member'] == path and custody['sha256'] == before['sha256'] and
                int(custody['mode'], 8) == int(before['mode'][-3:], 8),
                'Infrastructure custody identity mismatch')
    return paths


def check_infrastructure_custody(batch):
    archives = {}
    for row in batch['files']:
        custody = row['rollback']
        path = Path(custody['archive'])
        if path not in archives:
            require(hashlib.sha256(path.read_bytes()).hexdigest() == custody['archive_sha256'],
                    'Infrastructure archive changed')
            archives[path] = custody['archive_sha256']
        require(archives[path] == custody['archive_sha256'], 'Infrastructure archive pin mismatch')
    for path in archives:
        with tarfile.open(path) as archive:
            for row in batch['files']:
                custody = row['rollback']
                if Path(custody['archive']) != path:
                    continue
                member = archive.getmember(custody['member'])
                require(member.isfile() and member.mode == int(custody['mode'], 8),
                        'Infrastructure archive mode changed')
                require(hashlib.sha256(archive.extractfile(member).read()).hexdigest() == row['before']['sha256'],
                        'Infrastructure archive bytes changed')


# Values are replaced only from the committed reviewed applied packets.
CONVERSION_SKILLS_BATCHES = [{'packet': 'CONVERSION_HELPER_RETIREMENT_APPLIED.json', 'sha256': '5a5decd5b85fd1dcf512fd3752056d33ce05fc535a314c7a674dcda38d95a92e', 'commit': 'd293b17fe05f70633826017eb581f5315359f80f', 'kind': 'conversion', 'files': 7, 'removals': 6, 'scope_sha256': 'cfead904d1e1a84b86e469c04ec8e32120fbdbf32d2be64e0f1f703c7740f00f'}, {'packet': 'CODEFRIEND_SKILL_FAMILY_APPLIED.json', 'sha256': 'df23c00b87f41144ac19d30c8c6a8e7e806304bb09cc0c6d1e8a51d783ac08ca', 'commit': 'beea825590544b5412d4dcbbdf599d03837e1148', 'kind': 'skills', 'files': 143, 'removals': 127, 'scope_sha256': 'b1130b8ce1ba252a0f64cc8a74070dfb4b2ce84b055028e2a6337e0cbdf0c515'}]
TRANSIENT_CONVERSION_BRIDGE = 'adl/tools/test_issue872_historical_manifest.py'


def check_conversion_skills_family(batch, before_tree, after_tree, prior_removed, spec):
    rows = batch['files']
    removed = [r for r in rows if r['action'] == 'remove']
    paths = {r['path'] for r in removed}
    digest = hashlib.sha256(json.dumps(sorted(paths), separators=(',', ':')).encode()).hexdigest()
    require(len(rows) == len({r['path'] for r in rows}) == spec['files'] and
            len(removed) == len(paths) == spec['removals'] and
            digest == spec['scope_sha256'] and not paths.intersection(prior_removed),
            'Conversion/skills exact scope changed')
    require(not any(p.startswith(('.csdlc/', 'csdlc-v3/operator/', 'docs/templates/')) for p in paths),
            'Conversion/skills protected path removal')
    for row in rows:
        before, after, path = row['before'], row['after'], row['path']
        require(before_tree.get(path) == [before['mode'], before['blob']] and
                after_tree.get(path) == ([after['mode'], after['blob']] if after else None),
                'Conversion/skills applied identity mismatch')
    return paths


def check_conversion_skills_custody(batch, spec):
    archive = batch['rollback']['supplement'] if spec['kind'] == 'conversion' else batch['custody']
    path = Path(archive['path'])
    require(hashlib.sha256(path.read_bytes()).hexdigest() == archive['sha256'],
            'Conversion/skills archive changed')
    rows = [r for r in batch['files'] if r['action'] == 'remove']
    with tarfile.open(path) as tar:
        require({m.name for m in tar.getmembers()} == {'source/' + r['path'] for r in rows},
                'Conversion/skills custody member scope changed')
        for row in rows:
            before = row['before']
            member = tar.getmember('source/' + row['path'])
            require(member.isfile() and member.mode == int(before['mode'][-3:], 8) and
                    hashlib.sha256(tar.extractfile(member).read()).hexdigest() == before['sha256'],
                    'Conversion/skills custody identity mismatch')

TRANSIENT_BRIDGE_IDENTITY = ['100644', '480dcfe6e6b6355184b1fa729de3cd6f457e7bef']

# Exact reviewed committed owner-family applications; proposals are not authority.
RECENT_OWNER_BATCHES = [{'packet': 'CSDLC_SKILL_FAMILY_APPLIED.json', 'sha256': '273a859462a937e1cfea30d7db9aacdd5317eb7a4bf254079695b19afa51d699', 'commit': '5860e28608d93c29d2d599a5392162a29e62dd59', 'files': 96, 'removals': 83, 'scope_sha256': '5f465697e7b2820a530e7d8d5c2ab06b909b67c9b1d96f73545950638bf9fa91', 'archives': [{'path': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'sha256': '9385ec068473fae8f6cc0ec051fdff46deb35e6d57026172fee7d19e85a6d46e'}, {'path': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/caller-closure/preimages.tar', 'sha256': 'b4c6f5dc9fc639043b9735c860eaf8bb76dcf3fb040665a8494c2859db3e9885'}], 'custody': {'adl/tools/skills/demo-operator/SKILL.md': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/demo-operator/SKILL.md'}, 'adl/tools/skills/demo-operator/adl-skill.yaml': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/demo-operator/adl-skill.yaml'}, 'adl/tools/skills/demo-operator/agents/openai.yaml': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/demo-operator/agents/openai.yaml'}, 'adl/tools/skills/demo-operator/references/demo-playbook.md': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/demo-operator/references/demo-playbook.md'}, 'adl/tools/skills/demo-operator/references/output-contract.md': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/demo-operator/references/output-contract.md'}, 'adl/tools/skills/docs/DEMO_OPERATOR_SKILL_INPUT_SCHEMA.md': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/docs/DEMO_OPERATOR_SKILL_INPUT_SCHEMA.md'}, 'adl/tools/skills/docs/DOCUMENTATION_SPECIALIST_SKILL_INPUT_SCHEMA.md': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/docs/DOCUMENTATION_SPECIALIST_SKILL_INPUT_SCHEMA.md'}, 'adl/tools/skills/docs/GAP_ANALYSIS_SKILL_INPUT_SCHEMA.md': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/docs/GAP_ANALYSIS_SKILL_INPUT_SCHEMA.md'}, 'adl/tools/skills/docs/ISSUE_FOLDING_SKILL_INPUT_SCHEMA.md': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/docs/ISSUE_FOLDING_SKILL_INPUT_SCHEMA.md'}, 'adl/tools/skills/docs/ISSUE_SPLITTER_SKILL_INPUT_SCHEMA.md': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/docs/ISSUE_SPLITTER_SKILL_INPUT_SCHEMA.md'}, 'adl/tools/skills/docs/ISSUE_WATCHER_SKILL_INPUT_SCHEMA.md': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/docs/ISSUE_WATCHER_SKILL_INPUT_SCHEMA.md'}, 'adl/tools/skills/docs/PORTABLE_CONTRACT_NORMALIZER_SKILL_INPUT_SCHEMA.md': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/docs/PORTABLE_CONTRACT_NORMALIZER_SKILL_INPUT_SCHEMA.md'}, 'adl/tools/skills/docs/REFACTORING_HELPER_SKILL_INPUT_SCHEMA.md': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/docs/REFACTORING_HELPER_SKILL_INPUT_SCHEMA.md'}, 'adl/tools/skills/docs/RELEASE_EVIDENCE_SKILL_INPUT_SCHEMA.md': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/docs/RELEASE_EVIDENCE_SKILL_INPUT_SCHEMA.md'}, 'adl/tools/skills/docs/REVIEW_COMMENT_TRIAGE_SKILL_INPUT_SCHEMA.md': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/docs/REVIEW_COMMENT_TRIAGE_SKILL_INPUT_SCHEMA.md'}, 'adl/tools/skills/docs/TEST_GENERATOR_SKILL_INPUT_SCHEMA.md': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/docs/TEST_GENERATOR_SKILL_INPUT_SCHEMA.md'}, 'adl/tools/skills/docs/USE_CASE_WRITER_SKILL_INPUT_SCHEMA.md': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/docs/USE_CASE_WRITER_SKILL_INPUT_SCHEMA.md'}, 'adl/tools/skills/documentation-specialist/SKILL.md': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/documentation-specialist/SKILL.md'}, 'adl/tools/skills/documentation-specialist/adl-skill.yaml': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/documentation-specialist/adl-skill.yaml'}, 'adl/tools/skills/documentation-specialist/agents/openai.yaml': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/documentation-specialist/agents/openai.yaml'}, 'adl/tools/skills/documentation-specialist/references/output-contract.md': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/documentation-specialist/references/output-contract.md'}, 'adl/tools/skills/gap-analysis/SKILL.md': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/gap-analysis/SKILL.md'}, 'adl/tools/skills/gap-analysis/adl-skill.yaml': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/gap-analysis/adl-skill.yaml'}, 'adl/tools/skills/gap-analysis/agents/openai.yaml': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/gap-analysis/agents/openai.yaml'}, 'adl/tools/skills/gap-analysis/references/gap-analysis-playbook.md': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/gap-analysis/references/gap-analysis-playbook.md'}, 'adl/tools/skills/gap-analysis/references/output-contract.md': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/gap-analysis/references/output-contract.md'}, 'adl/tools/skills/gap-analysis/scripts/analyze_gaps.py': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/gap-analysis/scripts/analyze_gaps.py'}, 'adl/tools/skills/issue-folding/SKILL.md': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/issue-folding/SKILL.md'}, 'adl/tools/skills/issue-folding/adl-skill.yaml': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/issue-folding/adl-skill.yaml'}, 'adl/tools/skills/issue-folding/agents/openai.yaml': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/issue-folding/agents/openai.yaml'}, 'adl/tools/skills/issue-folding/references/issue-folding-playbook.md': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/issue-folding/references/issue-folding-playbook.md'}, 'adl/tools/skills/issue-folding/references/output-contract.md': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/issue-folding/references/output-contract.md'}, 'adl/tools/skills/issue-folding/scripts/classify_issue_folding.py': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/issue-folding/scripts/classify_issue_folding.py'}, 'adl/tools/skills/issue-splitter/SKILL.md': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/issue-splitter/SKILL.md'}, 'adl/tools/skills/issue-splitter/adl-skill.yaml': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/issue-splitter/adl-skill.yaml'}, 'adl/tools/skills/issue-splitter/agents/openai.yaml': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/issue-splitter/agents/openai.yaml'}, 'adl/tools/skills/issue-splitter/references/issue-splitter-playbook.md': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/issue-splitter/references/issue-splitter-playbook.md'}, 'adl/tools/skills/issue-splitter/references/output-contract.md': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/issue-splitter/references/output-contract.md'}, 'adl/tools/skills/issue-splitter/scripts/plan_issue_split.py': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/issue-splitter/scripts/plan_issue_split.py'}, 'adl/tools/skills/issue-watcher/SKILL.md': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/issue-watcher/SKILL.md'}, 'adl/tools/skills/issue-watcher/adl-skill.yaml': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/issue-watcher/adl-skill.yaml'}, 'adl/tools/skills/issue-watcher/agents/openai.yaml': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/issue-watcher/agents/openai.yaml'}, 'adl/tools/skills/issue-watcher/references/output-contract.md': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/issue-watcher/references/output-contract.md'}, 'adl/tools/skills/portable-contract-normalizer/SKILL.md': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/portable-contract-normalizer/SKILL.md'}, 'adl/tools/skills/portable-contract-normalizer/adl-skill.yaml': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/portable-contract-normalizer/adl-skill.yaml'}, 'adl/tools/skills/portable-contract-normalizer/agents/openai.yaml': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/portable-contract-normalizer/agents/openai.yaml'}, 'adl/tools/skills/portable-contract-normalizer/references/output-contract.md': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/portable-contract-normalizer/references/output-contract.md'}, 'adl/tools/skills/portable-contract-normalizer/references/portable-contract-normalizer-playbook.md': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/portable-contract-normalizer/references/portable-contract-normalizer-playbook.md'}, 'adl/tools/skills/portable-contract-normalizer/scripts/normalize_portable_contracts.py': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/portable-contract-normalizer/scripts/normalize_portable_contracts.py'}, 'adl/tools/skills/refactoring-helper/SKILL.md': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/refactoring-helper/SKILL.md'}, 'adl/tools/skills/refactoring-helper/adl-skill.yaml': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/refactoring-helper/adl-skill.yaml'}, 'adl/tools/skills/refactoring-helper/agents/openai.yaml': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/refactoring-helper/agents/openai.yaml'}, 'adl/tools/skills/refactoring-helper/references/output-contract.md': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/refactoring-helper/references/output-contract.md'}, 'adl/tools/skills/refactoring-helper/references/refactoring-playbook.md': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/refactoring-helper/references/refactoring-playbook.md'}, 'adl/tools/skills/refactoring-helper/scripts/plan_refactor.py': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/refactoring-helper/scripts/plan_refactor.py'}, 'adl/tools/skills/release-evidence/SKILL.md': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/release-evidence/SKILL.md'}, 'adl/tools/skills/release-evidence/adl-skill.yaml': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/release-evidence/adl-skill.yaml'}, 'adl/tools/skills/release-evidence/agents/openai.yaml': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/release-evidence/agents/openai.yaml'}, 'adl/tools/skills/release-evidence/references/output-contract.md': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/release-evidence/references/output-contract.md'}, 'adl/tools/skills/release-evidence/references/release-evidence-playbook.md': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/release-evidence/references/release-evidence-playbook.md'}, 'adl/tools/skills/release-evidence/scripts/assemble_release_evidence.py': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/release-evidence/scripts/assemble_release_evidence.py'}, 'adl/tools/skills/review-comment-triage/SKILL.md': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/review-comment-triage/SKILL.md'}, 'adl/tools/skills/review-comment-triage/adl-skill.yaml': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/review-comment-triage/adl-skill.yaml'}, 'adl/tools/skills/review-comment-triage/agents/openai.yaml': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/review-comment-triage/agents/openai.yaml'}, 'adl/tools/skills/review-comment-triage/fixtures/actionable_comments.json': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/review-comment-triage/fixtures/actionable_comments.json'}, 'adl/tools/skills/review-comment-triage/fixtures/already_fixed_comments.json': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/review-comment-triage/fixtures/already_fixed_comments.json'}, 'adl/tools/skills/review-comment-triage/fixtures/follow_on_comments.json': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/review-comment-triage/fixtures/follow_on_comments.json'}, 'adl/tools/skills/review-comment-triage/fixtures/mixed_comments.json': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/review-comment-triage/fixtures/mixed_comments.json'}, 'adl/tools/skills/review-comment-triage/fixtures/stale_comments.json': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/review-comment-triage/fixtures/stale_comments.json'}, 'adl/tools/skills/review-comment-triage/references/output-contract.md': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/review-comment-triage/references/output-contract.md'}, 'adl/tools/skills/review-comment-triage/references/review-comment-triage-playbook.md': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/review-comment-triage/references/review-comment-triage-playbook.md'}, 'adl/tools/skills/review-comment-triage/scripts/triage_review_comments.py': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/review-comment-triage/scripts/triage_review_comments.py'}, 'adl/tools/skills/test-generator/SKILL.md': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/test-generator/SKILL.md'}, 'adl/tools/skills/test-generator/adl-skill.yaml': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/test-generator/adl-skill.yaml'}, 'adl/tools/skills/test-generator/agents/openai.yaml': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/test-generator/agents/openai.yaml'}, 'adl/tools/skills/test-generator/references/output-contract.md': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/test-generator/references/output-contract.md'}, 'adl/tools/skills/test-generator/references/test-playbook.md': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/test-generator/references/test-playbook.md'}, 'adl/tools/skills/use-case-writer/SKILL.md': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/use-case-writer/SKILL.md'}, 'adl/tools/skills/use-case-writer/adl-skill.yaml': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/use-case-writer/adl-skill.yaml'}, 'adl/tools/skills/use-case-writer/agents/openai.yaml': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/use-case-writer/agents/openai.yaml'}, 'adl/tools/skills/use-case-writer/references/output-contract.md': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/use-case-writer/references/output-contract.md'}, 'adl/tools/skills/use-case-writer/references/use-case-writing-playbook.md': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/use-case-writer/references/use-case-writing-playbook.md'}, 'adl/tools/skills/use-case-writer/scripts/write_use_cases.py': {'archive': '/Volumes/FastWork/c1223/csdlc83-skill-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/use-case-writer/scripts/write_use_cases.py'}}}, {'packet': 'AWS_REMOTE_FAMILY_APPLIED.json', 'sha256': 'e88324d672c1455fba7e1d521b18e0efd6fa7756215af35dba19b718a1add8e1', 'commit': '1f438d059a37442c85059992c503740aa1bf088e', 'files': 25, 'removals': 14, 'scope_sha256': 'f0ce6bc7d6f5d03dad5203cb518788cf11c5356adff349754df615b3d2dade17', 'archives': [{'path': '/Volumes/FastWork/c1223/aws-remote-family-prepared-patch/preimages.tar', 'sha256': '1127b5d479732004da7abbbdd41ecc3be8c34fbd74b392e7ea804e8d59505b8c'}], 'custody': {'tools/aws_remote_validation/BOOTSTRAP_IDENTITIES.md': {'archive': '/Volumes/FastWork/c1223/aws-remote-family-prepared-patch/preimages.tar', 'member': 'source/tools/aws_remote_validation/BOOTSTRAP_IDENTITIES.md'}, 'tools/aws_remote_validation/Cargo.lock': {'archive': '/Volumes/FastWork/c1223/aws-remote-family-prepared-patch/preimages.tar', 'member': 'source/tools/aws_remote_validation/Cargo.lock'}, 'tools/aws_remote_validation/Cargo.toml': {'archive': '/Volumes/FastWork/c1223/aws-remote-family-prepared-patch/preimages.tar', 'member': 'source/tools/aws_remote_validation/Cargo.toml'}, 'tools/aws_remote_validation/scripts/remote_validation_runner.sh': {'archive': '/Volumes/FastWork/c1223/aws-remote-family-prepared-patch/preimages.tar', 'member': 'source/tools/aws_remote_validation/scripts/remote_validation_runner.sh'}, 'tools/aws_remote_validation/scripts/ssh_debug_control.sh': {'archive': '/Volumes/FastWork/c1223/aws-remote-family-prepared-patch/preimages.tar', 'member': 'source/tools/aws_remote_validation/scripts/ssh_debug_control.sh'}, 'tools/aws_remote_validation/scripts/test_verified_bootstrap.py': {'archive': '/Volumes/FastWork/c1223/aws-remote-family-prepared-patch/preimages.tar', 'member': 'source/tools/aws_remote_validation/scripts/test_verified_bootstrap.py'}, 'tools/aws_remote_validation/scripts/verified_bootstrap.pvf.json': {'archive': '/Volumes/FastWork/c1223/aws-remote-family-prepared-patch/preimages.tar', 'member': 'source/tools/aws_remote_validation/scripts/verified_bootstrap.pvf.json'}, 'tools/aws_remote_validation/scripts/verified_bootstrap.py': {'archive': '/Volumes/FastWork/c1223/aws-remote-family-prepared-patch/preimages.tar', 'member': 'source/tools/aws_remote_validation/scripts/verified_bootstrap.py'}, 'tools/aws_remote_validation/src/aws_remote_validation.rs': {'archive': '/Volumes/FastWork/c1223/aws-remote-family-prepared-patch/preimages.tar', 'member': 'source/tools/aws_remote_validation/src/aws_remote_validation.rs'}, 'tools/aws_remote_validation/src/bin/adl_aws_remote_validation.rs': {'archive': '/Volumes/FastWork/c1223/aws-remote-family-prepared-patch/preimages.tar', 'member': 'source/tools/aws_remote_validation/src/bin/adl_aws_remote_validation.rs'}, 'tools/aws_remote_validation/src/cli/observability.rs': {'archive': '/Volumes/FastWork/c1223/aws-remote-family-prepared-patch/preimages.tar', 'member': 'source/tools/aws_remote_validation/src/cli/observability.rs'}, 'tools/aws_remote_validation/tests/portable_adapter.rs': {'archive': '/Volumes/FastWork/c1223/aws-remote-family-prepared-patch/preimages.tar', 'member': 'source/tools/aws_remote_validation/tests/portable_adapter.rs'}, 'adl/src/bin/adl_aws_remote_validation.rs': {'archive': '/Volumes/FastWork/c1223/aws-remote-family-prepared-patch/preimages.tar', 'member': 'source/adl/src/bin/adl_aws_remote_validation.rs'}, '.github/workflows/aws-spot-remote-validation.yaml': {'archive': '/Volumes/FastWork/c1223/aws-remote-family-prepared-patch/preimages.tar', 'member': 'source/.github/workflows/aws-spot-remote-validation.yaml'}}}, {'packet': 'PLANNING_CONTRACT_FAMILY_APPLIED.json', 'sha256': 'b500906ed25661a8b4cad66ca462797309b5d50916279aa87c0107663dffd32f', 'commit': '09b73f53b0f85db42b219c7ed7f300f1f11e443e', 'files': 12, 'removals': 9, 'scope_sha256': 'b840e364a3919c41dec1f806460ab21ce2bfe3b66a0eb3069d393466d16b0e36', 'archives': [{'path': '/Volumes/FastWork/c1223/planning-contract-family-preparation/current-preimages.tar', 'sha256': 'f915a0d901cdcb104c601fd67d1129b80956aa1d6ac118a10a5dd7e4510865c8'}, {'path': '/Volumes/FastWork/c1223/planning-contract-family-preparation/actual-registry-preimage-1f438d.tar', 'sha256': '829dea9fd3c3a041005b9f55ca7a5bec4fcebb17ba1d7c68d20a1df0eaf31fea'}], 'custody': {'adl/tools/check_compression_finish_profiles.sh': {'archive': '/Volumes/FastWork/c1223/planning-contract-family-preparation/current-preimages.tar', 'member': 'source/adl/tools/check_compression_finish_profiles.sh'}, 'adl/tools/check_issue_goal_terminal_state.py': {'archive': '/Volumes/FastWork/c1223/planning-contract-family-preparation/current-preimages.tar', 'member': 'source/adl/tools/check_issue_goal_terminal_state.py'}, 'adl/tools/check_v090_milestone_state.py': {'archive': '/Volumes/FastWork/c1223/planning-contract-family-preparation/current-preimages.tar', 'member': 'source/adl/tools/check_v090_milestone_state.py'}, 'adl/tools/plan_multi_agent_workcell.py': {'archive': '/Volumes/FastWork/c1223/planning-contract-family-preparation/current-preimages.tar', 'member': 'source/adl/tools/plan_multi_agent_workcell.py'}, 'adl/tools/test_check_v090_milestone_state.sh': {'archive': '/Volumes/FastWork/c1223/planning-contract-family-preparation/current-preimages.tar', 'member': 'source/adl/tools/test_check_v090_milestone_state.sh'}, 'adl/tools/test_issue_goal_terminal_state_truth.sh': {'archive': '/Volumes/FastWork/c1223/planning-contract-family-preparation/current-preimages.tar', 'member': 'source/adl/tools/test_issue_goal_terminal_state_truth.sh'}, 'adl/tools/test_plan_multi_agent_workcell.sh': {'archive': '/Volumes/FastWork/c1223/planning-contract-family-preparation/current-preimages.tar', 'member': 'source/adl/tools/test_plan_multi_agent_workcell.sh'}, 'adl/tools/test_validate_multi_agent_workcell_state.sh': {'archive': '/Volumes/FastWork/c1223/planning-contract-family-preparation/current-preimages.tar', 'member': 'source/adl/tools/test_validate_multi_agent_workcell_state.sh'}, 'adl/tools/validate_multi_agent_workcell_state.py': {'archive': '/Volumes/FastWork/c1223/planning-contract-family-preparation/current-preimages.tar', 'member': 'source/adl/tools/validate_multi_agent_workcell_state.py'}}}, {'packet': 'MANUAL_FAMILY_APPLIED.json', 'sha256': '7155f1c1d38aaaae7a132ebdfeec5069ef020f35f158a080e2541f2af69291d0', 'commit': 'be5cdb719496fefe9a0ed55d5666cf3c1c14fc3b', 'files': 58, 'removals': 56, 'scope_sha256': 'a579af5b34b8e994b5e86e3cedda02e533215a714b84cfeafaa66a97c52f71cf', 'archives': [{'path': '/Volumes/FastWork/c1223/manual-family-preparation/current-preimages.tar', 'sha256': '50b15efb58650072708e6de791af67e3b1c0aeb8484c9760c3bce632dd62245a'}], 'custody': {'docs/csdlc-v3/man/examples/clean.json': {'archive': '/Volumes/FastWork/c1223/manual-family-preparation/current-preimages.tar', 'member': 'source/docs/csdlc-v3/man/examples/clean.json'}, 'docs/csdlc-v3/man/examples/cutover.json': {'archive': '/Volumes/FastWork/c1223/manual-family-preparation/current-preimages.tar', 'member': 'source/docs/csdlc-v3/man/examples/cutover.json'}, 'docs/csdlc-v3/man/examples/finish-primary.json': {'archive': '/Volumes/FastWork/c1223/manual-family-preparation/current-preimages.tar', 'member': 'source/docs/csdlc-v3/man/examples/finish-primary.json'}, 'docs/csdlc-v3/man/examples/finish.json': {'archive': '/Volumes/FastWork/c1223/manual-family-preparation/current-preimages.tar', 'member': 'source/docs/csdlc-v3/man/examples/finish.json'}, 'docs/csdlc-v3/man/examples/install.json': {'archive': '/Volumes/FastWork/c1223/manual-family-preparation/current-preimages.tar', 'member': 'source/docs/csdlc-v3/man/examples/install.json'}, 'docs/csdlc-v3/man/examples/local.json': {'archive': '/Volumes/FastWork/c1223/manual-family-preparation/current-preimages.tar', 'member': 'source/docs/csdlc-v3/man/examples/local.json'}, 'docs/csdlc-v3/man/examples/merge.json': {'archive': '/Volumes/FastWork/c1223/manual-family-preparation/current-preimages.tar', 'member': 'source/docs/csdlc-v3/man/examples/merge.json'}, 'docs/csdlc-v3/man/examples/mutation.json': {'archive': '/Volumes/FastWork/c1223/manual-family-preparation/current-preimages.tar', 'member': 'source/docs/csdlc-v3/man/examples/mutation.json'}, 'docs/csdlc-v3/man/examples/pr-create.json': {'archive': '/Volumes/FastWork/c1223/manual-family-preparation/current-preimages.tar', 'member': 'source/docs/csdlc-v3/man/examples/pr-create.json'}, 'docs/csdlc-v3/man/examples/proof.json': {'archive': '/Volumes/FastWork/c1223/manual-family-preparation/current-preimages.tar', 'member': 'source/docs/csdlc-v3/man/examples/proof.json'}, 'docs/csdlc-v3/man/examples/release.json': {'archive': '/Volumes/FastWork/c1223/manual-family-preparation/current-preimages.tar', 'member': 'source/docs/csdlc-v3/man/examples/release.json'}, 'docs/csdlc-v3/man/examples/remote.json': {'archive': '/Volumes/FastWork/c1223/manual-family-preparation/current-preimages.tar', 'member': 'source/docs/csdlc-v3/man/examples/remote.json'}, 'docs/csdlc-v3/man/examples/review.json': {'archive': '/Volumes/FastWork/c1223/manual-family-preparation/current-preimages.tar', 'member': 'source/docs/csdlc-v3/man/examples/review.json'}, 'docs/csdlc-v3/man/examples/rollback.json': {'archive': '/Volumes/FastWork/c1223/manual-family-preparation/current-preimages.tar', 'member': 'source/docs/csdlc-v3/man/examples/rollback.json'}, 'docs/csdlc-v3/man/examples/shadow.json': {'archive': '/Volumes/FastWork/c1223/manual-family-preparation/current-preimages.tar', 'member': 'source/docs/csdlc-v3/man/examples/shadow.json'}, 'docs/csdlc-v3/man/examples/soak.json': {'archive': '/Volumes/FastWork/c1223/manual-family-preparation/current-preimages.tar', 'member': 'source/docs/csdlc-v3/man/examples/soak.json'}, 'docs/csdlc-v3/man/examples/sprint.json': {'archive': '/Volumes/FastWork/c1223/manual-family-preparation/current-preimages.tar', 'member': 'source/docs/csdlc-v3/man/examples/sprint.json'}, 'docs/csdlc-v3/man/examples/typed-review.json': {'archive': '/Volumes/FastWork/c1223/manual-family-preparation/current-preimages.tar', 'member': 'source/docs/csdlc-v3/man/examples/typed-review.json'}, 'docs/csdlc-v3/man/inventory.json': {'archive': '/Volumes/FastWork/c1223/manual-family-preparation/current-preimages.tar', 'member': 'source/docs/csdlc-v3/man/inventory.json'}, 'docs/csdlc-v3/man/man1/csdlc-bind.1': {'archive': '/Volumes/FastWork/c1223/manual-family-preparation/current-preimages.tar', 'member': 'source/docs/csdlc-v3/man/man1/csdlc-bind.1'}, 'docs/csdlc-v3/man/man1/csdlc-clean.1': {'archive': '/Volumes/FastWork/c1223/manual-family-preparation/current-preimages.tar', 'member': 'source/docs/csdlc-v3/man/man1/csdlc-clean.1'}, 'docs/csdlc-v3/man/man1/csdlc-cutover.1': {'archive': '/Volumes/FastWork/c1223/manual-family-preparation/current-preimages.tar', 'member': 'source/docs/csdlc-v3/man/man1/csdlc-cutover.1'}, 'docs/csdlc-v3/man/man1/csdlc-doctor.1': {'archive': '/Volumes/FastWork/c1223/manual-family-preparation/current-preimages.tar', 'member': 'source/docs/csdlc-v3/man/man1/csdlc-doctor.1'}, 'docs/csdlc-v3/man/man1/csdlc-edit.1': {'archive': '/Volumes/FastWork/c1223/manual-family-preparation/current-preimages.tar', 'member': 'source/docs/csdlc-v3/man/man1/csdlc-edit.1'}, 'docs/csdlc-v3/man/man1/csdlc-eligibility.1': {'archive': '/Volumes/FastWork/c1223/manual-family-preparation/current-preimages.tar', 'member': 'source/docs/csdlc-v3/man/man1/csdlc-eligibility.1'}, 'docs/csdlc-v3/man/man1/csdlc-finish.1': {'archive': '/Volumes/FastWork/c1223/manual-family-preparation/current-preimages.tar', 'member': 'source/docs/csdlc-v3/man/man1/csdlc-finish.1'}, 'docs/csdlc-v3/man/man1/csdlc-foundation.1': {'archive': '/Volumes/FastWork/c1223/manual-family-preparation/current-preimages.tar', 'member': 'source/docs/csdlc-v3/man/man1/csdlc-foundation.1'}, 'docs/csdlc-v3/man/man1/csdlc-github-issue.1': {'archive': '/Volumes/FastWork/c1223/manual-family-preparation/current-preimages.tar', 'member': 'source/docs/csdlc-v3/man/man1/csdlc-github-issue.1'}, 'docs/csdlc-v3/man/man1/csdlc-github-pr.1': {'archive': '/Volumes/FastWork/c1223/manual-family-preparation/current-preimages.tar', 'member': 'source/docs/csdlc-v3/man/man1/csdlc-github-pr.1'}, 'docs/csdlc-v3/man/man1/csdlc-github.1': {'archive': '/Volumes/FastWork/c1223/manual-family-preparation/current-preimages.tar', 'member': 'source/docs/csdlc-v3/man/man1/csdlc-github.1'}, 'docs/csdlc-v3/man/man1/csdlc-install.1': {'archive': '/Volumes/FastWork/c1223/manual-family-preparation/current-preimages.tar', 'member': 'source/docs/csdlc-v3/man/man1/csdlc-install.1'}, 'docs/csdlc-v3/man/man1/csdlc-issue.1': {'archive': '/Volumes/FastWork/c1223/manual-family-preparation/current-preimages.tar', 'member': 'source/docs/csdlc-v3/man/man1/csdlc-issue.1'}, 'docs/csdlc-v3/man/man1/csdlc-local.1': {'archive': '/Volumes/FastWork/c1223/manual-family-preparation/current-preimages.tar', 'member': 'source/docs/csdlc-v3/man/man1/csdlc-local.1'}, 'docs/csdlc-v3/man/man1/csdlc-pr-state.1': {'archive': '/Volumes/FastWork/c1223/manual-family-preparation/current-preimages.tar', 'member': 'source/docs/csdlc-v3/man/man1/csdlc-pr-state.1'}, 'docs/csdlc-v3/man/man1/csdlc-prepare.1': {'archive': '/Volumes/FastWork/c1223/manual-family-preparation/current-preimages.tar', 'member': 'source/docs/csdlc-v3/man/man1/csdlc-prepare.1'}, 'docs/csdlc-v3/man/man1/csdlc-proof.1': {'archive': '/Volumes/FastWork/c1223/manual-family-preparation/current-preimages.tar', 'member': 'source/docs/csdlc-v3/man/man1/csdlc-proof.1'}, 'docs/csdlc-v3/man/man1/csdlc-publish.1': {'archive': '/Volumes/FastWork/c1223/manual-family-preparation/current-preimages.tar', 'member': 'source/docs/csdlc-v3/man/man1/csdlc-publish.1'}, 'docs/csdlc-v3/man/man1/csdlc-rebuild.1': {'archive': '/Volumes/FastWork/c1223/manual-family-preparation/current-preimages.tar', 'member': 'source/docs/csdlc-v3/man/man1/csdlc-rebuild.1'}, 'docs/csdlc-v3/man/man1/csdlc-recover.1': {'archive': '/Volumes/FastWork/c1223/manual-family-preparation/current-preimages.tar', 'member': 'source/docs/csdlc-v3/man/man1/csdlc-recover.1'}, 'docs/csdlc-v3/man/man1/csdlc-release-preflight.1': {'archive': '/Volumes/FastWork/c1223/manual-family-preparation/current-preimages.tar', 'member': 'source/docs/csdlc-v3/man/man1/csdlc-release-preflight.1'}, 'docs/csdlc-v3/man/man1/csdlc-remote.1': {'archive': '/Volumes/FastWork/c1223/manual-family-preparation/current-preimages.tar', 'member': 'source/docs/csdlc-v3/man/man1/csdlc-remote.1'}, 'docs/csdlc-v3/man/man1/csdlc-requests.1': {'archive': '/Volumes/FastWork/c1223/manual-family-preparation/current-preimages.tar', 'member': 'source/docs/csdlc-v3/man/man1/csdlc-requests.1'}, 'docs/csdlc-v3/man/man1/csdlc-review.1': {'archive': '/Volumes/FastWork/c1223/manual-family-preparation/current-preimages.tar', 'member': 'source/docs/csdlc-v3/man/man1/csdlc-review.1'}, 'docs/csdlc-v3/man/man1/csdlc-rollback.1': {'archive': '/Volumes/FastWork/c1223/manual-family-preparation/current-preimages.tar', 'member': 'source/docs/csdlc-v3/man/man1/csdlc-rollback.1'}, 'docs/csdlc-v3/man/man1/csdlc-schedule.1': {'archive': '/Volumes/FastWork/c1223/manual-family-preparation/current-preimages.tar', 'member': 'source/docs/csdlc-v3/man/man1/csdlc-schedule.1'}, 'docs/csdlc-v3/man/man1/csdlc-shadow.1': {'archive': '/Volumes/FastWork/c1223/manual-family-preparation/current-preimages.tar', 'member': 'source/docs/csdlc-v3/man/man1/csdlc-shadow.1'}, 'docs/csdlc-v3/man/man1/csdlc-shepherd.1': {'archive': '/Volumes/FastWork/c1223/manual-family-preparation/current-preimages.tar', 'member': 'source/docs/csdlc-v3/man/man1/csdlc-shepherd.1'}, 'docs/csdlc-v3/man/man1/csdlc-soak.1': {'archive': '/Volumes/FastWork/c1223/manual-family-preparation/current-preimages.tar', 'member': 'source/docs/csdlc-v3/man/man1/csdlc-soak.1'}, 'docs/csdlc-v3/man/man1/csdlc-sprint.1': {'archive': '/Volumes/FastWork/c1223/manual-family-preparation/current-preimages.tar', 'member': 'source/docs/csdlc-v3/man/man1/csdlc-sprint.1'}, 'docs/csdlc-v3/man/man1/csdlc-status.1': {'archive': '/Volumes/FastWork/c1223/manual-family-preparation/current-preimages.tar', 'member': 'source/docs/csdlc-v3/man/man1/csdlc-status.1'}, 'docs/csdlc-v3/man/man1/csdlc-validate.1': {'archive': '/Volumes/FastWork/c1223/manual-family-preparation/current-preimages.tar', 'member': 'source/docs/csdlc-v3/man/man1/csdlc-validate.1'}, 'docs/csdlc-v3/man/man1/csdlc-workflow.1': {'archive': '/Volumes/FastWork/c1223/manual-family-preparation/current-preimages.tar', 'member': 'source/docs/csdlc-v3/man/man1/csdlc-workflow.1'}, 'docs/csdlc-v3/man/man1/csdlc.1': {'archive': '/Volumes/FastWork/c1223/manual-family-preparation/current-preimages.tar', 'member': 'source/docs/csdlc-v3/man/man1/csdlc.1'}, 'docs/csdlc-v3/man/manual.json': {'archive': '/Volumes/FastWork/c1223/manual-family-preparation/current-preimages.tar', 'member': 'source/docs/csdlc-v3/man/manual.json'}, 'docs/csdlc-v3/man/pvf.json': {'archive': '/Volumes/FastWork/c1223/manual-family-preparation/current-preimages.tar', 'member': 'source/docs/csdlc-v3/man/pvf.json'}, 'docs/csdlc-v3/man/render.py': {'archive': '/Volumes/FastWork/c1223/manual-family-preparation/current-preimages.tar', 'member': 'source/docs/csdlc-v3/man/render.py'}}}, {'packet': 'HISTORICAL_SKILL_FAMILY_APPLIED.json', 'sha256': '3a6219bfc27326a42dae695607b17129b98a368c3a7ce51d1ac4ccbd6ecc16f7', 'commit': 'c1db31d3589aac55eadd7225d2403167e37b69d6', 'files': 92, 'removals': 83, 'scope_sha256': '12cc657879f26ffa6f59167144e3252c4ddfcfb9b7bf53a3626a41acf7fad0b4', 'archives': [{'path': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'sha256': 'b05adfb4c9f66890d11eb13b59fdf74484a955b83c13e577868fd43adcb94701'}], 'custody': {'adl/tools/skills/adl-milestone-creator/SKILL.md': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/adl-milestone-creator/SKILL.md'}, 'adl/tools/skills/adl-milestone-creator/adl-skill.yaml': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/adl-milestone-creator/adl-skill.yaml'}, 'adl/tools/skills/adl-milestone-creator/agents/openai.yaml': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/adl-milestone-creator/agents/openai.yaml'}, 'adl/tools/skills/adl-milestone-creator/references/output-contract.md': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/adl-milestone-creator/references/output-contract.md'}, 'adl/tools/skills/docs/ADL_MILESTONE_CREATOR_SKILL_INPUT_SCHEMA.md': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/docs/ADL_MILESTONE_CREATOR_SKILL_INPUT_SCHEMA.md'}, 'adl/tools/skills/docs/CI_RUNTIME_POLICY_GUIDE.md': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/docs/CI_RUNTIME_POLICY_GUIDE.md'}, 'adl/tools/skills/docs/PLANNING_DOC_EDITOR_SKILL_INPUT_SCHEMA.md': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/docs/PLANNING_DOC_EDITOR_SKILL_INPUT_SCHEMA.md'}, 'adl/tools/skills/docs/PR_INIT_SKILL_INPUT_SCHEMA.md': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/docs/PR_INIT_SKILL_INPUT_SCHEMA.md'}, 'adl/tools/skills/docs/PR_JANITOR_SKILL_INPUT_SCHEMA.md': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/docs/PR_JANITOR_SKILL_INPUT_SCHEMA.md'}, 'adl/tools/skills/docs/PR_READY_SKILL_INPUT_SCHEMA.md': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/docs/PR_READY_SKILL_INPUT_SCHEMA.md'}, 'adl/tools/skills/docs/PR_RUN_SKILL_INPUT_SCHEMA.md': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/docs/PR_RUN_SKILL_INPUT_SCHEMA.md'}, 'adl/tools/skills/docs/PR_STACK_MANAGER_SKILL_INPUT_SCHEMA.md': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/docs/PR_STACK_MANAGER_SKILL_INPUT_SCHEMA.md'}, 'adl/tools/skills/docs/RECORDS_HYGIENE_SKILL_INPUT_SCHEMA.md': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/docs/RECORDS_HYGIENE_SKILL_INPUT_SCHEMA.md'}, 'adl/tools/skills/docs/SIP_EDITOR_SKILL_INPUT_SCHEMA.md': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/docs/SIP_EDITOR_SKILL_INPUT_SCHEMA.md'}, 'adl/tools/skills/docs/SOR_EDITOR_SKILL_INPUT_SCHEMA.md': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/docs/SOR_EDITOR_SKILL_INPUT_SCHEMA.md'}, 'adl/tools/skills/docs/SPP_EDITOR_SKILL_INPUT_SCHEMA.md': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/docs/SPP_EDITOR_SKILL_INPUT_SCHEMA.md'}, 'adl/tools/skills/docs/SPRINT_CONDUCTOR_SKILL_INPUT_SCHEMA.md': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/docs/SPRINT_CONDUCTOR_SKILL_INPUT_SCHEMA.md'}, 'adl/tools/skills/docs/SRP_EDITOR_SKILL_INPUT_SCHEMA.md': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/docs/SRP_EDITOR_SKILL_INPUT_SCHEMA.md'}, 'adl/tools/skills/docs/STP_EDITOR_SKILL_INPUT_SCHEMA.md': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/docs/STP_EDITOR_SKILL_INPUT_SCHEMA.md'}, 'adl/tools/skills/planning-doc-editor/SKILL.md': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/planning-doc-editor/SKILL.md'}, 'adl/tools/skills/planning-doc-editor/adl-skill.yaml': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/planning-doc-editor/adl-skill.yaml'}, 'adl/tools/skills/planning-doc-editor/agents/openai.yaml': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/planning-doc-editor/agents/openai.yaml'}, 'adl/tools/skills/planning-doc-editor/references/output-contract.md': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/planning-doc-editor/references/output-contract.md'}, 'adl/tools/skills/pr-finish/SKILL.md': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/pr-finish/SKILL.md'}, 'adl/tools/skills/pr-init/SKILL.md': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/pr-init/SKILL.md'}, 'adl/tools/skills/pr-init/adl-skill.yaml': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/pr-init/adl-skill.yaml'}, 'adl/tools/skills/pr-init/agents/openai.yaml': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/pr-init/agents/openai.yaml'}, 'adl/tools/skills/pr-init/references/init-playbook.md': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/pr-init/references/init-playbook.md'}, 'adl/tools/skills/pr-init/references/output-contract.md': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/pr-init/references/output-contract.md'}, 'adl/tools/skills/pr-janitor/SKILL.md': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/pr-janitor/SKILL.md'}, 'adl/tools/skills/pr-janitor/adl-skill.yaml': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/pr-janitor/adl-skill.yaml'}, 'adl/tools/skills/pr-janitor/agents/openai.yaml': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/pr-janitor/agents/openai.yaml'}, 'adl/tools/skills/pr-janitor/references/janitor-playbook.md': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/pr-janitor/references/janitor-playbook.md'}, 'adl/tools/skills/pr-janitor/references/output-contract.md': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/pr-janitor/references/output-contract.md'}, 'adl/tools/skills/pr-ready/SKILL.md': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/pr-ready/SKILL.md'}, 'adl/tools/skills/pr-ready/adl-skill.yaml': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/pr-ready/adl-skill.yaml'}, 'adl/tools/skills/pr-ready/agents/openai.yaml': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/pr-ready/agents/openai.yaml'}, 'adl/tools/skills/pr-ready/references/output-contract.md': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/pr-ready/references/output-contract.md'}, 'adl/tools/skills/pr-ready/references/ready-playbook.md': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/pr-ready/references/ready-playbook.md'}, 'adl/tools/skills/pr-run/adl-skill.yaml': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/pr-run/adl-skill.yaml'}, 'adl/tools/skills/pr-run/agents/openai.yaml': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/pr-run/agents/openai.yaml'}, 'adl/tools/skills/pr-run/references/output-contract.md': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/pr-run/references/output-contract.md'}, 'adl/tools/skills/pr-stack-manager/SKILL.md': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/pr-stack-manager/SKILL.md'}, 'adl/tools/skills/pr-stack-manager/adl-skill.yaml': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/pr-stack-manager/adl-skill.yaml'}, 'adl/tools/skills/pr-stack-manager/agents/openai.yaml': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/pr-stack-manager/agents/openai.yaml'}, 'adl/tools/skills/pr-stack-manager/references/output-contract.md': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/pr-stack-manager/references/output-contract.md'}, 'adl/tools/skills/pr-stack-manager/references/pr-stack-manager-playbook.md': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/pr-stack-manager/references/pr-stack-manager-playbook.md'}, 'adl/tools/skills/records-hygiene/SKILL.md': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/records-hygiene/SKILL.md'}, 'adl/tools/skills/records-hygiene/adl-skill.yaml': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/records-hygiene/adl-skill.yaml'}, 'adl/tools/skills/records-hygiene/agents/openai.yaml': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/records-hygiene/agents/openai.yaml'}, 'adl/tools/skills/records-hygiene/references/output-contract.md': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/records-hygiene/references/output-contract.md'}, 'adl/tools/skills/records-hygiene/references/records-hygiene-playbook.md': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/records-hygiene/references/records-hygiene-playbook.md'}, 'adl/tools/skills/sip-editor/SKILL.md': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/sip-editor/SKILL.md'}, 'adl/tools/skills/sip-editor/adl-skill.yaml': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/sip-editor/adl-skill.yaml'}, 'adl/tools/skills/sip-editor/agents/openai.yaml': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/sip-editor/agents/openai.yaml'}, 'adl/tools/skills/sip-editor/references/edit-playbook.md': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/sip-editor/references/edit-playbook.md'}, 'adl/tools/skills/sip-editor/references/output-contract.md': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/sip-editor/references/output-contract.md'}, 'adl/tools/skills/sor-editor/SKILL.md': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/sor-editor/SKILL.md'}, 'adl/tools/skills/sor-editor/adl-skill.yaml': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/sor-editor/adl-skill.yaml'}, 'adl/tools/skills/sor-editor/agents/openai.yaml': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/sor-editor/agents/openai.yaml'}, 'adl/tools/skills/sor-editor/references/edit-playbook.md': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/sor-editor/references/edit-playbook.md'}, 'adl/tools/skills/sor-editor/references/output-contract.md': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/sor-editor/references/output-contract.md'}, 'adl/tools/skills/spp-editor/SKILL.md': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/spp-editor/SKILL.md'}, 'adl/tools/skills/spp-editor/adl-skill.yaml': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/spp-editor/adl-skill.yaml'}, 'adl/tools/skills/spp-editor/agents/openai.yaml': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/spp-editor/agents/openai.yaml'}, 'adl/tools/skills/spp-editor/references/edit-playbook.md': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/spp-editor/references/edit-playbook.md'}, 'adl/tools/skills/spp-editor/references/output-contract.md': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/spp-editor/references/output-contract.md'}, 'adl/tools/skills/sprint-conductor/SKILL.md': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/sprint-conductor/SKILL.md'}, 'adl/tools/skills/sprint-conductor/adl-skill.yaml': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/sprint-conductor/adl-skill.yaml'}, 'adl/tools/skills/sprint-conductor/agents/openai.yaml': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/sprint-conductor/agents/openai.yaml'}, 'adl/tools/skills/sprint-conductor/references/conductor-playbook.md': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/sprint-conductor/references/conductor-playbook.md'}, 'adl/tools/skills/sprint-conductor/references/output-contract.md': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/sprint-conductor/references/output-contract.md'}, 'adl/tools/skills/sprint-conductor/scripts/validate_review_subagent_policy.py': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/sprint-conductor/scripts/validate_review_subagent_policy.py'}, 'adl/tools/skills/srp-editor/SKILL.md': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/srp-editor/SKILL.md'}, 'adl/tools/skills/srp-editor/adl-skill.yaml': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/srp-editor/adl-skill.yaml'}, 'adl/tools/skills/srp-editor/agents/openai.yaml': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/srp-editor/agents/openai.yaml'}, 'adl/tools/skills/srp-editor/references/edit-playbook.md': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/srp-editor/references/edit-playbook.md'}, 'adl/tools/skills/srp-editor/references/output-contract.md': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/srp-editor/references/output-contract.md'}, 'adl/tools/skills/stp-editor/SKILL.md': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/stp-editor/SKILL.md'}, 'adl/tools/skills/stp-editor/adl-skill.yaml': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/stp-editor/adl-skill.yaml'}, 'adl/tools/skills/stp-editor/agents/openai.yaml': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/stp-editor/agents/openai.yaml'}, 'adl/tools/skills/stp-editor/references/edit-playbook.md': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/stp-editor/references/edit-playbook.md'}, 'adl/tools/skills/stp-editor/references/output-contract.md': {'archive': '/Volumes/FastWork/c1223/csdlc-historical83-preparation/current-preimages.tar', 'member': 'source/adl/tools/skills/stp-editor/references/output-contract.md'}}}]


def check_recent_owner_family(batch, before_tree, after_tree, prior_removed, spec):
    rows = batch['files']
    removed = [r for r in rows if r['action'] == 'remove']
    paths = {r['path'] for r in removed}
    digest = hashlib.sha256(json.dumps(sorted(paths), separators=(',', ':')).encode()).hexdigest()
    require(len(rows) == len({r['path'] for r in rows}) == spec['files'] and
            len(removed) == len(paths) == spec['removals'] and
            digest == spec['scope_sha256'] and not paths.intersection(prior_removed),
            'Recent owner exact scope changed')
    require(not any(p.startswith(('.csdlc/', 'csdlc-v3/operator/', 'docs/templates/'))
                    or p in PROTECTED for p in paths), 'Recent owner protected removal')
    for row in rows:
        before, after, path = row['before'], row['after'], row['path']
        require(row['action'] in ('remove', 'modify', 'add') and
                ((row['action'] == 'remove' and before is not None and after is None) or
                 (row['action'] == 'modify' and before is not None and after is not None) or
                 (row['action'] == 'add' and before is None and after is not None)),
                'Recent owner invalid transition')
        require(before_tree.get(path) == ([before['mode'], before['blob']] if before else None) and
                after_tree.get(path) == ([after['mode'], after['blob']] if after else None),
                'Recent owner applied identity mismatch')
    return paths


def check_recent_owner_custody(batch, spec):
    removed = {r['path']: r for r in batch['files'] if r['action'] == 'remove'}
    require(set(spec['custody']) == set(removed), 'Recent owner custody scope changed')
    for archive in spec['archives']:
        path = Path(archive['path'])
        require(hashlib.sha256(path.read_bytes()).hexdigest() == archive['sha256'],
                'Recent owner archive changed')
        with tarfile.open(path) as tar:
            for source, custody in spec['custody'].items():
                if custody['archive'] != archive['path']:
                    continue
                before = removed[source]['before']
                member = tar.getmember(custody['member'])
                require(member.isfile() and member.mode == int(before['mode'][-3:], 8) and
                        hashlib.sha256(tar.extractfile(member).read()).hexdigest() == before['sha256'],
                        'Recent owner custody identity mismatch')

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
    batch_bytes = committed_bytes(PACKET / 'CODEFRIEND_SIX_HELPER_APPLIED.json')
    require(hashlib.sha256(batch_bytes).hexdigest() == CODEFRIEND_BATCH_SHA256,
            'CodeFriend applied packet changed')
    batch = json.loads(batch_bytes)
    removed.update(check_codefriend_batch(batch, tree(CODEFRIEND_IMPLEMENTATION + '^'),
                                         tree(CODEFRIEND_IMPLEMENTATION), removed))
    family_bytes = committed_bytes(PACKET / 'CODEFRIEND_FAMILY_APPLIED.json')
    require(hashlib.sha256(family_bytes).hexdigest() == CODEFRIEND_FAMILY_SHA256,
            'CodeFriend family applied packet changed')
    family = json.loads(family_bytes)
    removed.update(check_codefriend_family(family, tree(CODEFRIEND_FAMILY_IMPLEMENTATION + '^'),
                                          tree(CODEFRIEND_FAMILY_IMPLEMENTATION), removed))
    csdlc_bytes = committed_bytes(PACKET / 'CSDLC_SOURCE_FAMILY_APPLIED.json')
    require(hashlib.sha256(csdlc_bytes).hexdigest() == CSDLC_FAMILY_SHA256,
            'C-SDLC family applied packet changed')
    csdlc = json.loads(csdlc_bytes)
    removed.update(check_csdlc_family(csdlc, tree(CSDLC_FAMILY_IMPLEMENTATION + '^'),
                                    tree(CSDLC_FAMILY_IMPLEMENTATION), removed))
    runtime_bytes = committed_bytes(PACKET / 'RUNTIME_SOURCE_FAMILY_APPLIED.json')
    require(hashlib.sha256(runtime_bytes).hexdigest() == RUNTIME_FAMILY_SHA256,
            'Runtime family applied packet changed')
    runtime = json.loads(runtime_bytes)
    removed.update(check_runtime_family(runtime, tree(RUNTIME_FAMILY_IMPLEMENTATION + '^'),
                                      tree(RUNTIME_FAMILY_IMPLEMENTATION), removed))
    infrastructure_bytes = committed_bytes(PACKET / 'INFRASTRUCTURE_SOURCE_FAMILY_APPLIED.json')
    require(hashlib.sha256(infrastructure_bytes).hexdigest() == INFRASTRUCTURE_FAMILY_SHA256,
            'Infrastructure family applied packet changed')
    infrastructure = json.loads(infrastructure_bytes)
    removed.update(check_infrastructure_family(infrastructure, tree(INFRASTRUCTURE_FAMILY_IMPLEMENTATION + '^'),
                                               tree(INFRASTRUCTURE_FAMILY_IMPLEMENTATION), removed))
    for spec in CONVERSION_SKILLS_BATCHES:
        batch_bytes = committed_bytes(PACKET / spec['packet'])
        require(hashlib.sha256(batch_bytes).hexdigest() == spec['sha256'],
                'Conversion/skills applied packet changed')
        conversion_skills_batch = json.loads(batch_bytes)
        removed.update(check_conversion_skills_family(conversion_skills_batch, tree(spec['commit'] + '^'),
                                                      tree(spec['commit']), removed, spec))
        check_conversion_skills_custody(conversion_skills_batch, spec)
    for spec in RECENT_OWNER_BATCHES:
        applied_bytes = committed_bytes(PACKET / spec['packet'])
        require(hashlib.sha256(applied_bytes).hexdigest() == spec['sha256'],
                'Recent owner applied packet changed')
        applied_batch = json.loads(applied_bytes)
        removed.update(check_recent_owner_family(applied_batch, tree(spec['commit'] + '^'),
                                                tree(spec['commit']), removed, spec))
        check_recent_owner_custody(applied_batch, spec)
    require(len(removed) == 2576, 'executed removal count changed')

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
    archive_path = metadata / 'csdlc-v3/local/evidence/1223/retirement/baseline.tar.gz'
    check_rollback(archive_path)
    with tarfile.open(archive_path, 'r:gz') as archive:
        for row in batch['removed']:
            member = archive.getmember(row['rollback']['archive_member'])
            require(member.isfile() and member.mode == int(row['rollback']['mode'], 8),
                    'CodeFriend archive member mode changed')
            require(hashlib.sha256(archive.extractfile(member).read()).hexdigest() == row['before']['sha256'],
                    'CodeFriend archive member bytes changed')
    supplement = family['rollback_custody']['supplement']
    supplement_path = Path(supplement['path'])
    require(hashlib.sha256(supplement_path.read_bytes()).hexdigest() == supplement['sha256'],
            'CodeFriend family supplement changed')
    family_rows = {r['path']: r for r in family['files'] if r['action'] == 'remove'}
    with tarfile.open(archive_path, 'r:gz') as original, tarfile.open(supplement_path) as extra:
        for custody in family['rollback_custody']['rows']:
            archive = original if custody['current_bytes_match_archive'] else extra
            member = archive.getmember('source/' + custody['path'])
            before = family_rows[custody['path']]['before']
            require(member.isfile() and member.mode == int(before['mode'][-3:], 8),
                    'CodeFriend family archive mode changed')
            require(hashlib.sha256(archive.extractfile(member).read()).hexdigest() == before['sha256'],
                    'CodeFriend family archive bytes changed')
    check_csdlc_custody(csdlc, archive_path)
    check_runtime_custody(runtime, archive_path)
    check_infrastructure_custody(infrastructure)
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
            'candidate_rows': 4134, 'existing_branch_removals': 34, 'approved_component_removals': 36, 'public_cutover_removals': 1, 'codefriend_helper_removals': 6, 'codefriend_family_removals': 198, 'csdlc_family_removals': 815, 'runtime_family_removals': 896, 'infrastructure_family_removals': 212, 'csdlc_skill_family_removals': 83, 'aws_remote_family_removals': 14, 'planning_helper_removals': 9, 'manual_family_removals': 56, 'historical_family_removals': 83, 'additional_inventory_paths': 22, 'executed_retirement_actions': len(removed), 'baseline_removals': len(removed) - 1, 'transient_removals': 1, 'total_removals': len(removed), 'direct_reference_files': scanned,
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
