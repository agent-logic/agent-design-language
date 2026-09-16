#!/usr/bin/env python3

from __future__ import annotations

import copy
import hashlib
import importlib.util
import json
from pathlib import Path
import subprocess
import tempfile
import unittest


HERE = Path(__file__).resolve().parent
EVIDENCE = HERE.parent
FIXTURES = HERE / "fixtures"
SPEC = importlib.util.spec_from_file_location(
    "validate_retry_comparison", EVIDENCE / "validate_retry_comparison.py"
)
MODULE = importlib.util.module_from_spec(SPEC)
assert SPEC.loader is not None
SPEC.loader.exec_module(MODULE)


def load(name: str):
    return json.loads((FIXTURES / name).read_text(encoding="utf-8"))


class RetryComparisonTests(unittest.TestCase):
    def setUp(self):
        self.map_path = FIXTURES / "retry-map.json"
        self.map_sha256 = hashlib.sha256(self.map_path.read_bytes()).hexdigest()
        self.scenario_map = MODULE.validate_map(load("retry-map.json"))
        self.predecessor = load("predecessor-ledger.json")
        self.candidate = load("candidate-ledger.json")
        self.predecessor["scenario_map_sha256"] = self.map_sha256
        self.candidate["scenario_map_sha256"] = self.map_sha256

    def validate(self):
        predecessor = MODULE.validate_ledger(
            self.predecessor, "predecessor", self.scenario_map, self.map_sha256, FIXTURES
        )
        candidate = MODULE.validate_ledger(
            self.candidate, "candidate", self.scenario_map, self.map_sha256, FIXTURES
        )
        return MODULE.compare(self.scenario_map, predecessor, candidate)

    def test_exact_fifty_percent_reduction_passes(self):
        result = self.validate()
        self.assertEqual(result["status"], "pass")
        self.assertEqual(result["avoidable_retry_reduction_fraction"], 0.5)
        self.assertEqual(result["denominators"]["predecessor_avoidable_retries"], 2)
        self.assertEqual(result["denominators"]["candidate_avoidable_retries"], 1)

    def test_zero_predecessor_denominator_is_not_proven(self):
        self.predecessor["attempts"] = [
            attempt
            for attempt in self.predecessor["attempts"]
            if attempt["retry_class"] != "avoidable"
        ]
        self.predecessor["attempts"][0]["outcome_class"] = "transition_applied"
        self.predecessor["attempts"][0]["reason_code"] = "command_completed"
        result = self.validate()
        self.assertEqual(result["status"], "not_proven")
        self.assertIsNone(result["avoidable_retry_reduction_fraction"])

    def test_changed_relevant_facts_are_rejected(self):
        self.candidate["attempts"][0]["relevant_facts_sha256"] = "0" * 64
        with self.assertRaisesRegex(MODULE.ValidationError, "relevant facts"):
            self.validate()

    def test_expected_guard_cannot_be_counted_as_avoidable(self):
        invalid = copy.deepcopy(self.candidate["attempts"][2])
        invalid["attempt_id"] = "c-4"
        invalid["retry_of"] = "c-3"
        invalid["retry_class"] = "avoidable"
        self.candidate["attempts"].append(invalid)
        with self.assertRaisesRegex(MODULE.ValidationError, "retry is not admitted"):
            self.validate()

    def test_interrupted_retry_cannot_be_relabelled_avoidable(self):
        first = self.predecessor["attempts"][0]
        first["outcome_class"] = "interrupted"
        first["result_envelope_present"] = False
        first["correlation_id"] = None
        self.predecessor["attempts"][1]["retry_class"] = "avoidable"
        with self.assertRaisesRegex(MODULE.ValidationError, "derived as useful"):
            self.validate()

    def test_fault_retry_cannot_be_relabelled_useful(self):
        self.predecessor["attempts"][1]["retry_class"] = "useful"
        with self.assertRaisesRegex(MODULE.ValidationError, "derived as avoidable"):
            self.validate()

    def test_repeated_step_requires_retry_link(self):
        self.predecessor["attempts"][1]["retry_of"] = None
        self.predecessor["attempts"][1]["retry_class"] = "not_retry"
        with self.assertRaisesRegex(MODULE.ValidationError, "without retry_of"):
            self.validate()

    def test_retry_attempt_uses_declared_variant_specific_argv(self):
        transition = self.scenario_map["scenarios"][0]["semantic_steps"][0]
        retry_argv = ["csdlc-predecessor", "transition", "--retry-fixture"]
        transition["retry_argv"] = {
            "predecessor": retry_argv,
            "candidate": ["csdlc-candidate", "transition", "--retry-fixture"],
        }
        self.predecessor["attempts"][1]["argv"] = retry_argv
        self.predecessor["attempts"][2]["argv"] = retry_argv
        MODULE.validate_ledger(
            self.predecessor,
            "predecessor",
            self.scenario_map,
            self.map_sha256,
            FIXTURES,
        )
        self.predecessor["attempts"][1]["argv"] = transition["argv"]["predecessor"]
        with self.assertRaisesRegex(MODULE.ValidationError, "argv differs"):
            MODULE.validate_ledger(
                self.predecessor,
                "predecessor",
                self.scenario_map,
                self.map_sha256,
                FIXTURES,
            )

    def test_completed_journey_cannot_end_in_fault(self):
        self.candidate["attempts"] = [
            attempt
            for attempt in self.candidate["attempts"]
            if attempt["attempt_id"] != "c-2"
        ]
        with self.assertRaisesRegex(MODULE.ValidationError, "status must be derived as failed"):
            self.validate()

    def test_semantic_steps_cannot_be_reordered(self):
        self.candidate["attempts"] = [
            self.candidate["attempts"][2],
            self.candidate["attempts"][0],
            self.candidate["attempts"][1],
        ]
        with self.assertRaisesRegex(MODULE.ValidationError, "step order differs"):
            self.validate()

    def test_invalid_cli_replaces_stale_passing_output(self):
        with tempfile.TemporaryDirectory() as directory:
            output = Path(directory) / "comparison.json"
            output.write_text('{"status":"pass"}\n', encoding="utf-8")
            completed = subprocess.run(
                [
                    str(EVIDENCE / "validate_retry_comparison.py"),
                    "--scenario-map",
                    str(self.map_path),
                    "--predecessor-ledger",
                    str(FIXTURES / "predecessor-ledger.json"),
                    "--candidate-ledger",
                    str(FIXTURES / "candidate-ledger.json"),
                    "--output",
                    str(output),
                ],
                check=False,
                capture_output=True,
                text=True,
            )
            self.assertEqual(completed.returncode, 2)
            retained = json.loads(output.read_text(encoding="utf-8"))
            self.assertEqual(retained["status"], "invalid")

    def test_binary_provenance_mismatch_is_rejected(self):
        self.candidate["binary"]["sha256"] = "0" * 64
        with self.assertRaisesRegex(MODULE.ValidationError, "binary provenance"):
            self.validate()

    def test_changed_retained_observation_is_rejected(self):
        self.candidate["observations"][0]["sha256"] = "0" * 64
        with self.assertRaisesRegex(MODULE.ValidationError, "observation digest mismatch"):
            self.validate()

    def test_deleted_retained_observation_is_rejected(self):
        self.candidate["observations"][0]["path"] = "missing-raw.json"
        with self.assertRaisesRegex(MODULE.ValidationError, "observation is missing"):
            self.validate()

    def test_outside_retained_observation_path_is_rejected(self):
        self.candidate["observations"][0]["path"] = "../candidate-raw.json"
        with self.assertRaisesRegex(MODULE.ValidationError, "relative and contained"):
            self.validate()

    def test_operational_scenario_map_is_strictly_valid(self):
        operational = MODULE.validate_map(
            json.loads((EVIDENCE / "retry-scenario-map.json").read_text(encoding="utf-8"))
        )
        self.assertEqual(operational["corpus_id"], "sim07-pre-resume-installed-journeys-v1")
        self.assertEqual(len(operational["scenarios"]), 2)


if __name__ == "__main__":
    unittest.main()
