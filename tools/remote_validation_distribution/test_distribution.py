import copy
import gzip
import io
from pathlib import Path
import tarfile
import tempfile
import unittest
from unittest.mock import patch

import package


class DistributionTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.artifact = package.HERE / package.MANIFEST["artifact"]["name"]

    def test_frozen_git_export(self):
        package.export(package.HERE.parents[1], self.root / "export")
        self.assertEqual((self.root / "export" / self.artifact.name).read_bytes(), self.artifact.read_bytes())

    def test_verified_extraction(self):
        package.verify(self.artifact, self.root / "extract")
        for name, row in package.expected_entries().items():
            self.assertEqual(package.sha((self.root / "extract" / name).read_bytes()), row["sha256"])

    def test_tampered_payload_and_identity(self):
        damaged = bytearray(self.artifact.read_bytes())
        damaged[-1] ^= 1
        path = self.root / "bad.tar.gz"
        path.write_bytes(damaged)
        with self.assertRaisesRegex(ValueError, "digest"):
            package.verify(path, self.root / "never-created")
        self.assertFalse((self.root / "never-created").exists())
        wrong = copy.deepcopy(package.MANIFEST)
        wrong["version"] = "9.9.9"
        with patch.object(package, "MANIFEST", wrong), self.assertRaisesRegex(ValueError, "identity"):
            package.verify(self.artifact)

    def test_unsafe_archive_entries(self):
        original = package.inspect_archive(self.artifact.read_bytes())
        for kind in ["traversal", "symlink", "duplicate", "missing", "altered"]:
            with self.subTest(kind=kind):
                raw = io.BytesIO()
                rows = list(original.items())
                if kind == "missing":
                    rows.pop()
                if kind == "duplicate":
                    rows.append(rows[0])
                with tarfile.open(fileobj=raw, mode="w") as archive:
                    for index, (name, data) in enumerate(rows):
                        if index == 0 and kind == "traversal":
                            name = "../outside"
                        if index == 0 and kind == "altered":
                            data = bytes([data[0] ^ 1]) + data[1:]
                        member = tarfile.TarInfo(name)
                        member.size = len(data)
                        if index == 0 and kind == "symlink":
                            member.type, member.linkname, member.size = tarfile.SYMTYPE, "/outside", 0
                        archive.addfile(member, io.BytesIO(data))
                with self.assertRaises(ValueError):
                    package.inspect_archive(gzip.compress(raw.getvalue()))

    def test_existing_destination_preserved(self):
        dest = self.root / "existing"
        dest.mkdir()
        sentinel = dest / "sentinel"
        sentinel.write_text("preserve")
        with self.assertRaises(FileExistsError):
            package.verify(self.artifact, dest)
        self.assertEqual(sentinel.read_text(), "preserve")
        with self.assertRaises(FileExistsError):
            package.export(package.HERE.parents[1], dest)
