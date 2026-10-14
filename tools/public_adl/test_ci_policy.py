"""PVF local_contract: deterministic Git/routing negatives, small CPU/disk, no network; required CI."""
import importlib.util
from pathlib import Path
import subprocess
import tempfile
import unittest

SCRIPT = Path(__file__).with_name('ci_policy.py')
spec = importlib.util.spec_from_file_location('ci_policy', SCRIPT)
policy = importlib.util.module_from_spec(spec)
spec.loader.exec_module(policy)


class PublicCIPolicy(unittest.TestCase):
    def test_docs_do_not_select_builds(self):
        d = policy.select('pull_request', ['docs/design.md'])
        self.assertEqual(d['public_adl_validation_required'], 'false')
        self.assertEqual(d['transpiler_distribution_required'], 'false')
        self.assertEqual(d['ci_contracts_required'], 'true')

    def test_source_installer_unknown_and_deleted_private_paths(self):
        for path in ['adl/src/main.rs', 'adl/tools/install_owner_binaries.sh',
                     'adl-runtime/src/lib.rs', 'unknown/value.bin', 'docs/api/contract.json']:
            self.assertEqual(policy.select('pull_request', [path])['public_adl_validation_required'], 'true')

    def test_manual_and_empty_are_conservative(self):
        for event, paths in [('workflow_dispatch', ['docs/x.md']), ('pull_request', [])]:
            self.assertEqual(policy.select(event, paths)['public_adl_validation_required'], 'true')

    def test_aggregate_rejects_missing_failed_cancelled_and_unexpected(self):
        for selected in ['true', 'false', '', 'malformed', None]:
            for result in ['success', 'skipped', 'failure', 'cancelled', '', 'malformed', None]:
                if (selected, result) in [('true', 'success'), ('false', 'skipped')]:
                    policy.validate_result(selected, result)
                else:
                    with self.assertRaises(ValueError):
                        policy.validate_result(selected, result)

    def test_aggregate_requires_classifier_and_contract_success(self):
        good = dict(PATH_POLICY_RESULT='success', PUBLIC_REQUIRED='true', PUBLIC_RESULT='success',
                    CONTRACT_RESULT='success', TRANSPILE_REQUIRED='false', TRANSPILE_RESULT='skipped')
        policy.aggregate('ci', good)
        for field in ['PATH_POLICY_RESULT', 'CONTRACT_RESULT']:
            for result in ['skipped', 'failure', 'cancelled', '', 'malformed']:
                with self.assertRaises(ValueError):
                    policy.aggregate('ci', dict(good, **{field: result}))

    def test_real_git_deleted_source_and_space_path(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            def git(*args):
                return subprocess.check_output(['git', '-C', directory, *args], stderr=subprocess.DEVNULL).decode().strip()
            git('init', '-q'); git('config', 'user.name', 'CI fixture'); git('config', 'user.email', 'fixture@example.invalid')
            (root/'adl-runtime').mkdir(); old = root/'adl-runtime/source.rs'; old.write_text('fixture')
            git('add', '.'); git('-c', 'commit.gpgsign=false', 'commit', '-qm', 'base'); base = git('rev-parse', 'HEAD')
            old.unlink(); (root/'unknown file.txt').write_text('fixture')
            git('add', '-A'); git('-c', 'commit.gpgsign=false', 'commit', '-qm', 'change')
            result = subprocess.run(['python3', str(SCRIPT), '--event-name', 'pull_request', '--base', base,
                                     '--head', 'HEAD'], cwd=root, text=True, capture_output=True, check=True)
            self.assertIn('public_adl_validation_required=true\n', result.stdout)
            invalid = subprocess.run(['python3', str(SCRIPT), '--event-name', 'pull_request', '--base', 'missing-ref'],
                                     cwd=root, capture_output=True)
            self.assertNotEqual(invalid.returncode, 0)


if __name__ == '__main__':
    unittest.main()
