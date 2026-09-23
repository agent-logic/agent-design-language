#!/usr/bin/env python3
"""PVF: deterministic local contract regression; tiny CPU/disk; no network.
Proof role: real scaffold CLI output and failure semantics, not dependency audit.
Release gate: focused #1160 pre-publication proof; no CI ownership claimed.
"""
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

SCRIPT = Path(__file__).with_name("prepare_dependency_review.py")


class ScaffoldTests(unittest.TestCase):
    def run_packet(self, evidence=None, raw=None, missing=False):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            if not missing:
                (root / "evidence_index.json").write_text(
                    raw if raw is not None else json.dumps({"evidence": evidence}))
            result = subprocess.run([sys.executable, str(SCRIPT), str(root)],
                                    capture_output=True, text=True)
            output = root / "dependency-review/dependency_review_scaffold.json"
            return result, json.loads(output.read_text()) if output.exists() else None

    def test_lifecycle_locks_cannot_displace_material_inventory(self):
        noise = [{"path": f".git/csdlc/locks/{i}.lock", "category": "lockfile",
                  "specialist_lanes": ["dependency"]} for i in range(120)]
        material = [{"path": f"crates/c{i}/{name}"} for i in range(100)
                    for name in ("Cargo.toml", "Cargo.lock")]
        result, data = self.run_packet(noise + material)
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(data["inventory"]["material_path_count"], 200)
        self.assertEqual(data["inventory"]["packet_entry_count"], 320)
        self.assertEqual(data["inventory"]["excluded_non_dependency_lock_count"], 120)
        self.assertEqual(data["inventory"]["omitted_selected_entry_count"], 0)
        self.assertEqual(set(data["dependency_surface_map"]["manifest"]),
                         {item["path"] for item in material if item["path"].endswith(".toml")})
        self.assertEqual(len(data["candidate_supply_chain_findings"]), 200)

    def test_misleading_names_and_broad_categories_do_not_imply_dependencies(self):
        paths = ["docs/clock.md", "src/circuit.py", "tests/test_clock.py",
                 "docs/manifesto.md", "src/package_handler.py", "src/noticeboard.py"]
        result, data = self.run_packet([{ "path": path, "category": "code",
                                        "reason": "dependency lock ci install"} for path in paths])
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(data["dependency_evidence"], [])
        self.assertEqual(data["inventory"]["status"], "no_material_evidence_in_packet")
        self.assertFalse(data["inventory"]["review_performed"])

    def test_ecosystems_and_explicit_custom_evidence(self):
        paths = ["Cargo.toml", "Cargo.lock", "package.json", "pnpm-lock.yaml",
                 "pyproject.toml", "uv.lock", "go.mod", "go.sum", "Gemfile",
                 "Gemfile.lock", "composer.json", "composer.lock", "build.gradle.kts",
                 "gradle.lockfile", "requirements-dev.txt", "app.csproj",
                 "compose.yaml", ".cargo/config.toml", "ci/Dockerfile.builder",
                 ".github/workflows/build.yml", "tests/test_dependency_install.py"]
        result, data = self.run_packet([{"path": p} for p in paths] + [
            {"path": "custom/platform.cfg", "specialist_lanes": ["dependencies"]}])
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(data["inventory"]["selected_entry_count"], len(paths) + 1)
        self.assertIn("custom/platform.cfg", data["dependency_surface_map"]["dependency_related"])
        self.assertIn("tests/test_dependency_install.py", data["dependency_surface_map"]["dependency_test_surface"])

    def test_empty_packet_is_explicitly_non_proving(self):
        result, data = self.run_packet([])
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(data["inventory"]["packet_entry_count"], 0)
        self.assertEqual(data["inventory"]["status"], "no_material_evidence_in_packet")

    def test_missing_malformed_or_invalid_index_fails(self):
        for kwargs in [{"missing": True}, {"raw": "{"}, {"raw": "{}"},
                       {"evidence": [{}]}, {"evidence": [{"path": 3}]},
                       {"evidence": [{"path": "  "}]}]:
            with self.subTest(kwargs=kwargs):
                result, data = self.run_packet(**kwargs)
                self.assertNotEqual(result.returncode, 0)
                self.assertIsNone(data)

    def test_order_independent_output_and_distinct_path_denominator(self):
        entries = [{"path": "z/Cargo.lock"}, {"path": "a/Cargo.toml"},
                   {"path": "a/Cargo.toml"}]
        _, first = self.run_packet(entries)
        _, second = self.run_packet(list(reversed(entries)))
        self.assertEqual(first["dependency_evidence"], second["dependency_evidence"])
        self.assertEqual(first["inventory"]["selected_entry_count"], 3)
        self.assertEqual(first["inventory"]["selected_unique_path_count"], 2)
        self.assertEqual(first["inventory"]["material_path_count"], 2)


if __name__ == "__main__":
    unittest.main()
