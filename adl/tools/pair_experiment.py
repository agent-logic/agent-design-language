#!/usr/bin/env python3
"""Bounded PAIR result accounting. Valid accounting is not runtime qualification.

PVF: deterministic local tooling contract, small CPU/filesystem, required #904
harness gate. Real two-node raw/Runtime execution is a separate required gate.
"""
from __future__ import annotations

import argparse
import concurrent.futures
import hashlib
import http.client
import ipaddress
import itertools
import json
import math
import re
import statistics
import socket
import sys
import time
import threading
from urllib.parse import urlsplit
from pathlib import Path

ROUTES = ("baseline", "raw_pair", "runtime")
SCENARIOS = ("healthy", "node_loss")
# Small-CPU accounting gate: reject oversized matrices before constructing keys.
MAX_MATRIX_RECORDS = 100_000
HEX = re.compile(r"[0-9a-f]{64}\Z")
ID = re.compile(r"[a-zA-Z0-9_.-]{1,100}\Z")


class InvalidExperiment(ValueError):
    """An incomplete or incomparable result cannot produce a summary."""


def require(condition, code):
    if not condition:
        raise InvalidExperiment(code)


def digest(value):
    return hashlib.sha256(json.dumps(value, sort_keys=True, separators=(",", ":"),
                                     allow_nan=False).encode()).hexdigest()


def text_digest(value):
    return hashlib.sha256(value.encode()).hexdigest()


def exact_fields(value, fields, code):
    require(isinstance(value, dict) and set(value) == set(fields.split()), code)


def positive(value, code):
    require(type(value) in (int, float) and math.isfinite(value) and value > 0, code)


def validate_plan(plan):
    exact_fields(plan, "schema candidate_sha pair_revision model_revision tokenizer_revision "
                 "provider_definition_sha256 corpus sampling warmth repetitions concurrency "
                 "nodes lost_node max_request_seconds max_run_seconds", "plan_fields")
    require(plan["schema"] == "adl.pair.plan.v1", "plan_schema")
    for key in ("candidate_sha", "pair_revision", "model_revision", "tokenizer_revision"):
        require(isinstance(plan[key], str) and re.fullmatch(r"[0-9a-f]{40,64}", plan[key]), key)
    require(isinstance(plan["provider_definition_sha256"], str)
            and HEX.fullmatch(plan["provider_definition_sha256"]), "provider_digest")
    require(plan["warmth"] in ("cold", "warm"), "warmth")
    exact_fields(plan["sampling"], "temperature seed max_tokens", "sampling_fields")
    require(type(plan["sampling"]["temperature"]) in (int, float)
            and plan["sampling"]["temperature"] == 0, "deterministic_sampling_required")
    require(type(plan["sampling"]["seed"]) is int, "seed")
    require(type(plan["sampling"]["max_tokens"]) is int
            and 0 < plan["sampling"]["max_tokens"] <= 4096, "max_tokens")
    require(type(plan["repetitions"]) is int and 2 <= plan["repetitions"] <= 100, "repetitions")
    concurrency = plan["concurrency"]
    require(isinstance(concurrency, list) and concurrency
            and all(type(x) is int and 1 <= x <= 64 for x in concurrency)
            and len(set(concurrency)) == len(concurrency) and 1 in concurrency
            and max(concurrency) > 1, "concurrency")
    nodes = plan["nodes"]
    require(isinstance(nodes, list) and len(nodes) == 2 and len(set(nodes)) == 2
            and all(isinstance(x, str) and ID.fullmatch(x) for x in nodes), "nodes")
    require(plan["lost_node"] in nodes, "lost_node")
    positive(plan["max_request_seconds"], "max_request_seconds")
    positive(plan["max_run_seconds"], "max_run_seconds")
    corpus = plan["corpus"]
    require(isinstance(corpus, list) and 1 <= len(corpus) <= 1000, "corpus")
    seen = set()
    for request in corpus:
        exact_fields(request, "id prompt expected_text", "corpus_fields")
        require(isinstance(request["id"], str) and ID.fullmatch(request["id"])
                and request["id"] not in seen, "corpus_id")
        seen.add(request["id"])
        for key in ("prompt", "expected_text"):
            require(isinstance(request[key], str) and 0 < len(request[key]) <= 65536, key)
    require(len(corpus) >= max(concurrency), "insufficient_concurrent_requests")
    cardinality = len(ROUTES) * len(SCENARIOS) * len(concurrency) * plan["repetitions"] * len(corpus)
    require(cardinality <= MAX_MATRIX_RECORDS, "matrix_resource_bound")
    return digest(plan)


def summarize(plan, packet, provider_bytes):
    """Validate the complete matrix; never accept supplied success/disposition flags.

    Records are untrusted measurement inputs. Hashes bind their declared plan,
    not authenticity. Independent route/node/candidate evidence remains required.
    """
    plan_digest = validate_plan(plan)
    require(hashlib.sha256(provider_bytes).hexdigest() == plan["provider_definition_sha256"],
            "provider_definition_changed")
    exact_fields(packet, "schema plan_sha256 records node_events", "packet_fields")
    require(packet["schema"] == "adl.pair.measurements.v1", "packet_schema")
    require(packet["plan_sha256"] == plan_digest, "plan_changed")
    corpus = {row["id"]: row for row in plan["corpus"]}
    expected = set(itertools.product(ROUTES, SCENARIOS, plan["concurrency"],
                                    range(plan["repetitions"]), corpus))
    records = packet["records"]
    require(isinstance(records, list) and len(records) == len(expected), "matrix_count")
    observed = {}
    groups = {}
    for row in records:
        exact_fields(row, "route scenario concurrency repetition request_id plan_sha256 "
                     "prompt_sha256 started_seconds ended_seconds output error serving_node "
                     "resource_sample_sha256", "record_fields")
        key = tuple(row[k] for k in ("route", "scenario", "concurrency", "repetition", "request_id"))
        require(type(row["concurrency"]) is int and type(row["repetition"]) is int,
                "record_indices")
        require(key in expected and key not in observed, "matrix_identity")
        require(row["plan_sha256"] == plan_digest, "record_plan_changed")
        require(row["prompt_sha256"] == text_digest(corpus[row["request_id"]]["prompt"]),
                "prompt_changed")
        start, end = row["started_seconds"], row["ended_seconds"]
        require(type(start) in (int, float) and math.isfinite(start) and start >= 0,
                "invalid_start")
        positive(end, "invalid_end")
        require(0 < end - start <= plan["max_request_seconds"]
                and end <= plan["max_run_seconds"], "request_bound")
        require(isinstance(row["resource_sample_sha256"], str)
                and HEX.fullmatch(row["resource_sample_sha256"]), "resource_sample")
        require(row["serving_node"] in plan["nodes"] or row["serving_node"] is None,
                "serving_node")
        if row["error"] is None:
            require(row["output"] == corpus[row["request_id"]]["expected_text"], "incorrect_output")
            require(row["serving_node"] is not None, "missing_serving_node")
        else:
            require(row["error"] in ("timeout", "unavailable", "cancelled", "malformed_response")
                    and row["output"] is None, "error_record")
        observed[key] = row
        groups.setdefault(key[:4], []).append(row)
    require(set(observed) == expected, "matrix_missing")
    events = packet["node_events"]
    require(isinstance(events, list) and events, "missing_node_events")
    for event in events:
        exact_fields(event, "node available at_seconds evidence_sha256", "event_fields")
        require(event["node"] in plan["nodes"] and type(event["available"]) is bool, "event_node")
        require(type(event["at_seconds"]) in (int, float)
                and math.isfinite(event["at_seconds"])
                and 0 <= event["at_seconds"] <= plan["max_run_seconds"], "event_time")
        require(isinstance(event["evidence_sha256"], str) and HEX.fullmatch(event["evidence_sha256"]),
                "event_evidence")
    losses = [e for e in events if e["node"] == plan["lost_node"] and not e["available"]]
    require(losses, "node_loss_not_observed")
    loss_time = min(e["at_seconds"] for e in losses)
    for key, rows in groups.items():
        route, scenario, concurrency, _ = key
        if scenario == "healthy":
            require(all(r["ended_seconds"] <= loss_time for r in rows), "healthy_after_loss")
        else:
            require(all(r["started_seconds"] >= loss_time for r in rows), "loss_before_event")
            require(all(r["serving_node"] != plan["lost_node"] for r in rows
                        if r["error"] is None), "lost_node_served")
        if concurrency >= 1:
            timeline = sorted([(r["started_seconds"], 1) for r in rows]
                              + [(r["ended_seconds"], -1) for r in rows])
            active = maximum = 0
            for _, delta in timeline:
                active += delta
                maximum = max(maximum, active)
            require(maximum == concurrency, "concurrency_not_observed")
    for route in ("raw_pair", "runtime"):
        serving = {r["serving_node"] for k, r in observed.items()
                   if k[0] == route and k[1] == "healthy" and r["error"] is None}
        require(serving == set(plan["nodes"]), "two_node_routing_not_observed")
    summaries = []
    for route, scenario, concurrency in itertools.product(ROUTES, SCENARIOS, plan["concurrency"]):
        selected = [r for k, r in observed.items() if k[:3] == (route, scenario, concurrency)]
        failures = sum(r["error"] is not None for r in selected)
        durations = [r["ended_seconds"] - r["started_seconds"] for r in selected]
        batch_seconds = sum(max(r["ended_seconds"] for r in groups[(route, scenario, concurrency, i)])
                            - min(r["started_seconds"] for r in groups[(route, scenario, concurrency, i)])
                            for i in range(plan["repetitions"]))
        summaries.append(dict(route=route, scenario=scenario, concurrency=concurrency,
                              attempts=len(selected), failures=failures,
                              median_latency_seconds=statistics.median(durations),
                              completed_requests_per_second=(len(selected)-failures)/batch_seconds))
    comparisons = []
    for scenario, concurrency in itertools.product(SCENARIOS, plan["concurrency"]):
        selected = {r["route"]: r for r in summaries
                    if r["scenario"] == scenario and r["concurrency"] == concurrency}
        baseline = selected["baseline"]["completed_requests_per_second"]
        for route in ("raw_pair", "runtime"):
            comparable = baseline > 0 and not selected["baseline"]["failures"] and not selected[route]["failures"]
            comparisons.append(dict(route=route, scenario=scenario, concurrency=concurrency,
                                    throughput_ratio=(selected[route]["completed_requests_per_second"] / baseline
                                                      if comparable else None)))
    return dict(schema="adl.pair.accounting.v1", plan_sha256=plan_digest,
                measurement_sha256=digest(packet), attempts=len(records), summaries=summaries, comparisons=comparisons,
                qualification="not_established_by_accounting", disposition="requires_independent_real_run_review")


def collect_ollama(endpoint, model, prompt, sampling, timeout_seconds=30, keep_alive_seconds=0):
    """Collect one actual local Ollama-compatible response, without route claims.

    PAIR's application proxy is local-only. No proxies, redirects, DNS, model
    acquisition, configuration changes or inferred Runtime/node identity.
    """
    exact_fields(sampling, "temperature seed max_tokens", "sampling_fields")
    require(type(sampling["temperature"]) in (int, float) and sampling["temperature"] == 0,
            "deterministic_sampling_required")
    require(type(sampling["seed"]) is int, "seed")
    require(type(sampling["max_tokens"]) is int and 0 < sampling["max_tokens"] <= 4096,
            "max_tokens")
    require(type(keep_alive_seconds) is int and 0 <= keep_alive_seconds <= 300,
            "collector_residency_bound")
    positive(timeout_seconds, "collector_timeout")
    require(timeout_seconds <= 120, "collector_timeout")
    require(isinstance(model, str) and 0 < len(model) <= 256, "collector_model")
    require(isinstance(prompt, str) and 0 < len(prompt) <= 65536, "collector_prompt")
    url = urlsplit(endpoint)
    require(url.scheme == "http" and not url.username and not url.password
            and not url.query and not url.fragment and url.path in ("", "/"), "collector_endpoint")
    try:
        address = ipaddress.ip_address(url.hostname)
        port = url.port
    except (ValueError, TypeError):
        raise InvalidExperiment("collector_endpoint") from None
    require(address.is_loopback and port is not None and 0 < port < 65536, "collector_endpoint")
    payload = json.dumps(dict(model=model, prompt=prompt, stream=False,
                              keep_alive=keep_alive_seconds,
                              options=dict(temperature=sampling["temperature"], seed=sampling["seed"],
                                           num_predict=sampling["max_tokens"]))).encode()
    started = time.monotonic()
    deadline = started + timeout_seconds
    connection = None
    response = None
    timer = None
    try:
        connection = socket.create_connection((str(address), port), timeout=timeout_seconds)
        def interrupt():
            try:
                connection.shutdown(socket.SHUT_RDWR)
            except OSError:
                pass
        timer = threading.Timer(max(0, deadline - time.monotonic()), interrupt)
        timer.daemon = True
        timer.start()
        host = f"[{address}]:{port}" if address.version == 6 else f"{address}:{port}"
        headers = (f"POST /api/generate HTTP/1.1\r\nHost: {host}\r\n"
                   f"Connection: close\r\nContent-Type: application/json\r\n"
                   f"Accept: application/json\r\nContent-Length: {len(payload)}\r\n\r\n")
        connection.sendall(headers.encode() + payload)
        connection.settimeout(max(0.001, deadline - time.monotonic()))
        response = http.client.HTTPResponse(connection)
        response.begin()
        require(response.status == 200, "collector_http_status")
        # read1 avoids waiting for an entire requested-size buffer on slow peers.
        chunks = []
        size = 0
        while True:
            remaining = deadline - time.monotonic()
            require(remaining > 0, "collector_timeout")
            connection.settimeout(remaining)
            chunk = response.read1(min(65536, 1024 * 1024 + 1 - size))
            if not chunk:
                break
            size += len(chunk)
            require(size <= 1024 * 1024, "collector_response_bound")
            chunks.append(chunk)
        decoded = json.loads(b"".join(chunks))
        require(isinstance(decoded, dict) and decoded.get("done") is True
                and isinstance(decoded.get("response"), str) and decoded["response"].strip()
                and not decoded.get("error"), "collector_response")
        elapsed = time.monotonic() - started
        require(elapsed <= timeout_seconds, "collector_timeout")
        return dict(schema="adl.pair.ollama_observation.v1", elapsed_seconds=elapsed,
                    prompt_sha256=text_digest(prompt), output=decoded["response"],
                    output_sha256=text_digest(decoded["response"]),
                    response_sha256=hashlib.sha256(b"".join(chunks)).hexdigest(),
                    route_identity="not_verified", serving_node="not_verified")
    except (OSError, ValueError, http.client.HTTPException, AttributeError):
        raise InvalidExperiment("collector_transport_or_response") from None
    finally:
        if timer is not None:
            timer.cancel()
        if response is not None:
            response.close()
        if connection is not None:
            connection.close()


def collect_ollama_batch(endpoint, model, corpus, sampling, repetitions, concurrency,
                         timeout_seconds=30, keep_alive_seconds=0):
    """Collect a bounded same-corpus batch; route/node claims remain unverified.

    The caller selects an already verified baseline or PAIR loopback endpoint.
    Output text is retained so private evidence can prove exact correctness;
    publish only a redacted projection with content hashes.
    """
    require(isinstance(corpus, list) and 1 <= len(corpus) <= 1000, "batch_corpus")
    require(type(repetitions) is int and 1 <= repetitions <= 100, "batch_repetitions")
    require(type(concurrency) is int and 1 <= concurrency <= 64
            and concurrency <= len(corpus), "batch_concurrency")
    require(len(corpus) * repetitions <= MAX_MATRIX_RECORDS, "batch_resource_bound")
    seen = set()
    for row in corpus:
        exact_fields(row, "id prompt expected_text", "batch_corpus_fields")
        require(isinstance(row["id"], str) and ID.fullmatch(row["id"])
                and row["id"] not in seen, "batch_corpus_id")
        seen.add(row["id"])
        for key in ("prompt", "expected_text"):
            require(isinstance(row[key], str) and 0 < len(row[key]) <= 65536,
                    f"batch_{key}")
    records = []
    batches = []
    for repetition in range(repetitions):
        batch_started = time.monotonic()
        with concurrent.futures.ThreadPoolExecutor(max_workers=concurrency) as executor:
            futures = [
                (row, executor.submit(collect_ollama, endpoint, model, row["prompt"], sampling,
                                      timeout_seconds, keep_alive_seconds))
                for row in corpus
            ]
            for row, future in futures:
                observation = future.result()
                records.append(dict(
                    repetition=repetition,
                    request_id=row["id"],
                    correct=observation["output"].strip() == row["expected_text"],
                    **observation,
                ))
        batches.append(dict(repetition=repetition,
                            elapsed_seconds=time.monotonic() - batch_started))
    return dict(schema="adl.pair.ollama_batch.v1", model=model,
                corpus_sha256=digest(corpus), sampling=sampling,
                repetitions=repetitions, concurrency=concurrency,
                attempts=len(records), records=records, batches=batches,
                route_identity="not_verified", serving_nodes="not_verified",
                qualification="not_established_by_collection")


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--plan", type=Path, required=True)
    parser.add_argument("--measurements", type=Path, required=True)
    parser.add_argument("--provider-definitions", type=Path, required=True)
    args = parser.parse_args(argv)
    try:
        # Reject non-finite JSON values instead of letting NaN weaken comparisons.
        def read(path):
            require(path.stat().st_size <= 32 * 1024 * 1024, "input_size")
            return json.loads(path.read_text(), parse_constant=lambda _: (_ for _ in ()).throw(
                InvalidExperiment("nonfinite_json")))
        require(args.provider_definitions.stat().st_size <= 32 * 1024 * 1024, "provider_size")
        result = summarize(read(args.plan), read(args.measurements), args.provider_definitions.read_bytes())
    except (InvalidExperiment, ValueError, TypeError, KeyError, OSError):
        print("adl_event pair_accounting_rejected: invalid or unavailable input", file=sys.stderr)
        return 2
    print(json.dumps(result, sort_keys=True, allow_nan=False))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
