"""PVF small/local/deterministic archive-identity regression; issue acceptance gate."""
import sys
sys.dont_write_bytecode = True
import importlib.util
import json
from pathlib import Path
import subprocess
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location('archive_guard', Path(__file__).with_name('guard.py'))
guard = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(guard)
RAW = Path(__file__).with_name('transition.json').read_bytes()
PLAN = json.loads(RAW)


def tree(ref):
    text = subprocess.check_output(['git', 'ls-tree', '-r', ref], cwd=ROOT, text=True)
    return {line.split('\t', 1)[1]: line.split('\t', 1)[0].split()[::2] for line in text.splitlines()}


class ArchiveContract(unittest.TestCase):
    def setUp(self):
        self.base = tree(PLAN['source_commit'])
        self.candidate = dict(self.base)
        for row in PLAN['entries']:
            del self.candidate[row['path']]
        caller = PLAN['caller_change']
        self.candidate[caller['path']] = caller['after']

    def test_exact_transition_restores_historical_tree(self):
        self.assertEqual(guard.normalize_candidate(ROOT, self.candidate, tree), self.base)

    def test_restored_removed_path_is_rejected(self):
        path = PLAN['entries'][0]['path']
        self.candidate[path] = self.base[path]
        with self.assertRaises(ValueError):
            guard.normalize_candidate(ROOT, self.candidate, tree)

    def test_caller_drift_is_rejected(self):
        self.candidate[PLAN['caller_change']['path']] = ['100644', '0' * 40]
        with self.assertRaises(ValueError):
            guard.normalize_candidate(ROOT, self.candidate, tree)

    def test_foreign_deletion_is_not_hidden(self):
        del self.candidate['README.md']
        normalized = guard.normalize_candidate(ROOT, self.candidate, tree)
        self.assertNotIn('README.md', normalized)
        self.assertNotEqual(normalized, self.base)

    def test_manifest_tampering_is_rejected(self):
        with tempfile.TemporaryDirectory() as tmp:
            p = Path(tmp) / 'tools/public_readiness_archive'
            p.mkdir(parents=True)
            (p / 'transition.json').write_bytes(RAW + b' ')
            with self.assertRaises(ValueError):
                guard.normalize_candidate(tmp, self.candidate, tree)


if __name__ == '__main__':
    import sys
    if '--self-test' in sys.argv:
        sys.argv.remove('--self-test')
    result = unittest.main(exit=False)
    if result.result.wasSuccessful():
        print('archive contract passed')
    sys.exit(not result.result.wasSuccessful())
