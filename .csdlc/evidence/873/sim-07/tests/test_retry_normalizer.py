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
        self.assertEqual(len(ledger["attempts"]), 35)
        self.assertEqual([journey["status"] for journey in ledger["journeys"]], ["completed", "completed"])

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

    def test_expected_denial_cannot_be_relabelled_invalid(self):
        attempt = self.observations["primary-linked-edit"][1]["attempts"][0]
        attempt["result"]["envelope"]["reason_code"] = "intent_other_failure"
        with self.assertRaisesRegex(MODULE.NormalizationError, "outcome invalid_intent is not declared"):
            self.normalize()

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
