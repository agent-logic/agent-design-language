#!/usr/bin/env python3
"""PVF local_contract: deterministic small scratch/sandbox regression; no builds/network service.

Storage cases are portable. Sandbox cases require macOS and prove actual OS
read/network refusal, not the final five-package installed qualification.
"""
import contextlib
import importlib.util
import io
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest
from unittest import mock

HERE = Path(__file__).resolve().parent

def module(name):
    spec = importlib.util.spec_from_file_location(name, HERE / (name + '.py'))
    result = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(result)
    return result

proof = module('verify_install')
validator = module('verify_install_validator')


class StorageTests(unittest.TestCase):
    def check_output_root(self, override):
        with tempfile.TemporaryDirectory(prefix='adl-proof-storage-test-') as temporary:
            base = Path(temporary).resolve()
            checkout = base / 'checkout'
            expected = base / 'explicit-proof' if override else checkout / 'target/public-adl-proof'
            env = dict(os.environ)
            env.pop('ADL_PUBLIC_ADL_PROOF_ROOT', None)
            if override:
                env['ADL_PUBLIC_ADL_PROOF_ROOT'] = str(expected)
            captured = io.StringIO()
            # Stop at a deliberately failed verifier; path selection must still
            # retain the actual failure evidence without invoking Git-common-dir.
            failed = subprocess.CompletedProcess([], 2, 'fixture stdout', 'fixture failure')
            with mock.patch.object(validator, 'ROOT', checkout), mock.patch.object(validator, 'run', return_value=failed), mock.patch.dict(os.environ, env, clear=True), mock.patch.object(sys, 'argv', ['validator', '--issue', '1223']), contextlib.redirect_stdout(captured):
                with self.assertRaises(SystemExit) as exited:
                    validator.main()
            self.assertEqual(exited.exception.code, 2)
            result = json.loads(captured.getvalue())
            destination = Path(result['evidence_directory'])
            self.assertTrue(destination.is_relative_to(expected))
            self.assertEqual((destination / 'stderr.txt').read_text(), 'fixture failure')
            self.assertEqual(result['status'], 'failed')

    def test_worktree_default_keeps_failed_evidence(self):
        self.check_output_root(False)

    def test_explicit_root_keeps_failed_evidence(self):
        self.check_output_root(True)


@unittest.skipUnless(Path('/usr/bin/sandbox-exec').is_file(), 'macOS sandbox required')
class ScratchSandboxTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix='adl-proof-sandbox-test-')
        self.addCleanup(self.temporary.cleanup)
        self.base = Path(self.temporary.name).resolve()
        self.home = self.base / 'denied-home'
        self.work = self.home / 'scratch'
        self.work.mkdir(parents=True)
        self.secret = self.work / 'private-credential'
        self.secret.write_text('synthetic secret')
        self.producer = self.home / 'producer'
        self.producer.mkdir()
        (self.producer / 'source').write_text('synthetic producer source')
        self.fixture = self.work / 'fixture'
        self.fixture.write_text('public fixture')
        self.profile = self.work / 'consumer.sb'
        self.profile.write_text(proof.consumer_sandbox_profile(self.work, self.home, self.base / 'toolchain', self.base / 'cache', self.secret, self.producer))

    def execute(self, *command):
        return subprocess.run(['/usr/bin/sandbox-exec', '-f', str(self.profile), *command], text=True, capture_output=True)

    def denied(self, *command):
        result = self.execute(*command)
        self.assertNotIn('sandbox_apply:', result.stderr, 'sandbox setup failed before the probe')
        self.assertNotEqual(result.returncode, 0)
        self.assertTrue(any(word in result.stderr.lower() for word in ('operation not permitted', 'permission denied')), result.stderr)

    def test_scratch_read_inside_denied_parent(self):
        result = self.execute('/bin/cat', str(self.fixture))
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(result.stdout, 'public fixture')

    def test_private_sentinel_stays_denied_inside_scratch(self):
        self.denied('/bin/cat', str(self.secret))

    def test_producer_stays_denied_next_to_scratch(self):
        self.denied('/bin/cat', str(self.producer / 'source'))

    def test_network_stays_denied(self):
        self.denied('/usr/bin/ruby', '-rsocket', '-e', 'TCPSocket.new("127.0.0.1", 9)')


if __name__ == '__main__':
    unittest.main()
