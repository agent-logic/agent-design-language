#!/usr/bin/env python3
"""PVF required deterministic local accounting/negative contract; no inference/network."""
import copy
import hashlib
import itertools
import json
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

import pair_experiment as pair


def fixture():
    provider = b'{"providers":{}}'
    plan = dict(schema="adl.pair.plan.v1", candidate_sha="a" * 40,
                pair_revision="b" * 40, model_revision="c" * 40,
                tokenizer_revision="d" * 40,
                provider_definition_sha256=hashlib.sha256(provider).hexdigest(),
                corpus=[dict(id=f"q{i}", prompt=f"Return {i}", expected_text=str(i)) for i in range(2)],
                sampling=dict(temperature=0, seed=7, max_tokens=8), warmth="warm",
                repetitions=2, concurrency=[1, 2], nodes=["alpha", "beta"], lost_node="beta",
                max_request_seconds=5, max_run_seconds=100)
    sha = pair.validate_plan(plan)
    rows = []
    for route, scenario, concurrency, repeat, i in itertools.product(
            pair.ROUTES, pair.SCENARIOS, [1, 2], range(2), range(2)):
        start = (1 if scenario == "healthy" else 51) + repeat * 5 + (i if concurrency == 1 else 0)
        rows.append(dict(route=route, scenario=scenario, concurrency=concurrency,
                         repetition=repeat, request_id=f"q{i}", plan_sha256=sha,
                         prompt_sha256=pair.text_digest(f"Return {i}"), started_seconds=start,
                         ended_seconds=start+1, output=str(i), error=None,
                         serving_node="alpha" if scenario == "node_loss" else plan["nodes"][i],
                         resource_sample_sha256="e"*64))
    packet = dict(schema="adl.pair.measurements.v1", plan_sha256=sha, records=rows,
                  node_events=[dict(node="beta", available=False, at_seconds=50,
                                    evidence_sha256="f"*64)])
    return plan, packet, provider


class AccountingTests(unittest.TestCase):
    def test_complete_matrix_preserves_null_benefit_without_qualification(self):
        plan, packet, provider = fixture()
        summary = pair.summarize(plan, packet, provider)
        self.assertEqual(summary["attempts"], 48)
        self.assertEqual(summary["qualification"], "not_established_by_accounting")
        rates = [r["completed_requests_per_second"] for r in summary["summaries"]
                 if r["scenario"] == "healthy" and r["concurrency"] == 2]
        self.assertEqual(rates, [2, 2, 2])
        self.assertTrue(all(row["throughput_ratio"] == 1 for row in summary["comparisons"]))
        self.assertEqual(summary, pair.summarize(plan, packet, provider))

    def test_failures_retained_in_denominator(self):
        plan, packet, provider = fixture()
        row = packet["records"][-1]
        row.update(output=None, error="unavailable", serving_node=None)
        summary = pair.summarize(plan, packet, provider)
        self.assertEqual(summary["attempts"], 48)
        self.assertEqual(sum(x["failures"] for x in summary["summaries"]), 1)

    def test_negative_benefit_is_reported_without_hiding_failures(self):
        plan, packet, provider = fixture()
        for row in packet["records"]:
            if row["route"] == "raw_pair" and row["concurrency"] == 2:
                row["ended_seconds"] += 1
        result = pair.summarize(plan, packet, provider)
        self.assertTrue(all(r["throughput_ratio"] == 0.5 for r in result["comparisons"]
                            if r["route"] == "raw_pair" and r["concurrency"] == 2))
        packet["records"][-1].update(output=None, error="timeout", serving_node=None)
        result = pair.summarize(plan, packet, provider)
        self.assertIsNone(result["comparisons"][-1]["throughput_ratio"])

    def test_reject_matrix_and_receipt_mutations(self):
        changes = {
            "missing_request": lambda p: p["records"].pop(),
            "duplicate_request": lambda p: p["records"].__setitem__(1, copy.deepcopy(p["records"][0])),
            "wrong_route": lambda p: p["records"][0].update(route="mock_runtime"),
            "caller_success": lambda p: p.update(success=True),
            "record_success": lambda p: p["records"][0].update(success=True),
            "wrong_plan": lambda p: p["records"][0].update(plan_sha256="0"*64),
            "wrong_prompt": lambda p: p["records"][0].update(prompt_sha256="0"*64),
            "wrong_output": lambda p: p["records"][0].update(output="incorrect"),
            "empty_output": lambda p: p["records"][0].update(output=""),
            "negative_latency": lambda p: p["records"][0].update(ended_seconds=0),
            "nan_latency": lambda p: p["records"][0].update(ended_seconds=float("nan")),
            "over_budget": lambda p: p["records"][0].update(ended_seconds=101),
            "missing_resource": lambda p: p["records"][0].update(resource_sample_sha256=""),
            "missing_events": lambda p: p.update(node_events=[]),
            "fake_loss": lambda p: p["node_events"][0].update(available=True),
            "late_loss": lambda p: p["node_events"][0].update(at_seconds=99),
            "lost_node_serves": lambda p: p["records"][-1].update(serving_node="beta"),
            "error_with_output": lambda p: p["records"][0].update(error="timeout"),
            "unknown_node": lambda p: p["records"][0].update(serving_node="gamma"),
            "caller_error_text": lambda p: p["records"][0].update(error="secret", output=None),
        }
        for name, change in changes.items():
            with self.subTest(name=name):
                plan, packet, provider = fixture()
                change(packet)
                with self.assertRaises(pair.InvalidExperiment):
                    pair.summarize(plan, packet, provider)

    def test_comparison_context_changes_reject(self):
        for field, value in [("model_revision", "e"*40), ("warmth", "cold"),
                             ("concurrency", [1, 3]), ("tokenizer_revision", "f"*40)]:
            with self.subTest(field=field):
                plan, packet, provider = fixture()
                plan[field] = value
                with self.assertRaises(pair.InvalidExperiment):
                    pair.summarize(plan, packet, provider)
        plan, packet, _ = fixture()
        with self.assertRaises(pair.InvalidExperiment):
            pair.summarize(plan, packet, b"changed provider")

    def test_serialized_requests_do_not_prove_concurrency(self):
        plan, packet, provider = fixture()
        for row in packet["records"]:
            if row["concurrency"] == 2 and row["request_id"] == "q1":
                row["started_seconds"] += 1
                row["ended_seconds"] += 1
        with self.assertRaisesRegex(pair.InvalidExperiment, "concurrency_not_observed"):
            pair.summarize(plan, packet, provider)

    def test_two_nodes_and_actual_concurrency_are_required(self):
        for change in ("one_node", "serial_overlap"):
            with self.subTest(change=change):
                plan, packet, provider = fixture()
                for row in packet["records"]:
                    if change == "one_node":
                        row["serving_node"] = "alpha"
                    elif row["concurrency"] == 1 and row["request_id"] == "q1":
                        row["started_seconds"] -= 1
                        row["ended_seconds"] -= 1
                with self.assertRaises(pair.InvalidExperiment):
                    pair.summarize(plan, packet, provider)

    def test_cli_machine_channel_and_redacted_failure(self):
        plan, packet, provider = fixture()
        with tempfile.TemporaryDirectory(dir=Path(__file__).resolve().parent.parent / "target") as tmp:
            root = Path(tmp)
            (root/"plan.json").write_text(json.dumps(plan))
            (root/"measurements.json").write_text(json.dumps(packet))
            (root/"provider.json").write_bytes(provider)
            command = [sys.executable, str(Path(pair.__file__)), "--plan", str(root/"plan.json"),
                       "--measurements", str(root/"measurements.json"),
                       "--provider-definitions", str(root/"provider.json")]
            result = subprocess.run(command, capture_output=True, text=True)
            self.assertEqual(result.returncode, 0, result.stderr)
            self.assertEqual(json.loads(result.stdout)["attempts"], 48)
            self.assertEqual(result.stderr, "")
            (root/"measurements.json").write_text('{"private":"synthetic-secret"}')
            result = subprocess.run(command, capture_output=True, text=True)
            self.assertEqual(result.returncode, 2)
            self.assertEqual(result.stdout, "")
            self.assertNotIn("synthetic-secret", result.stderr)
            self.assertNotIn(str(root), result.stderr)


if __name__ == "__main__":
    (Path(__file__).resolve().parent.parent / "target").mkdir(exist_ok=True)
    unittest.main()
