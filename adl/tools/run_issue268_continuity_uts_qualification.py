#!/usr/bin/env python3
"""Couple #268's six-resident UTS work to the #414 continuity boundary."""

from __future__ import annotations

import argparse
import hashlib
import json
import pathlib
import subprocess
import sys
from typing import Any


ROOT = pathlib.Path(__file__).resolve().parents[2]
DEFAULT_PLAN = ROOT / "adl/tools/issue268_six_resident_uts_plan.json"
DEFAULT_UTS_RUNNER = ROOT / "adl/tools/run_issue268_six_resident_uts_cycle.py"


def sha256_bytes(value: bytes) -> str:
    return hashlib.sha256(value).hexdigest()


def sha256_file(path: pathlib.Path) -> str:
    return sha256_bytes(path.read_bytes())


def read_json(path: pathlib.Path) -> Any:
    return json.loads(path.read_text(encoding="utf-8"))


def write_json(path: pathlib.Path, value: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def canonical_digest(value: Any) -> str:
    return sha256_bytes(json.dumps(value, separators=(",", ":"), sort_keys=True).encode("utf-8"))


def run(command: list[str], *, cwd: pathlib.Path = ROOT) -> None:
    completed = subprocess.run(command, cwd=cwd, check=False)
    if completed.returncode != 0:
        raise SystemExit(f"command failed with {completed.returncode}: {' '.join(command)}")


def continuity(
    binary: pathlib.Path,
    command: str,
    input_path: pathlib.Path,
    runtime_root: pathlib.Path,
    output_path: pathlib.Path,
) -> dict[str, Any]:
    run(
        [
            str(binary),
            command,
            "--input",
            str(input_path),
            "--runtime-root",
            str(runtime_root),
            "--output",
            str(output_path),
        ]
    )
    return read_json(output_path)


def binding_for(resident: dict[str, Any], retained: dict[str, Any]) -> dict[str, str]:
    model = resident["model"]
    return {
        "agent_id": resident["agent_id"],
        "model": model,
        # The immutable model artifact and execution configuration are bound
        # separately. The qualification bootstrap must replace model_ref_sha256
        # with the exact Ollama artifact digest when it materializes the plan.
        "artifact_sha256": resident["model_ref_sha256"],
        "quantization": resident["quantization"],
        "configuration_sha256": resident["configuration_sha256"],
        # #414 signs these two exact fields inside the resident population.
        # Bind the complete #268 role/tool/task/lineage checkpoint rather than
        # only the report file, so distinct resident authority survives restore.
        "completed_task_sha256": canonical_digest(
            {
                "agent_id": resident["agent_id"],
                "role_digest": retained["role_digest"],
                "tool_authority_digest": retained["tool_authority_digest"],
                "sequence": retained["sequence"],
                "completed_case_ids": retained["completed_case_ids"],
                "uts_report_sha256": retained["uts_report_sha256"],
                "checkpoint_lineage": retained["checkpoint_lineage"],
            }
        ),
        "continuation_request_sha256": canonical_digest(
            {
                "agent_id": resident["agent_id"],
                "pending_case_ids": retained["pending_case_ids"],
                "request_sha256": retained["continuation_request_sha256"],
            }
        ),
    }


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--continuity-bin", required=True, type=pathlib.Path)
    parser.add_argument("--runtime-bin", required=True, type=pathlib.Path)
    parser.add_argument("--runtime-root", required=True, type=pathlib.Path)
    parser.add_argument("--build-cache-root", required=True, type=pathlib.Path)
    parser.add_argument("--agent-spec-dir", required=True, type=pathlib.Path)
    parser.add_argument("--runtime-volume-identity-sha256", required=True)
    parser.add_argument("--state", required=True, type=pathlib.Path)
    parser.add_argument("--evidence-dir", required=True, type=pathlib.Path)
    parser.add_argument("--plan", type=pathlib.Path, default=DEFAULT_PLAN)
    parser.add_argument("--uts-runner", type=pathlib.Path, default=DEFAULT_UTS_RUNNER)
    args = parser.parse_args()

    if not args.continuity_bin.is_file():
        raise SystemExit("#414 continuity binary is absent")
    if not args.runtime_bin.is_file():
        raise SystemExit("Runtime agent binary is absent")
    if len(args.runtime_volume_identity_sha256) != 64:
        raise SystemExit("retained Runtime volume identity must be an exact SHA-256")

    plan = read_json(args.plan)
    materialization = plan.get("materialization") or {}
    if materialization.get("schema") != "adl.issue268.ollama_plan_materialization.v1":
        raise SystemExit("execution requires an exact materialized Ollama plan")
    template = DEFAULT_PLAN
    if materialization.get("template_sha256") != sha256_file(template):
        raise SystemExit("materialized plan does not bind the reviewed template")
    residents = plan.get("residents") or []
    if len(residents) != 6:
        raise SystemExit("six-resident plan is required")
    required_binding_fields = {"model_ref_sha256", "quantization", "configuration_sha256"}
    for resident in residents:
        missing = required_binding_fields - resident.keys()
        if missing:
            raise SystemExit(f"{resident.get('agent_id')}: missing continuity binding fields {sorted(missing)}")
        for field in ("model_ref_sha256", "configuration_sha256"):
            value = resident[field]
            if len(value) != 64 or any(character not in "0123456789abcdef" for character in value):
                raise SystemExit(f"{resident.get('agent_id')}: {field} is not a materialized SHA-256")

    specs = [args.agent_spec_dir / resident["agent_id"] / "agent.yaml" for resident in residents]
    missing_specs = [str(path) for path in specs if not path.is_file()]
    if missing_specs:
        raise SystemExit(f"six existing-agent specs are required: {missing_specs}")
    if len(residents) != len(specs):
        raise SystemExit("resident and durable agent-spec populations differ")
    for resident, spec_path in zip(residents, specs):
        spec = read_json(spec_path)
        expected = {
            "schema": "adl.issue268.resident_agent_spec.v1",
            "agent_id": resident["agent_id"],
            "role": resident["role"],
            "role_digest": canonical_digest({"agent_id": resident["agent_id"], "role": resident["role"]}),
            "tool_authority": resident["tool_authority"],
            "tool_authority_digest": canonical_digest({"agent_id": resident["agent_id"], "tool_authority": resident["tool_authority"]}),
            "model": resident["model"],
            "model_ref_sha256": resident["model_ref_sha256"],
            "configuration_sha256": resident["configuration_sha256"],
        }
        if spec != expected:
            raise SystemExit(f"{resident['agent_id']}: durable agent spec does not bind the reviewed resident identity")

    args.evidence_dir.mkdir(parents=True, exist_ok=True)
    uts_evidence = args.evidence_dir / "uts"
    uts_command = [
        sys.executable,
        str(args.uts_runner),
        "--state",
        str(args.state),
        "--evidence-dir",
        str(uts_evidence),
        "--plan",
        str(args.plan),
        "--runtime-bin",
        str(args.runtime_bin),
        "--runtime-root",
        str(args.runtime_root),
    ]
    run(uts_command + ["--phase", "pre"])

    state = read_json(args.state)
    specs = [pathlib.Path(state["residents"][resident["agent_id"]]["runtime_agent_spec"]) for resident in residents]
    if any(not path.is_file() for path in specs):
        raise SystemExit("six retained Runtime agent specs are required after pre-cycle execution")
    for resident in residents:
        retained = state["residents"][resident["agent_id"]]
        if retained.get("role") != resident["role"] or retained.get("role_digest") != canonical_digest({"agent_id": resident["agent_id"], "role": resident["role"]}):
            raise SystemExit(f"{resident['agent_id']}: pre-cycle role binding drifted")
        if retained.get("tool_authority_digest") != canonical_digest({"agent_id": resident["agent_id"], "tool_authority": resident["tool_authority"]}):
            raise SystemExit(f"{resident['agent_id']}: pre-cycle tool authority binding drifted")
    bindings = [binding_for(resident, state["residents"][resident["agent_id"]]) for resident in residents]
    dehydration_input = {
        "residents": bindings,
        "existing_agent_specs": [str(path) for path in specs],
        "retained_runtime_root": str(args.runtime_root),
        "build_cache_root": str(args.build_cache_root),
        "runtime_volume_identity_sha256": args.runtime_volume_identity_sha256,
        "source_host": "issue268-r7i-qualification",
        "target_host": "issue268-r7i-qualification",
        "spot_notice": None,
    }
    dehydration_input_path = args.evidence_dir / "dehydration-input.json"
    write_json(dehydration_input_path, dehydration_input)
    preflight = continuity(args.continuity_bin, "preflight", dehydration_input_path, args.runtime_root, args.evidence_dir / "preflight.json")
    if preflight.get("status") != "passed" or preflight.get("resident_count") != 6:
        raise SystemExit("#414 preflight did not admit the exact six-resident population")
    dehydrated = continuity(args.continuity_bin, "dehydrate", dehydration_input_path, args.runtime_root, args.evidence_dir / "dehydration.json")
    if dehydrated.get("resident_count") != 6 or dehydrated.get("admission_open") is not False:
        raise SystemExit("#414 dehydration did not close admission for all six residents")
    if not isinstance(dehydrated.get("generation"), int) or dehydrated["generation"] < 1:
        raise SystemExit("#414 dehydration lacks a signed continuity generation")
    population_sha256 = dehydrated.get("population_sha256")
    if not isinstance(population_sha256, str) or len(population_sha256) != 64:
        raise SystemExit("#414 dehydration lacks an exact signed population digest")

    restore_input = {
        "residents": bindings,
        "retained_runtime_root": str(args.runtime_root),
        "build_cache_root": str(args.build_cache_root),
        "runtime_volume_identity_sha256": args.runtime_volume_identity_sha256,
    }
    restore_input_path = args.evidence_dir / "restore-input.json"
    write_json(restore_input_path, restore_input)
    restored = continuity(args.continuity_bin, "restore", restore_input_path, args.runtime_root, args.evidence_dir / "restore.json")
    if restored.get("resident_count") != 6 or restored.get("admission_open") is not True:
        raise SystemExit("#414 restore did not reopen admission for all six residents")
    if restored.get("generation") != dehydrated["generation"] or restored.get("population_sha256") != population_sha256:
        raise SystemExit("#414 restore does not match the signed dehydration generation")

    run(uts_command + ["--phase", "post"])
    state = read_json(args.state)
    if state.get("phase") != "post_complete" or state.get("all_pending_empty") is not True:
        raise SystemExit("post-restore UTS continuation is incomplete")
    continuations = []
    resident_receipts = []
    if len(residents) != len(bindings):
        raise SystemExit("resident and continuity-binding populations differ")
    for resident, binding in zip(residents, bindings):
        retained = state["residents"][resident["agent_id"]]
        continuations.append(
            {
                **binding,
                "next_task_sha256": retained["post_restore_uts_report_sha256"],
            }
        )
        resident_receipts.append(
            {
                "agent_id": resident["agent_id"],
                "role_digest": retained["role_digest"],
                "tool_authority_digest": retained["tool_authority_digest"],
                "sequence": retained["sequence"],
                "completed_case_ids": retained["completed_case_ids"],
                "pending_case_ids": retained["pending_case_ids"],
                "pre_uts_report_sha256": retained["uts_report_sha256"],
                "post_uts_report_sha256": retained["post_restore_uts_report_sha256"],
                "checkpoint_lineage": retained["checkpoint_lineage"],
                "replay_denied": resident["pre_recovery_case"] != resident["post_recovery_case"],
            }
        )
    continuation_path = args.evidence_dir / "continuation-input.json"
    write_json(continuation_path, {"residents": continuations})
    completed = continuity(args.continuity_bin, "complete", continuation_path, args.runtime_root, args.evidence_dir / "continuation.json")
    if completed.get("resident_count") != 6 or completed.get("continuation_verified") is not True:
        raise SystemExit("#414 did not verify all six completed continuations")
    if completed.get("generation") != dehydrated["generation"] or completed.get("population_sha256") != population_sha256:
        raise SystemExit("#414 completion does not bind the signed dehydration population")

    receipt = {
        "schema": "adl.issue268.continuity_uts_qualification.v1",
        "status": "passed",
        "resident_count": 6,
        "plan_sha256": sha256_file(args.plan),
        "completed_uts_state_sha256": sha256_file(args.state),
        "continuity_generation": completed.get("generation"),
        "signed_population_sha256": population_sha256,
        "dehydration_receipt_sha256": sha256_file(args.evidence_dir / "dehydration.json"),
        "restore_receipt_sha256": sha256_file(args.evidence_dir / "restore.json"),
        "completion_receipt_sha256": sha256_file(args.evidence_dir / "continuation.json"),
        "continuation_verified": True,
        "replay_denied": all(item["replay_denied"] for item in resident_receipts),
        "residents": resident_receipts,
    }
    write_json(args.evidence_dir / "qualification-receipt.json", receipt)
    print(json.dumps(receipt, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
