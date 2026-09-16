#!/usr/bin/env python3

from __future__ import annotations

import importlib.util
import json
from pathlib import Path
import subprocess
import tempfile
import unittest


HERE = Path(__file__).resolve().parent
EVIDENCE = HERE.parent
SPEC = importlib.util.spec_from_file_location(
    "run_predecessor_retry_journeys", EVIDENCE / "run_predecessor_retry_journeys.py"
)
MODULE = importlib.util.module_from_spec(SPEC)
assert SPEC.loader is not None
SPEC.loader.exec_module(MODULE)


class PredecessorExecutorTests(unittest.TestCase):
    def test_fixture_capture_keeps_only_first_harness_fixture(self):
        with tempfile.TemporaryDirectory() as directory:
            base = Path(directory)
            first = base / "first"
            second = base / "second"
            for root, marker in ((first, "first"), (second, "second")):
                (root / ".git/installed-candidate").mkdir(parents=True)
                (root / ".git/installed-candidate/plan.json").write_text("{}\n")
                (root / "marker").write_text(marker)
            slot = base / "slot"
            slot.write_text("#!/bin/sh\nexit 0\n")
            slot.chmod(0o755)
            harness = base / "harness"
            harness.write_text(
                "#!/bin/sh\n"
                f"'{slot}' '{first}/.git/installed-candidate/plan.json' || true\n"
                f"'{slot}' '{second}/.git/installed-candidate/plan.json' || true\n"
            )
            harness.chmod(0o755)
            destination = base / "captured"

            MODULE.capture_fixture(harness, slot, "fixture-filter", destination)

            self.assertEqual((destination / "marker").read_text(), "first")
            self.assertFalse((destination / second.name).exists())
            self.assertEqual(slot.read_text(), "#!/bin/sh\nexit 0\n")

    def test_cleanup_control_matches_retained_separate_clean_topology(self):
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
            (primary / ".adl/worktree-policy.json").write_text(
                json.dumps({"schema": "adl.worktree_policy.v1", "required_parent": str(worktrees)}),
                encoding="utf-8",
            )
            (primary / "tracked").write_text("baseline\n", encoding="utf-8")
            subprocess.run(["git", "-C", str(primary), "add", "."], check=True)
            subprocess.run(["git", "-C", str(primary), "commit", "-qm", "baseline"], check=True)
            worktrees.mkdir()
            subprocess.run(
                ["git", "-C", str(primary), "worktree", "add", "-q", "-b", "codex/1505-fixture", str(linked), "HEAD"],
                check=True,
            )
            native_state = linked / ".csdlc/v3/issues/1505/state.json"
            native_state.parent.mkdir(parents=True)
            native_state.write_text("{}\n", encoding="utf-8")

            control = MODULE.create_cleanup_control(primary, linked, logs)

            self.assertEqual(control, worktrees / "cleanup-control")
            self.assertTrue(native_state.is_file())
            self.assertEqual(
                subprocess.check_output(
                    ["git", "-C", str(control), "status", "--porcelain", "--untracked-files=all"],
                    text=True,
                ),
                "",
            )
            manifest = json.loads((logs / "cleanup-control-topology.json").read_text())
            self.assertEqual(manifest["baseline_head"], manifest["control_head"])
            self.assertEqual(manifest["control_status_porcelain"], "")
            self.assertTrue(manifest["active_issue_retained"])
            self.assertEqual(
                manifest["primary_git_common_dir"], manifest["control_git_common_dir"]
            )
            self.assertEqual(
                set(manifest["registered_paths"]),
                {str(primary.resolve()), str(linked.resolve()), str(control.resolve())},
            )

    def test_primary_finish_uses_exact_git_metadata_terminal_paths(self):
        with tempfile.TemporaryDirectory() as directory:
            primary = Path(directory) / "primary"
            primary.mkdir()
            subprocess.run(["git", "init", "-q", str(primary)], check=True)
            value = MODULE.primary_terminal_state(primary, 1505)
            git_dir = (primary / ".git").resolve()
            self.assertEqual(value["repository_root"], str(primary))
            self.assertEqual(
                value["state_path"],
                str(git_dir / "csdlc-v3/local/v3/issues/1505/terminal.json"),
            )
            self.assertEqual(
                value["receipt_path"],
                str(git_dir / "csdlc-v3/local/evidence/1505/terminal-receipt.json"),
            )

    def test_merge_review_receipt_copy_is_exact_and_primary_contained(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            primary = root / "primary"
            linked = root / "worktrees/adl-issue-1505-installed-intent-fixture"
            logs = root / "logs"
            primary.mkdir()
            linked.mkdir(parents=True)
            logs.mkdir()
            source = linked / ".csdlc/evidence/1505/review.json"
            source.parent.mkdir(parents=True)
            source.write_bytes(b'{"typed":"review"}\n')
            path = MODULE.stage_review_receipt_for_primary(linked, primary, logs)
            self.assertTrue(path.is_absolute())
            self.assertEqual(
                path,
                primary.resolve() / ".csdlc/evidence/1505/review.json",
            )
            self.assertEqual(path.read_bytes(), source.read_bytes())
            manifest = json.loads((logs / "primary-review-receipt-copy.json").read_text())
            self.assertTrue(manifest["byte_equal"])
            self.assertEqual(manifest["source_sha256"], manifest["destination_sha256"])
            with self.assertRaisesRegex(RuntimeError, "destination already exists"):
                MODULE.stage_review_receipt_for_primary(linked, primary, logs)

    def test_terminal_topology_matches_all_eighteen_common_semantic_steps(self):
        scenario_map = json.loads((EVIDENCE / "retry-scenario-map.json").read_text())
        scenario = next(
            value for value in scenario_map["scenarios"]
            if value["id"] == "terminal-journey"
        )
        actual = [
            MODULE.terminal_topology(step["id"])
            for step in scenario["semantic_steps"]
        ]
        expected = [
            "primary", "primary", "linked", "linked",
            "primary", "primary", "primary", "primary", "primary", "primary",
            "linked", "primary",
            "primary", "primary", "primary", "primary", "primary", "primary",
        ]
        self.assertEqual(actual, expected)

    def test_linked_finish_request_uses_exact_canonical_terminal_paths(self):
        linked = Path("/fixture/worktrees/adl-issue-1505-installed-intent-fixture")
        value = MODULE.linked_terminal_state(linked, 1505)
        expected = {
            "repository_root": str(linked),
            "state_path": ".csdlc/v3/issues/1505/terminal.json",
            "receipt_path": ".csdlc/evidence/1505/terminal-receipt.json",
        }
        self.assertEqual(value, expected)
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "finish-linked-request.json"
            MODULE.write_json(path, {"terminal_state": value})
            self.assertEqual(
                path.read_bytes(),
                (json.dumps({"terminal_state": expected}, indent=2, sort_keys=True) + "\n").encode(),
            )

    def test_synthetic_terminal_issue_readback_is_exact_closed_fixture(self):
        with tempfile.TemporaryDirectory() as directory:
            linked = Path(directory)
            env = MODULE.configure_fake_remote(linked, "d" * 40)
            evidence = linked / ".csdlc/evidence/1505"
            observed = subprocess.run(
                [
                    str(evidence / "fake-bin/curl"),
                    "https://api.github.com/repos/agent-logic/agent-design-language/issues/1505",
                ],
                env=env,
                text=True,
                capture_output=True,
                check=True,
            )
            issue = json.loads(observed.stdout)
            self.assertEqual(issue["number"], 1505)
            self.assertEqual(issue["state"], "open")
            self.assertIsNone(issue["state_reason"])
            self.assertIsNone(issue["closed_at"])
            (evidence / "remote-pr.json").write_text('{"merged":true}\n')
            closed = subprocess.run(
                [
                    str(evidence / "fake-bin/curl"),
                    "https://api.github.com/repos/agent-logic/agent-design-language/issues/1505",
                ],
                env=env, text=True, capture_output=True, check=True,
            )
            closed_issue = json.loads(closed.stdout)
            self.assertEqual(closed_issue["state"], "closed")
            self.assertEqual(closed_issue["state_reason"], "completed")
            self.assertEqual(closed_issue["updated_at"], closed_issue["closed_at"])
            self.assertEqual(
                (evidence / "remote-requests").read_text(),
                "GET:https://api.github.com/repos/agent-logic/agent-design-language/issues/1505\n" * 2,
            )

    def test_synthetic_merge_admission_has_no_additional_active_rules(self):
        with tempfile.TemporaryDirectory() as directory:
            linked = Path(directory)
            env = MODULE.configure_fake_remote(linked, "c" * 40)
            evidence = linked / ".csdlc/evidence/1505"
            observed = subprocess.run(
                [
                    str(evidence / "fake-bin/curl"),
                    "https://api.github.com/repos/agent-logic/agent-design-language/rules/branches/main?per_page=100&page=1",
                ],
                env=env,
                text=True,
                capture_output=True,
                check=True,
            )
            self.assertEqual(json.loads(observed.stdout), [])
            self.assertEqual(
                (evidence / "remote-requests").read_text(),
                "GET:https://api.github.com/repos/agent-logic/agent-design-language/rules/branches/main?per_page=100&page=1\n",
            )

    def test_synthetic_merge_admission_matches_observed_graphql_call_shape(self):
        head = "b" * 40
        with tempfile.TemporaryDirectory() as directory:
            linked = Path(directory)
            env = MODULE.configure_fake_remote(linked, head)
            evidence = linked / ".csdlc/evidence/1505"
            observed = subprocess.run(
                [
                    str(evidence / "fake-bin/curl"),
                    "https://api.github.com/graphql",
                ],
                env=env,
                text=True,
                capture_output=True,
                check=True,
            )
            repository = json.loads(observed.stdout)["data"]["repository"]
            pull_request = repository["pullRequest"]
            self.assertEqual(repository["nameWithOwner"], "agent-logic/agent-design-language")
            self.assertEqual(pull_request["headRefOid"], head)
            self.assertEqual(pull_request["baseRefName"], "main")
            self.assertEqual(pull_request["mergeable"], "MERGEABLE")
            self.assertEqual(pull_request["mergeStateStatus"], "CLEAN")
            required = pull_request["baseRef"]["branchProtectionRule"]
            self.assertTrue(required["requiresStatusChecks"])
            self.assertEqual(
                required["requiredStatusChecks"],
                [
                    {
                        "context": "issue-873-synthetic-check",
                        "app": {"databaseId": 873},
                    }
                ],
            )
            commit = pull_request["commits"]["nodes"][0]["commit"]
            self.assertEqual(commit["oid"], head)
            checks = commit["statusCheckRollup"]["contexts"]["nodes"]
            self.assertEqual(len(checks), 1)
            self.assertEqual(
                checks[0],
                {
                    "__typename": "CheckRun",
                    "name": "issue-873-synthetic-check",
                    "status": "COMPLETED",
                    "conclusion": "SUCCESS",
                    "isRequired": True,
                    "checkSuite": {"app": {"databaseId": 873}},
                },
            )
            self.assertEqual(
                (evidence / "remote-requests").read_text(),
                "GET:https://api.github.com/graphql\n",
            )

    def test_synthetic_merge_transport_returns_and_retains_documented_readback(self):
        head = "a" * 40
        merge_commit = "e" * 40
        with tempfile.TemporaryDirectory() as directory:
            linked = Path(directory)
            env = MODULE.configure_fake_remote(linked, head)
            evidence = linked / ".csdlc/evidence/1505"
            (evidence / "remote-pr.json").write_text(
                json.dumps(
                    {
                        "number": 639,
                        "id": 639,
                        "node_id": "PR_ready639",
                        "state": "open",
                        "merged": False,
                        "base": {"ref": "main"},
                        "head": {
                            "sha": head,
                            "ref": "codex/1505-installed-intent-fixture",
                        },
                        "draft": False,
                    },
                    separators=(",", ":"),
                ),
                encoding="utf-8",
            )
            merged = subprocess.run(
                [
                    str(evidence / "fake-bin/curl"),
                    "--request",
                    "PUT",
                    "https://api.github.com/repos/agent-logic/agent-design-language/pulls/639/merge",
                ],
                env=env,
                text=True,
                capture_output=True,
                check=True,
            )
            self.assertEqual(
                json.loads(merged.stdout),
                {
                    "sha": merge_commit,
                    "merged": True,
                    "message": "Pull Request successfully merged",
                },
            )
            retained = json.loads((evidence / "remote-pr.json").read_text(encoding="utf-8"))
            self.assertEqual(retained["state"], "closed")
            self.assertTrue(retained["merged"])
            self.assertEqual(retained["merge_commit_sha"], merge_commit)
            self.assertEqual((evidence / "remote-effects").read_text(), "pr-merge\n")
            readback = subprocess.run(
                [str(evidence / "fake-bin/curl"), "https://api.github.com/graphql"],
                env=env,
                text=True,
                capture_output=True,
                check=True,
            )
            pull_request = json.loads(readback.stdout)["data"]["repository"]["pullRequest"]
            self.assertEqual(pull_request["headRefOid"], head)
            self.assertEqual(pull_request["baseRefName"], "main")
            self.assertEqual(pull_request["state"], "MERGED")
            self.assertTrue(pull_request["merged"])
            self.assertEqual(pull_request["mergeCommit"]["oid"], merge_commit)
            self.assertEqual(
                pull_request["mergeCommit"]["parents"]["nodes"],
                [{"oid": "1" * 40}, {"oid": head}],
            )


if __name__ == "__main__":
    unittest.main()
