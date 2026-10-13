#!/usr/bin/env python3
"""PVF: deterministic local Python contract; fixture-only, no native execution.
Proof role: historical manifest selection/refusal; resource: local temporary files;
release gate: focused source-retirement regression, not rehearsal acceptance.
"""
import importlib.util
from pathlib import Path
import tempfile
import unittest

SPEC = importlib.util.spec_from_file_location(
    "issue872_runner", Path(__file__).with_name("run_issue872_conversion_rehearsal.py"))
assert SPEC and SPEC.loader
RUNNER = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(RUNNER)
MANIFEST = Path(__file__).parent / "fixtures/issue872/csdlc-v3-Cargo.toml.historical"


class HistoricalManifestTests(unittest.TestCase):
    def test_original_manifest_precedes_fallback(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            original = root / "csdlc-v3/Cargo.toml"
            original.parent.mkdir()
            original.write_text("original fixture manifest\n")
            fallback = root / "fallback"
            fallback.write_text("invalid fallback\n")
            self.assertEqual(RUNNER.select_fault_manifest(root, fallback), original)

    def test_exact_retained_fallback(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            fallback = root / "historical.toml"
            fallback.write_bytes(MANIFEST.read_bytes())
            selected = RUNNER.historical_manifest_from_request(
                {"historical_manifest_path": str(fallback)}, root)
            self.assertEqual(RUNNER.select_fault_manifest(root, selected), fallback)

    def test_missing_fallback_refuses(self):
        with tempfile.TemporaryDirectory() as tmp:
            with self.assertRaisesRegex(ValueError, "historical_manifest_path is required"):
                RUNNER.select_fault_manifest(Path(tmp), None)

    def test_wrong_bytes_refuse(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            fallback = root / "historical.toml"
            fallback.write_text("not the preserved manifest\n")
            with self.assertRaisesRegex(ValueError, "provenance mismatch"):
                RUNNER.select_fault_manifest(root, fallback)

    def test_request_outside_fixture_refuses(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            fixture = root / "fixture"
            fixture.mkdir()
            outside = root / "historical.toml"
            outside.write_bytes(MANIFEST.read_bytes())
            with self.assertRaisesRegex(ValueError, "must remain inside isolated fixture"):
                RUNNER.historical_manifest_from_request(
                    {"historical_manifest_path": str(outside)}, fixture)


if __name__ == "__main__":
    unittest.main()
