#!/usr/bin/env python3
"""PVF tooling lane: deterministic local evidence-integrity negatives, issue gate.

Synthetic transcript fixtures test admission only. Actual Cargo measurements
at both immutable commits are a separate required issue acceptance surface.
"""
import copy
import importlib.util
import json
from pathlib import Path
import subprocess
import tempfile
import unittest

SPEC = importlib.util.spec_from_file_location('inventory', Path(__file__).with_name('revision_validation_inventory.py'))
I = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(I)


class InventoryTests(unittest.TestCase):
    def setUp(self):
        # Keep test fixtures under the issue worktree rather than a shared /tmp.
        parent = Path(__file__).resolve().parents[2] / '.csdlc/evidence/899/fixtures'
        parent.mkdir(parents=True, exist_ok=True)
        self.temp = tempfile.TemporaryDirectory(dir=parent)
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        (self.root / 'adl').mkdir()
        (self.root / 'adl/Cargo.toml').write_text('[package]\nname="adl"\nversion="0.1.0"\n')
        (self.root / 'adl/Cargo.lock').write_text('version = 4\n')
        for args in [['init', '-q'], ['add', '.'], ['-c', 'user.name=Fixture', '-c', 'user.email=fixture@example.invalid', 'commit', '-qm', 'fixture']]:
            subprocess.run(['git', '-C', str(self.root), *args], check=True)
        self.revision = I.git(self.root, 'rev-parse', 'HEAD').decode().strip()
        self.commands = []
        target = {'name': 'adl', 'kind': ['lib'], 'src_path': 'SNAPSHOT/adl/src/lib.rs', 'test': True}
        metadata = {'packages': [{'name': 'adl', 'manifest_path': 'SNAPSHOT/adl/Cargo.toml', 'features': {}, 'targets': [target]}]}
        shared = ['--manifest-path', 'SNAPSHOT/adl/Cargo.toml', '--locked']
        self.add_command('rustc', ['rustc', '+1.92.0', '-vV'], 'rustc 1.92.0\nhost: fixture-host\n')
        self.add_command('metadata', ['cargo', '+1.92.0', 'metadata'] + shared + ['--no-deps', '--format-version', '1'], json.dumps(metadata))
        artifact = {'reason': 'compiler-artifact', 'profile': {'test': True}, 'executable': 'TARGET/debug/deps/adl', 'target': target}
        self.add_command('build', ['cargo', '+1.92.0', 'test'] + shared + ['--package', 'adl', '--tests', '--no-run', '--message-format=json'], json.dumps(artifact)+'\n')
        self.add_command('list:lib:adl', ['TARGET/debug/deps/adl', '--list', '--format', 'terse'], 'unit::one: test\nunit::ignored: test\n')
        self.add_command('ignored:lib:adl', ['TARGET/debug/deps/adl', '--list', '--ignored', '--format', 'terse'], 'unit::ignored: test\n')
        self.add_command('doctest', ['cargo', '+1.92.0', 'test'] + shared + ['--package', 'adl', '--doc', '--', '--list'], 'adl/src/lib.rs - Example (line 1): test\n')
        self.report = {'schema': I.SCHEMA, 'identity': I.revision_identity(self.root, self.revision), 'profile': I.PROFILE.copy(), 'commands': self.commands, 'rustc': 'rustc 1.92.0\nhost: fixture-host\n', 'limitations': [], **I.derive(self.root, self.commands)}

    def add_command(self, role, argv, stdout):
        n = len(self.commands)
        record = {'role': role, 'argv': argv, 'exit_code': 0}
        for stream, value in [('stdout', stdout), ('stderr', '')]:
            name = f'{n}.{stream}'
            (self.root / name).write_text(value)
            record[stream] = {'file': name, 'sha256': I.digest(value.encode())}
        self.commands.append(record)

    def verify(self, report=None, revision=None):
        path = self.root / 'inventory.json'
        I.write_json(path, self.report if report is None else report)
        return I.verify(self.root, path, self.revision if revision is None else revision)

    def rejected(self, mutate, message):
        report = copy.deepcopy(self.report)
        mutate(report)
        with self.assertRaisesRegex(ValueError, message):
            self.verify(report)

    def test_valid_measured_fixture(self):
        self.assertEqual(self.verify()['counts']['enumerated_cases'], 2)
        self.assertEqual(self.verify()['counts']['test_bodies_run'], 0)

    def test_wrong_revision(self):
        with self.assertRaisesRegex(ValueError, 'wrong revision'):
            self.verify(revision='0' * 40)

    def test_manifest_mismatch(self):
        self.rejected(lambda r: r['profile'].update(manifest='Cargo.toml'), 'profile')

    def test_features_mismatch(self):
        self.rejected(lambda r: r['profile'].update(features='all'), 'profile')

    def test_duplicate_target(self):
        self.rejected(lambda r: r['targets'].append(r['targets'][0]), 'targets')

    def test_omitted_target(self):
        self.rejected(lambda r: r['targets'].clear(), 'targets')

    def test_source_count_substitution(self):
        self.rejected(lambda r: r['counts'].update(enumerated_cases=54), 'counts')

    def test_lexical_reduction_claim(self):
        self.rejected(lambda r: r.update(behavior_reduction=True), 'unsupported claim')

    def test_missing_scenario(self):
        self.rejected(lambda r: r['targets'][0]['cases'].pop(), 'targets')

    def test_listing_command_not_executed(self):
        self.rejected(lambda r: r['commands'][3].update(argv=['echo', 'unit::one: test']), 'command mismatch')

    def test_failed_build(self):
        self.rejected(lambda r: r['commands'][2].update(exit_code=1), 'mismatch')

    def test_wrong_build_features(self):
        self.rejected(lambda r: r['commands'][2]['argv'].append('--all-features'), 'command/profile')

    def test_changed_transcript(self):
        (self.root / '3.stdout').write_text('different: test\n')
        with self.assertRaisesRegex(ValueError, 'digest mismatch'):
            self.verify()

    def test_omitted_command(self):
        self.rejected(lambda r: r['commands'].pop(3), 'omitted')

    def test_wrong_manifest_identity(self):
        self.rejected(lambda r: r['identity'].update(manifest_sha256='0' * 64), 'source identity')

    def test_toolchain_substitution(self):
        self.rejected(lambda r: r.update(rustc='invented'), 'toolchain')

    def test_duplicate_case(self):
        with self.assertRaisesRegex(ValueError, 'duplicate'):
            I.cases('same: test\nsame: test\n')


if __name__ == '__main__':
    unittest.main()
