import copy
import hashlib
import importlib.util
import json
from pathlib import Path
import tempfile
import subprocess
import unittest

HERE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('retirement_check', HERE / 'check.py')
check = importlib.util.module_from_spec(spec)
spec.loader.exec_module(check)


class RetirementNegatives(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.ledger = json.loads((check.ROOT / check.PACKET / 'DISPOSITION_REGISTER.json').read_text())
        cls.frozen = check.tree(cls.ledger['original_revision'])
        cls.observed = check.tree(cls.ledger['observed_main'])
        cls.candidate = dict(cls.observed)
        for r in cls.ledger['rows']:
            if r['action'] == 'existing_branch_removal':
                del cls.candidate[r['path']]
            elif r['action'] == 'existing_branch_surgical':
                cls.candidate[r['path']] = r['expected_surgical_identity']
            elif r['action'] == 'consumer_reference_repair':
                data = (check.ROOT / r['path']).read_bytes()
                blob = hashlib.sha1(b'blob ' + str(len(data)).encode() + b'\0' + data).hexdigest()
                cls.candidate[r['path']] = [cls.observed[r['path']][0], blob]

    def test_complete_original_denominator_and_existing_delta(self):
        self.assertEqual(len(check.check_inventory(self.ledger, self.frozen, self.observed, self.candidate)), 34)

    def test_integrated_main_fix_cannot_be_reverted(self):
        candidate = dict(self.candidate)
        path = 'csdlc-v3/src/storage/semantic.rs'
        candidate[path] = self.frozen[path]
        self.assertNotEqual(candidate[path], self.observed[path])
        with self.assertRaisesRegex(ValueError, 'held source'):
            check.check_inventory(self.ledger, self.frozen, self.observed, candidate)

    def test_missing_candidate_rejected(self):
        ledger = copy.deepcopy(self.ledger)
        ledger['rows'].pop()
        with self.assertRaisesRegex(ValueError, 'denominator'):
            check.check_inventory(ledger, self.frozen, self.observed, self.candidate)

    def test_required_evidence_loss_rejected(self):
        candidate = dict(self.candidate)
        candidate.pop('csdlc-v3/operator/authority-selector.json')
        with self.assertRaises(ValueError):
            check.check_inventory(self.ledger, self.frozen, self.observed, candidate)

    def test_unapproved_source_loss_rejected(self):
        candidate = dict(self.candidate)
        path = next(r['path'] for r in self.ledger['rows'] if r['action'] == 'retain')
        candidate.pop(path)
        with self.assertRaisesRegex(ValueError, 'held source'):
            check.check_inventory(self.ledger, self.frozen, self.observed, candidate)

    def test_deletion_outside_original_denominator_rejected(self):
        candidate = dict(self.candidate)
        candidate.pop('tools/public_adl/verify_install.py')
        with self.assertRaisesRegex(ValueError, 'new deletion outside'):
            check.check_inventory(self.ledger, self.frozen, self.observed, candidate)

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

    def test_surgical_repair_cannot_hide_configuration_change(self):
        candidate = dict(self.candidate)
        candidate['infra/runtime-v3/runtime-init.toml'] = ['100644', '0' * 40]
        with self.assertRaisesRegex(ValueError, 'surgical change exceeds'):
            check.check_inventory(self.ledger, self.frozen, self.observed, candidate)

    def test_false_approval_rejected(self):
        ledger = dict(self.ledger, new_removals_authorized=True)
        with self.assertRaisesRegex(ValueError, 'approval'):
            check.check_inventory(ledger, self.frozen, self.observed, self.candidate)

    def test_consumer_notice_cannot_hide_runbook_changes(self):
        candidate = dict(self.candidate)
        candidate[check.INFRA_RUNBOOKS[0]] = ['100644', '0' * 40]
        with self.assertRaisesRegex(ValueError, 'consumer repair exceeds'):
            check.check_inventory(self.ledger, self.frozen, self.observed, candidate)

    def test_consumer_repair_cannot_authorize_another_path(self):
        with self.assertRaisesRegex(ValueError, 'unapproved consumer repair'):
            check.consumer_reference_repair('adl/src/live.rs', b'live source\n')

    def test_missing_rollback_rejected(self):
        with tempfile.TemporaryDirectory() as directory:
            with self.assertRaisesRegex(ValueError, 'archive missing'):
                check.check_rollback(Path(directory) / 'absent.tar.gz')

    def test_corrupt_rollback_rejected(self):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / 'archive.tar.gz'
            path.write_bytes(b'not the preserved archive')
            with self.assertRaisesRegex(ValueError, 'digest mismatch'):
                check.check_rollback(path)

    def test_live_reference_not_hidden_by_historical_allowlist(self):
        with self.assertRaisesRegex(ValueError, 'unclassified retained reference'):
            check.check_references(['adl/src/live.rs'], ['.csdlc/issues/594/cards/sor.md'])


if __name__ == '__main__':
    unittest.main()
