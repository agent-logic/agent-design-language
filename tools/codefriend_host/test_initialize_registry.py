import os
import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch
from initialize_registry import initialize

class Initialization(unittest.TestCase):
    def test_create_once_then_preserve(self):
        with tempfile.TemporaryDirectory() as directory:
            directory=Path(directory); directory.chmod(0o700)
            # macOS /var is a symlink; resolve only the test fixture's parent.
            directory=directory.resolve()
            self.assertEqual(initialize(directory,os.getuid(),os.getgid()),'deny_all_registry_created')
            path=directory/'gateway-credentials.json'
            self.assertEqual(path.read_bytes(),b'[]\n'); self.assertEqual(path.stat().st_mode & 0o777,0o600)
            path.write_bytes(b'[]  ')
            self.assertEqual(initialize(directory,os.getuid(),os.getgid()),'existing_registry_preserved')
            self.assertEqual(path.read_bytes(),b'[]  ')
    def test_existing_symlink_is_not_followed_or_replaced(self):
        with tempfile.TemporaryDirectory() as directory:
            directory=Path(directory).resolve(); directory.chmod(0o700)
            target=directory/'keep'; target.write_bytes(b'unchanged')
            (directory/'gateway-credentials.json').symlink_to(target)
            with self.assertRaises(Exception): initialize(directory,os.getuid(),os.getgid())
            self.assertEqual(target.read_bytes(),b'unchanged')
    def test_public_directory_denied(self):
        with tempfile.TemporaryDirectory() as directory:
            directory=Path(directory).resolve(); directory.chmod(0o755)
            with self.assertRaises(Exception): initialize(directory,os.getuid(),os.getgid())

    def test_failed_initial_write_does_not_become_ready_on_retry(self):
        with tempfile.TemporaryDirectory() as directory:
            directory=Path(directory).resolve(); directory.chmod(0o700)
            with patch('initialize_registry.os.fchown',side_effect=OSError('injected')):
                with self.assertRaises(OSError): initialize(directory,os.getuid(),os.getgid())
            self.assertEqual((directory/'gateway-credentials.json').read_bytes(),b'')
            with self.assertRaises(Exception): initialize(directory,os.getuid(),os.getgid())

    def test_invalid_existing_entries_duplicates_and_oversize_deny(self):
        valid={'subject':'github_1','token_hash':'a'*64,'mode':'hosted','expires_at':1}
        for payload in [b'[{}]',json.dumps([valid,valid]).encode(),b'['+b' '*131072+b']']:
            with self.subTest(size=len(payload)), tempfile.TemporaryDirectory() as directory:
                directory=Path(directory).resolve(); directory.chmod(0o700)
                path=directory/'gateway-credentials.json'; path.write_bytes(payload); path.chmod(0o600)
                with self.assertRaises(Exception): initialize(directory,os.getuid(),os.getgid())
                self.assertEqual(path.read_bytes(),payload)
