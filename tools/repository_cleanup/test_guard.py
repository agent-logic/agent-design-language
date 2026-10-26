#!/usr/bin/env python3
"""PVF local_contract: exact archival admission and tamper refusal; tiny local
fixtures only, deterministic, no network/build; required cleanup guard proof.
"""
import hashlib
import importlib.util
import json
from pathlib import Path
import subprocess
import tempfile
import unittest
from unittest.mock import patch

spec = importlib.util.spec_from_file_location('guard', Path(__file__).with_name('guard.py'))
guard = importlib.util.module_from_spec(spec)
spec.loader.exec_module(guard)


class ArchiveGuard(unittest.TestCase):
    def test_checked_in_transition_matches_git_candidate(self):
        root = Path(__file__).resolve().parents[2]
        def tree(revision):
            rows = subprocess.check_output(['git', 'ls-tree', '-rz', revision], cwd=root)
            result = {}
            for row in rows.split(b'\0'):
                if row:
                    meta, path = row.split(b'\t')
                    mode, _, blob = meta.decode().split()
                    result[path.decode()] = [mode, blob]
            return result
        transition = json.loads((root / 'tools/repository_cleanup/transition.json').read_text())
        candidate = guard.normalize_language_cleanup(root, tree('HEAD'), tree)
        # Compose the newer exact archive transition before checking the older one,
        # matching tools/source_retirement/check.py's production order.
        newer_spec = importlib.util.spec_from_file_location(
            'public_readiness_guard', root / 'tools/public_readiness_archive/guard.py')
        newer_guard = importlib.util.module_from_spec(newer_spec)
        newer_spec.loader.exec_module(newer_guard)
        candidate = newer_guard.normalize_candidate(root, candidate, tree)
        baseline = tree(transition['source_commit'])
        normalized = guard.normalize_candidate(root, candidate, baseline)
        for row in transition['changes']:
            self.assertEqual(normalized[row['path']], baseline[row['path']])

    def test_language_transition_refuses_drift(self):
        baseline = {'old.rb': ['100644', 'old'], 'caller': ['100755', 'before']}
        candidate = {'caller': ['100755', 'after'], 'unrelated': ['100644', 'drift']}
        transition = {'source_commit': 'source', 'removed': ['old.rb'], 'changes': [
            {'path': path, 'before': before, 'after': candidate.get(path),
             'before_sha256': hashlib.sha256(b'original').hexdigest()}
            for path, before in baseline.items()]}
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            manifest = root / 'docs/language-cleanup/ruby-transition.json'
            manifest.parent.mkdir(parents=True)
            def install(value):
                manifest.write_text(json.dumps(value))
                guard.LANGUAGE_TRANSITION_SHA256 = hashlib.sha256(manifest.read_bytes()).hexdigest()
            self.addCleanup(setattr, guard, 'LANGUAGE_TRANSITION_SHA256',
                            guard.LANGUAGE_TRANSITION_SHA256)
            install(transition)
            with patch('subprocess.check_output', return_value=b'original'):
                normalized = guard._normalize_language_transition(root, candidate, lambda _: baseline, "ruby-transition.json", guard.LANGUAGE_TRANSITION_SHA256)
                self.assertEqual(normalized, {**baseline, 'unrelated': candidate['unrelated']})
                for bad in [{**candidate, 'old.rb': baseline['old.rb']},
                            {**candidate, 'caller': ['100755', 'changed']}]:
                    with self.assertRaisesRegex(ValueError, 'transition mismatch'):
                        guard._normalize_language_transition(root, bad, lambda _: baseline, "ruby-transition.json", guard.LANGUAGE_TRANSITION_SHA256)
                manifest.write_text(manifest.read_text() + '\n')
                with self.assertRaisesRegex(ValueError, 'identity changed'):
                    guard._normalize_language_transition(root, candidate, lambda _: baseline, "ruby-transition.json", guard.LANGUAGE_TRANSITION_SHA256)
                install({**transition, 'changes': transition['changes'] * 2})
                with self.assertRaisesRegex(ValueError, 'transition mismatch'):
                    guard._normalize_language_transition(root, candidate, lambda _: baseline, "ruby-transition.json", guard.LANGUAGE_TRANSITION_SHA256)
                install({**transition, 'removed': []})
                with self.assertRaisesRegex(ValueError, 'removal set mismatch'):
                    guard._normalize_language_transition(root, candidate, lambda _: baseline, "ruby-transition.json", guard.LANGUAGE_TRANSITION_SHA256)
                install(transition)
            with patch('subprocess.check_output', return_value=b'wrong preimage'):
                with self.assertRaisesRegex(ValueError, 'custody mismatch'):
                    guard._normalize_language_transition(root, candidate, lambda _: baseline, "ruby-transition.json", guard.LANGUAGE_TRANSITION_SHA256)

    def test_exact_admission_and_rejection(self):
        baseline = {'payload': ['100644', 'old'], 'runner': ['100755', 'before']}
        candidate = {'runner': ['100755', 'after'], 'unrelated': ['100644', 'keep']}
        archive = {'source_commit': 'source', 'entries': [
            {'path': 'payload', 'mode': '100644', 'git_blob': 'old'}]}
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            ap = root / 'docs/archives/POST_SPLIT_PAYLOADS_1284.json'
            tp = root / 'tools/repository_cleanup/transition.json'
            ap.parent.mkdir(parents=True)
            tp.parent.mkdir(parents=True)
            ap.write_text(json.dumps(archive))
            transition = {'source_commit': 'source',
                'archive_manifest_sha256': hashlib.sha256(ap.read_bytes()).hexdigest(),
                'changes': [
                    {'path': 'payload', 'before': baseline['payload'], 'after': None},
                    {'path': 'runner', 'before': baseline['runner'], 'after': candidate['runner']}]}
            tp.write_text(json.dumps(transition))
            original_pin = guard.TRANSITION_SHA256
            self.addCleanup(setattr, guard, 'TRANSITION_SHA256', original_pin)
            guard.TRANSITION_SHA256 = hashlib.sha256(tp.read_bytes()).hexdigest()
            self.assertEqual(guard.normalize_candidate(root, candidate, baseline),
                             {**baseline, 'unrelated': candidate['unrelated']})
            for bad in [{**candidate, 'payload': baseline['payload']},
                        {**candidate, 'runner': ['100755', 'tampered']},
                        {'unrelated': candidate['unrelated']}]:
                with self.assertRaises(ValueError):
                    guard.normalize_candidate(root, bad, baseline)
            ap.write_text(json.dumps({**archive, 'source_commit': 'changed'}))
            with self.assertRaisesRegex(ValueError, 'archive manifest identity'):
                guard.normalize_candidate(root, candidate, baseline)
            ap.write_text(json.dumps(archive))
            tp.write_text(tp.read_text() + '\n')
            with self.assertRaisesRegex(ValueError, 'transition identity'):
                guard.normalize_candidate(root, candidate, baseline)


if __name__ == '__main__':
    unittest.main()
