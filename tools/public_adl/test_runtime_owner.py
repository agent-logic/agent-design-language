#!/usr/bin/env python3
"""PVF local_contract: selection refusals and argv/status transport, no real qualification.

Synthetic installed layouts exercise lightweight consumer selection. They are not
Runtime-owner package qualification. Real installed parity is a separate gate.
"""
import json
import os
from pathlib import Path
import platform
import shutil
import signal
import subprocess
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[2]
IDENTITY = 'a' * 64


class RuntimeSelection(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.build = tempfile.TemporaryDirectory(prefix='adl-owner-probe-')
        cls.addClassCleanup(cls.build.cleanup)
        root = Path(cls.build.name)
        (root / 'Cargo.toml').write_text('''[package]
name = "adl-runtime-owner-probe"
version = "0.1.0"
edition = "2021"
[dependencies]
anyhow = "1"
serde_json = "1"
[workspace]
''')
        (root / 'src').mkdir()
        (root / 'src/main.rs').write_text(
            '#[path = ' + json.dumps(str(ROOT / 'adl/src/cli/runtime_owner.rs')) + '] mod runtime_owner;\n'
            'fn main() { let a: Vec<String> = std::env::args().skip(1).collect(); '
            'let selected = runtime_owner::SelectedRuntime::from_installation(std::path::Path::new(&a[0]), &a[1]); '
            'let result = selected.and_then(|s| { if std::env::var_os("PROBE_EXEC").is_some() { s.exec(&a[2..])?; unreachable!() } else { s.invoke(&a[2..]) } }); '
            'match result { Ok(status) => std::process::exit(status.code().unwrap_or(125)), '
            'Err(e) => { eprintln!("{e:#}"); std::process::exit(2); } } }\n'
        )
        subprocess.run(['cargo', 'build', '--offline', '--manifest-path', str(root / 'Cargo.toml')],
                       check=True, capture_output=True, text=True,
                       env={**os.environ, 'CARGO_TARGET_DIR': str(root / 'target')})
        cls.binary = root / 'target/debug/adl-runtime-owner-probe'

    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix='adl-owner-layout-')
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name).resolve()
        self.prefix = self.root / 'installed'
        self.package = self.prefix / 'versions' / IDENTITY
        (self.package / 'bin').mkdir(parents=True)
        self.executable = self.package / 'bin/adl-runtime'
        self.executable.write_text('#!/bin/sh\nprintf "%s\\n" "$@"\nprintf "fixture-stderr\\n" >&2\nexit 17\n')
        self.executable.chmod(0o755)
        (self.package / 'archive.sha256').write_text(IDENTITY + '\n')
        (self.package / 'archive.tar').write_bytes(b'synthetic unqualified test receipt')
        self.manifest = {'schema': 'runtime.local-package.v1',
                         'target': {'arm64': 'aarch64'}.get(platform.machine(), platform.machine())
                         + {'Darwin': '-apple-darwin', 'Linux': '-unknown-linux-gnu'}[platform.system()],
                         'files': {'bin/adl-runtime': {'mode': 0o755, 'sha256': 'b' * 64}}}
        self.write_manifest()
        (self.prefix / 'current').symlink_to('versions/' + IDENTITY)

    def write_manifest(self):
        (self.package / 'manifest.json').write_text(json.dumps(self.manifest))

    def invoke(self, *args, identity=IDENTITY, prefix=None, execute=False):
        return subprocess.run([str(self.binary), str(prefix or self.prefix), identity, *args],
                              cwd=self.root, capture_output=True, text=True, timeout=5,
                              env={**os.environ, **({'PROBE_EXEC': '1'} if execute else {})})

    def refused(self, result):
        self.assertEqual(result.returncode, 2, result.stderr)
        self.assertEqual(result.stdout, '')
        self.assertNotIn('fixture-stderr', result.stderr)

    def test_literal_argv_standard_streams_and_nonzero_status(self):
        result = self.invoke('run', 'space ; $(literal).yaml', '--out', 'out path')
        self.assertEqual((result.returncode, result.stdout, result.stderr),
                         (17, 'run\nspace ; $(literal).yaml\n--out\nout path\n', 'fixture-stderr\n'))

    @unittest.skipUnless(os.name == 'posix', 'Unix process replacement')
    def test_exec_preserves_status_signals_cwd_and_environment(self):
        result = self.invoke('space ; $(literal)', execute=True)
        self.assertEqual((result.returncode, result.stdout, result.stderr),
                         (17, 'space ; $(literal)\n', 'fixture-stderr\n'))
        self.executable.write_text('#!/bin/sh\npwd\nprintf "%s\\n" "$PROBE_EXEC"\nkill -TERM $$\n')
        result = self.invoke(execute=True)
        self.assertEqual(result.returncode, -signal.SIGTERM)
        self.assertEqual(result.stdout, str(self.root) + '\n1\n')
        self.assertEqual(result.stderr, '')

    def test_missing_and_wrong_archive_identity(self):
        self.refused(self.invoke(identity=''))
        self.refused(self.invoke(identity='c' * 64))
        (self.package / 'archive.sha256').write_text('c' * 64)
        self.refused(self.invoke())

    def test_wrong_host_schema_and_binary_identity(self):
        for key, value in [('target', 'foreign-target'), ('schema', 'foreign-package')]:
            old = self.manifest[key]
            self.manifest[key] = value
            self.write_manifest()
            self.refused(self.invoke())
            self.manifest[key] = old
        self.manifest['files']['bin/adl-runtime']['sha256'] = 'invalid'
        self.write_manifest()
        self.refused(self.invoke())

    def test_symlink_and_unmanaged_layout_refused(self):
        alias = self.root / 'alias'
        alias.symlink_to(self.prefix, target_is_directory=True)
        self.refused(self.invoke(prefix=alias))
        (self.prefix / 'current').unlink()
        (self.prefix / 'current').mkdir()
        self.refused(self.invoke())

    def test_missing_receipt_executable_and_bad_mode(self):
        (self.package / 'archive.tar').unlink()
        self.refused(self.invoke())
        (self.package / 'archive.tar').write_bytes(b'synthetic')
        for mode in (0o644, 0o4755):
            with self.subTest(mode=oct(mode)):
                self.executable.chmod(mode)
                if self.executable.stat().st_mode & 0o7777 != mode:
                    self.skipTest("filesystem does not retain requested permission bits")
                self.refused(self.invoke())
        self.executable.unlink()
        self.refused(self.invoke())

    def test_aliased_members_refused(self):
        for name in ['manifest.json', 'archive.sha256', 'archive.tar', 'bin/adl-runtime']:
            path = self.package / name
            saved = self.root / ('saved-' + path.name)
            shutil.move(path, saved)
            path.symlink_to(saved)
            self.refused(self.invoke())
            path.unlink()
            shutil.move(saved, path)


if __name__ == '__main__':
    unittest.main()
