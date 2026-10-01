"""PVF fast deterministic data-contract regressions; offline, no Runtime proof."""
import tempfile
from pathlib import Path
import unittest
import package


class DistributionTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name).resolve() / 'installed'
        self.archive = package.HERE / package.LOCK['artifact']['name']
        package.verify(self.archive, self.root)

    def test_standalone_consumer_reads_exact_seven_inputs(self):
        package.verify_root(self.root)
        rows = package.manifest()['files']
        self.assertEqual(7, len([r for r in rows if r['path'].startswith('adl-spec/')]))
        for row in rows:
            self.assertEqual(row['sha256'], package.sha((self.root / row['path']).read_bytes()))

    def test_reproducible_archive_bytes(self):
        payloads = package.inspect_archive(self.archive.read_bytes())
        self.assertEqual(self.archive.read_bytes(), package.pack(payloads))

    def test_changed_payload_refuses(self):
        (self.root / package.manifest()['files'][0]['path']).write_bytes(b'{}')
        with self.assertRaises(ValueError):
            package.verify_root(self.root)

    def test_missing_input_refuses(self):
        (self.root / package.manifest()['files'][1]['path']).unlink()
        with self.assertRaises(ValueError):
            package.verify_root(self.root)

    def test_symlink_input_refuses(self):
        target = self.root / package.manifest()['files'][0]['path']
        data = target.read_bytes(); target.unlink()
        outside = self.root.parent / 'aliased'; outside.write_bytes(data)
        target.symlink_to(outside)
        with self.assertRaises(ValueError):
            package.verify_root(self.root)

    def test_manifest_and_unexpected_file_refuse(self):
        (self.root / 'source-manifest.json').write_bytes(b'{}')
        with self.assertRaises(ValueError):
            package.verify_root(self.root)
        (self.root / 'source-manifest.json').write_bytes(package.MANIFEST_BYTES)
        (self.root / 'extra').write_bytes(b'')
        with self.assertRaises(ValueError):
            package.verify_root(self.root)

    def test_tampered_archive_never_creates_destination(self):
        altered = self.root.parent / 'tampered.tar.gz'
        altered.write_bytes(self.archive.read_bytes() + b'changed')
        output = self.root.parent / 'must-not-exist'
        with self.assertRaises(ValueError):
            package.verify(altered, output)
        self.assertFalse(output.exists())

    def test_installed_root_required_and_create_only(self):
        with self.assertRaises(ValueError):
            package.verify_root(self.root.parent / 'absent')
        with self.assertRaises(FileExistsError):
            package.verify(self.archive, self.root)


if __name__ == '__main__':
    unittest.main()
