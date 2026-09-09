"""PVF: deterministic offline Python contract tests; small; not a release gate."""
import copy
import importlib.util
import json
from pathlib import Path
import subprocess
import tempfile
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[1]
spec = importlib.util.spec_from_file_location('builder', ROOT / 'scripts/build_repo_packet.py')
b = importlib.util.module_from_spec(spec)
spec.loader.exec_module(b)


def packet(paths):
    with patch.object(b, 'count_lines', return_value=1):
        evidence = b.build_evidence(Path('.'), paths)
    return b.lane_denominators(paths), evidence, b.assignments_from_evidence(evidence)


class DenominatorTests(unittest.TestCase):
    def test_exact_issue_520(self):
        paths = (ROOT / 'tests/fixtures/issue-520-changed-paths.txt').read_text().splitlines()
        self.assertEqual(len(paths), 5481)
        self.assertEqual(b.paths_digest(paths), '59c4c5de57d5aac07549a97bf508c00cbc2b5234e63f984a2eece6a8e07acb33')
        d, e, a = packet(paths)
        b.validate_packet(d, e, a, paths)
        for lane in b.REVIEW_LANES:
            self.assertTrue(a[lane], lane)
            self.assertTrue(all(lane in b.lanes_for(b.category_for(Path(p)), Path(p)) for p in a[lane]))
        self.assertEqual(len(e), 5481)
        self.assertEqual(packet(list(reversed(paths))), (d, e, a))

    def test_manifests_cannot_starve_lanes(self):
        paths = [f'dep/{n:03}/Cargo.lock' for n in range(200)] + ['src/core.rs', 'tests/core.rs', 'docs/guide.md']
        d, e, a = packet(paths)
        self.assertEqual(a['code'], ['src/core.rs'])
        self.assertEqual(a['tests'], ['tests/core.rs'])
        self.assertEqual(a['docs'], ['docs/guide.md'])
        self.assertIn('src/core.rs', a['architecture'])
        self.assertIn('src/core.rs', a['security'])
        b.validate_packet(d, e, a, paths)

    def test_integrity_negatives(self):
        paths = ['src/core.rs', 'tests/core.rs', 'Cargo.lock', 'docs/guide.md']
        original = packet(paths)
        def empty(d, e, a): a['tests'].clear()
        def category(d, e, a): a['code'][:] = ['Cargo.lock']
        def digest(d, e, a): d['lanes']['tests']['source_sha256'] = '0' * 64
        def exclusion(d, e, a): d['lanes']['code']['exclusions']['not_eligible'] = []
        def missing(d, e, a): e.pop()
        def forged(d, e, a): e[0]['category'] = 'other'
        for mutate in [empty, category, digest, exclusion, missing, forged]:
            with self.subTest(mutate=mutate.__name__):
                d, e, a = copy.deepcopy(original)
                mutate(d, e, a)
                with self.assertRaises(ValueError): b.validate_packet(d, e, a, paths)
        # Re-sealing an incomplete packet must still fail against the independent inventory.
        with self.assertRaises(ValueError): b.validate_packet(*packet(paths[:-1]), paths)

    def test_absent_lanes_are_explicit(self):
        d, e, a = packet(['docs/guide.md'])
        self.assertEqual(d['lanes']['tests']['source_count'], 0)
        b.validate_packet(d, e, a, ['docs/guide.md'])

    def test_git_filename_preservation(self):
        with tempfile.TemporaryDirectory() as tmp:
            repo = Path(tmp)
            def git(*args):
                return subprocess.check_output(['git', '-C', tmp, *args], text=True).strip()
            git('init', '-q'); git('config', 'user.email', 'test@example.invalid'); git('config', 'user.name', 'test')
            git('commit', '--allow-empty', '-qm', 'base'); base = git('rev-parse', 'HEAD')
            paths = [' leading.py', 'trailing.py ', 'unicodé.py', 'tab\tname.py']
            for p in paths: (repo / p).write_text('pass\n')
            git('add', '.'); git('commit', '-qm', 'paths')
            self.assertEqual(b.scoped_files(repo, None, base), sorted(paths))
            self.assertEqual(b.scoped_files(repo, None, None), sorted(paths))
            b.validate_packet(*packet(paths), paths)

    def test_cli_diff_and_validator(self):
        with tempfile.TemporaryDirectory() as tmp:
            repo = Path(tmp)
            def git(*args):
                return subprocess.check_output(['git', '-C', tmp, *args], text=True).strip()
            git('init', '-q'); git('config', 'user.email', 'test@example.invalid'); git('config', 'user.name', 'test')
            (repo / 'base.py').write_text('pass\n')
            git('add', '.'); git('commit', '-qm', 'base'); base = git('rev-parse', 'HEAD')
            (repo / 'test_new.py').write_text('pass\n')
            git('add', '.'); git('commit', '-qm', 'change')
            out = repo / 'packet'
            subprocess.run(['python3', str(ROOT / 'scripts/build_repo_packet.py'), tmp, '--diff-base', base, '--out', str(out)], check=True, capture_output=True)
            d = json.loads((out / 'lane_denominators.json').read_text())
            self.assertEqual(d['source_paths'], ['test_new.py'])
            self.assertEqual(d['lanes']['tests']['selected_paths'], ['test_new.py'])
            inventory = repo / 'source.txt'; inventory.write_text('test_new.py\n')
            argv = ['python3', str(ROOT / 'scripts/validate_repo_packet.py'), str(out), '--source-inventory', str(inventory)]
            self.assertEqual(subprocess.run(argv, capture_output=True).returncode, 0)
            d['source_sha256'] = 'bad'; (out / 'lane_denominators.json').write_text(json.dumps(d))
            self.assertNotEqual(subprocess.run(argv, capture_output=True).returncode, 0)


if __name__ == '__main__':
    unittest.main()
