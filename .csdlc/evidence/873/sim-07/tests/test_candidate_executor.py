#!/usr/bin/env python3
from __future__ import annotations

import importlib.util
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest


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
    def test_declared_candidate_common_corpus_is_exactly_thirteen_and_eighteen(self):
        scenario_map = json.loads((EVIDENCE / "retry-scenario-map.json").read_text())
        scenarios = {value["id"]: value for value in scenario_map["scenarios"]}
        self.assertEqual(len(scenarios["primary-linked-edit"]["semantic_steps"]), 13)
        self.assertEqual(len(scenarios["terminal-journey"]["semantic_steps"]), 18)
        self.assertEqual(
            [value["id"] for value in scenarios["terminal-journey"]["semantic_steps"][-6:]],
            [
                "cleanup-foreign-dirty-guard", "cleanup-tracked-dirty-guard",
                "cleanup-preview", "cleanup-stale-preview-guard",
                "cleanup-refresh-preview", "cleanup-execute",
            ],
        )

    def test_candidate_topology_handoff_retains_dirty_peer_and_clean_control(self):
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

            control, topology = MODULE.move_bound_worktree_to_cleanup_control(primary, linked, logs)

            self.assertEqual(control, worktrees / "cleanup-control")
            self.assertTrue(control.is_dir())
            self.assertTrue(linked.is_dir())
            self.assertEqual(topology["baseline_head"], topology["control_head"])
            self.assertEqual(topology["control_status_porcelain"], "")
            self.assertTrue(topology["active_status_porcelain"])
            self.assertTrue(topology["active_issue_retained"])
            self.assertEqual(topology["handoff"], "git_worktree_move_then_detached_retained_peer")

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
