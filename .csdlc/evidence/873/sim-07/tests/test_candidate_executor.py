#!/usr/bin/env python3
from __future__ import annotations

import importlib.util
import json
from pathlib import Path
import subprocess
import sys
import tempfile
from types import SimpleNamespace
import unittest
from unittest import mock


HERE = Path(__file__).resolve().parent
EVIDENCE = HERE.parent
sys.path.insert(0, str(EVIDENCE))
SPEC = importlib.util.spec_from_file_location(
    "run_candidate_retry_journeys", EVIDENCE / "run_candidate_retry_journeys.py"
)
MODULE = importlib.util.module_from_spec(SPEC)
assert SPEC.loader is not None
SPEC.loader.exec_module(MODULE)


class CandidateExecutorTests(unittest.TestCase):
    def test_common_fixture_is_captured_without_authentic_state_or_source_reset(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory) / "captured"
            def capture(harness, slot, test_filter, destination):
                self.assertEqual(test_filter, "installed_prepare_bind_edit_and_observations_use_canonical_context")
                plan = destination / ".git/installed-candidate/plan.json"
                plan.parent.mkdir(parents=True)
                plan.write_text(json.dumps({"publication": {"body": "Closes #505"}}))
            binary = Path(directory) / "csdlc"
            binary.write_text("frozen-binary")
            args = SimpleNamespace(fixture=root, scenario="primary-linked-edit", harness=Path("harness"), slot=Path("slot"), binary=binary)
            with mock.patch.object(MODULE.common, "capture_fixture", side_effect=capture), mock.patch.object(MODULE.common, "relocate_fixture"), mock.patch.object(MODULE.shutil, "copytree") as copy, mock.patch.object(MODULE.subprocess, "run") as run:
                self.assertEqual(MODULE.prepare_isolated_fixture(args, authentic_adoption=False), root.resolve())
                copy.assert_not_called()
                run.assert_not_called()
            self.assertEqual(json.loads((root / ".git/installed-candidate/plan.json").read_text())["publication"]["body"], "Closes #1505")
            self.assertFalse((root / ".csdlc").exists())

    def test_candidate_fake_remote_does_not_initialize_lifecycle_evidence(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            transport = root / ".git/installed-candidate/fake-remote"
            MODULE.common.configure_fake_remote(root, "a" * 40, 1505, transport_root=transport)
            self.assertFalse((root / ".csdlc").exists())
            self.assertTrue((transport / "fake-bin/curl").is_file())
            for name in ("graphql-open.json", "graphql-merged.json"):
                observed = json.loads((transport / name).read_text())
                pull = observed["data"]["repository"]["pullRequest"]
                self.assertEqual(pull["body"], "Closes #1505")
                self.assertEqual(pull["closingIssuesReferences"]["nodes"][0]["number"], 1505)

    def test_declared_candidate_common_corpus_is_exactly_thirteen_and_nineteen(self):
        scenario_map = json.loads((EVIDENCE / "retry-scenario-map.json").read_text())
        scenarios = {value["id"]: value for value in scenario_map["scenarios"]}
        self.assertEqual(len(scenarios["primary-linked-edit"]["semantic_steps"]), 13)
        self.assertEqual(len(scenarios["terminal-journey"]["semantic_steps"]), 19)
        for scenario in scenarios.values():
            facts = scenario["relevant_facts"]["initial_fixture_facts"]
            self.assertEqual(facts["issue_number"], 1505)
            self.assertEqual(facts["lifecycle_phase"], "unprepared")
            self.assertNotIn("candidate_issue_number", facts)
            self.assertNotIn("predecessor_issue_number", facts)
            for step in scenario["semantic_steps"]:
                self.assertNotIn("874", step["argv"]["candidate"])
        self.assertEqual(
            [value["id"] for value in scenarios["terminal-journey"]["semantic_steps"][-6:]],
            [
                "cleanup-foreign-dirty-guard", "cleanup-tracked-dirty-guard",
                "cleanup-preview", "cleanup-stale-preview-guard",
                "cleanup-refresh-preview", "cleanup-execute",
            ],
        )

    def test_candidate_topology_keeps_bound_cleanup_target_and_dirty_detached_peer(self):
        with tempfile.TemporaryDirectory() as directory:
            base = Path(directory)
            primary = base / "primary"
            worktrees = base / "worktrees"
            linked = worktrees / "adl-issue-1505-installed-intent-fixture"
            logs = base / "logs"
            primary.mkdir()
            logs.mkdir()
            subprocess.run(["git", "init", "-q", str(primary)], check=True)
            subprocess.run(["git", "-C", str(primary), "config", "user.name", "fixture"], check=True)
            subprocess.run(["git", "-C", str(primary), "config", "user.email", "fixture@example.invalid"], check=True)
            (primary / ".adl").mkdir()
            (primary / ".adl/worktree-policy.json").write_text(json.dumps({"required_parent": str(worktrees)}))
            (primary / "tracked").write_text("baseline\n")
            subprocess.run(["git", "-C", str(primary), "add", "."], check=True)
            subprocess.run(["git", "-C", str(primary), "commit", "-qm", "baseline"], check=True)
            worktrees.mkdir()
            subprocess.run(["git", "-C", str(primary), "worktree", "add", "-q", "-b", "codex/1505-fixture", str(linked), "HEAD"], check=True)
            for relative in (
                ".csdlc/evidence/1505/proof.json",
                ".csdlc/issues/1505/sip.md",
                ".csdlc/transactions/completed/1505/prepare.json",
                ".csdlc/v3/issues/1505/state.json",
            ):
                path = linked / relative
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_text("{}\n", encoding="utf-8")

            cleanup_target, topology = MODULE.move_bound_worktree_to_cleanup_control(primary, linked, logs)
            retained_peer = worktrees / "cleanup-control"

            self.assertEqual(cleanup_target, linked)
            self.assertTrue(retained_peer.is_dir())
            self.assertTrue(linked.is_dir())
            self.assertEqual(topology["baseline_head"], topology["control_head"])
            self.assertTrue(topology["control_status_porcelain"])
            self.assertTrue(topology["active_status_porcelain"])
            self.assertTrue(topology["active_issue_retained"])
            self.assertEqual(topology["active_issue_worktree"], str(retained_peer.resolve()))
            self.assertEqual(topology["cleanup_control_worktree"], str(linked.resolve()))
            self.assertEqual(topology["handoff"], "exact_bound_cleanup_candidate_with_detached_retained_peer")

            subprocess.run(
                ["git", "-C", str(primary), "worktree", "remove", "--force", str(cleanup_target)],
                check=True,
            )
            self.assertFalse(cleanup_target.exists())
            self.assertTrue(retained_peer.is_dir())
            self.assertTrue(
                subprocess.check_output(
                    [
                        "git", "-C", str(retained_peer), "status", "--porcelain",
                        "--untracked-files=all",
                    ],
                    text=True,
                )
            )

    def test_authentic_874_adoption_is_retained_outside_comparator(self):
        result = {
            "performed_mutation": None,
            "envelope": {
                "status": "failed",
                "reason_code": "issue_already_initialized",
                "effects": {"outcome": "unknown"},
            },
        }
        with tempfile.TemporaryDirectory() as directory, mock.patch.object(
            MODULE.common,
            "run",
            return_value=(2, json.dumps(result), "", 7),
        ):
            root = Path(directory)
            args = SimpleNamespace(
                binary=root / "csdlc",
                binary_blake3="a" * 64,
                source_revision="b" * 40,
                logs=root / "logs",
            )
            acceptance = MODULE.execute_authentic_adoption_acceptance(args, root)
        self.assertEqual(acceptance["issue"], 874)
        self.assertFalse(acceptance["included_in_retry_comparison"])
        self.assertEqual(
            acceptance["initial_fixture_facts"]["lifecycle_phase"],
            "already_initialized",
        )
        self.assertEqual(acceptance["attempt"]["exit_code"], 2)

    def test_operational_runner_uses_explicit_variant_drivers(self):
        runner = (EVIDENCE / "run-retry-qualification.sh").read_text()
        self.assertIn('driver="$EVIDENCE/run_${variant}_retry_journeys.py"', runner)
        self.assertNotIn('run_scenario "$variant" primary-linked-edit', runner)
        self.assertNotIn('run_scenario "$variant" terminal-journey', runner)
        self.assertIn("trap restore_harness_binary EXIT INT TERM", runner)

    def test_guard_assertion_requires_exit_two_unknown_effect_and_equal_snapshot(self):
        attempt = {
            "exit_code": 2,
            "result": {
                "performed_mutation": None,
                "envelope": {"effects": {"outcome": "unknown"}},
            },
        }
        MODULE.assert_exact_guard(attempt, "a" * 64, "a" * 64)
        self.assertTrue(attempt["guard_invariants"]["equal"])
        for field, value in (("exit_code", 0),):
            changed = json.loads(json.dumps(attempt))
            changed[field] = value
            with self.assertRaisesRegex(RuntimeError, "fail closed"):
                MODULE.assert_exact_guard(changed, "a" * 64, "a" * 64)
        with self.assertRaisesRegex(RuntimeError, "fail closed"):
            MODULE.assert_exact_guard(attempt, "a" * 64, "b" * 64)

    def test_checkout_inventory_detects_byte_changes_on_already_dirty_paths(self):
        with tempfile.TemporaryDirectory() as directory:
            checkout = Path(directory)
            subprocess.run(["git", "init", "-q", str(checkout)], check=True)
            subprocess.run(["git", "-C", str(checkout), "config", "user.name", "fixture"], check=True)
            subprocess.run(["git", "-C", str(checkout), "config", "user.email", "fixture@example.invalid"], check=True)
            tracked = checkout / "tracked"
            tracked.write_text("baseline\n")
            subprocess.run(["git", "-C", str(checkout), "add", "tracked"], check=True)
            subprocess.run(["git", "-C", str(checkout), "commit", "-qm", "baseline"], check=True)
            tracked.write_text("dirty-one\n")
            untracked = checkout / "existing.tmp"
            untracked.write_text("untracked-one\n")
            first = MODULE.checkout_content_digest(checkout)
            tracked.write_text("dirty-two\n")
            second = MODULE.checkout_content_digest(checkout)
            self.assertNotEqual(first, second)
            tracked.write_text("dirty-one\n")
            untracked.write_text("untracked-two\n")
            third = MODULE.checkout_content_digest(checkout)
            self.assertNotEqual(first, third)

    def test_checkout_inventory_excludes_only_remote_request_telemetry(self):
        with tempfile.TemporaryDirectory() as directory:
            checkout = Path(directory)
            subprocess.run(["git", "init", "-q", str(checkout)], check=True)
            telemetry = checkout / ".csdlc/evidence/1505/remote-requests"
            telemetry.parent.mkdir(parents=True)
            telemetry.write_text("GET:first\n")
            first = MODULE.checkout_content_digest(checkout)
            telemetry.write_text("GET:first\nGET:second\n")
            self.assertEqual(first, MODULE.checkout_content_digest(checkout))

    def test_primary_fake_transport_reads_exact_synthetic_issue_1505(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            env = MODULE.common.configure_fake_remote(root, "a" * 40)
            curl = root / ".csdlc/evidence/1505/fake-bin/curl"
            observed = subprocess.run(
                [str(curl), "https://api.github.com/repos/agent-logic/agent-design-language/issues/1505"],
                env=env, text=True, capture_output=True, check=True,
            )
            issue = json.loads(observed.stdout)
            self.assertEqual(issue["number"], 1505)
            self.assertEqual(issue["state"], "open")
            self.assertEqual(env["ADL_GITHUB_TOKEN_FILE"], str(root / ".csdlc/evidence/1505/token"))


if __name__ == "__main__":
    unittest.main()
