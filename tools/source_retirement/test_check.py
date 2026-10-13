import copy
import importlib.util
import json
from pathlib import Path
import subprocess
import tempfile
import unittest

HERE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('retirement_check', HERE / 'check.py')
check = importlib.util.module_from_spec(spec)
spec.loader.exec_module(check)


class ExecutionOverlayTests(unittest.TestCase):
    def setUp(self):
        self.before = ['100644', 'a' * 40]
        self.after = ['100755', 'b' * 40]
        self.baseline = {'retained': self.before, 'removed': self.before, 'held': self.before}
        self.candidate = {'retained': self.after, 'held': self.before}
        self.manifest = {'schema': 'adl.source_retirement.execution_overlay.v1', 'issue': 1223,
                         'status': 'complete', 'missing_entries': [], 'entries': [
            {'path': 'retained', 'action': 'modify', 'before': self.before, 'after': self.after,
             'review_reference': 'reviewed fixture', 'implementation_commit': 'fixture', 'approval_reference': 'fixture'}]}

    def validate(self):
        return check.check_execution_overlay(self.manifest, self.baseline, self.candidate, {'removed'}, set(self.baseline))

    def test_exact_retained_repair_and_approved_deletion(self):
        self.assertEqual(self.validate(), ['removed'])

    def test_incomplete_manifest_refuses(self):
        self.manifest['status'] = 'incomplete'
        self.manifest['missing_entries'] = ['reviewed repair missing']
        with self.assertRaisesRegex(ValueError, 'incomplete'):
            self.validate()

    def test_stale_before_identity_refuses(self):
        self.manifest['entries'][0]['before'] = self.after
        with self.assertRaisesRegex(ValueError, 'before identity'):
            self.validate()

    def test_unreviewed_provenance_refuses(self):
        self.manifest['entries'][0]['review_reference'] = ''
        with self.assertRaisesRegex(ValueError, 'provenance'):
            self.validate()

    def test_duplicate_path_refuses(self):
        self.manifest['entries'] *= 2
        with self.assertRaisesRegex(ValueError, 'duplicate'):
            self.validate()

    def test_later_unapproved_change_or_mode_refuses(self):
        for identity in (self.before, ['100644', self.after[1]]):
            self.candidate['retained'] = identity
            with self.assertRaisesRegex(ValueError, 'exceeds execution identity'):
                self.validate()

    def test_conditional_path_cannot_be_deleted(self):
        del self.candidate['held']
        with self.assertRaisesRegex(ValueError, 'deletion outside'):
            self.validate()

    def test_extra_deletion_outside_inventory_refuses(self):
        self.baseline['outside'] = self.before
        with self.assertRaisesRegex(ValueError, 'deletion outside'):
            self.validate()

    def test_removed_path_cannot_reappear(self):
        self.candidate['removed'] = self.before
        with self.assertRaisesRegex(ValueError, 'deletion outside'):
            self.validate()

    def test_proposal_cannot_authorize_removal(self):
        self.manifest['entries'].append({'path': 'held', 'action': 'remove', 'before': self.before,
                                        'after': None, 'review_reference': 'proposal',
                                        'implementation_commit': 'fixture', 'approval_reference': 'proposal',
                                        'rollback_custody': 'fixture'})
        with self.assertRaisesRegex(ValueError, 'unapproved removal'):
            self.validate()

    def test_explicit_removal_requires_custody(self):
        self.manifest['entries'].append({'path': 'removed', 'action': 'remove', 'before': self.before,
                                        'after': None, 'review_reference': 'review',
                                        'implementation_commit': 'fixture', 'approval_reference': 'approval'})
        with self.assertRaisesRegex(ValueError, 'missing custody'):
            self.validate()


class InventoryAndCustodyTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.original = check.committed_json(check.PACKET / 'DISPOSITION_REGISTER.json')
        cls.combined = check.committed_json(check.PACKET / 'COMBINED_REMAINING_DELTA.json')
        cls.groups = check.committed_json(check.PACKET / 'GROUPED_REMAINING_HOLDS.json')
        cls.frozen = check.tree(cls.original['original_revision'])
        cls.observed = check.tree(cls.original['observed_main'])

    def validate(self, combined=None, groups=None):
        check.check_combined(self.original, combined or self.combined, groups or self.groups, self.frozen, self.observed)

    def test_current_committed_denominator_and_groups(self):
        self.validate()

    def test_reordered_inventory_refuses(self):
        data = copy.deepcopy(self.combined)
        data['rows'][0], data['rows'][1] = data['rows'][1], data['rows'][0]
        with self.assertRaisesRegex(ValueError, 'captured current identities|ordered inventory'):
            self.validate(combined=data)

    def test_additional_overlap_refuses(self):
        data = copy.deepcopy(self.combined)
        data['additional_paths'][0]['path'] = data['rows'][0]['path']
        with self.assertRaisesRegex(ValueError, 'captured current identities|additional inventory'):
            self.validate(combined=data)

    def test_original_false_approval_refuses(self):
        for field, value in [('new_removals_authorized', False), ('all_holds_resolved', True)]:
            original = dict(self.original, **{field: value})
            with self.assertRaisesRegex(ValueError, 'approval flags'):
                check.check_combined(original, self.combined, self.groups, self.frozen, self.observed)

    def test_original_and_additional_captured_identity_refuse_tampering(self):
        for group in ('rows', 'additional_paths'):
            for field, value in [('mode', '100755'), ('blob', '0' * 40)]:
                data = copy.deepcopy(self.combined)
                identity = data[group][0]['current_identity']
                identity[field] = value if identity[field] != value else '100644'
                with self.assertRaisesRegex(ValueError, 'captured current identities'):
                    self.validate(combined=data)

    def test_incomplete_hold_group_refuses(self):
        data = copy.deepcopy(self.groups)
        data['groups'][0]['rows'].pop()
        with self.assertRaisesRegex(ValueError, 'held groups'):
            self.validate(groups=data)

    def test_missing_and_corrupt_rollback_refuse(self):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / 'archive.tar.gz'
            with self.assertRaisesRegex(ValueError, 'archive missing'):
                check.check_rollback(path)
            path.write_bytes(b'not the preserved archive')
            with self.assertRaisesRegex(ValueError, 'digest mismatch'):
                check.check_rollback(path)

    def test_operational_reference_not_historical(self):
        with self.assertRaisesRegex(ValueError, 'unclassified'):
            check.check_references(['adl/src/live.rs'], ['.csdlc/issues/594/cards/sor.md'])

    def test_committed_input_ignores_dirty_override(self):
        old_root = check.ROOT
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            def git(*args):
                subprocess.run(['git', '-C', str(root), *args], check=True, capture_output=True)
            git('init', '-q')
            (root / 'input.json').write_text('{"allowed": []}')
            git('add', 'input.json')
            git('-c', 'user.name=Retirement fixture', '-c', 'user.email=fixture@example.invalid',
                '-c', 'commit.gpgsign=false', 'commit', '-qm', 'fixture')
            (root / 'input.json').write_text('{"allowed": ["adl/src/live.rs"]}')
            try:
                check.ROOT = root
                self.assertEqual(check.committed_json('input.json'), {'allowed': []})
            finally:
                check.ROOT = old_root


class CodeFriendBatchTests(unittest.TestCase):
    def setUp(self):
        self.batch = check.committed_json(check.PACKET / 'CODEFRIEND_SIX_HELPER_APPLIED.json')
        self.before = check.tree(check.CODEFRIEND_IMPLEMENTATION + '^')
        self.after = check.tree(check.CODEFRIEND_IMPLEMENTATION)

    def test_exact_batch_and_scope_refusal(self):
        self.assertEqual(check.check_codefriend_batch(self.batch, self.before, self.after, set()),
                         set(check.CODEFRIEND_REMOVALS.values()))
        self.batch['removed'][0]['id'] = 'RD07-1288'
        with self.assertRaisesRegex(ValueError, 'scope changed'):
            check.check_codefriend_batch(self.batch, self.before, self.after, set())

    def test_preimage_and_custody_refusal(self):
        for kind in ('mode', 'archive_member_sha256'):
            batch = copy.deepcopy(self.batch)
            if kind == 'mode':
                batch['removed'][0]['before']['mode'] = '100755'
            else:
                batch['removed'][0]['rollback'][kind] = '0' * 64
            with self.assertRaisesRegex(ValueError, 'identity mismatch'):
                check.check_codefriend_batch(batch, self.before, self.after, set())


class CodeFriendFamilyTests(unittest.TestCase):
    def setUp(self):
        self.batch = check.committed_json(check.PACKET / 'CODEFRIEND_FAMILY_APPLIED.json')
        self.before = check.tree(check.CODEFRIEND_FAMILY_IMPLEMENTATION + '^')
        self.after = check.tree(check.CODEFRIEND_FAMILY_IMPLEMENTATION)

    def test_exact_family_and_repeated_scope_refusal(self):
        paths = check.check_codefriend_family(self.batch, self.before, self.after, set())
        self.assertEqual(len(paths), 198)
        with self.assertRaisesRegex(ValueError, 'scope changed'):
            check.check_codefriend_family(self.batch, self.before, self.after, paths)

    def test_family_preimage_and_custody_refusal(self):
        row = next(r for r in self.batch['files'] if r['action'] == 'remove')
        changed_before = dict(self.before)
        changed_before[row['path']] = ['100755', row['before']['blob']]
        with self.assertRaisesRegex(ValueError, 'identity mismatch'):
            check.check_codefriend_family(self.batch, changed_before, self.after, set())
        self.batch['rollback_custody']['rows'][0]['preimage_sha256'] = '0' * 64
        with self.assertRaisesRegex(ValueError, 'custody identity mismatch'):
            check.check_codefriend_family(self.batch, self.before, self.after, set())



class CsdlcFamilyTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.batch = check.committed_json(check.PACKET / 'CSDLC_SOURCE_FAMILY_APPLIED.json')
        cls.before = check.tree(check.CSDLC_FAMILY_IMPLEMENTATION + '^')
        cls.after = check.tree(check.CSDLC_FAMILY_IMPLEMENTATION)

    def test_exact_scope_and_duplicate_removal_refusal(self):
        paths = check.check_csdlc_family(self.batch, self.before, self.after, set())
        self.assertEqual(len(paths), 815)
        with self.assertRaisesRegex(ValueError, 'removal scope changed'):
            check.check_csdlc_family(self.batch, self.before, self.after, paths)

    def test_protected_authority_and_history_refuse(self):
        for path in ('csdlc-v3/operator/authority-selector.json',
                     'csdlc-v2/operator/skills/retained.md',
                     '.csdlc/evidence/872/old-binary', 'docs/templates/prompts/current.json'):
            batch = copy.deepcopy(self.batch)
            next(r for r in batch['files'] if r['action'] == 'remove')['path'] = path
            with self.assertRaisesRegex(ValueError, 'protected path'):
                check.check_csdlc_family(batch, self.before, self.after, set())

    def test_preimage_and_custody_tamper_refuse(self):
        row = next(r for r in self.batch['files'] if r['action'] == 'remove')
        before = dict(self.before)
        before[row['path']] = ['100755', '0' * 40]
        with self.assertRaisesRegex(ValueError, 'applied identity mismatch'):
            check.check_csdlc_family(self.batch, before, self.after, set())
        batch = copy.deepcopy(self.batch)
        batch['rollback']['rows'][0]['sha256'] = '0' * 64
        with self.assertRaisesRegex(ValueError, 'custody identity mismatch'):
            check.check_csdlc_family(batch, self.before, self.after, set())
        batch['rollback']['rows'][0]['id'] = 'RD07-9999'
        with self.assertRaisesRegex(ValueError, 'custody scope changed'):
            check.check_csdlc_family(batch, self.before, self.after, set())

    def test_substituted_archive_refuses_before_readback(self):
        batch = copy.deepcopy(self.batch)
        with tempfile.TemporaryDirectory() as tmp:
            fake = Path(tmp) / 'substituted.tar'
            fake.write_bytes(b'not the pinned archive')
            batch['rollback']['supplement']['path'] = str(fake)
            with self.assertRaisesRegex(ValueError, 'supplement changed'):
                check.check_csdlc_custody(batch, Path(tmp) / 'unused-original.tar.gz')


class RuntimeFamilyTests(unittest.TestCase):
    # PVF: deterministic source guard; local lightweight unit proof, no owner execution.
    @classmethod
    def setUpClass(cls):
        cls.batch = check.committed_json(check.PACKET / 'RUNTIME_SOURCE_FAMILY_APPLIED.json')
        cls.before = check.tree(check.RUNTIME_FAMILY_IMPLEMENTATION + '^')
        cls.after = check.tree(check.RUNTIME_FAMILY_IMPLEMENTATION)

    def test_exact_scope_and_repeat_refusal(self):
        paths = check.check_runtime_family(self.batch, self.before, self.after, set())
        self.assertEqual(len(paths), 896)
        with self.assertRaisesRegex(ValueError, 'removal scope changed'):
            check.check_runtime_family(self.batch, self.before, self.after, paths)

    def test_protected_paths_refuse(self):
        for path in sorted(check.RUNTIME_PUBLIC_RETAINS) + ['.csdlc/state.json',
                'docs/templates/prompts/current.json', 'csdlc-v3/operator/authority-selector.json',
                'csdlc-v2/operator/authority-selector.json']:
            batch = copy.deepcopy(self.batch)
            next(r for r in batch['files'] if r['action'] == 'remove')['path'] = path
            with self.assertRaisesRegex(ValueError, 'protected path'):
                check.check_runtime_family(batch, self.before, self.after, set())

    def test_preimage_and_custody_refuse(self):
        row = next(r for r in self.batch['files'] if r['action'] == 'remove')
        before = dict(self.before)
        before[row['path']] = ['100755', '0' * 40]
        with self.assertRaisesRegex(ValueError, 'applied identity mismatch'):
            check.check_runtime_family(self.batch, before, self.after, set())
        batch = copy.deepcopy(self.batch)
        batch['prepared_patch']['deletions'][0]['current_sha256'] = '0' * 64
        with self.assertRaisesRegex(ValueError, 'custody identity mismatch'):
            check.check_runtime_family(batch, self.before, self.after, set())
        batch['prepared_patch']['deletions'][0]['id'] = 'RD07-9999'
        with self.assertRaisesRegex(ValueError, 'custody scope changed'):
            check.check_runtime_family(batch, self.before, self.after, set())

    def test_retained_public_identity_refuses(self):
        after = dict(self.after)
        after[next(iter(check.RUNTIME_PUBLIC_RETAINS))] = ['100644', '0' * 40]
        with self.assertRaisesRegex(ValueError, 'retained identity changed'):
            check.check_runtime_family(self.batch, self.before, after, set())

    def test_substituted_supplement_refuses(self):
        batch = copy.deepcopy(self.batch)
        expected = {r['path'] for r in batch['prepared_patch']['deletions'] if not r['rollback']['matches_current']}
        supplement = next(a for a in batch['prepared_patch']['custody'] if {r['path'] for r in a['entries']} == expected)
        with tempfile.TemporaryDirectory() as tmp:
            fake = Path(tmp) / 'bad.tar'
            fake.write_bytes(b'not the pinned archive')
            supplement['path'] = str(fake)
            with self.assertRaisesRegex(ValueError, 'supplement changed'):
                check.check_runtime_custody(batch, Path(tmp) / 'unused-original.tar.gz')




class InfrastructureFamilyTests(unittest.TestCase):
    # PVF: deterministic local guard unit tests; small CPU/disk; no owner execution.
    @classmethod
    def setUpClass(cls):
        cls.batch = check.committed_json(check.PACKET / 'INFRASTRUCTURE_SOURCE_FAMILY_APPLIED.json')
        cls.before = check.tree(check.INFRASTRUCTURE_FAMILY_IMPLEMENTATION + '^')
        cls.after = check.tree(check.INFRASTRUCTURE_FAMILY_IMPLEMENTATION)

    def test_exact_scope_and_repeat_refusal(self):
        paths = check.check_infrastructure_family(self.batch, self.before, self.after, set())
        self.assertEqual(len(paths), 212)
        with self.assertRaisesRegex(ValueError, 'removal scope changed'):
            check.check_infrastructure_family(self.batch, self.before, self.after, paths)

    def test_identity_substitution_and_resurrection_refuse(self):
        row = next(r for r in self.batch['files'] if r['action'] == 'remove')
        altered = copy.deepcopy(self.batch)
        next(r for r in altered['files'] if r['action'] == 'remove')['id'] = 'RD07-9999'
        with self.assertRaisesRegex(ValueError, 'removal scope changed'):
            check.check_infrastructure_family(altered, self.before, self.after, set())
        after = dict(self.after)
        after[row['path']] = self.before[row['path']]
        with self.assertRaisesRegex(ValueError, 'applied identity mismatch'):
            check.check_infrastructure_family(self.batch, self.before, after, set())

    def test_caller_and_custody_drift_refuse(self):
        caller = next(r for r in self.batch['files'] if r['action'] != 'remove')
        after = dict(self.after)
        after[caller['path']] = ['100755', '0'*40]
        with self.assertRaisesRegex(ValueError, 'applied identity mismatch'):
            check.check_infrastructure_family(self.batch, self.before, after, set())
        altered = copy.deepcopy(self.batch)
        altered['files'][0]['rollback']['sha256'] = '0'*64
        with self.assertRaisesRegex(ValueError, 'custody identity mismatch'):
            check.check_infrastructure_family(altered, self.before, self.after, set())

    def test_substituted_archive_refuses(self):
        altered = copy.deepcopy(self.batch)
        with tempfile.TemporaryDirectory() as tmp:
            fake = Path(tmp)/'substituted.tar'
            fake.write_bytes(b'not the pinned archive')
            altered['files'][0]['rollback']['archive'] = str(fake)
            with self.assertRaisesRegex(ValueError, 'archive changed'):
                check.check_infrastructure_custody(altered)



class TransientBridgeTests(unittest.TestCase):
    def setUp(self):
        self.path = check.TRANSIENT_CONVERSION_BRIDGE
        identity = check.TRANSIENT_BRIDGE_IDENTITY
        self.entry = {'path': self.path, 'action': 'remove_added', 'before': None, 'after': None,
                      'review_reference': 'reviewed', 'implementation_commit': 'fixture',
                      'approval_reference': 'pinned packet',
                      'rollback_custody': {'mode': identity[0], 'blob': identity[1]},
                      'source_chain': [{'commit': 'f598c0711d9b270a5b304c7cde64dcdf3068619e',
                                        'before': None, 'after': identity}],
                      'supplemental_source_chain': [{'commit': 'd293b17fe05f70633826017eb581f5315359f80f',
                                                    'before': identity, 'after': None}]}
        self.manifest = {'schema': 'adl.source_retirement.execution_overlay.v1', 'issue': 1223,
                         'status': 'complete', 'missing_entries': [], 'entries': [self.entry]}

    def validate(self):
        return check.check_execution_overlay(self.manifest, {}, {}, {self.entry['path']}, set())

    def test_exact_add_then_remove(self):
        self.assertEqual(self.validate(), [self.path])

    def test_different_absent_path_refuses(self):
        self.entry['path'] = 'adl/tools/unapproved.py'
        with self.assertRaisesRegex(ValueError, 'unapproved transient'):
            self.validate()

    def test_missing_add_chain_refuses(self):
        self.entry['source_chain'] = []
        with self.assertRaisesRegex(ValueError, 'chain mismatch'):
            self.validate()

    def test_missing_removal_chain_refuses(self):
        self.entry['supplemental_source_chain'] = []
        with self.assertRaisesRegex(ValueError, 'chain mismatch'):
            self.validate()

    def test_changed_custody_blob_refuses(self):
        self.entry['rollback_custody']['blob'] = '0' * 40
        with self.assertRaisesRegex(ValueError, 'chain mismatch'):
            self.validate()



class RecentOwnerFamilyTests(unittest.TestCase):
    # PVF: deterministic local guard regression; small disk/CPU; no owner execution.
    def batches(self):
        for spec in check.RECENT_OWNER_BATCHES:
            yield (spec, check.committed_json(check.PACKET / spec['packet']),
                   check.tree(spec['commit'] + '^'), check.tree(spec['commit']))

    def test_exact_families_and_custody(self):
        total = 0
        for spec, batch, before, after in self.batches():
            paths = check.check_recent_owner_family(batch, before, after, set(), spec)
            self.assertEqual(len(paths), spec['removals'])
            check.check_recent_owner_custody(batch, spec)
            total += len(paths)
        self.assertEqual(total, 457)

    def test_expanded_scope_and_repeat_refuse(self):
        for spec, batch, before, after in self.batches():
            paths = {r['path'] for r in batch['files'] if r['action'] == 'remove'}
            if paths:
                with self.assertRaisesRegex(ValueError, 'scope changed'):
                    check.check_recent_owner_family(batch, before, after, paths, spec)
            altered = copy.deepcopy(batch)
            altered['files'].append(copy.deepcopy(altered['files'][0]))
            with self.assertRaisesRegex(ValueError, 'scope changed'):
                check.check_recent_owner_family(altered, before, after, set(), spec)

    def test_source_resurrection_and_caller_drift_refuse(self):
        for spec, batch, before, after in self.batches():
            sources = [r for r in batch['files'] if r['action'] == 'remove']
            if sources:
                source = sources[0]
                altered = dict(after)
                altered[source['path']] = before[source['path']]
                with self.assertRaisesRegex(ValueError, 'identity mismatch'):
                    check.check_recent_owner_family(batch, before, altered, set(), spec)
            callers = [r for r in batch['files'] if r['action'] != 'remove']
            if callers:
                caller = callers[0]
                altered = dict(after)
                altered[caller['path']] = ['100644', '0' * 40]
                with self.assertRaisesRegex(ValueError, 'identity mismatch'):
                    check.check_recent_owner_family(batch, before, altered, set(), spec)

    def test_zero_removal_caller_repair_has_no_removal_authority(self):
        spec, batch, before, after = next(x for x in self.batches()
                                         if x[0]['removals'] == 0)
        self.assertTrue(batch['files'])
        self.assertTrue(all(r['action'] == 'modify' for r in batch['files']))
        self.assertEqual(check.check_recent_owner_family(
            batch, before, after, {'previously/removed'}, spec), set())
        altered = copy.deepcopy(batch)
        altered['files'][0]['action'] = 'remove'
        altered['files'][0]['after'] = None
        with self.assertRaisesRegex(ValueError, 'scope changed'):
            check.check_recent_owner_family(altered, before, after, set(), spec)
        # A caller-only batch still requires its precise preimage and afterimage.
        caller = batch['files'][0]['path']
        changed_before = dict(before)
        changed_before[caller] = ['100644', '0' * 40]
        with self.assertRaisesRegex(ValueError, 'identity mismatch'):
            check.check_recent_owner_family(batch, changed_before, after, set(), spec)

    def test_added_helper_cannot_replace_existing_file(self):
        spec, batch, before, after = next(x for x in self.batches()
                                         if any(r['action'] == 'add' for r in x[1]['files']))
        helper = next(r for r in batch['files'] if r['action'] == 'add')
        altered = dict(before)
        altered[helper['path']] = ['100644', '0' * 40]
        with self.assertRaisesRegex(ValueError, 'identity mismatch'):
            check.check_recent_owner_family(batch, altered, after, set(), spec)

    def test_substituted_archive_refuses(self):
        spec, batch, _, _ = next(self.batches())
        changed = copy.deepcopy(spec)
        with tempfile.TemporaryDirectory() as tmp:
            fake = Path(tmp) / 'substituted.tar'
            fake.write_bytes(b'not the reviewed archive')
            changed['archives'][0]['path'] = str(fake)
            with self.assertRaisesRegex(ValueError, 'archive changed'):
                check.check_recent_owner_custody(batch, changed)

    def test_wrong_custody_member_bytes_refuse(self):
        spec, batch, _, _ = next(self.batches())
        altered = copy.deepcopy(batch)
        next(r for r in altered['files'] if r['action'] == 'remove')['before']['sha256'] = '0' * 64
        with self.assertRaisesRegex(ValueError, 'custody identity mismatch'):
            check.check_recent_owner_custody(altered, spec)

if __name__ == '__main__':
    unittest.main()
