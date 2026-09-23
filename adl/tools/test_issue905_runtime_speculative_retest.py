#!/usr/bin/env python3
"""PVF lane: local-unit; role: #905 safety regression; deterministic;
resource profile: local CPU/filesystem with mocked Ollama, no service or model load;
release gate: required for the #905 harness publication.
"""

import json
from pathlib import Path
import sys
from types import ModuleType, SimpleNamespace
import tempfile
import unittest
from unittest.mock import patch

lifecycle = ModuleType("issue855_provider_lifecycle")


def write_json(path: Path, payload: dict) -> Path:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(payload, indent=2) + "\n")
    return path


lifecycle.write = write_json
original_lifecycle = sys.modules.get("issue855_provider_lifecycle")
sys.modules["issue855_provider_lifecycle"] = lifecycle

import issue905_runtime_speculative_retest as retest

if original_lifecycle is None:
    del sys.modules["issue855_provider_lifecycle"]
else:
    sys.modules["issue855_provider_lifecycle"] = original_lifecycle


class RuntimeRetestSafetyTests(unittest.TestCase):
    def test_unknown_and_malformed_base_blob_cannot_prove_same_weights(self):
        for digest in (None, "", "sha256-" + "a" * 64, "g" * 64, "a" * 63):
            for baseline, speculative in ((digest, digest), ("a" * 64, digest), (digest, "a" * 64)):
                with self.subTest(baseline=baseline, speculative=speculative), self.assertRaisesRegex(
                    AssertionError, "same-model comparison is not proven"
                ):
                    retest.validate_base_blob_identity(
                        {"base_blob_sha256": baseline}, {"base_blob_sha256": speculative})
        retest.validate_base_blob_identity({"base_blob_sha256": "a" * 64}, {"base_blob_sha256": "a" * 64})
        with self.assertRaisesRegex(AssertionError, "differs at base_blob_sha256"):
            retest.validate_base_blob_identity({"base_blob_sha256": "a" * 64}, {"base_blob_sha256": "b" * 64})

    def test_model_metadata_without_supported_from_is_non_proving(self):
        for modelfile in (None, "FROM alias-a", "FROM C:\\models\\sha256-" + "a" * 64,
                          "FROM /models/sha256-invalid"):
            shown = {} if modelfile is None else {"modelfile": modelfile}
            with self.subTest(modelfile=modelfile), patch.object(retest, "ollama_json", side_effect=[
                shown, {"models": [{"name": "fixture:latest", "digest": "manifest", "size": 1}]}]):
                identity = retest.model_identity("fixture:latest")
            with self.assertRaisesRegex(AssertionError, "same-model comparison is not proven"):
                retest.validate_base_blob_identity(identity, identity)
        with patch.object(retest, "ollama_json", side_effect=[
            {"modelfile": "FROM /models/sha256-" + "a" * 64},
            {"models": [{"name": "fixture:latest", "digest": "manifest", "size": 1}]}]):
            identity = retest.model_identity("fixture:latest")
        retest.validate_base_blob_identity(identity, identity)

    def argv(self, root: Path, *, source: str = "Qwen3.5:9b", baseline: str = "adl-905-arm-a:latest", speculative: str = "adl-905-arm-b:latest", repeats: int = 2) -> list[str]:
        binaries = []
        for name in ("csm", "csmctl", "guardian", "kernel", "vector"):
            path = root / name
            path.write_text("fixture")
            binaries.extend((f"--{name}", str(path)))
        return [
            "retest",
            *binaries,
            "--output", str(root / "run"),
            "--source-revision", "fixture-head",
            "--source-model", source,
            "--baseline-model", baseline,
            "--speculative-model", speculative,
            "--repeats", str(repeats),
        ]

    def test_run_scoped_alias_preserves_registry_and_tag(self):
        self.assertEqual(
            retest.run_scoped_model_name("registry.example/team/model:9b", "run123"),
            "registry.example/team/model-run123:9b",
        )
        self.assertEqual(
            retest.run_scoped_model_name("team/model", "run123"),
            "team/model-run123:latest",
        )

    def test_aliases_reject_source_existing_and_duplicate_names(self):
        existing = {"qwen3.5:9b", "occupied:latest"}
        cases = (
            ("Qwen3.5:9b", ("Qwen3.5:9b", "fresh-b", "fresh-invalid")),
            ("Qwen3.5:9b", ("occupied", "fresh-b", "fresh-invalid")),
            ("Qwen3.5:9b", ("same", "SAME:latest", "fresh-invalid")),
        )
        for source, aliases in cases:
            with self.subTest(aliases=aliases), self.assertRaises(AssertionError):
                retest.validate_model_aliases(source, aliases, existing)

    def test_existing_requested_alias_is_replaced_by_run_scoped_name(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            argv = self.argv(root, baseline="occupied:latest")
            cleanup_calls = []

            def subprocess_run(args, **_kwargs):
                cleanup_calls.append(args)
                return SimpleNamespace(returncode=0, stdout="", stderr="")

            with patch("sys.argv", argv), \
                    patch.object(retest.secrets, "token_hex", return_value="run123"), \
                    patch.object(retest, "installed_model_names", return_value={"qwen3.5:9b", "occupied:latest"}), \
                    patch.object(retest, "create_model", side_effect=RuntimeError("stop after collision proof")) as create, \
                    patch.object(retest.subprocess, "run", side_effect=subprocess_run):
                with self.assertRaises(RuntimeError):
                    retest.main()

            create.assert_called_once()
            self.assertEqual(create.call_args.args[0], "occupied-run123:latest")
            self.assertEqual(cleanup_calls, [["ollama", "rm", "occupied-run123:latest"]])
            report = json.loads((root / "run/report.json").read_text())
            self.assertEqual(report["result"], "failed")
            self.assertEqual(report["error_class"], "RuntimeError")
            self.assertEqual(
                report["cleanup"]["models"],
                [{"name": "occupied-run123:latest", "removed": True}],
            )

    def test_identity_setup_failure_records_report_and_removes_run_owned_aliases(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            argv = self.argv(root)
            removed = []

            def create(name, _modelfile, allow_failure=False):
                return (1, "expected invalid") if allow_failure else (0, "")

            def subprocess_run(args, **_kwargs):
                if args[:2] == ["ollama", "rm"]:
                    removed.append(args[2])
                return SimpleNamespace(returncode=0, stdout="", stderr="")

            with patch("sys.argv", argv), \
                    patch.object(retest.secrets, "token_hex", return_value="run123"), \
                    patch.object(retest, "installed_model_names", return_value={"qwen3.5:9b"}), \
                    patch.object(retest, "create_model", side_effect=create), \
                    patch.object(retest, "model_identity", side_effect=RuntimeError("fixture identity failure")), \
                    patch.object(retest.subprocess, "run", side_effect=subprocess_run):
                with self.assertRaises(RuntimeError):
                    retest.main()

            self.assertEqual(
                removed,
                [
                    "adl-905-invalid-draft-run123:latest",
                    "adl-905-arm-b-run123:latest",
                    "adl-905-arm-a-run123:latest",
                ],
            )
            report = json.loads((root / "run/report.json").read_text())
            self.assertEqual(report["result"], "failed")
            self.assertEqual(report["error_class"], "RuntimeError")
            self.assertEqual(
                [item["name"] for item in report["cleanup"]["models"]],
                removed,
            )
            self.assertTrue(all(item["removed"] for item in report["cleanup"]["models"]))

    def test_unknown_base_blob_fails_run_before_runtime_start(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            with patch("sys.argv", self.argv(root)), \
                    patch.object(retest, "installed_model_names", return_value={"qwen3.5:9b"}), \
                    patch.object(retest, "create_model", side_effect=lambda _name, _file, allow_failure=False:
                                 (1, "expected invalid") if allow_failure else (0, "")), \
                    patch.object(retest, "model_identity", return_value={"base_blob_sha256": None}), \
                    patch.object(retest.subprocess, "run", return_value=SimpleNamespace(returncode=0)), \
                    patch.object(retest.subprocess, "Popen") as launch:
                with self.assertRaisesRegex(AssertionError, "same-model comparison is not proven"):
                    retest.main()
            launch.assert_not_called()
            report = json.loads((root / "run/report.json").read_text())
            self.assertEqual(report["result"], "failed")
            self.assertNotIn("same_base_blob", json.dumps(report))

    def test_guardian_kill_error_is_recorded_without_escaping_cleanup(self):
        guardian = SimpleNamespace(pid=905)
        guardian.wait = unittest.mock.Mock(
            side_effect=retest.subprocess.TimeoutExpired("guardian", 20)
        )
        with patch.object(retest.os, "killpg", side_effect=(None, PermissionError("denied"))):
            result = retest.cleanup({"guardian": guardian}, [])
        self.assertEqual(result["models"], [])
        self.assertEqual(
            result["errors"],
            [{"resource": "guardian", "error_class": "PermissionError"}],
        )

    def test_nonpositive_repeats_records_failure_report(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            argv = self.argv(root, repeats=0)
            with patch("sys.argv", argv), patch.object(retest, "cleanup", side_effect=PermissionError("cleanup denied")):
                with self.assertRaises(AssertionError):
                    retest.main()
            report = json.loads((root / "run/report.json").read_text())
            self.assertEqual(report["result"], "failed")
            self.assertEqual(report["error_class"], "AssertionError")
            self.assertEqual(
                report["cleanup"]["errors"],
                [{"resource": "cleanup", "error_class": "PermissionError"}],
            )

    def test_ambiguous_create_timeout_removes_run_owned_alias(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            argv = self.argv(root)
            removed = []

            def subprocess_run(args, **_kwargs):
                if args[:2] == ["ollama", "rm"]:
                    removed.append(args[2])
                return SimpleNamespace(returncode=0, stdout="", stderr="")

            with patch("sys.argv", argv), \
                    patch.object(retest.secrets, "token_hex", return_value="run123"), \
                    patch.object(retest, "installed_model_names", return_value={"qwen3.5:9b"}), \
                    patch.object(retest, "create_model", side_effect=retest.subprocess.TimeoutExpired("ollama create", 120)), \
                    patch.object(retest.subprocess, "run", side_effect=subprocess_run):
                with self.assertRaises(retest.subprocess.TimeoutExpired):
                    retest.main()

            self.assertEqual(removed, ["adl-905-arm-a-run123:latest"])
            report = json.loads((root / "run/report.json").read_text())
            self.assertEqual(report["result"], "failed")
            self.assertEqual(report["error_class"], "TimeoutExpired")
            self.assertEqual(report["cleanup"]["models"][0]["name"], removed[0])

    def test_resource_accessor_error_cannot_skip_owned_model_cleanup(self):
        class BrokenResource:
            @property
            def server(self):
                raise RuntimeError("broken resource")

        removed = []

        def subprocess_run(args, **_kwargs):
            removed.append(args[2])
            return SimpleNamespace(returncode=0, stdout="", stderr="")

        with patch.object(retest.subprocess, "run", side_effect=subprocess_run):
            result = retest.cleanup({"fixture": BrokenResource()}, ["owned-run123:latest"])

        self.assertEqual(removed, ["owned-run123:latest"])
        self.assertEqual(
            result["errors"],
            [{"resource": "resource_cleanup", "error_class": "RuntimeError"}],
        )
        self.assertTrue(result["models"][0]["removed"])


if __name__ == "__main__":
    unittest.main()
