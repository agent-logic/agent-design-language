#!/usr/bin/env python3

from __future__ import annotations

import copy
import hashlib
import importlib.util
import json
from pathlib import Path
import sys
import unittest


HERE = Path(__file__).resolve().parent
EVIDENCE = HERE.parent
sys.path.insert(0, str(EVIDENCE))
SPEC = importlib.util.spec_from_file_location(
    "normalize_retry_ledger", EVIDENCE / "normalize_retry_ledger.py"
)
MODULE = importlib.util.module_from_spec(SPEC)
assert SPEC.loader is not None
SPEC.loader.exec_module(MODULE)


def load(path: Path):
    return json.loads(path.read_text(encoding="utf-8"))


class RetryNormalizerTests(unittest.TestCase):
    def setUp(self):
        self.map_path = EVIDENCE / "retry-scenario-map.json"
        self.scenario_map = MODULE.contract.validate_map(load(self.map_path))
        self.primary_path = EVIDENCE / "observations/installed_exact_primary_linked_edit.attempts.json"
        self.terminal_path = EVIDENCE / "observations/installed_exact_terminal_journey.attempts.json"
        self.observations = {
            "primary-linked-edit": (self.primary_path, load(self.primary_path)),
            "terminal-journey": (self.terminal_path, load(self.terminal_path)),
        }
        candidate_issue = 1505
        for _, observation in self.observations.values():
            observation["provenance"]["installed_binary_blake3"] = self.scenario_map[
                "binaries"
            ]["candidate"]["blake3"]
            for attempt in observation["attempts"]:
                attempt["argv"] = [
                    token.replace("505", str(candidate_issue))
                    for token in attempt["argv"]
                ]
                input_value = attempt.get("input")
                if isinstance(input_value, dict) and "issue" in input_value:
                    input_value["issue"] = candidate_issue
                result = attempt.get("result")
                if isinstance(result, dict):
                    if "request_issue" in result:
                        result["request_issue"] = candidate_issue
                    owner_result = result.get("result")
                    if isinstance(owner_result, dict) and "issue" in owner_result:
                        owner_result["issue"] = candidate_issue
                    envelope = result.get("envelope")
                    if isinstance(envelope, dict):
                        envelope_issue = envelope.get("issue")
                        if isinstance(envelope_issue, dict) and "number" in envelope_issue:
                            envelope_issue["number"] = candidate_issue
        terminal = self.observations["terminal-journey"][1]
        proof_input_admission = copy.deepcopy(terminal["attempts"][2])
        proof_input_admission["attempt_id"] = "candidate-proof-input-admission"
        proof_input_admission["argv"] = [
            "edit",
            str(candidate_issue),
            "--changes",
            "$FIXTURE_ROOT/.git/installed-candidate/validator-changes.json",
        ]
        terminal["attempts"].insert(2, proof_input_admission)
        primary_finish = copy.deepcopy(terminal["attempts"][11])
        primary_finish["attempt_id"] = "candidate-primary-finish"
        primary_finish["topology"] = "primary"
        terminal["attempts"].insert(12, primary_finish)
        terminal["attempts"] = terminal["attempts"][:17] + terminal["attempts"][21:23]
        terminal["attempted"] = len(terminal["attempts"])
        primary_root = "/fixture/primary"
        active = "/fixture/worktrees/cleanup-control"
        control = "/fixture/worktrees/adl-issue-1505-installed-intent-fixture"
        terminal["cleanup_topology"] = {
            "schema": "csdlc.v3.issue873.cleanup_control_topology.v1",
            "primary_root": primary_root,
            "active_issue_worktree": active,
            "cleanup_control_worktree": control,
            "baseline_head": "a" * 40,
            "control_head": "a" * 40,
            "tracked_inventory_sha256": "b" * 64,
            "control_status_porcelain": "?? .csdlc/evidence/1505/proof.json\n",
            "active_status_porcelain": "?? issue-873-active-retained.tmp\n",
            "registered_paths": [primary_root, active, control],
            "active_issue_retained": True,
        }
        cleanup_attempts = terminal["attempts"][-6:]
        for attempt in cleanup_attempts:
            attempt["input"] = {
                "issue": candidate_issue,
                "cleanup_candidate": control,
                "active_issue_worktree": active,
            }
            owner = attempt.get("result", {}).get("result")
            cleanup = owner.get("cleanup") if isinstance(owner, dict) else None
            if isinstance(cleanup, dict) and "path" in cleanup:
                cleanup["path"] = control
        for offset, placeholder in ((3, "$STALE_PREVIEW_DIGEST"), (5, "$PREVIEW_DIGEST")):
            attempt = cleanup_attempts[offset]
            digest = "c" * 64 if offset == 3 else "d" * 64
            attempt["argv"][-1] = placeholder
            attempt["input"]["preview_receipt_digest"] = digest
            attempt["executed_argv"] = [
                "clean", str(candidate_issue), "--execute", "--preview", digest
            ]
        for _, observation in self.observations.values():
            for attempt in observation["attempts"]:
                result = attempt.get("result")
                envelope = result.get("envelope") if isinstance(result, dict) else None
                if isinstance(envelope, dict) and envelope.get("reason_code") in MODULE.EXPECTED_GUARD_REASONS:
                    attempt["guard_invariants"] = {
                        "before_sha256": "f" * 64,
                        "after_sha256": "f" * 64,
                        "equal": True,
                        "exit_code": 2,
                        "effects_outcome": "unknown",
                        "performed_mutation": None,
                    }
        self.candidate_sha = self.scenario_map["binaries"]["candidate"]["sha256"]

    def normalize(self):
        return MODULE.normalize(
            self.scenario_map,
            "candidate",
            self.observations,
            self.candidate_sha,
            EVIDENCE,
        )

    def test_retained_candidate_attempts_normalize_to_strict_denominator(self):
        ledger = self.normalize()
        ledger["scenario_map_sha256"] = hashlib.sha256(self.map_path.read_bytes()).hexdigest()
        MODULE.contract.validate_ledger(
            ledger,
            "candidate",
            self.scenario_map,
            ledger["scenario_map_sha256"],
            EVIDENCE,
        )
        self.assertEqual(len(ledger["attempts"]), 32)
        self.assertEqual([journey["status"] for journey in ledger["journeys"]], ["completed", "completed"])

    def test_cleanup_target_must_match_declared_control(self):
        terminal = self.observations["terminal-journey"][1]
        terminal["attempts"][-1]["input"]["cleanup_candidate"] = "/fixture/worktrees/wrong"
        with self.assertRaisesRegex(MODULE.NormalizationError, "cleanup target differs"):
            self.normalize()

    def test_cleanup_active_worktree_must_match_declared_retained_peer(self):
        terminal = self.observations["terminal-journey"][1]
        terminal["attempts"][-1]["input"]["active_issue_worktree"] = "/fixture/worktrees/wrong"
        with self.assertRaisesRegex(MODULE.NormalizationError, "active worktree identity differs"):
            self.normalize()

    def test_cleanup_executed_preview_operand_must_match_raw_input(self):
        terminal = self.observations["terminal-journey"][1]
        terminal["attempts"][-1]["executed_argv"][-1] = "e" * 64
        with self.assertRaisesRegex(MODULE.NormalizationError, "preview operand"):
            self.normalize()

    def test_candidate_guard_requires_nonzero_exit_and_unchanged_invariants(self):
        attempt = self.observations["primary-linked-edit"][1]["attempts"][0]
        attempt["exit_code"] = 0
        with self.assertRaisesRegex(MODULE.NormalizationError, "outcome invalid_intent"):
            self.normalize()
        attempt["exit_code"] = 2
        attempt["guard_invariants"]["after_sha256"] = "0" * 64
        with self.assertRaisesRegex(MODULE.NormalizationError, "outcome invalid_intent"):
            self.normalize()

    def test_candidate_guard_rejects_reported_mutation(self):
        attempt = self.observations["primary-linked-edit"][1]["attempts"][0]
        attempt["result"]["performed_mutation"] = True
        with self.assertRaisesRegex(MODULE.NormalizationError, "outcome invalid_intent"):
            self.normalize()

    def test_synthetic_issue_map_uses_no_cutover_or_505_only_command(self):
        allowed = {
            "prepare", "issue", "bind", "proof", "review", "publish", "github-pr",
            "finish", "clean", "status", "doctor", "validate", "edit",
        }
        for scenario in self.scenario_map["scenarios"]:
            fixture_facts = scenario["relevant_facts"]["initial_fixture_facts"]
            self.assertEqual(fixture_facts["issue_number"], 1505)
            self.assertEqual(fixture_facts["lifecycle_phase"], "unprepared")
            for step in scenario["semantic_steps"]:
                self.assertNotIn("cutover", step["intended_operation"])
                self.assertNotIn("rollback", step["intended_operation"])
                for mapping_name in ("argv", "retry_argv"):
                    for tokens in step.get(mapping_name, {}).values():
                        self.assertIn(tokens[0], allowed)
                        self.assertNotIn("505", tokens)

    def test_retry_denominator_excludes_candidate_only_cleanup_archive_recovery(self):
        terminal = next(
            scenario for scenario in self.scenario_map["scenarios"]
            if scenario["id"] == "terminal-journey"
        )
        step_ids = [step["id"] for step in terminal["semantic_steps"]]
        self.assertEqual(
            step_ids[-4:],
            [
                "cleanup-preview",
                "cleanup-stale-preview-guard",
                "cleanup-refresh-preview",
                "cleanup-execute",
            ],
        )
        for excluded in (
            "cleanup-injected-failure",
            "cleanup-interruption-one",
            "cleanup-interruption-two",
            "cleanup-recovery-status",
            "cleanup-recovery-preview",
            "cleanup-primary-complete",
        ):
            self.assertNotIn(excluded, step_ids)
        exclusion = terminal["relevant_facts"]["excluded_from_retry_comparison"]
        self.assertEqual(
            exclusion["surface"],
            "candidate_cleanup_archive_interruption_and_recovery",
        )
        self.assertIn("no archive crash or recovery contract", exclusion["reason"])
        self.assertEqual(exclusion["qualification"], "retained separately under issue 873")

    def test_missing_step_is_rejected(self):
        self.observations["terminal-journey"][1]["attempts"].pop()
        self.observations["terminal-journey"][1]["attempted"] -= 1
        with self.assertRaisesRegex(MODULE.NormalizationError, "missing mapped steps"):
            self.normalize()

    def test_wrong_binary_is_rejected(self):
        with self.assertRaisesRegex(MODULE.NormalizationError, "binary sha256 mismatch"):
            MODULE.normalize(
                self.scenario_map,
                "candidate",
                self.observations,
                "0" * 64,
            )

    def test_mapping_drift_is_rejected(self):
        self.observations["primary-linked-edit"][1]["attempts"][0]["argv"][0] = "status"
        with self.assertRaisesRegex(MODULE.NormalizationError, "argv mapping drift"):
            self.normalize()

    def test_raw_issue_identity_must_match_relevant_facts(self):
        attempt = self.observations["terminal-journey"][1]["attempts"][0]
        attempt["result"]["envelope"]["issue"]["number"] = 505
        with self.assertRaisesRegex(MODULE.NormalizationError, "result issue differs"):
            self.normalize()

    def test_predecessor_typed_input_must_bind_issue_identity(self):
        attempt = {
            "input": {"issue": 505},
            "result": {
                "envelope": {"issue": {"status": "identified", "number": 505}}
            },
        }
        with self.assertRaisesRegex(MODULE.NormalizationError, "input issue differs"):
            MODULE.validate_raw_issue_identity(
                [attempt], "predecessor", "fixture", 1505
            )

    def test_expected_denial_cannot_be_relabelled_invalid(self):
        attempt = self.observations["primary-linked-edit"][1]["attempts"][0]
        attempt["result"]["envelope"]["reason_code"] = "intent_other_failure"
        with self.assertRaisesRegex(MODULE.NormalizationError, "outcome invalid_intent is not declared"):
            self.normalize()

    def test_predecessor_usage_is_guard_only_for_declared_preview_steps(self):
        guard = {
            "result": {
                "status": "failed",
                "reason_code": "usage",
                "correlation_id": "guard",
                "findings": [
                    {
                        "message": "usage: csdlc issue --request <path>; unexpected argument --preview"
                    }
                ],
            }
        }
        outcome = MODULE.classify_outcome(
            guard, "predecessor", "prepare-preview-guard"
        )[0]
        self.assertEqual(outcome, "expected_guard_denial")
        outcome = MODULE.classify_outcome(guard, "predecessor", "prepare")[0]
        self.assertEqual(outcome, "unexpected_fault")
        outcome = MODULE.classify_outcome(guard, "candidate", "prepare-preview-guard")[0]
        self.assertEqual(outcome, "unexpected_fault")
        guard["result"]["findings"][0]["message"] = "different usage regression"
        outcome = MODULE.classify_outcome(
            guard, "predecessor", "prepare-preview-guard"
        )[0]
        self.assertEqual(outcome, "unexpected_fault")

    def test_predecessor_merge_denial_requires_exact_authorization_diagnostic(self):
        guard = {
            "result": {
                "status": "blocked",
                "reason_code": "github_merge_ineligible",
                "correlation_id": "guard",
                "findings": [
                    {"message": "explicit operator merge authorization reference required"}
                ],
            }
        }
        outcome = MODULE.classify_outcome(
            guard, "predecessor", "merge-internal-input-guard"
        )[0]
        self.assertEqual(outcome, "expected_guard_denial")
        self.assertEqual(
            MODULE.classify_outcome(guard, "candidate", "merge-internal-input-guard")[0],
            "unexpected_fault",
        )
        guard["result"]["findings"][0]["message"] = "different merge denial"
        self.assertEqual(
            MODULE.classify_outcome(
                guard, "predecessor", "merge-internal-input-guard"
            )[0],
            "unexpected_fault",
        )

    def test_predecessor_dirty_cleanup_guard_requires_exact_non_effecting_decision(self):
        guard = {
            "result": {
                "performed_mutation": False,
                "result": {"cleanup": {"decision": "dirty", "path": "/fixture"}},
                "envelope": {
                    "status": "ready",
                    "reason_code": "command_completed",
                    "correlation_id": "dirty-guard",
                    "effects": {"outcome": "none"},
                    "findings": [],
                },
            }
        }
        for step_id in ("cleanup-foreign-dirty-guard", "cleanup-tracked-dirty-guard"):
            self.assertEqual(
                MODULE.classify_outcome(guard, "predecessor", step_id)[0],
                "expected_guard_denial",
            )
        missing = copy.deepcopy(guard)
        del missing["result"]["result"]["cleanup"]["decision"]
        self.assertEqual(
            MODULE.classify_outcome(missing, "predecessor", "cleanup-foreign-dirty-guard")[0],
            "expected_wait",
        )
        effected = copy.deepcopy(guard)
        effected["result"]["performed_mutation"] = True
        effected["result"]["envelope"]["effects"]["outcome"] = "performed"
        self.assertEqual(
            MODULE.classify_outcome(effected, "predecessor", "cleanup-foreign-dirty-guard")[0],
            "transition_applied",
        )
        self.assertEqual(
            MODULE.classify_outcome(guard, "predecessor", "cleanup-preview")[0],
            "expected_wait",
        )

    def test_predecessor_read_only_review_is_exact_successful_no_op(self):
        review = {
            "result": {
                "read_only": True,
                "envelope": {
                    "status": "ready",
                    "reason_code": "command_completed",
                    "correlation_id": "review",
                    "effects": {"outcome": "none"},
                    "findings": [],
                },
            }
        }
        self.assertEqual(
            MODULE.classify_outcome(review, "predecessor", "review")[0],
            "successful_no_op",
        )
        changed = copy.deepcopy(review)
        changed["result"]["read_only"] = False
        self.assertEqual(
            MODULE.classify_outcome(changed, "predecessor", "review")[0],
            "expected_wait",
        )
        self.assertEqual(
            MODULE.classify_outcome(review, "predecessor", "finish-preview")[0],
            "expected_wait",
        )

    def test_predecessor_stale_preview_guard_requires_exact_non_effecting_diagnostic(self):
        guard = {
            "result": {
                "performed_mutation": False,
                "envelope": {
                    "status": "blocked",
                    "reason_code": "preview_receipt_mismatch",
                    "correlation_id": "stale-preview",
                    "effects": {"outcome": "none"},
                    "findings": [
                        {
                            "message": "cleanup removal requires a preview receipt for the same Git registration"
                        }
                    ],
                },
            }
        }
        self.assertEqual(
            MODULE.classify_outcome(
                guard, "predecessor", "cleanup-stale-preview-guard"
            )[0],
            "expected_guard_denial",
        )
        for changed in (
            ("message", "different mismatch"),
            ("effect", "performed"),
            ("mutation", True),
            ("step", "cleanup-preview"),
        ):
            probe = copy.deepcopy(guard)
            step_id = "cleanup-stale-preview-guard"
            if changed[0] == "message":
                probe["result"]["envelope"]["findings"][0]["message"] = changed[1]
            elif changed[0] == "effect":
                probe["result"]["envelope"]["effects"]["outcome"] = changed[1]
            elif changed[0] == "mutation":
                probe["result"]["performed_mutation"] = changed[1]
            else:
                step_id = changed[1]
            self.assertEqual(
                MODULE.classify_outcome(probe, "predecessor", step_id)[0],
                "unexpected_fault",
            )

    def test_noncontiguous_retry_is_rejected(self):
        terminal = self.observations["terminal-journey"][1]
        repeated = copy.deepcopy(terminal["attempts"][0])
        repeated["attempt_id"] = "late-prepare-retry"
        terminal["attempts"].append(repeated)
        terminal["attempted"] += 1
        with self.assertRaisesRegex(MODULE.NormalizationError, "after all mapped steps"):
            self.normalize()

    def test_raw_binary_blake3_mismatch_is_rejected(self):
        self.observations["primary-linked-edit"][1]["provenance"]["installed_binary_blake3"] = "0" * 64
        with self.assertRaisesRegex(MODULE.NormalizationError, "binary provenance mismatch"):
            self.normalize()

    def test_raw_attempt_count_is_bounded(self):
        primary = self.observations["primary-linked-edit"][1]
        primary["attempts"] = [copy.deepcopy(primary["attempts"][0]) for _ in range(257)]
        primary["attempted"] = 257
        with self.assertRaisesRegex(MODULE.NormalizationError, "raw attempts exceed 256"):
            self.normalize()


if __name__ == "__main__":
    unittest.main()
