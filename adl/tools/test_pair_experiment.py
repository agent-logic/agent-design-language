#!/usr/bin/env python3
"""PVF required deterministic local accounting/negative contract; no inference/network."""
import copy
import hashlib
import itertools
import threading
from contextlib import contextmanager
from http.server import BaseHTTPRequestHandler, HTTPServer
import json
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch
from pathlib import Path

import pair_experiment as pair

SAMPLING = dict(temperature=0, seed=7, max_tokens=8)


def fixture():
    provider = b'{"providers":{}}'
    resources = b'{"samples":[]}'
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
                         resource_sample_sha256=hashlib.sha256(resources).hexdigest()))
    packet = dict(schema="adl.pair.measurements.v1", plan_sha256=sha, records=rows,
                  node_events=[dict(node="beta", available=False, at_seconds=50,
                                    evidence_sha256="f"*64)])
    return plan, packet, provider, resources


@contextmanager
def endpoint(response_body=b'{"done":true,"response":"0"}', status=200, stall=False):
    class Handler(BaseHTTPRequestHandler):
        def log_message(self, *args):
            pass

        def do_POST(self):
            self.server.requests.append((self.path, json.loads(self.rfile.read(int(self.headers["Content-Length"])))))
            if stall:
                self.server.release.wait(2)
            self.send_response(status)
            self.send_header("Content-Length", str(len(response_body)))
            self.end_headers()
            try:
                self.wfile.write(response_body)
            except (BrokenPipeError, ConnectionResetError):
                pass
    server = HTTPServer(("127.0.0.1", 0), Handler)
    server.requests = []
    server.release = threading.Event()
    worker = threading.Thread(target=server.serve_forever, daemon=True)
    worker.start()
    try:
        yield f"http://127.0.0.1:{server.server_port}", server
    finally:
        server.release.set()
        server.shutdown()
        server.server_close()
        worker.join(2)


class AccountingTests(unittest.TestCase):
    def test_complete_matrix_preserves_null_benefit_without_qualification(self):
        plan, packet, provider, resources = fixture()
        summary = pair.summarize(plan, packet, provider, resources)
        self.assertEqual(summary["attempts"], 48)
        self.assertEqual(summary["qualification"], "not_established_by_accounting")
        rates = [r["completed_requests_per_second"] for r in summary["summaries"]
                 if r["scenario"] == "healthy" and r["concurrency"] == 2]
        self.assertEqual(rates, [2, 2, 2])
        self.assertTrue(all(row["throughput_ratio"] == 1 for row in summary["comparisons"]))
        self.assertEqual(summary, pair.summarize(plan, packet, provider, resources))

    def test_bounded_batch_runs_concurrently_and_preserves_exact_outputs(self):
        corpus = [
            dict(id="alpha", prompt="Return alpha", expected_text="alpha"),
            dict(id="beta", prompt="Return beta", expected_text="beta"),
        ]
        barrier = threading.Barrier(2)

        def collect(_endpoint, _model, prompt, _sampling, _timeout, _residency):
            barrier.wait(1)
            output = prompt.removeprefix("Return ")
            return dict(schema="adl.pair.ollama_observation.v1", elapsed_seconds=0.1,
                        prompt_sha256=pair.text_digest(prompt), output=output,
                        output_sha256=pair.text_digest(output), response_sha256="a" * 64,
                        route_identity="not_verified", serving_node="not_verified")

        with patch.object(pair, "collect_ollama", side_effect=collect):
            result = pair.collect_ollama_batch(
                "http://127.0.0.1:1", "fixture", corpus, SAMPLING, 2, 2, 1, 0)
        self.assertEqual(result["attempts"], 4)
        self.assertTrue(all(row["correct"] for row in result["records"]))
        self.assertEqual(result["qualification"], "not_established_by_collection")
        self.assertEqual(result["route_identity"], "not_verified")

    def test_batch_rejects_unbounded_or_duplicate_work_before_collection(self):
        corpus = [dict(id="same", prompt="x", expected_text="x")] * 2
        with patch.object(pair, "collect_ollama") as collect:
            with self.assertRaisesRegex(pair.InvalidExperiment, "batch_corpus_id"):
                pair.collect_ollama_batch(
                    "http://127.0.0.1:1", "fixture", corpus, SAMPLING, 1, 1)
            collect.assert_not_called()
        corpus = [dict(id=f"q{i}", prompt="x", expected_text="x") for i in range(1000)]
        with patch.object(pair, "MAX_MATRIX_RECORDS", 99_999):
            with self.assertRaisesRegex(pair.InvalidExperiment, "batch_resource_bound"):
                pair.collect_ollama_batch(
                    "http://127.0.0.1:1", "fixture", corpus, SAMPLING, 100, 1)

    def test_failures_retained_in_denominator(self):
        plan, packet, provider, resources = fixture()
        row = packet["records"][-1]
        row.update(output=None, error="unavailable", serving_node=None)
        summary = pair.summarize(plan, packet, provider, resources)
        self.assertEqual(summary["attempts"], 48)
        self.assertEqual(sum(x["failures"] for x in summary["summaries"]), 1)

    def test_negative_benefit_is_reported_without_hiding_failures(self):
        plan, packet, provider, resources = fixture()
        for row in packet["records"]:
            if row["route"] == "raw_pair" and row["concurrency"] == 2:
                row["ended_seconds"] += 1
        result = pair.summarize(plan, packet, provider, resources)
        self.assertTrue(all(r["throughput_ratio"] == 0.5 for r in result["comparisons"]
                            if r["route"] == "raw_pair" and r["concurrency"] == 2))
        packet["records"][-1].update(output=None, error="timeout", serving_node=None)
        result = pair.summarize(plan, packet, provider, resources)
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
                plan, packet, provider, resources = fixture()
                change(packet)
                with self.assertRaises(pair.InvalidExperiment):
                    pair.summarize(plan, packet, provider, resources)

    def test_comparison_context_changes_reject(self):
        for field, value in [("model_revision", "e"*40), ("warmth", "cold"),
                             ("concurrency", [1, 3]), ("tokenizer_revision", "f"*40)]:
            with self.subTest(field=field):
                plan, packet, provider, resources = fixture()
                plan[field] = value
                with self.assertRaises(pair.InvalidExperiment):
                    pair.summarize(plan, packet, provider, resources)
        plan, packet, _, resources = fixture()
        with self.assertRaises(pair.InvalidExperiment):
            pair.summarize(plan, packet, b"changed provider", resources)

    def test_valid_but_wrong_resource_digest_rejects(self):
        plan, packet, provider, resources = fixture()
        packet["records"][0]["resource_sample_sha256"] = "e" * 64
        with self.assertRaisesRegex(pair.InvalidExperiment, "resource_sample"):
            pair.summarize(plan, packet, provider, resources)

    def test_serialized_requests_do_not_prove_concurrency(self):
        plan, packet, provider, resources = fixture()
        for row in packet["records"]:
            if row["concurrency"] == 2 and row["request_id"] == "q1":
                row["started_seconds"] += 1
                row["ended_seconds"] += 1
        with self.assertRaisesRegex(pair.InvalidExperiment, "concurrency_not_observed"):
            pair.summarize(plan, packet, provider, resources)

    def test_failover_topology_and_actual_concurrency_are_required(self):
        for change in ("one_node", "serial_overlap"):
            with self.subTest(change=change):
                plan, packet, provider, resources = fixture()
                for row in packet["records"]:
                    if change == "one_node":
                        row["serving_node"] = "alpha"
                    elif row["concurrency"] == 1 and row["request_id"] == "q1":
                        row["started_seconds"] -= 1
                        row["ended_seconds"] -= 1
                with self.assertRaises(pair.InvalidExperiment):
                    pair.summarize(plan, packet, provider, resources)

    def test_fastest_node_then_survivor_proves_two_node_failover(self):
        plan, packet, provider, resources = fixture()
        for row in packet["records"]:
            if row["route"] in ("raw_pair", "runtime"):
                row["serving_node"] = "beta" if row["scenario"] == "healthy" else "alpha"
        summary = pair.summarize(plan, packet, provider, resources)
        self.assertEqual(summary["attempts"], len(packet["records"]))

    def test_healthy_route_must_observe_node_that_is_later_lost(self):
        plan, packet, provider, resources = fixture()
        for row in packet["records"]:
            if row["route"] in ("raw_pair", "runtime"):
                row["serving_node"] = "alpha"
        with self.assertRaisesRegex(pair.InvalidExperiment, "lost_node_not_observed_healthy"):
            pair.summarize(plan, packet, provider, resources)

    def test_oversized_matrix_rejects_before_product_allocation(self):
        plan, packet, provider, resources = fixture()
        plan["repetitions"] = 100
        plan["concurrency"] = list(range(1, 65))
        plan["corpus"] = [dict(id=f"q{i}", prompt="Return 1", expected_text="1")
                          for i in range(1000)]
        with patch.object(pair.itertools, "product", side_effect=AssertionError("must not allocate")):
            with self.assertRaisesRegex(pair.InvalidExperiment, "matrix_resource_bound"):
                pair.summarize(plan, packet, provider, resources)

    def test_collector_sends_real_ollama_request_without_runtime_claim(self):
        with endpoint() as (url, server):
            observed = pair.collect_ollama(url, "fixture-model", "Return 0", SAMPLING, 1)
            self.assertEqual(server.requests, [("/api/generate", dict(model="fixture-model",
                             prompt="Return 0", stream=False, keep_alive=0,
                             options=dict(temperature=0, seed=7, num_predict=8)))])
            self.assertEqual(observed["output"], "0")
            self.assertEqual(observed["route_identity"], "not_verified")
            self.assertEqual(observed["serving_node"], "not_verified")
            self.assertGreater(observed["elapsed_seconds"], 0)

    def test_collector_requires_bounded_sampling_and_residency_before_dispatch(self):
        with patch.object(pair.socket, "create_connection", side_effect=AssertionError("no dispatch")):
            for sampling, residency in [(SAMPLING, -1), (SAMPLING, 301),
                                         (dict(temperature=1, seed=7, max_tokens=8), 0),
                                         (dict(temperature=0, seed=7, max_tokens=0), 0)]:
                with self.subTest(sampling=sampling, residency=residency), self.assertRaises(pair.InvalidExperiment):
                    pair.collect_ollama("http://127.0.0.1:11434", "model", "prompt", sampling,
                                        keep_alive_seconds=residency)

    def test_collector_rejects_redirect_malformed_and_oversized_response(self):
        for body, status in [(b"{}", 200), (b"secret-error", 302),
                             (b"x"*(1024*1024+1), 200)]:
            with self.subTest(status=status, size=len(body)), endpoint(body, status) as (url, _):
                with self.assertRaises(pair.InvalidExperiment):
                    pair.collect_ollama(url, "model", "prompt", SAMPLING, 1)

    def test_collector_deadline_interrupts_stalled_headers(self):
        with endpoint(stall=True) as (url, _):
            start = pair.time.monotonic()
            with self.assertRaises(pair.InvalidExperiment):
                pair.collect_ollama(url, "model", "prompt", SAMPLING, 0.05)
            self.assertLess(pair.time.monotonic() - start, 1)

    def test_collector_rejects_nonlocal_or_credential_endpoints_before_socket(self):
        with patch.object(pair.socket, "create_connection", side_effect=AssertionError("no dispatch")):
            for url in ["http://example.com:8080", "http://192.168.1.2:8080",
                        "http://user:secret@127.0.0.1:8080", "https://127.0.0.1:8080",
                        "http://127.0.0.1:8080/api/generate", "http://127.0.0.1:8080?token=secret"]:
                with self.subTest(url=url), self.assertRaises(pair.InvalidExperiment):
                    pair.collect_ollama(url, "model", "prompt", SAMPLING)

    def test_cli_machine_channel_and_redacted_failure(self):
        plan, packet, provider, resources = fixture()
        with tempfile.TemporaryDirectory(dir=Path(__file__).resolve().parent.parent / "target") as tmp:
            root = Path(tmp)
            (root/"plan.json").write_text(json.dumps(plan))
            (root/"measurements.json").write_text(json.dumps(packet))
            (root/"provider.json").write_bytes(provider)
            (root/"resources.json").write_bytes(resources)
            command = [sys.executable, str(Path(pair.__file__)), "--plan", str(root/"plan.json"),
                       "--measurements", str(root/"measurements.json"),
                       "--provider-definitions", str(root/"provider.json"),
                       "--resource-samples", str(root/"resources.json")]
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
