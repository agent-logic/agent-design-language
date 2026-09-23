#!/usr/bin/env python3
"""Deterministic private draft packet checks. Never grants release approval."""
import argparse
import copy
import hashlib
from functools import lru_cache
import json
import re
import subprocess
from pathlib import Path, PurePosixPath

ROOT = Path(__file__).resolve().parents[5]
PACKET = Path(__file__).resolve().parent
INPUTS = {
    'issue917': ('c10757270098aea35e36453c91054ea8b4f947db', 1152),
    'issue916': ('e119223cd13ebbb6a07c0349ba6772f8ee2ecece', 1151),
}

DRAFT_BASE = "3c6ebcb1e42ee993a1bc9152857c3008be655181"
DRAFT_IDS = {"release_notes", "review_guide", "destination_inventory", "version_inventory"}
ARTIFACT_PATHS = {'article_inventory': 'docs/milestones/v0.92/publication/articles/medium-2026-09/README.md',
 'cargo_audit': 'docs/milestones/v0.92.2/evidence/issue-917/CARGO_MANIFEST_AUDIT.json',
 'deferral': 'docs/milestones/v0.92.2/evidence/issue-916/SPRINT10_DEFERRAL.json',
 'destination_inventory': 'docs/milestones/v0.92.2/evidence/issue-918/DESTINATIONS.json',
 'manuscript_custody': 'docs/milestones/v0.92.2/cognitive-sdlc/REVISION_4_REVIEW_HANDOFF.md',
 'quality_decision': 'docs/milestones/v0.92.2/evidence/issue-916/QUALITY_DECISION.json',
 'release_notes': 'docs/milestones/v0.92.2/RELEASE_NOTES_v0.92.2.md',
 'review_guide': 'docs/milestones/v0.92.2/evidence/issue-918/REVIEW_GUIDE.md',
 'task_ledger': 'docs/milestones/v0.92.2/evidence/issue-916/TASK_LEDGER.json',
 'upstream_handoff': 'docs/milestones/v0.92.2/evidence/issue-917/HANDOFF.md',
 'upstream_manifest': 'docs/milestones/v0.92.2/evidence/issue-917/HANDOFF_MANIFEST.json',
 'version_inventory': 'docs/milestones/v0.92.2/evidence/issue-918/VERSION_INVENTORY.json'}


def check(data, root=ROOT):
    failures = []
    if not isinstance(data, dict):
        return ['manifest_object_required']
    expected = {'schema': 'adl.v0922.publication_packet.v1', 'issue': 918,
                'status': 'draft_for_external_review', 'release_approved': False,
                'publication_authorized': False, 'final_acceptance': 'pending'}
    for key, value in expected.items():
        if data.get(key) != value or type(data.get(key)) is not type(value):
            failures.append('invalid_' + key)
    inputs = data.get('inputs', {})
    revisions = set()
    for key, (revision, pr) in INPUTS.items():
        row = inputs.get(key, {}) if isinstance(inputs, dict) else {}
        if not isinstance(row, dict):
            row = {}
        if row.get('revision') != revision or row.get('pr') != pr:
            failures.append('wrong_candidate_' + key)
        revisions.add(revision)
        field, value = ('acceptance', 'pending') if key == 'issue917' else ('decision', 'not_proven')
        if row.get(field) != value:
            failures.append('invalid_' + key + '_' + field)
    qualification = data.get('deferred_qualification', {})
    if not isinstance(qualification, dict) or any([
        qualification.get('status') != 'incomplete',
        qualification.get('successors') != [1148, 1149, 1150],
        qualification.get('depends_on') != {'1150': [1148, 1149]},
        qualification.get('gate') != 'before_beta1_launch',
    ]):
        failures.append('qualification_gate_invalid')
    versions = data.get('versions', {})
    if not isinstance(versions, dict) or versions.get('milestone') != 'v0.92.2' or versions.get('package_policy') != 'preserve_declared_versions_no_binary_relabeling':
        failures.append('version_policy_invalid')
    exclusions = data.get('exclusions')
    if not isinstance(exclusions, list) or not exclusions:
        failures.append('exclusions_required')
    elif any(not isinstance(row, dict) or not all(isinstance(row.get(key), str) and row[key].strip() for key in ['id', 'reason', 'status']) for row in exclusions):
        failures.append('exclusion_description_required')
    artifacts = data.get('artifacts')
    if not isinstance(artifacts, list) or not artifacts:
        return failures + ['artifacts_required']
    ids, paths = set(), set()
    for row in artifacts:
        if not isinstance(row, dict):
            failures.append('artifact_object_required')
            continue
        identity, name = row.get('id'), row.get('path')
        if not isinstance(identity, str) or not identity.strip() or identity in ids:
            failures.append('artifact_identity_invalid')
        else:
            ids.add(identity)
        if not isinstance(name, str) or not name or '\\' in name or PurePosixPath(name).is_absolute() or '..' in PurePosixPath(name).parts:
            failures.append('artifact_path_invalid')
            continue
        if identity not in ARTIFACT_PATHS or ARTIFACT_PATHS.get(identity) != name or row.get('role') != identity:
            failures.append('artifact_mapping_invalid')
        if row.get('format') != PurePosixPath(name).suffix.lstrip('.'):
            failures.append('artifact_format_invalid')
        path = (root / name).resolve()
        if not path.is_relative_to(root.resolve()):
            failures.append('artifact_path_escape')
            continue
        if path in paths:
            failures.append('artifact_path_duplicate')
        paths.add(path)
        if not path.is_file():
            failures.append('artifact_missing')
        else:
            content = path.read_bytes()
            if type(row.get('bytes')) is not int or row['bytes'] != len(content):
                failures.append('artifact_size_mismatch')
            if row.get('sha256') != hashlib.sha256(content).hexdigest():
                failures.append('artifact_hash_mismatch')
        for key in ['format', 'role']:
            if not isinstance(row.get(key), str) or not row[key].strip():
                failures.append('artifact_' + key + '_required')
        for key, value in [('approval', 'pending'), ('confidentiality', 'repository_private'), ('destination', 'private_repository_review')]:
            if row.get(key) != value:
                failures.append('artifact_' + key + '_invalid')
        revision = row.get('source_revision')
        if not isinstance(revision, str) or not re.fullmatch('[0-9a-f]{40}', revision):
            failures.append('artifact_source_revision_invalid')
        else:
            revisions.add(revision)
        if identity in DRAFT_IDS:
            if revision != DRAFT_BASE:
                failures.append('draft_source_revision_invalid')
            if row.get('provenance_basis') != 'drafting base plus exact candidate SHA256':
                failures.append('draft_provenance_invalid')
        else:
            if revision != INPUTS['issue917'][0]:
                failures.append('inherited_source_revision_invalid')
            if row.get('provenance_basis') != 'inherited source checkpoint':
                failures.append('inherited_provenance_invalid')
            if isinstance(revision, str) and re.fullmatch('[0-9a-f]{40}', revision):
                source = git_blob(root, revision, name)
                if source is None:
                    failures.append('inherited_source_blob_missing')
                elif row.get('sha256') != hashlib.sha256(source).hexdigest() or row.get('bytes') != len(source):
                    failures.append('inherited_source_bytes_mismatch')
    if ids != set(ARTIFACT_PATHS):
        failures.append('required_artifacts_missing')
    for revision in sorted(revisions):
        result = subprocess.run(['git', 'cat-file', '-t', revision], cwd=root, capture_output=True, text=True)
        if result.returncode != 0 or result.stdout.strip() != 'commit':
            failures.append('source_commit_unavailable')
    failures.extend(check_upstream(root))
    return failures


@lru_cache(maxsize=256)
def git_blob(root, revision, path):
    result = subprocess.run(['git', 'show', revision + ':' + path], cwd=root, capture_output=True)
    return result.stdout if result.returncode == 0 else None


@lru_cache(maxsize=1)
def check_upstream(root):
    # Check immutable predecessor bytes at its reviewed revision, not changed draft files.
    revision = INPUTS['issue917'][0]
    def blob(path):
        return subprocess.check_output(['git', 'show', revision + ':' + path], cwd=root, stderr=subprocess.DEVNULL)
    try:
        manifest = json.loads(blob('docs/milestones/v0.92.2/evidence/issue-917/HANDOFF_MANIFEST.json'))
        documents = manifest['documents']
        if not documents or len({row['path'] for row in documents}) != len(documents):
            return ['upstream_inventory_invalid']
        for row in documents:
            content = blob(row['path'])
            if hashlib.sha256(content).hexdigest() != row['sha256']:
                return ['upstream_hash_mismatch']
        return []
    except (subprocess.CalledProcessError, ValueError, KeyError, TypeError):
        return ['upstream_inventory_unavailable']


def self_test(data):
    """Mutate only memory; each fixture must trip the intended rejection."""
    cases = []
    def case(name, mutate, expected):
        candidate = copy.deepcopy(data)
        mutate(candidate)
        actual = check(candidate)
        cases.append((name, expected in actual))
    case('missing_artifact', lambda d: d['artifacts'][0].update(path='docs/issue918-does-not-exist'), 'artifact_missing')
    case('tampered_hash', lambda d: d['artifacts'][0].update(sha256='0' * 64), 'artifact_hash_mismatch')
    case('wrong_size', lambda d: d['artifacts'][0].update(bytes=-1), 'artifact_size_mismatch')
    case('wrong_candidate', lambda d: d['inputs']['issue917'].update(revision=INPUTS['issue916'][0]), 'wrong_candidate_issue917')
    case('release_approval', lambda d: d.update(release_approved=True), 'invalid_release_approved')
    case('publication_approval', lambda d: d.update(publication_authorized=True), 'invalid_publication_authorized')
    case('artifact_approval', lambda d: d['artifacts'][0].update(approval='approved'), 'artifact_approval_invalid')
    case('accepted_predecessor', lambda d: d['inputs']['issue917'].update(acceptance='accepted'), 'invalid_issue917_acceptance')
    case('path_traversal', lambda d: d['artifacts'][0].update(path='../outside'), 'artifact_path_invalid')
    case('absolute_path', lambda d: d['artifacts'][0].update(path=str(ROOT / 'README.md')), 'artifact_path_invalid')
    case('duplicate_inventory', lambda d: d['artifacts'].append(copy.deepcopy(d['artifacts'][0])), 'artifact_identity_invalid')
    case('duplicate_path', lambda d: d['artifacts'].append(dict(d['artifacts'][0], id='different-id')), 'artifact_path_duplicate')
    case('missing_dependency', lambda d: d['deferred_qualification'].update(depends_on={'1150': [1148]}), 'qualification_gate_invalid')
    case('public_destination', lambda d: d['artifacts'][0].update(destination='public'), 'artifact_destination_invalid')
    case('unknown_source', lambda d: d['artifacts'][0].update(source_revision='0' * 40), 'source_commit_unavailable')
    case('required_artifact_removed', lambda d: d.update(artifacts=[row for row in d['artifacts'] if row['id'] != 'release_notes']), 'required_artifacts_missing')
    def inherited(d):
        return next(row for row in d['artifacts'] if row['id'] == 'upstream_manifest')
    case('wrong_inherited_source', lambda d: inherited(d).update(source_revision=INPUTS['issue916'][0]), 'inherited_source_revision_invalid')
    case('missing_inherited_blob', lambda d: inherited(d).update(path='docs/issue918-missing-source.json'), 'inherited_source_blob_missing')
    case('inherited_bytes', lambda d: inherited(d).update(sha256='0' * 64), 'inherited_source_bytes_mismatch')
    case('wrong_draft_base', lambda d: d['artifacts'][0].update(source_revision=INPUTS['issue917'][0]), 'draft_source_revision_invalid')
    case('draft_provenance', lambda d: d['artifacts'][0].update(provenance_basis='inherited source checkpoint'), 'draft_provenance_invalid')
    case('inherited_provenance', lambda d: inherited(d).update(provenance_basis='drafting base plus exact candidate SHA256'), 'inherited_provenance_invalid')
    def swap_identity(d):
        left = next(row for row in d['artifacts'] if row['id'] == 'release_notes')
        right = next(row for row in d['artifacts'] if row['id'] == 'review_guide')
        left['id'], right['id'] = right['id'], left['id']
        left['role'], right['role'] = right['role'], left['role']
    case('swapped_identity', swap_identity, 'artifact_mapping_invalid')
    case('wrong_role', lambda d: d['artifacts'][0].update(role='review_guide'), 'artifact_mapping_invalid')
    case('empty_inventory', lambda d: d.update(artifacts=[]), 'artifacts_required')
    return cases


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--self-test', action='store_true')
    args = parser.parse_args()
    try:
        data = json.loads((PACKET / 'PUBLICATION_MANIFEST.json').read_text())
        failures = check(data)
        cases = self_test(data) if args.self_test and not failures else []
        failures.extend('negative_fixture_failed:' + name for name, passed in cases if not passed)
        result = {'status': 'fail' if failures else 'pass', 'failures': failures,
                  'artifacts': len(data.get('artifacts', [])), 'negative_fixtures': len(cases),
                  'release_approved': False, 'publication_authorized': False,
                  'final_acceptance': 'pending'}
    except (OSError, ValueError, TypeError, KeyError) as error:
        result = {'status': 'fail', 'failures': [type(error).__name__], 'negative_fixtures': 0}
    print(json.dumps(result, indent=2))
    return 1 if result['status'] == 'fail' else 0


if __name__ == '__main__':
    raise SystemExit(main())
