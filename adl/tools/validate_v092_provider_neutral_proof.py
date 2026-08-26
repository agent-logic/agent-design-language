#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import pathlib
import sys
from typing import Any

NEG = {"malformed_acip", "denied_authority", "interrupted_provider", "provider_unavailable", "provider_loss", "substitution_attempt"}
ASSERTS = {"names_identity_boundary", "names_continuity_boundary", "retains_witness_boundary", "rejects_startup_as_birthday"}

def fail(msg: str) -> None:
    raise SystemExit(f"issue341 provider-neutral proof validation failed: {msg}")

def expect(value: Any, msg: str) -> None:
    if not value:
        fail(msg)

def load_json(path: pathlib.Path) -> dict[str, Any]:
    try:
        value = json.loads(path.read_text(encoding="utf-8"))
    except FileNotFoundError:
        fail(f"missing json: {path}")
    expect(isinstance(value, dict), f"{path} must contain a json object")
    return value

def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("matrix", type=pathlib.Path)
    ap.add_argument("--require-live", action="store_true")
    ap.add_argument("--require-observatory", action="store_true")
    args = ap.parse_args()
    try:
        m = load_json(args.matrix)
    except FileNotFoundError:
        fail(f"missing matrix: {args.matrix}")
    expect(m.get("schema") == "adl.issue341.provider_neutral_birthday_matrix.v1", "unexpected schema")
    expect(m.get("issue") == 341, "issue must be 341")
    claims = m.get("claims")
    expect(isinstance(claims, dict), "claims object required")
    expect(claims.get("credential_material_recorded") is False, "credential material must not be recorded")
    expect(claims.get("raw_payloads_recorded") is False, "raw payloads must not be recorded")
    expect(claims.get("public_exposure_claimed") is False, "public exposure must not be claimed")
    if args.require_live:
        expect(claims.get("real_provider_positive_claimed") is True, "live positive claim required")
        expect(claims.get("local_reference_only") is False, "local reference cannot satisfy live positive proof")

    scenario_id = m.get("scenario", {}).get("id")
    columns = m.get("provider_columns")
    expect(isinstance(columns, list) and len(columns) >= 2, "at least two provider columns required")
    seen: set[str] = set()
    ops = None
    rejected_receipts: dict[str, set[str]] = {}
    for col in columns:
        expect(isinstance(col, dict), "provider column must be object")
        provider = col.get("provider")
        expect(isinstance(provider, str) and provider, "provider identity required")
        expect(provider not in seen, f"duplicate provider {provider}")
        seen.add(provider)
        expect(col.get("positive") is True, f"{provider} positive must be true")
        expect(col.get("scenario_id") == scenario_id, f"{provider} scenario mismatch")
        if args.require_live:
            expect(col.get("execution_mode") == "live_provider", f"{provider} must be live_provider")
        expect(col.get("credential_material_recorded") is False, f"{provider} recorded credential material")
        expect(col.get("raw_prompt_recorded") is False, f"{provider} recorded raw prompt")
        expect(col.get("raw_output_recorded") is False, f"{provider} recorded raw output")
        expect(isinstance(col.get("output_sha256"), str) and len(col["output_sha256"]) == 64, f"{provider} output digest invalid")
        receipt_sha = col.get("receipt_sha256")
        expect(isinstance(receipt_sha, str) and len(receipt_sha) == 64, f"{provider} receipt digest invalid")
        current_ops = json.dumps(col.get("acip_operations"), sort_keys=True, separators=(",", ":"))
        ops = current_ops if ops is None else ops
        expect(current_ops == ops, f"{provider} ACIP operations differ")
        trace_ref = col.get("trace_ref")
        expect(isinstance(trace_ref, str) and trace_ref.endswith(".json"), f"{provider} trace_ref required")
        if isinstance(trace_ref, str):
            trace_path = args.matrix.parent / trace_ref
            trace = load_json(trace_path)
            expect(trace.get("schema") == "adl.issue341.acip_trace.v1", f"{provider} trace schema mismatch")
            expect(trace.get("transport") == "localhost_tcp", f"{provider} trace transport mismatch")
            expect(trace.get("credential_material_recorded") is False, f"{provider} trace recorded credential material")
            expect(trace.get("raw_prompt_recorded") is False, f"{provider} trace recorded raw prompt")
            expect(trace.get("raw_output_recorded") is False, f"{provider} trace recorded raw output")
            events = trace.get("events")
            expect(isinstance(events, list) and len(events) >= 6, f"{provider} trace events missing")
            event_names = {event.get("event") for event in events if isinstance(event, dict)}
            for name in {"agent_listening", "acip_request_sent", "acip_request_received", "acip_response_sent", "acip_rejected"}:
                expect(name in event_names, f"{provider} trace missing {name}")
            agent_id = col.get("agent_id")
            positive_events = [
                event for event in events
                if isinstance(event, dict)
                and event.get("provider") == provider
                and event.get("agent_id") == agent_id
            ]
            positive_event_names = {event.get("event") for event in positive_events}
            for name in {"acip_request_sent", "acip_request_received", "acip_response_sent"}:
                expect(name in positive_event_names, f"{provider} trace missing same-agent {name}")
            positive_response_receipts = {
                event.get("receipt_sha256")
                for event in positive_events
                if event.get("event") == "acip_response_sent"
            }
            expect(receipt_sha in positive_response_receipts, f"{provider} receipt digest must be observed in ACIP response trace")
            for event in events:
                if not isinstance(event, dict) or event.get("event") != "acip_rejected":
                    continue
                case_name = event.get("case")
                receipt_sha = event.get("receipt_sha256")
                if isinstance(case_name, str) and isinstance(receipt_sha, str):
                    rejected_receipts.setdefault(case_name, set()).add(receipt_sha)
        assertions = col.get("semantic_assertions")
        expect(isinstance(assertions, dict), f"{provider} assertions required")
        expect(ASSERTS <= set(assertions), f"{provider} missing semantic assertions")
        for key in ASSERTS:
            expect(assertions.get(key) is True, f"{provider}.{key} must be true")

    cases = m.get("negative_cases")
    expect(isinstance(cases, list), "negative cases array required")
    names = {c.get("case") for c in cases if isinstance(c, dict)}
    expect(NEG <= names, f"missing negative cases: {sorted(NEG - names)}")
    for case in cases:
        expect(case.get("outcome") == "non_pass", f"{case.get('case')} must be non_pass")
        expect(case.get("visible") is True, f"{case.get('case')} must be visible")
        receipt_sha = case.get("receipt_sha256")
        expect(isinstance(receipt_sha, str) and len(receipt_sha) == 64, f"{case.get('case')} receipt digest required")
        expect(
            receipt_sha in rejected_receipts.get(case.get("case"), set()),
            f"{case.get('case')} receipt digest must be observed in ACIP rejection trace",
        )

    obs = m.get("observatory")
    if args.require_observatory:
        expect(isinstance(obs, dict), "observatory packet required")
    if isinstance(obs, dict):
        expect(obs.get("visibility") == "private", "observatory must be private")
        expect(obs.get("public_exposure") == "not_claimed", "observatory public exposure not allowed")
        expect(obs.get("observation_method") == "live_localhost_tcp_listener_probe", "observatory must be observed from live TCP listener probe")
        agents = obs.get("agents")
        expect(isinstance(agents, list), "observatory agents required")
        running = [a for a in agents if isinstance(a, dict) and a.get("status") == "running"]
        expect(len(running) >= 3, "at least three running agents required")
        for agent in running:
            expect(agent.get("acip_direct_tcp") is True, f"{agent.get('agent_id')} missing direct ACIP TCP")
            if agent.get("role") != "shepherd":
                expect(agent.get("ssm_access") == "none", f"{agent.get('agent_id')} must not have SSM")

    print(json.dumps({"schema": "adl.issue341.validation.v1", "status": "passed", "matrix": str(args.matrix)}, sort_keys=True))
    return 0

if __name__ == "__main__":
    sys.exit(main())
