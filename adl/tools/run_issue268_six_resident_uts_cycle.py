#!/usr/bin/env python3
"""Run one real Runtime/UTS/ACC cycle for each of the six #268 residents."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import pathlib
import subprocess
import time
from typing import Any


ROOT = pathlib.Path(__file__).resolve().parents[2]
DEFAULT_PLAN = ROOT / "adl/tools/issue268_six_resident_uts_plan.json"
DEFAULT_TASK_PANEL = ROOT / "adl/tools/issue268_runtime_uts_task_panel.json"
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


def write_workflow(path: pathlib.Path, resident: dict[str, Any], phase: str, task: dict[str, Any]) -> None:
    task_id = task["id"]
    expected = json.dumps(proposal(resident["agent_id"], phase, task_id), separators=(",", ":"))
    document = f'''version: "0.5"
providers:
  local_ollama:
    type: "ollama"
    base_url: "http://127.0.0.1:11434"
    config:
      model: "{resident["model"]}"
      timeout_secs: 900
agents:
  resident:
    provider: "local_ollama"
    model: "{resident["model"]}"
tasks:
  governed_runtime_observation:
    prompt:
      system: 'Return exactly the supplied JSON object. Do not add reasoning, commentary, markdown, or a code fence.'
      user: 'Objective: {task["objective"]} Return this exact proposal: {expected}'
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
        # A relative state root is essential: #414 restores the capsule under
        # the generation directory and the restored spec must use that state,
        # not reach back into the pre-dehydration location.
        "state_root": "state",
        "workflow": {"kind": "adl_workflow", "name": f"issue268-{resident['agent_id']}", "path": "workflow.adl.yaml", "run_args": {
            "freedom_gate_policy_decision": "allowed", "tool_registry": runtime_observe_registry(),
            "tool_policy_context": policy, "tool_risk_class": "low",
            "citizen_boundary_ref": "runtime.resident.boundary",
            "tool_gate_context": {"policy_decision": "allowed", "requires_operator_review": False,
                "requires_human_challenge": False, "escalation_available": False,
                "citizen_action_boundary_intact": True, "operator_action_boundary_intact": True,
                "private_arguments_redacted": True}}},
        "heartbeat": {"interval_secs": 1, "max_cycles": 3, "stale_lease_after_secs": 900},
        "checkpoint": {"interval_secs": 1, "allow_agent_requested": True, "min_request_interval_secs": 1},
        "safety": {"allow_network": False, "allow_broker": False,
            "allow_filesystem_writes_outside_state_root": False, "allow_real_world_side_effects": False,
            "require_public_artifact_sanitization": True, "financial_advice": False,
            "max_cycle_runtime_secs": 900, "max_consecutive_failures": 1},
        "memory": {}, "resident_role": resident["role"], "tool_authority": binding,
    }
    atomic_json(path, spec)
    return binding


def run_agent(runtime_bin: pathlib.Path, spec: pathlib.Path) -> int:
    completed = subprocess.run([str(runtime_bin), "agent", "tick", "--spec", str(spec), "--json"], cwd=ROOT, check=False)
    return completed.returncode


def run_daemon(csm_bin: pathlib.Path, spec: pathlib.Path, agent_dir: pathlib.Path) -> int:
    process = subprocess.Popen([
        str(csm_bin), "daemon", "--spec", str(spec),
        "--checkpoint-interval-secs", "1", "--interval-secs", "1",
        "--test-supervisor-failure-after-restarts", "1",
        "--api-bind", "127.0.0.1:0", "--no-sleep", "--json",
    ], cwd=ROOT)
    deadline = time.monotonic() + 90
    cycles_root = agent_dir / "state" / "cycles"
    try:
        while time.monotonic() < deadline:
            if process.poll() is not None:
                return process.returncode
            cycle_dirs = sorted(cycles_root.glob("cycle-*")) if cycles_root.exists() else []
            if any((cycle_dir / "resident_tool_receipts.json").is_file() for cycle_dir in cycle_dirs):
                process.terminate()
                try:
                    process.wait(timeout=5)
                except subprocess.TimeoutExpired:
                    process.kill()
                    process.wait(timeout=5)
                return 0
            time.sleep(1)
    finally:
        if process.poll() is None:
            process.terminate()
            try:
                process.wait(timeout=5)
            except subprocess.TimeoutExpired:
                process.kill()
                process.wait(timeout=5)
    return process.returncode if process.returncode is not None else 1


def normalize_and_validate_runtime_spec(runtime_bin: pathlib.Path, spec: pathlib.Path, agent_dir: pathlib.Path) -> None:
    """Use the Runtime-owned locked spec as #414's exact existing-agent input."""
    locked_path = agent_dir / "state" / "agent_spec.locked.json"
    if not locked_path.is_file():
        raise SystemExit(f"{agent_dir.name}: Runtime locked agent spec is absent")
    locked = json.loads(locked_path.read_text(encoding="utf-8"))
    if locked.get("agent_instance_id") != agent_dir.name:
        raise SystemExit(f"{agent_dir.name}: Runtime locked agent identity mismatch")
    atomic_json(spec, locked)
    completed = subprocess.run(
        [str(runtime_bin), "agent", "status", "--spec", str(spec), "--json"],
        cwd=ROOT,
        check=False,
    )
    if completed.returncode != 0:
        raise SystemExit(f"{agent_dir.name}: canonical Runtime spec failed locked-spec validation")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--phase", required=True, choices=("pre", "replay", "post"))
    parser.add_argument("--state", required=True, type=pathlib.Path)
    parser.add_argument("--evidence-dir", required=True, type=pathlib.Path)
    parser.add_argument("--plan", type=pathlib.Path, default=DEFAULT_PLAN)
    parser.add_argument("--runtime-bin", required=True, type=pathlib.Path)
    parser.add_argument("--runtime-root", required=True, type=pathlib.Path)
    parser.add_argument("--task-panel", type=pathlib.Path, default=DEFAULT_TASK_PANEL)
    parser.add_argument("--restore-receipt", type=pathlib.Path)
    parser.add_argument("--restored-population-root", type=pathlib.Path)
    args = parser.parse_args()
    if not args.runtime_bin.is_file():
        raise SystemExit("real Runtime binary is required")
    csm_bin = args.runtime_bin.parent / "csm"
    if not csm_bin.is_file():
        raise SystemExit("real CSM daemon binary is required")
    plan = json.loads(args.plan.read_text(encoding="utf-8"))
    panel = json.loads(args.task_panel.read_text(encoding="utf-8"))
    if panel.get("schema") != "adl.issue268.runtime_uts_task_panel.v1" or panel.get("tool") != "runtime.observe":
        raise SystemExit("exact issue268 Runtime UTS task panel is required")
    tasks = panel.get("tasks") or []
    task_by_id = {task.get("id"): task for task in tasks}
    if len(tasks) != 12 or len(task_by_id) != 12:
        raise SystemExit("exact twelve-task Runtime UTS panel is required")
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
            raise SystemExit("recovery phase requires exact completed pre state")
        if not args.restore_receipt or not args.restored_population_root:
            raise SystemExit("recovery phase requires the exact #414 restore receipt and population")
        restore = json.loads(args.restore_receipt.read_text(encoding="utf-8"))
        if restore.get("schema") != "adl.runtime.resident_shepherd_restore_receipt.v1" or restore.get("admission_open") is not True:
            raise SystemExit("recovery phase requires an admission-open #414 restore receipt")
        restored_root = args.restored_population_root.resolve()
        if restored_root.name != f'generation-{restore.get("generation")}' or not restored_root.is_dir():
            raise SystemExit("restored population does not match the #414 generation")
        state["restore_receipt_sha256"] = digest_file(args.restore_receipt)
        state["restored_population_root"] = str(restored_root)
        if args.phase == "replay":
            for resident in residents:
                agent_id = safe_id(resident["agent_id"])
                attempted = resident["pre_recovery_case"]
                retained = state["residents"].get(agent_id) or {}
                if attempted not in retained.get("completed_case_ids", []):
                    raise SystemExit(f"{agent_id}: replay probe does not target a completed case")
                task = task_by_id.get(attempted)
                agent_dir = restored_root / agent_id
                spec_path = agent_dir / "agent.yaml"
                locked = json.loads((agent_dir / "state" / "agent_spec.locked.json").read_text(encoding="utf-8"))
                if locked.get("agent_instance_id") != agent_id:
                    raise SystemExit(f"{agent_id}: restored replay identity mismatch")
                write_workflow(agent_dir / "workflow.adl.yaml", resident, "pre", task)
                runtime_exit_code = run_agent(args.runtime_bin, spec_path)
                cycles = sorted((agent_dir / "state" / "cycles").glob("cycle-*"))
                if not cycles:
                    raise SystemExit(f"{agent_id}: replay attempt produced no Runtime cycle")
                runtime_receipts_path = cycles[-1] / "resident_tool_receipts.json"
                runtime_receipts = json.loads(runtime_receipts_path.read_text(encoding="utf-8"))
                if (runtime_exit_code == 0 or len(runtime_receipts) != 1
                        or runtime_receipts[0].get("decision") != "denied"
                        or runtime_receipts[0].get("reason_code") != "proposal_replay_denied"):
                    raise SystemExit(f"{agent_id}: Runtime did not deny the completed proposal replay")
                atomic_json(args.evidence_dir / f"replay-{agent_id}.json", {
                    "schema": "adl.issue268.runtime_uts_replay_denial.v1",
                    "agent_id": agent_id,
                    "attempted_case_id": attempted,
                    "decision": runtime_receipts[0]["decision"],
                    "reason_code": runtime_receipts[0]["reason_code"],
                    "runtime_receipt_sha256": digest_file(runtime_receipts_path),
                    "runtime_exit_code": runtime_exit_code,
                    "restore_receipt_sha256": state["restore_receipt_sha256"],
                })
                retained["replay_denial_receipt_sha256"] = digest_file(args.evidence_dir / f"replay-{agent_id}.json")
            atomic_json(args.state, state)
            print(json.dumps({"status": "pass", "phase": "replay_denied", "resident_count": 6}, sort_keys=True))
            return 0
        state["phase"] = "post_in_progress"
        atomic_json(args.state, state)
    args.evidence_dir.mkdir(parents=True, exist_ok=True)
    for resident in residents:
        agent_id = safe_id(resident["agent_id"])
        task_id = resident["pre_recovery_case" if args.phase == "pre" else "post_recovery_case"]
        task = task_by_id.get(task_id)
        if not task or task.get("resident") != agent_id or task.get("phase") != args.phase:
            raise SystemExit(f"{agent_id}: task is absent from the exact Runtime UTS panel")
        retained = state["residents"].get(agent_id)
        if args.phase == "post" and (retained is None or retained["pending_case_ids"] != [task_id]):
            raise SystemExit(f"{agent_id}: only the exact pending case may resume")
        if args.phase == "pre":
            agent_dir = runtime_root / "agent-specs" / agent_id
            agent_dir.mkdir(parents=True, exist_ok=True)
            spec_path = agent_dir / "agent.json"
            binding = write_spec(spec_path, resident, runtime_root)
        else:
            agent_dir = args.restored_population_root.resolve() / agent_id
            spec_path = agent_dir / "agent.yaml"
            if not spec_path.is_file():
                raise SystemExit(f"{agent_id}: restored #414 agent spec is absent")
            restored_spec = json.loads((agent_dir / "state" / "agent_spec.locked.json").read_text(encoding="utf-8"))
            if restored_spec.get("agent_instance_id") != agent_id:
                raise SystemExit(f"{agent_id}: restored #414 agent identity mismatch")
            binding = restored_spec.get("tool_authority") or {}
            if binding.get("authority_sha256") != retained.get("runtime_authority_sha256"):
                raise SystemExit(f"{agent_id}: restored #414 tool authority mismatch")
        write_workflow(agent_dir / "workflow.adl.yaml", resident, args.phase, task)
        runtime_exit_code = run_daemon(csm_bin, spec_path, agent_dir) if args.phase == "pre" else run_agent(args.runtime_bin, spec_path)
        if args.phase == "pre":
            normalize_and_validate_runtime_spec(args.runtime_bin, spec_path, agent_dir)
        cycle_dirs = sorted((agent_dir / "state" / "cycles").glob("cycle-*"))
        if not cycle_dirs:
            raise SystemExit(f"{agent_id}: Runtime agent tick produced no cycle")
        cycle_dir = cycle_dirs[-1]
        receipt_path = cycle_dir / "resident_tool_receipts.json"
        receipts = json.loads(receipt_path.read_text(encoding="utf-8"))
        if len(receipts) != 1 or receipts[0].get("schema") != RUNTIME_RECEIPT or receipts[0].get("decision") not in {"executed", "denied"}:
            raise SystemExit(f"{agent_id}: Runtime UTS/ACC terminal receipt is absent")
        receipt = receipts[0]
        if (receipt["decision"] == "executed") != (runtime_exit_code == 0):
            raise SystemExit(f"{agent_id}: Runtime exit status contradicts its ACC receipt")
        if receipt.get("resident_id") != agent_id or receipt.get("authority_sha256") != binding["authority_sha256"]:
            raise SystemExit(f"{agent_id}: Runtime receipt authority mismatch")
        report_path = args.evidence_dir / f"{args.phase}-{agent_id}.json"
        report = {"schema": "adl.issue268.runtime_resident_cycle.v1", "agent_id": agent_id,
            "role": resident["role"], "task_id": task_id, "model": resident["model"],
            "task_definition_sha256": canonical_digest(task),
            "runtime_receipt": receipt, "runtime_receipt_sha256": digest_file(receipt_path),
            "agent_test_outcome": receipt["decision"], "runtime_exit_code": runtime_exit_code,
            "cycle_id": receipt["cycle_id"]}
        atomic_json(report_path, report)
        report_sha = digest_file(report_path)
        if args.phase == "pre":
            state["residents"][agent_id] = {"role": resident["role"], "model": resident["model"],
                "role_digest": canonical_digest({"agent_id": agent_id, "role": resident["role"]}),
                "tool_authority_digest": canonical_digest({"agent_id": agent_id, "tool_authority": resident["tool_authority"]}),
                "runtime_authority_sha256": binding["authority_sha256"], "sequence": 1,
                "runtime_agent_spec": str(spec_path),
                "task_panel_sha256": digest_file(args.task_panel),
                "pre_task_definition_sha256": canonical_digest(task),
                "pre_agent_test_outcome": receipt["decision"],
                "completed_case_ids": [task_id], "pending_case_ids": [resident["post_recovery_case"]],
                "uts_report_sha256": report_sha,
                "continuation_request_sha256": hashlib.sha256(resident["post_recovery_case"].encode()).hexdigest(),
                "checkpoint_lineage": [receipt["checkpoint_lineage"]]}
        else:
            retained["sequence"] = 2
            retained["restored_runtime_agent_spec"] = str(spec_path)
            retained["restored_runtime_agent_spec_sha256"] = digest_file(spec_path)
            retained["completed_case_ids"].append(task_id)
            retained["pending_case_ids"] = []
            retained["post_restore_uts_report_sha256"] = report_sha
            retained["post_task_definition_sha256"] = canonical_digest(task)
            retained["post_agent_test_outcome"] = receipt["decision"]
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
