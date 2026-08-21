#!/usr/bin/env python3
"""Run one real Runtime/UTS/ACC cycle for each of the six #268 residents."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import pathlib
import subprocess
from typing import Any


ROOT = pathlib.Path(__file__).resolve().parents[2]
DEFAULT_PLAN = ROOT / "adl/tools/issue268_six_resident_uts_plan.json"
RUNTIME_RECEIPT = "adl.runtime.resident_tool_receipt.v1"


def canonical_bytes(value: Any) -> bytes:
    return json.dumps(value, separators=(",", ":"), sort_keys=True).encode("utf-8")


def canonical_digest(value: Any) -> str:
    return hashlib.sha256(canonical_bytes(value)).hexdigest()


def digest_file(path: pathlib.Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def atomic_json(path: pathlib.Path, value: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    temporary = path.with_suffix(path.suffix + ".tmp")
    temporary.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    os.replace(temporary, path)


def safe_id(value: str) -> str:
    if not value or any(character not in "abcdefghijklmnopqrstuvwxyz0123456789-" for character in value):
        raise SystemExit(f"unsafe resident id: {value!r}")
    return value


def authority(resident: dict[str, Any]) -> dict[str, Any]:
    agent_id = resident["agent_id"]
    authority_id = f"authority.{agent_id}"
    authority_ref = f"runtime://resident/{agent_id}/tool-authority"
    allowed_tools = sorted(set(resident["tool_authority"]) | {"runtime.observe"})
    material = {
        "authority_id": authority_id,
        "authority_ref": authority_ref,
        "allowed_tools": allowed_tools,
    }
    return {**material, "authority_sha256": canonical_digest(material)}


def proposal(agent_id: str, phase: str, task_id: str) -> dict[str, Any]:
    return {
        "tool_proposal": {
            "proposal_id": f"issue268.{agent_id}.{phase}.{task_id}",
            "tool_name": "runtime.observe",
            "tool_version": "1.0.0",
            "adapter_id": "adapter.runtime.observe.dry_run",
            "arguments": {},
            "dry_run_requested": True,
            "ambiguous": False,
        }
    }


def write_workflow(path: pathlib.Path, resident: dict[str, Any], phase: str, task_id: str) -> None:
    expected = json.dumps(proposal(resident["agent_id"], phase, task_id), separators=(",", ":"))
    document = f'''version: "0.5"
providers:
  local_ollama:
    type: "ollama"
    base_url: "http://127.0.0.1:11434"
    config:
      model: "{resident["model"]}"
agents:
  resident:
    provider: "local_ollama"
    model: "{resident["model"]}"
tasks:
  governed_runtime_observation:
    prompt:
      system: 'Return exactly the supplied JSON object. Do not add reasoning, commentary, markdown, or a code fence.'
      user: '{expected}'
run:
  name: "issue268-{resident["agent_id"]}-{phase}-{task_id}"
  workflow:
    kind: "sequential"
    steps:
      - id: "resident-tool-proposal"
        agent: "resident"
        task: "governed_runtime_observation"
'''
    path.write_text(document, encoding="utf-8")


def runtime_observe_registry() -> dict[str, Any]:
    empty = {"type": "object", "additionalProperties": False, "properties": {}, "required": []}
    uts = {
        "schema_version": "uts.v1.1", "compatible_versions": ["uts.v1", "uts.v1.1"],
        "name": "runtime.observe", "version": "1.0.0",
        "description": "Return a redacted aggregate observation of the current Runtime.",
        "categories": ["read_only", "observability_sensitive"],
        "input_schema": empty, "output_schema": empty, "side_effect_class": "read",
        "side_effects": ["none"], "determinism": "bounded_nondeterministic",
        "replay_safety": "replay_safe", "idempotence": "idempotent",
        "resources": [{"resource_type": "runtime", "scope": "aggregate-observation"}],
        "authentication": {"mode": "none", "required": False},
        "data_sensitivity": "internal", "exfiltration_risk": "none",
        "execution_environment": {"kind": "dry_run", "isolation": "runtime-owned aggregate-only adapter"},
        "errors": [{"code": "runtime_observation_unavailable", "message": "The redacted Runtime observation is unavailable.", "retryable": False}],
        "observability": "governance", "planning": {"review_recommended": False}, "extensions": {},
    }
    return {
        "schema_version": "tool_registry.v1", "registry_id": "runtime.resident.tools",
        "tools": [{"registry_tool_id": "runtime.observe.v1", "tool_name": "runtime.observe", "tool_version": "1.0.0", "active": True, "uts": uts, "approved_adapter_ids": ["adapter.runtime.observe.dry_run"]}],
        "adapters": [{"adapter_id": "adapter.runtime.observe.dry_run", "tool_name": "runtime.observe", "tool_version": "1.0.0", "capability_id": "capability.runtime.observe.v1", "side_effect_class": "read", "execution_environment": "dry_run", "supports_dry_run": True, "approved_for_binding": True}],
    }


def write_spec(path: pathlib.Path, resident: dict[str, Any], runtime_root: pathlib.Path) -> dict[str, Any]:
    binding = authority(resident)
    policy = {
        "actor_id": resident["agent_id"], "role": resident["role"], "standing": "active",
        "authenticated": True, "grant_id": binding["authority_id"],
        "grantor_actor_id": "issue268.runtime", "grant_status": "active", "delegation": None,
        "allowed_side_effects": ["read"], "allowed_resource_scopes": ["aggregate-observation"],
        "allow_sensitive_data": False, "visibility_constructible": True,
        "replay_allowed": True, "execution_approved": True,
    }
    spec = {
        "schema": "adl.long_lived_agent_spec.v1", "agent_instance_id": resident["agent_id"],
        "display_name": f"Issue 268 {resident['role']}",
        "state_root": str(runtime_root / "residents" / resident["agent_id"]),
        "workflow": {"kind": "adl_workflow", "name": f"issue268-{resident['agent_id']}", "path": "workflow.adl.yaml", "run_args": {
            "freedom_gate_policy_decision": "allowed", "tool_registry": runtime_observe_registry(),
            "tool_policy_context": policy, "tool_risk_class": "low",
            "citizen_boundary_ref": "runtime.resident.boundary",
            "tool_gate_context": {"policy_decision": "allowed", "requires_operator_review": False,
                "requires_human_challenge": False, "escalation_available": False,
                "citizen_action_boundary_intact": True, "operator_action_boundary_intact": True,
                "private_arguments_redacted": True}}},
        "heartbeat": {"interval_secs": 1, "max_cycles": 2, "stale_lease_after_secs": 900},
        "checkpoint": {"interval_secs": 1, "allow_agent_requested": True, "min_request_interval_secs": 1},
        "safety": {"allow_network": False, "allow_broker": False,
            "allow_filesystem_writes_outside_state_root": False, "allow_real_world_side_effects": False,
            "require_public_artifact_sanitization": True, "financial_advice": False,
            "max_cycle_runtime_secs": 900, "max_consecutive_failures": 1},
        "memory": {}, "resident_role": resident["role"], "tool_authority": binding,
    }
    atomic_json(path, spec)
    return binding


def run_agent(runtime_bin: pathlib.Path, spec: pathlib.Path) -> None:
    completed = subprocess.run([str(runtime_bin), "agent", "tick", "--spec", str(spec), "--json"], cwd=ROOT, check=False)
    if completed.returncode != 0:
        raise SystemExit(f"Runtime agent tick failed ({completed.returncode}): {spec}")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--phase", required=True, choices=("pre", "post"))
    parser.add_argument("--state", required=True, type=pathlib.Path)
    parser.add_argument("--evidence-dir", required=True, type=pathlib.Path)
    parser.add_argument("--plan", type=pathlib.Path, default=DEFAULT_PLAN)
    parser.add_argument("--runtime-bin", required=True, type=pathlib.Path)
    parser.add_argument("--runtime-root", required=True, type=pathlib.Path)
    args = parser.parse_args()
    if not args.runtime_bin.is_file():
        raise SystemExit("real Runtime binary is required")
    plan = json.loads(args.plan.read_text(encoding="utf-8"))
    residents = plan.get("residents") or []
    if len(residents) != 6 or len({row["agent_id"] for row in residents}) != 6:
        raise SystemExit("six distinct residents are required")
    runtime_root = args.runtime_root.resolve()
    if args.phase == "pre":
        if args.state.exists():
            raise SystemExit("pre state already exists; refusing replay")
        state = {"schema": "adl.issue268.six_resident_uts_state.v2", "plan_sha256": digest_file(args.plan), "phase": "pre_in_progress", "residents": {}}
    else:
        state = json.loads(args.state.read_text(encoding="utf-8"))
        if state.get("schema") != "adl.issue268.six_resident_uts_state.v2" or state.get("phase") != "pre_complete":
            raise SystemExit("post phase requires exact completed pre state")
        state["phase"] = "post_in_progress"
        atomic_json(args.state, state)
    args.evidence_dir.mkdir(parents=True, exist_ok=True)
    for resident in residents:
        agent_id = safe_id(resident["agent_id"])
        task_id = resident["pre_recovery_case" if args.phase == "pre" else "post_recovery_case"]
        retained = state["residents"].get(agent_id)
        if args.phase == "post" and (retained is None or retained["pending_case_ids"] != [task_id]):
            raise SystemExit(f"{agent_id}: only the exact pending case may resume")
        agent_dir = runtime_root / "agent-specs" / agent_id
        agent_dir.mkdir(parents=True, exist_ok=True)
        spec_path = agent_dir / "agent.json"
        binding = write_spec(spec_path, resident, runtime_root)
        write_workflow(agent_dir / "workflow.adl.yaml", resident, args.phase, task_id)
        run_agent(args.runtime_bin, spec_path)
        cycle_number = 1 if args.phase == "pre" else 2
        cycle_dir = runtime_root / "residents" / agent_id / "cycles" / f"cycle-{cycle_number:06d}"
        receipt_path = cycle_dir / "resident_tool_receipts.json"
        receipts = json.loads(receipt_path.read_text(encoding="utf-8"))
        if len(receipts) != 1 or receipts[0].get("schema") != RUNTIME_RECEIPT or receipts[0].get("decision") != "executed":
            raise SystemExit(f"{agent_id}: Runtime UTS/ACC proposal did not execute")
        receipt = receipts[0]
        if receipt.get("resident_id") != agent_id or receipt.get("authority_sha256") != binding["authority_sha256"]:
            raise SystemExit(f"{agent_id}: Runtime receipt authority mismatch")
        report_path = args.evidence_dir / f"{args.phase}-{agent_id}.json"
        report = {"schema": "adl.issue268.runtime_resident_cycle.v1", "agent_id": agent_id,
            "role": resident["role"], "task_id": task_id, "model": resident["model"],
            "runtime_receipt": receipt, "runtime_receipt_sha256": digest_file(receipt_path),
            "cycle_id": f"cycle-{cycle_number:06d}"}
        atomic_json(report_path, report)
        report_sha = digest_file(report_path)
        if args.phase == "pre":
            state["residents"][agent_id] = {"role": resident["role"], "model": resident["model"],
                "role_digest": canonical_digest({"agent_id": agent_id, "role": resident["role"]}),
                "tool_authority_digest": canonical_digest({"agent_id": agent_id, "tool_authority": resident["tool_authority"]}),
                "runtime_authority_sha256": binding["authority_sha256"], "sequence": 1,
                "runtime_agent_spec": str(spec_path),
                "completed_case_ids": [task_id], "pending_case_ids": [resident["post_recovery_case"]],
                "uts_report_sha256": report_sha,
                "continuation_request_sha256": hashlib.sha256(resident["post_recovery_case"].encode()).hexdigest(),
                "checkpoint_lineage": [receipt["checkpoint_lineage"]]}
        else:
            retained["sequence"] = 2
            retained["completed_case_ids"].append(task_id)
            retained["pending_case_ids"] = []
            retained["post_restore_uts_report_sha256"] = report_sha
            retained["checkpoint_lineage"].append(receipt["checkpoint_lineage"])
        atomic_json(args.state, state)
    state["phase"] = "pre_complete" if args.phase == "pre" else "post_complete"
    state["resident_count"] = 6
    state["all_pending_empty"] = all(not value["pending_case_ids"] for value in state["residents"].values())
    atomic_json(args.state, state)
    print(json.dumps({"status": "pass", "phase": state["phase"], "resident_count": 6}, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
