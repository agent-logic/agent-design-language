#!/usr/bin/env python3
"""Fail-closed validation for the #268 six-resident UTS qualification plan."""

from __future__ import annotations

import json
import pathlib
import sys


ROOT = pathlib.Path(__file__).resolve().parents[2]
PLAN = ROOT / "adl/tools/issue268_six_resident_uts_plan.json"
TASK_PANEL = ROOT / "adl/tools/benchmark/uts_33_task_panel.json"


def fail(message: str) -> None:
    raise SystemExit(f"issue268 UTS plan invalid: {message}")


def main() -> None:
    plan = json.loads(PLAN.read_text(encoding="utf-8"))
    task_panel = json.loads(TASK_PANEL.read_text(encoding="utf-8"))
    if plan.get("schema") != "adl.issue268.six_resident_uts_plan.v1":
        fail("schema")

    host = plan.get("host") or {}
    expected_host = {
        "instance_type": "r7i.2xlarge",
        "vcpus": 8,
        "memory_mib": 65536,
        "gpu_allowed": False,
        "max_concurrent_inference": 1,
        "max_loaded_models": 3,
        "compilation_concurrent": False,
    }
    if host != expected_host:
        fail("exact r7i.2xlarge resource envelope")

    provider = plan.get("provider") or {}
    expected_provider = {
        "kind": "ollama",
        "runtime_surface": "ollama_http",
        "local_required": True,
        "cloud_escalation_optional": True,
        "cloud_escalation_authoritative": False,
        "credentials_checkpointed": False,
    }
    if provider != expected_provider:
        fail("existing Ollama provider boundary")

    uts = plan.get("uts") or {}
    if uts.get("schema_version") != "uts.v1.1":
        fail("UTS schema version")
    if uts.get("runner") != "adl/tools/uts_benchmark_runner.py":
        fail("repo UTS runner")
    if uts.get("task_panel") != "adl/tools/benchmark/uts_33_task_panel.json":
        fail("repo UTS task panel")
    if uts.get("include_governed") is not True:
        fail("governed UTS+ACC lane")
    if "never replayed" not in str(uts.get("resume_rule")):
        fail("completed-case replay denial")

    residents = plan.get("residents") or []
    expected_roles = {
        "shepherd_controller",
        "planner",
        "tool_executor",
        "runtime_observer",
        "recovery_custodian",
        "reviewer_escalation",
    }
    expected_models = {
        "llama3.1:8b": 2,
        "qwen3:8b": 2,
        "phi4-mini:latest": 2,
    }
    if len(residents) != 6:
        fail("exactly six residents")
    ids = [row.get("agent_id") for row in residents]
    roles = [row.get("role") for row in residents]
    if len(set(ids)) != 6 or any(not isinstance(value, str) or not value for value in ids):
        fail("six unique stable resident identities")
    if set(roles) != expected_roles or len(set(roles)) != 6:
        fail("exact role allocation")
    authorities = [row.get("tool_authority") for row in residents]
    if any(not isinstance(value, list) or not value for value in authorities):
        fail("every resident requires explicit tool authority")
    if len({json.dumps(value, sort_keys=True) for value in authorities}) != 6:
        fail("six distinct resident tool-authority sets")
    observed_models: dict[str, int] = {}
    pre_cases: list[str] = []
    post_cases: list[str] = []
    task_ids = {row.get("id") for row in task_panel.get("tasks", [])}
    for resident in residents:
        model = resident.get("model")
        observed_models[model] = observed_models.get(model, 0) + 1
        before = resident.get("pre_recovery_case")
        after = resident.get("post_recovery_case")
        if before not in task_ids or after not in task_ids or before == after:
            fail(f"real distinct pre/post UTS cases for {resident.get('agent_id')}")
        pre_cases.append(before)
        post_cases.append(after)
        if resident.get("quantization") != "Q4_K_M":
            fail(f"reviewed CPU quantization for {resident.get('agent_id')}")
        if resident.get("model_ref_sha256") != "REPLACE_WITH_OLLAMA_ARTIFACT_SHA256":
            fail(f"model digest bootstrap slot for {resident.get('agent_id')}")
        if resident.get("configuration_sha256") != "REPLACE_WITH_EXECUTION_CONFIGURATION_SHA256":
            fail(f"configuration digest bootstrap slot for {resident.get('agent_id')}")
    if observed_models != expected_models:
        fail("two agents per pinned model family")
    if len(set(pre_cases)) != 6 or len(set(post_cases)) != 6:
        fail("six distinct named cases in each UTS phase")

    expected_checkpoint_fields = {
        "agent_id",
        "role_digest",
        "tool_authority_digest",
        "model_artifact_digest",
        "model_configuration_digest",
        "sequence",
        "completed_case_ids",
        "pending_case_ids",
        "uts_report_digest",
        "continuation_request_digest",
        "checkpoint_lineage",
    }
    fields = plan.get("checkpoint_fields") or []
    if len(fields) != len(set(fields)) or set(fields) != expected_checkpoint_fields:
        fail("exact checkpoint field set")

    print(
        json.dumps(
            {
                "schema": "adl.issue268.six_resident_uts_plan.validation.v1",
                "status": "pass",
                "resident_count": 6,
                "distinct_role_count": 6,
                "distinct_model_count": 3,
                "pre_recovery_case_count": 6,
                "post_recovery_case_count": 6,
                "instance_type": "r7i.2xlarge",
            },
            sort_keys=True,
        )
    )


if __name__ == "__main__":
    main()
