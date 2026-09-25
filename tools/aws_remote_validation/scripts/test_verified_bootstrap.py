#!/usr/bin/env python3
"""PVF: deterministic local contract/negative regression; no cloud or installs."""
import hashlib
import importlib.util
import io
import json
from pathlib import Path
import subprocess
import tarfile
import tempfile
import unittest
from unittest.mock import patch

SPEC = importlib.util.spec_from_file_location('bootstrap', Path(__file__).with_name('verified_bootstrap.py'))
b = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(b)
TARGET = 'x86_64-unknown-linux-gnu'


class BootstrapTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name)
        self.archive = self.root / 'tool.tar.gz'
        self.make_archive([('pkg/sccache', b'approved binary', tarfile.REGTYPE)])
        self.record = {'version': '0.16.0', 'sha256': hashlib.sha256(self.archive.read_bytes()).hexdigest(),
                       'source': 'https', 'url': 'https://example.invalid/v0.16.0/sccache.tar.gz'}
        self.manifest = self.root / 'identities.json'
        self.save()
        self.dest = self.root / 'bin'

    def save(self, binary='sccache'):
        self.manifest.write_text(json.dumps({'schema': 'adl.remote-bootstrap.v1', 'tools': {binary: {TARGET: self.record}}}))

    def make_archive(self, members):
        with tarfile.open(self.archive, 'w:gz') as bundle:
            for name, payload, kind in members:
                item = tarfile.TarInfo(name)
                item.type = kind
                item.size = len(payload) if kind == tarfile.REGTYPE else 0
                item.linkname = '../elsewhere' if kind in (tarfile.SYMTYPE, tarfile.LNKTYPE) else ''
                bundle.addfile(item, io.BytesIO(payload))

    def install(self, url=''):
        def download(record, path):
            path.write_bytes(self.archive.read_bytes())
        with patch.object(b, 'download', side_effect=download):
            b.install(self.manifest, 'sccache', TARGET, self.dest, url)

    def test_verified_archive_installs(self):
        self.install(self.record['url'])
        self.assertEqual((self.dest / 'sccache').read_bytes(), b'approved binary')
        self.assertEqual((self.dest / 'sccache').stat().st_mode & 0o777, 0o755)

    def test_bad_digest_preserves_existing_binary(self):
        self.dest.mkdir()
        (self.dest / 'sccache').write_bytes(b'existing')
        self.record['sha256'] = '0' * 64
        self.save()
        with self.assertRaises(ValueError):
            self.install()
        self.assertEqual((self.dest / 'sccache').read_bytes(), b'existing')
        self.assertEqual(list(self.dest.iterdir()), [self.dest / 'sccache'])

    def test_rejected_identity_never_downloads(self):
        for key, value in [('version', 'latest'), ('version', 'stable'), ('sha256', ''),
                           ('sha256', 'X' * 64), ('source', 'file'), ('url', 'http://example.invalid/tool'),
                           ('url', 'https://example.invalid/releases/latest/tool'),
                           ('url', 'https://user:secret@example.invalid/tool')]:
            with self.subTest(key=key, value=value), patch.object(b, 'download') as download:
                saved = self.record.copy()
                self.record[key] = value
                self.save()
                with self.assertRaises(ValueError):
                    b.install(self.manifest, 'sccache', TARGET, self.dest)
                download.assert_not_called()
                self.record = saved

    def test_wrong_configured_url_never_downloads(self):
        with patch.object(b, 'download') as download, self.assertRaises(ValueError):
            b.install(self.manifest, 'sccache', TARGET, self.dest, 'https://other.invalid/tool')
        download.assert_not_called()

    def test_missing_identity_rejected(self):
        with self.assertRaises(KeyError):
            b.identity(self.manifest, 'cargo-nextest', TARGET)

    def test_wrong_architecture_rejected(self):
        with self.assertRaises(KeyError):
            b.identity(self.manifest, 'sccache', 'aarch64-unknown-linux-gnu')

    def test_unknown_schema_rejected(self):
        self.manifest.write_text('{"schema":"unknown"}')
        with self.assertRaises(ValueError):
            b.identity(self.manifest, 'sccache', TARGET)

    def test_malformed_archive_rejected(self):
        self.archive.write_bytes(b'not a tarball')
        with self.assertRaises(tarfile.TarError):
            b.extract_binary(self.archive, 'sccache', self.root / 'output')

    def test_unsafe_archive_members_rejected(self):
        for name, kind in [('../escape', tarfile.REGTYPE), ('/absolute', tarfile.REGTYPE),
                           ('dir\\escape', tarfile.REGTYPE), ('link', tarfile.SYMTYPE),
                           ('link', tarfile.LNKTYPE), ('device', tarfile.CHRTYPE), ('pipe', tarfile.FIFOTYPE)]:
            with self.subTest(name=name, kind=kind):
                self.make_archive([('pkg/sccache', b'ok', tarfile.REGTYPE), (name, b'x', kind)])
                with self.assertRaises(ValueError):
                    b.extract_binary(self.archive, 'sccache', self.root / 'output')
                self.assertFalse((self.root / 'output').exists())

    def test_duplicate_binary_rejected(self):
        self.make_archive([('a/sccache', b'a', tarfile.REGTYPE), ('b/sccache', b'b', tarfile.REGTYPE)])
        with self.assertRaises(ValueError):
            b.extract_binary(self.archive, 'sccache', self.root / 'output')

    def test_duplicate_member_rejected(self):
        self.make_archive([('sccache', b'a', tarfile.REGTYPE), ('sccache', b'b', tarfile.REGTYPE)])
        with self.assertRaises(ValueError):
            b.extract_binary(self.archive, 'sccache', self.root / 'output')

    def test_missing_binary_rejected(self):
        self.make_archive([('other', b'a', tarfile.REGTYPE)])
        with self.assertRaises(ValueError):
            b.extract_binary(self.archive, 'sccache', self.root / 'output')

    def test_expansion_bound(self):
        with patch.object(b, 'MAX_ARCHIVE', 2), self.assertRaises(ValueError):
            b.extract_binary(self.archive, 'sccache', self.root / 'output')

    def test_s3_requires_real_version(self):
        self.record.update(source='s3', bucket='bucket', key='tools/sccache.tar.gz')
        for version in ['', 'null']:
            with self.subTest(version=version):
                self.record['version_id'] = version
                self.save()
                with self.assertRaises(ValueError):
                    b.identity(self.manifest, 'sccache', TARGET)

    def test_s3_download_uses_exact_version(self):
        self.record.update(source='s3', bucket='bucket', key='tools/sccache.tar.gz', version_id='exact-v1')
        self.save()
        with patch.object(b.subprocess, 'run') as run:
            b.download(b.identity(self.manifest, 'sccache', TARGET), self.archive)
        self.assertEqual(run.call_args.args[0], ['aws', 's3api', 'get-object', '--bucket', 'bucket', '--key', 'tools/sccache.tar.gz', '--version-id', 'exact-v1', str(self.archive)])

    def test_https_redirects_cannot_downgrade(self):
        with patch.object(b.subprocess, 'run') as run:
            b.download(self.record, self.archive)
        args = run.call_args.args[0]
        self.assertEqual(args[args.index('--proto-redir') + 1], '=https')

    def test_transport_error_never_installs(self):
        with patch.object(b, 'download', side_effect=subprocess.CalledProcessError(1, 'curl')):
            with self.assertRaises(subprocess.CalledProcessError):
                b.install(self.manifest, 'sccache', TARGET, self.dest)
        self.assertFalse((self.dest / 'sccache').exists())

    def test_rustup_hash_before_execution(self):
        self.record.update(toolchain='1.92.0', sha256='0' * 64)
        self.save('rustup-init')
        with patch.object(b, 'download', side_effect=lambda record, path: path.write_bytes(b'installer')):
            with patch.object(b.subprocess, 'run') as run, self.assertRaises(ValueError):
                b.install(self.manifest, 'rustup-init', TARGET, self.dest)
            run.assert_not_called()

    def test_rustup_exact_toolchain(self):
        self.record.update(toolchain='1.92.0', sha256=hashlib.sha256(b'installer').hexdigest())
        self.save('rustup-init')
        with patch.object(b, 'download', side_effect=lambda record, path: path.write_bytes(b'installer')):
            with patch.object(b.subprocess, 'run') as run:
                b.install(self.manifest, 'rustup-init', TARGET, self.dest)
            self.assertEqual(run.call_args.args[0][1:], ['-y', '--profile', 'minimal', '--default-toolchain', '1.92.0'])

    def test_rustup_floating_toolchain_rejected(self):
        self.record['toolchain'] = 'stable'
        self.save('rustup-init')
        with self.assertRaises(ValueError):
            b.identity(self.manifest, 'rustup-init', TARGET)

    def test_digest_checked_before_archive_parsing(self):
        self.record['sha256'] = '0' * 64
        self.save()
        with patch.object(b, 'extract_binary') as extract, self.assertRaises(ValueError):
            self.install()
        extract.assert_not_called()

    def test_missing_manifest_never_downloads(self):
        self.manifest.unlink()
        with patch.object(b, 'download') as download, self.assertRaises(FileNotFoundError):
            b.install(self.manifest, 'sccache', TARGET, self.dest)
        download.assert_not_called()

    def test_empty_binary_rejected(self):
        self.make_archive([('sccache', b'', tarfile.REGTYPE)])
        with self.assertRaises(ValueError):
            b.extract_binary(self.archive, 'sccache', self.root / 'output')

    def test_runner_rejection_never_reaches_package_manager(self):
        runner = Path(__file__).with_name('remote_validation_runner.sh').read_text()
        functions = runner[runner.index('ensure_validation_binary() {'):runner.index('export HOME=')]
        script = '''set -eu
BOOTSTRAP_IDENTITIES=/absent
ADL_BOOTSTRAP_IDENTITIES=explicit
install_binary_from_archive_path() { return 1; }
install_package_manager_binary() { echo UNSAFE_FALLBACK; }
log_progress() { :; }
''' + functions + '\nensure_validation_binary sccache https://example.invalid/tool\n'
        result = subprocess.run(['bash', '-c', script], capture_output=True, text=True)
        self.assertNotEqual(result.returncode, 0)
        self.assertNotIn('UNSAFE_FALLBACK', result.stdout)


if __name__ == '__main__':
    unittest.main()
