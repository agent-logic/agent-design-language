#!/usr/bin/env python3
"""Build and optionally run an ADL validation profile for a change set."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import platform
import shlex
import subprocess
import sys
import time
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
SELECTOR = ROOT / "adl/tools/select_validation_lanes.sh"
SLOW_PROOF_FAMILIES = ROOT / "adl/config/slow_proof_families.v0.91.6.json"
DEFAULT_MANIFEST = ROOT / "adl/config/validation_lane_selector.v0.91.6.json"
NESSUS_REMOTE_RUNNER = "bash adl/tools/run_nessus_remote_validation.sh"
AWS_SPOT_REMOTE_RUNNER = "bash adl/tools/run_aws_spot_remote_validation_lane.sh"
AWS_CODEFRIEND_BUILD_RUNNER = "bash adl/tools/run_aws_codefriend_build_lane.sh"
AWS_CODEFRIEND_BUILD_RUNNER_PATH = ROOT / "adl/tools/run_aws_codefriend_build_lane.sh"
VALIDATION_PLATFORMS = ("auto", "local", "nessus", "aws_spot", "codebuild", "wuji")
BUILD_ACTION_LOG_SCHEMA = "adl.build_action_log.v1"
BUILD_ACTION_LOG_MANIFEST_SCHEMA = "adl.build_action_log_manifest.v1"


def fail(message: str) -> None:
    print(f"validation_manager: {message}", file=sys.stderr)
    raise SystemExit(2)


def load_slow_proof_families() -> dict[str, Any]:
    payload = json.loads(SLOW_PROOF_FAMILIES.read_text())
    if payload.get("schema_version") != "adl.slow_proof_families.v1":
        fail("slow-proof families config returned unsupported schema_version")
    if not isinstance(payload.get("families"), list):
        fail("slow-proof families config must expose a families array")
    return payload


def selector_plan(args: argparse.Namespace) -> dict[str, Any]:
    cmd = ["bash", str(SELECTOR), "--json"]
    if args.manifest:
        cmd.extend(["--manifest", str(args.manifest.resolve())])
    if args.changed_files:
        cmd.extend(["--changed-files", str(args.changed_files.resolve())])
    else:
        cmd.extend(["--base", args.base, "--head", args.head])
    if args.include_working_tree:
        cmd.append("--include-working-tree")
    result = subprocess.run(
        cmd,
        cwd=ROOT,
        check=False,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )
    if result.returncode != 0:
        fail(f"selector failed: {result.stderr.strip()}")
    try:
        plan = json.loads(result.stdout)
    except json.JSONDecodeError as exc:
        fail(f"selector returned invalid JSON: {exc}")
    if plan.get("schema_version") != "adl.validation_lane_plan.v1":
        fail("selector returned unsupported schema_version")
    return plan


def load_manifest(path: Path) -> dict[str, Any]:
    try:
        manifest = json.loads(path.read_text())
    except FileNotFoundError as exc:
        fail(f"validation manifest not found: {exc.filename}")
    except json.JSONDecodeError as exc:
        fail(f"validation manifest is not valid JSON: {exc}")
    if manifest.get("schema_version") != "adl.validation_lane_selector.v1":
        fail("validation manifest returned unsupported schema_version")
    return manifest


def guardrail_int(value: Any, field: str, default: int) -> int:
    if value is None:
        return default
    try:
        return int(value)
    except (TypeError, ValueError):
        fail(f"manager guardrail {field} must be an integer")
    raise AssertionError("unreachable")


def manager_guardrails(manifest: dict[str, Any], max_selected_lanes: int) -> dict[str, Any]:
    configured = manifest.get("manager_guardrails", {})
    pr_fast = configured.get("pr_fast", {})
    configured_max_selected_lanes = guardrail_int(
        configured.get("max_selected_lanes"), "max_selected_lanes", max_selected_lanes
    )
    return {
        "max_selected_lanes": configured_max_selected_lanes,
        "docs_only_forbidden_lane_ids": list(
            configured.get("docs_only_forbidden_lane_ids", ["rust_pr_fast"])
        ),
        "pr_fast": {
            "max_rust_surface_count": guardrail_int(
                pr_fast.get("max_rust_surface_count"), "pr_fast.max_rust_surface_count", 4
            ),
            "max_filter_token_count": guardrail_int(
                pr_fast.get("max_filter_token_count"), "pr_fast.max_filter_token_count", 4
            ),
            "max_family_token_count": guardrail_int(
                pr_fast.get("max_family_token_count"), "pr_fast.max_family_token_count", 3
            ),
            "blocked_modes": list(pr_fast.get("blocked_modes", ["full", "contract_only"])),
        },
    }


def profile_id(plan: dict[str, Any]) -> str:
    aggregate = plan.get("aggregate_status", "unknown")
    lanes = sorted(plan.get("lanes", {}).keys())
    if not lanes:
        return "validation_none"
    if aggregate == "selected" and len(lanes) == 1:
        return f"{lanes[0]}_profile"
    return f"{aggregate}_{len(lanes)}_lane_profile"


def lane_requirement_ids(lane: dict[str, Any]) -> list[str]:
    requirement_ids = lane.get("requirement_ids", [])
    if not isinstance(requirement_ids, list):
        return []
    return [item for item in requirement_ids if isinstance(item, str) and item]


def split_csv(value: Any) -> list[str]:
    if not isinstance(value, str) or not value.strip():
        return []
    return [item for item in value.split(",") if item]


def lane_behavior_id(lane_id: str, lane: dict[str, Any]) -> str:
    proof_role = str(lane.get("proof_role", "")).strip()
    default_surface = str(lane.get("default_surface", "")).strip()
    mode = str(lane.get("mode", "")).strip()
    if lane_id == "rust_pr_fast":
        suffix = mode or "unknown"
        return f"rust_{suffix}_behavior"
    if proof_role:
        return f"{proof_role}_{lane_id}"
    if default_surface:
        return f"{default_surface}_behavior"
    return lane_id


def lane_behavior_surface(lane_id: str, lane: dict[str, Any]) -> dict[str, Any]:
    matched_paths = lane.get("matched_paths", [])
    requirement_ids = lane_requirement_ids(lane)
    if lane_id == "rust_pr_fast":
        requirement_ids.extend(split_csv(lane.get("filter_tokens", "")))
    return {
        "id": lane_behavior_id(lane_id, lane),
        "source": "validation_lane_selector",
        "lane_id": lane_id,
        "owner": lane.get("owner", "unknown"),
        "default_surface": lane.get("default_surface", "unknown"),
        "proof_role": lane.get("proof_role", "unknown"),
        "resource_class": lane.get("resource_class", "unknown"),
        "determinism_posture": lane.get("determinism_posture", "unknown"),
        "escalation_rule": lane.get("escalation_rule", "unknown"),
        "requirement_ids": requirement_ids or [lane_id],
        "matched_paths": matched_paths,
        "risk_class": lane.get("risk_class", "unknown"),
    }


def validation_dag_node(lane_id: str, lane: dict[str, Any], behavior_id: str) -> dict[str, Any]:
    status = lane.get("status", "unknown")
    if status == "selected":
        node_status = "runnable"
    elif status in {"escalated", "release_gate_required"}:
        node_status = "blocked_for_escalation"
    else:
        node_status = "not_selected"
    return {
        "id": f"node_{lane_id}",
        "lane_id": lane_id,
        "behavior_surface": behavior_id,
        "status": node_status,
        "proof_role": lane.get("proof_role", "unknown"),
        "owner": lane.get("owner", "unknown"),
        "resource_class": lane.get("resource_class", "unknown"),
        "determinism_posture": lane.get("determinism_posture", "unknown"),
        "command": lane.get("run_command") or lane.get("command", ""),
        "depends_on": [],
    }


def estimate_cost(selected: list[tuple[str, dict[str, Any]]], blocked: list[tuple[str, dict[str, Any]]]) -> dict[str, Any]:
    if blocked:
        runtime_class = "escalated"
    elif not selected:
        runtime_class = "none"
    elif len(selected) == 1:
        lane_id = selected[0][0]
        runtime_class = "tiny" if "docs" in lane_id else "normal"
    else:
        runtime_class = "normal"
    return {
        "runtime_class": runtime_class,
        "selected_lane_count": len(selected),
        "blocked_lane_count": len(blocked),
        "expected_test_scope": "focused_or_family" if not blocked else "requires_human_or_release_gate_decision",
        "token_review_cost": "low" if runtime_class in {"none", "tiny"} else "medium",
    }


def validation_split_contract(
    *,
    status: str,
    pr_publication_sufficient: bool,
    selected: list[tuple[str, dict[str, Any]]],
    slow_proof_families: list[dict[str, Any]],
    escalation_required: bool,
    escalation_reasons: list[dict[str, Any]],
) -> dict[str, Any]:
    return {
        "schema_version": "adl.validation_split.v1",
        "fast_lane": {
            "status": status,
            "selected_lanes": [lane_id for lane_id, _lane in selected],
            "runnable": status in {"ready_to_run", "no_validation_needed"},
            "pr_publication_sufficient": pr_publication_sufficient,
            "execution_model": "local_fast_lane",
        },
        "slow_families": [
            {
                "id": family["id"],
                "feature": family["feature"],
                "proof_role": family.get("proof_role", "slow_proof"),
                "disposition": "reserved_for_explicit_family_selection",
                "owner": "slow_proof_family_runner",
                "command": f"bash adl/tools/run_slow_proof_family.sh --family {family['id']} --run",
            }
            for family in slow_proof_families
        ],
        "fanout_policy": {
            "mode": "explicit_family_selection",
            "missing_or_unmapped_proof": "fail_closed",
            "release_gate_note": "Fast-lane proof does not replace required slow-proof or release-gate evidence.",
        },
        "fail_closed": {
            "required": escalation_required,
            "reason_count": len(escalation_reasons),
        },
    }


def manifest_rule_for(lane: dict[str, Any]) -> str:
    manifest_rule = lane.get("manifest_rule")
    if isinstance(manifest_rule, str) and manifest_rule:
        return manifest_rule
    return f"lane:{lane.get('lane_id', 'unknown')}"


def add_escalation_reason(
    escalation_reasons: list[dict[str, Any]],
    *,
    lane_id: str,
    status: str,
    reason: str,
    matched_paths: list[str],
    manifest_rule: str,
    remediation_hint: str,
    triggering_surface: str | None = None,
) -> None:
    item: dict[str, Any] = {
        "lane_id": lane_id,
        "status": status,
        "reason": reason,
        "matched_paths": matched_paths,
        "manifest_rule": manifest_rule,
        "remediation_hint": remediation_hint,
    }
    if triggering_surface:
        item["triggering_surface"] = triggering_surface
    escalation_reasons.append(item)


def add_diagnostic(
    diagnostics: list[dict[str, Any]],
    *,
    code: str,
    lane_id: str,
    message: str,
    matched_paths: list[str],
    manifest_rule: str,
    remediation_hint: str,
    triggering_surface: str | None = None,
) -> None:
    item: dict[str, Any] = {
        "code": code,
        "severity": "error",
        "lane_id": lane_id,
        "message": message,
        "matched_paths": matched_paths,
        "manifest_rule": manifest_rule,
        "remediation_hint": remediation_hint,
    }
    if triggering_surface:
        item["triggering_surface"] = triggering_surface
    diagnostics.append(item)


def docs_only_paths(paths: list[str]) -> bool:
    if not paths:
        return False
    return all(path.endswith(".md") or path.startswith("docs/") for path in paths)


def slow_proof_pr_fanout_workflow_disposition(changed_paths: list[str]) -> bool:
    changed = set(changed_paths)
    allowed = {
        ".github/workflows/ci-out-of-band.yaml",
        ".github/workflows/ci.yaml",
        "adl/config/slow_proof_families.v0.91.6.json",
        "adl/config/validation_lane_selector.v0.91.6.json",
        "adl/src/runtime_v2/private_state_observatory.rs",
        "adl/src/runtime_v2/tests.rs",
        "adl/tools/check_coverage_impact.sh",
        "adl/tools/ci_path_policy.sh",
        "adl/tools/run_pr_fast_test_lane.sh",
        "adl/tools/run_slow_proof_family.sh",
        "adl/tools/skills/docs/CI_RUNTIME_POLICY_GUIDE.md",
        "adl/tools/test_check_coverage_impact.sh",
        "adl/tools/test_ci_runtime_contracts.sh",
        "adl/tools/test_validate_ci_workflow_policy.rb",
        "adl/tools/test_run_pr_fast_test_lane.sh",
        "adl/tools/test_slow_proof_lane_contract.sh",
        "adl/tools/validation_manager.py",
        "adl/tools/validate_ci_workflow_policy.rb",
        "adl/tools/test_validation_manager.sh",
    }
    if not changed or not changed <= allowed:
        return False
    if ".github/workflows/ci.yaml" not in changed:
        return False
    if "adl/tools/test_ci_runtime_contracts.sh" not in changed:
        return False

    workflow = (ROOT / ".github/workflows/ci.yaml").read_text()
    out_of_band_workflow = (ROOT / ".github/workflows/ci-out-of-band.yaml").read_text()
    contract = (ROOT / "adl/tools/test_ci_runtime_contracts.sh").read_text()
    workflow_policy = (ROOT / "adl/tools/validate_ci_workflow_policy.rb").read_text()
    workflow_policy_contract = (ROOT / "adl/tools/test_validate_ci_workflow_policy.rb").read_text()
    manager = (ROOT / "adl/tools/validation_manager.py").read_text()
    manager_contract = (ROOT / "adl/tools/test_validation_manager.sh").read_text()
    slow_config = (ROOT / "adl/config/slow_proof_families.v0.91.6.json").read_text()
    coverage_impact = (ROOT / "adl/tools/check_coverage_impact.sh").read_text()
    coverage_impact_contract = (ROOT / "adl/tools/test_check_coverage_impact.sh").read_text()
    slow_runner = (ROOT / "adl/tools/run_slow_proof_family.sh").read_text()
    slow_contract = (ROOT / "adl/tools/test_slow_proof_lane_contract.sh").read_text()
    runtime_tests = (ROOT / "adl/src/runtime_v2/tests.rs").read_text()
    required_pr_only_workflow_fragments = [
        "  adl_path_policy:",
        "  adl_ci:",
        "  adl_coverage:",
        "runs-on: ${{ needs.adl_path_policy.outputs.required_runner }}",
    ]
    required_out_of_band_fragments = [
        "  workflow_dispatch:",
        "  adl-slow-proof:",
        "Determine PR fast coverage filters",
    ]
    required_workflow_policy_fragments = [
        'REQUIRED_PR_JOB_IDS = %w[adl_path_policy adl_ci adl_coverage].freeze',
        '"ordinary_pr_heavy_runner_max" => 1',
        '"optional_policy" => "explicit_dispatch_only"',
    ]
    required_workflow_policy_contract_fragments = [
        "test_valid_minimal_required_pr_surface_passes",
        "test_optional_job_is_rejected_even_when_job_if_is_false",
        "test_axis_only_heavy_matrix_is_rejected",
    ]
    required_pr_only_disposition = (
        all(fragment in workflow for fragment in required_pr_only_workflow_fragments)
        and "  adl-slow-proof:" not in workflow
        and "strategy:" not in workflow
        and "matrix:" not in workflow
        and all(fragment in out_of_band_workflow for fragment in required_out_of_band_fragments)
        and all(fragment in workflow_policy for fragment in required_workflow_policy_fragments)
        and all(
            fragment in workflow_policy_contract
            for fragment in required_workflow_policy_contract_fragments
        )
    )
    if required_pr_only_disposition:
        return True

    required_workflow_fragments = [
        "adl-slow-proof:",
        "needs: adl_path_policy",
        "needs.adl_path_policy.outputs.slow_proof_contract_required == 'true'",
        "shard: [1, 2, 3, 4]",
        'bash tools/run_slow_proof_family.sh --family all --run --partition "count:${{ matrix.shard }}/4"',
    ]
    required_contract_fragments = [
        'job_block("adl-slow-proof")',
        "slow_proof_contract_required == 'true'",
        "shard: [1, 2, 3, 4]",
        'bash tools/run_slow_proof_family.sh --family all --run --partition "count:${{ matrix.shard }}/4"',
        "must not use a broad slow-proof-tests run",
    ]
    required_slow_config_fragments = [
        '"module_selectors"',
        "runtime_v2::tests::governed_learning_substrate::",
        "runtime_v2::tests::intelligence_metric_architecture::",
        "runtime_v2::tests::memory_identity_architecture::",
        "runtime_v2::tests::private_state_observatory::",
        "runtime_v2::tests::observatory_flagship::",
    ]
    required_slow_runner_fragments = [
        'family_id == "all"',
        "module_selectors",
        'command_run=(cargo nextest run --lib --features "$feature" -E "$filter_expression")',
        "--partition",
    ]
    required_coverage_impact_fragments = [
        "adl/src/runtime_v2/private_state_observatory.rs",
        "private_state_observatory",
    ]
    required_coverage_impact_contract_fragments = [
        "private-state-observatory-changed.txt",
        "private_state_observatory",
    ]
    required_slow_contract_fragments = [
        "all-family slow-proof filter missing selector",
        "all-family slow-proof run must not use the broad runtime_v2_ substring filter",
        "slow runtime_v2 module is not gated",
    ]
    required_runtime_test_fragments = [
        'feature = "slow-proof-runtime"',
        'feature = "slow-proof-private-state"',
        'feature = "slow-proof-observatory"',
        "mod governed_learning_substrate;",
        "mod intelligence_metric_architecture;",
        "mod memory_identity_architecture;",
        "mod private_state_observatory;",
        "mod observatory_flagship;",
    ]
    required_manager_fragments = [
        "release_gate_slow_proof_pr_fanout_disposition",
        "slow_proof_pr_fanout_workflow_disposition(changed_paths)",
        'node["status"] = "disposition_recorded"',
    ]
    required_manager_contract_fragments = [
        "slow-proof-workflow.txt",
        "pr_publication_sufficient",
        "disposition_recorded",
    ]
    return (
        all(fragment in workflow for fragment in required_workflow_fragments)
        and all(fragment in contract for fragment in required_contract_fragments)
        and all(fragment in slow_config for fragment in required_slow_config_fragments)
        and all(fragment in coverage_impact for fragment in required_coverage_impact_fragments)
        and all(fragment in coverage_impact_contract for fragment in required_coverage_impact_contract_fragments)
        and all(fragment in slow_runner for fragment in required_slow_runner_fragments)
        and all(fragment in slow_contract for fragment in required_slow_contract_fragments)
        and all(fragment in runtime_tests for fragment in required_runtime_test_fragments)
        and all(fragment in manager for fragment in required_manager_fragments)
        and all(fragment in manager_contract for fragment in required_manager_contract_fragments)
    )


def build_profile(plan: dict[str, Any], guardrails: dict[str, Any], manifest_path: Path) -> dict[str, Any]:
    slow_proof_config = load_slow_proof_families()
    slow_proof_families = slow_proof_config.get("families", [])
    lanes = plan.get("lanes", {})
    changed_paths = plan.get("changed_paths", [])
    covered_paths = {
        path
        for lane in lanes.values()
        for path in lane.get("matched_paths", [])
    }
    uncovered_paths = [path for path in changed_paths if path not in covered_paths]
    selected = [
        (lane_id, lane)
        for lane_id, lane in lanes.items()
        if lane.get("status") == "selected"
    ]
    blocked = [
        (lane_id, lane)
        for lane_id, lane in lanes.items()
        if lane.get("status") in {"escalated", "release_gate_required"}
    ]
    blocked_lane_ids = {lane_id for lane_id, _lane in blocked}
    rust_lane_for_disposition = lanes.get("rust_pr_fast")
    slow_proof_contract_only_disposition = (
        blocked_lane_ids <= {"rust_pr_fast", "slow_proof_review", "release_gate_review"}
        and "rust_pr_fast" in blocked_lane_ids
        and "slow_proof_review" in blocked_lane_ids
        and isinstance(rust_lane_for_disposition, dict)
        and str(rust_lane_for_disposition.get("mode", "")).strip() == "contract_only"
        and str(rust_lane_for_disposition.get("reason", "")).strip()
        == "slow_proof_inventory_change_covered_by_contract_check"
    )
    release_gate_slow_proof_pr_fanout_disposition = (
        bool(blocked_lane_ids)
        and blocked_lane_ids <= {"release_gate_review", "slow_proof_review", "rust_pr_fast"}
        and "release_gate_review" in blocked_lane_ids
        and slow_proof_pr_fanout_workflow_disposition(changed_paths)
    )
    soft_disposition_recorded = (
        (
            slow_proof_contract_only_disposition
            and (
                "release_gate_review" not in blocked_lane_ids
                or release_gate_slow_proof_pr_fanout_disposition
            )
        )
        or (
            release_gate_slow_proof_pr_fanout_disposition
            and blocked_lane_ids <= {"release_gate_review", "slow_proof_review"}
        )
    )
    effective_blocked = [] if soft_disposition_recorded else blocked

    run = []
    behavior_surfaces = []
    dag_nodes = []
    diagnostics: list[dict[str, Any]] = []
    for lane_id, lane in selected:
        behavior = lane_behavior_surface(lane_id, lane)
        behavior_surfaces.append(behavior)
        dag_nodes.append(validation_dag_node(lane_id, lane, behavior["id"]))
        command = lane.get("run_command") or lane.get("command")
        if command:
            run.append(
                {
                    "lane_id": lane_id,
                    "command": command,
                    "reason": lane.get("reason", "selector_selected_lane"),
                    "matched_paths": lane.get("matched_paths", []),
                    "vpp_record": lane.get("vpp_record"),
                }
            )

    not_run = [
        {
            "surface": "full_workspace_nextest",
            "reason": "not selected by validation profile",
        },
        {
            "surface": "cargo_clippy_all_targets",
            "reason": "reserved for broad shared changes or release gates",
        },
        {
            "surface": "slow_proof",
            "reason": "reserved for explicit proof-family selection",
        },
        {
            "surface": "coverage_release_gate",
            "reason": "reserved for coverage or release policy selection",
        },
    ]
    not_run.extend(
        {
            "surface": f"slow_proof/{family['id']}",
            "reason": "reserved for explicit proof-family selection",
            "feature": family["feature"],
        }
        for family in slow_proof_families
    )

    unmapped_change_gap = bool(uncovered_paths)
    selected_lane_limit = int(guardrails["max_selected_lanes"])
    manifest_path_display = (
        str(manifest_path.relative_to(ROOT))
        if manifest_path.is_relative_to(ROOT)
        else str(manifest_path)
    )

    escalation_required = (
        bool(effective_blocked)
        or len(selected) > selected_lane_limit
        or unmapped_change_gap
    )
    escalation_reasons = []
    for lane_id, lane in blocked:
        behavior = lane_behavior_surface(lane_id, lane)
        behavior_surfaces.append(behavior)
        if (
            slow_proof_contract_only_disposition
            and lane_id in {"rust_pr_fast", "slow_proof_review"}
        ) or (
            release_gate_slow_proof_pr_fanout_disposition
            and lane_id in {"release_gate_review", "slow_proof_review"}
        ):
            node = validation_dag_node(lane_id, lane, behavior["id"])
            node["status"] = "disposition_recorded"
            dag_nodes.append(node)
            continue
        dag_nodes.append(validation_dag_node(lane_id, lane, behavior["id"]))
        manifest_rule = manifest_rule_for(lane)
        matched_paths = lane.get("matched_paths", [])
        remediation_hint = (
            "Record a release-gate disposition before publication."
            if lane_id == "release_gate_review"
            else "Record a slow-proof disposition before treating the change as ordinary PR proof."
            if lane_id == "slow_proof_review"
            else "Split the Rust change further or route it to the appropriate broad proof lane."
            if lane_id == "rust_pr_fast"
            else "Adjust the validation manifest or route the work to the correct owner lane."
        )
        add_escalation_reason(
            escalation_reasons,
            lane_id=lane_id,
            status=str(lane.get("status", "escalated")),
            reason=str(lane.get("reason", "selector_requires_escalation")),
            matched_paths=matched_paths,
            manifest_rule=manifest_rule,
            remediation_hint=remediation_hint,
            triggering_surface=matched_paths[0] if matched_paths else None,
        )
        add_diagnostic(
            diagnostics,
            code=f"{lane_id}_requires_escalation",
            lane_id=lane_id,
            message=f"{lane_id} requires escalation because {lane.get('reason', 'selector_requires_escalation')}",
            matched_paths=matched_paths,
            manifest_rule=manifest_rule,
            remediation_hint=remediation_hint,
            triggering_surface=matched_paths[0] if matched_paths else None,
        )
    if len(selected) > selected_lane_limit:
        add_escalation_reason(
            escalation_reasons,
            lane_id="selected_lane_threshold",
            status="escalated",
            reason=f"selected lane count {len(selected)} exceeds limit {selected_lane_limit}",
            matched_paths=plan.get("changed_paths", []),
            manifest_rule="manager_guardrails.max_selected_lanes",
            remediation_hint="Split the change set or raise the threshold intentionally in the validation manager guardrails.",
        )
        add_diagnostic(
            diagnostics,
            code="selected_lane_threshold_exceeded",
            lane_id="selected_lane_threshold",
            message=f"selected lane count {len(selected)} exceeds configured limit {selected_lane_limit}",
            matched_paths=plan.get("changed_paths", []),
            manifest_rule="manager_guardrails.max_selected_lanes",
            remediation_hint="Split the change set or raise the threshold intentionally in the validation manager guardrails.",
        )
    if unmapped_change_gap:
        add_escalation_reason(
            escalation_reasons,
            lane_id="unmapped_change_surface",
            status="escalated",
            reason="selector left changed paths without validation-lane coverage",
            matched_paths=uncovered_paths,
            manifest_rule=manifest_path_display,
            remediation_hint="Add or refine a path selector in the validation manifest so the changed surface maps to a proving lane.",
        )
        add_diagnostic(
            diagnostics,
            code="unmapped_change_surface",
            lane_id="unmapped_change_surface",
            message="selector left changed paths without validation-lane coverage",
            matched_paths=uncovered_paths,
            manifest_rule=manifest_path_display,
            remediation_hint="Add or refine a path selector in the validation manifest so the changed surface maps to a proving lane.",
        )

    if docs_only_paths(changed_paths):
        forbidden_lane_ids = set(guardrails.get("docs_only_forbidden_lane_ids", []))
        rust_docs_lanes = [lane_id for lane_id, _lane in selected + blocked if lane_id in forbidden_lane_ids]
        if rust_docs_lanes:
            escalation_required = True
            add_escalation_reason(
                escalation_reasons,
                lane_id="docs_only_rust_guardrail",
                status="escalated",
                reason=f"docs-only change selected forbidden lanes: {', '.join(sorted(rust_docs_lanes))}",
                matched_paths=changed_paths,
                manifest_rule="manager_guardrails.docs_only_forbidden_lane_ids",
                remediation_hint="Keep docs-only profiles mapped to docs proof only; route Rust-affecting docs through a separate non-docs issue if needed.",
            )
            add_diagnostic(
                diagnostics,
                code="docs_only_rust_guardrail",
                lane_id="docs_only_rust_guardrail",
                message=f"docs-only change selected forbidden lanes: {', '.join(sorted(rust_docs_lanes))}",
                matched_paths=changed_paths,
                manifest_rule="manager_guardrails.docs_only_forbidden_lane_ids",
                remediation_hint="Keep docs-only profiles mapped to docs proof only; route Rust-affecting docs through a separate non-docs issue if needed.",
            )

    rust_lane = lanes.get("rust_pr_fast")
    if isinstance(rust_lane, dict):
        pr_fast_guardrails = guardrails["pr_fast"]
        rust_surface_count = int(rust_lane.get("rust_surface_count", 0))
        filter_tokens = split_csv(rust_lane.get("filter_tokens", ""))
        mode = str(rust_lane.get("mode", "")).strip()
        matched_paths = rust_lane.get("matched_paths", [])
        if mode in pr_fast_guardrails["blocked_modes"] and not (
            slow_proof_contract_only_disposition and mode == "contract_only"
        ):
            escalation_required = True
            add_diagnostic(
                diagnostics,
                code=f"pr_fast_mode_{mode}",
                lane_id="rust_pr_fast",
                message=(
                    "slow-proof contract-only planning cannot be run as an ordinary PR-fast profile"
                    if mode == "contract_only"
                    else "PR-fast planning expanded beyond the configured ordinary profile guardrails"
                ),
                matched_paths=matched_paths,
                manifest_rule="manager_guardrails.pr_fast.blocked_modes",
                remediation_hint="Use the named slow-proof or broad proof path instead of forcing ordinary PR-fast execution.",
                triggering_surface=matched_paths[0] if matched_paths else None,
            )
        if mode == "focused" and len(filter_tokens) > int(pr_fast_guardrails["max_filter_token_count"]):
            escalation_required = True
            add_escalation_reason(
                escalation_reasons,
                lane_id="rust_pr_fast",
                status="escalated",
                reason=f"focused filter count {len(filter_tokens)} exceeds limit {pr_fast_guardrails['max_filter_token_count']}",
                matched_paths=matched_paths,
                manifest_rule="manager_guardrails.pr_fast.max_filter_token_count",
                remediation_hint="Split the change or raise the focused threshold intentionally in manager guardrails.",
                triggering_surface=matched_paths[0] if matched_paths else None,
            )
            add_diagnostic(
                diagnostics,
                code="pr_fast_filter_threshold_exceeded",
                lane_id="rust_pr_fast",
                message=f"focused filter count {len(filter_tokens)} exceeds configured limit {pr_fast_guardrails['max_filter_token_count']}",
                matched_paths=matched_paths,
                manifest_rule="manager_guardrails.pr_fast.max_filter_token_count",
                remediation_hint="Split the change or raise the focused threshold intentionally in manager guardrails.",
                triggering_surface=matched_paths[0] if matched_paths else None,
            )
        if mode == "family" and len(filter_tokens) > int(pr_fast_guardrails["max_family_token_count"]):
            escalation_required = True
            add_escalation_reason(
                escalation_reasons,
                lane_id="rust_pr_fast",
                status="escalated",
                reason=f"family filter count {len(filter_tokens)} exceeds limit {pr_fast_guardrails['max_family_token_count']}",
                matched_paths=matched_paths,
                manifest_rule="manager_guardrails.pr_fast.max_family_token_count",
                remediation_hint="Split the change or raise the family threshold intentionally in manager guardrails.",
                triggering_surface=matched_paths[0] if matched_paths else None,
            )
            add_diagnostic(
                diagnostics,
                code="pr_fast_family_threshold_exceeded",
                lane_id="rust_pr_fast",
                message=f"family filter count {len(filter_tokens)} exceeds configured limit {pr_fast_guardrails['max_family_token_count']}",
                matched_paths=matched_paths,
                manifest_rule="manager_guardrails.pr_fast.max_family_token_count",
                remediation_hint="Split the change or raise the family threshold intentionally in manager guardrails.",
                triggering_surface=matched_paths[0] if matched_paths else None,
            )
        if rust_surface_count > int(pr_fast_guardrails["max_rust_surface_count"]):
            escalation_required = True
            add_escalation_reason(
                escalation_reasons,
                lane_id="rust_pr_fast",
                status="escalated",
                reason=f"Rust surface count {rust_surface_count} exceeds limit {pr_fast_guardrails['max_rust_surface_count']}",
                matched_paths=matched_paths,
                manifest_rule="manager_guardrails.pr_fast.max_rust_surface_count",
                remediation_hint="Split the change or raise the Rust-surface threshold intentionally in manager guardrails.",
                triggering_surface=matched_paths[0] if matched_paths else None,
            )
            add_diagnostic(
                diagnostics,
                code="pr_fast_rust_surface_threshold_exceeded",
                lane_id="rust_pr_fast",
                message=f"Rust surface count {rust_surface_count} exceeds configured limit {pr_fast_guardrails['max_rust_surface_count']}",
                matched_paths=matched_paths,
                manifest_rule="manager_guardrails.pr_fast.max_rust_surface_count",
                remediation_hint="Split the change or raise the Rust-surface threshold intentionally in manager guardrails.",
                triggering_surface=matched_paths[0] if matched_paths else None,
            )

    status = "ready_to_run"
    if not changed_paths:
        status = "no_validation_needed"
    elif escalation_required:
        status = "escalation_required"
    elif plan.get("aggregate_status") != "selected" and not soft_disposition_recorded:
        status = "not_runnable"

    pr_publication_sufficient = (
        (bool(plan.get("pr_publication_sufficient")) or soft_disposition_recorded)
        and not unmapped_change_gap
        and not escalation_required
        and status == "ready_to_run"
    )
    validation_split = validation_split_contract(
        status=status,
        pr_publication_sufficient=pr_publication_sufficient,
        selected=selected,
        slow_proof_families=slow_proof_families,
        escalation_required=escalation_required,
        escalation_reasons=escalation_reasons,
    )

    return {
        "schema_version": "adl.validation_profile.v1",
        "selected_profile": profile_id(plan),
        "status": status,
        "selector_aggregate_status": plan.get("aggregate_status"),
        "pr_publication_sufficient": pr_publication_sufficient,
        "changed_paths": changed_paths,
        "run": run,
        "not_run": not_run,
        "deferred": [],
        "validation_split": validation_split,
        "behavior_surfaces": behavior_surfaces,
        "validation_dag": {
            "nodes": dag_nodes,
            "edges": [],
            "compression_note": "profile validates behavior surfaces rather than enumerating every test-bearing module",
        },
        "slow_proof_families": [
            {
                "id": family["id"],
                "feature": family["feature"],
                "proof_role": family.get("proof_role", "slow_proof"),
                "description": family.get("description", ""),
                "selection_mode": "manual_only",
                "command": f"bash adl/tools/run_slow_proof_family.sh --family {family['id']} --run",
                "sample_tests": family.get("sample_tests", []),
            }
            for family in slow_proof_families
        ],
        "estimated_cost": estimate_cost(selected, effective_blocked),
        "escalation": {
            "required": escalation_required,
            "reasons": escalation_reasons,
        },
        "diagnostics": diagnostics,
        "selector_plan": plan,
    }


def shell_quote(value: str) -> str:
    return shlex.quote(value)


def combined_run_command(profile: dict[str, Any]) -> str:
    commands = [
        str(item.get("command", "")).strip()
        for item in profile.get("run", [])
        if str(item.get("command", "")).strip()
    ]
    return " && ".join(commands)


def deterministic_or_evidence_bound(profile: dict[str, Any]) -> tuple[bool, list[str]]:
    allowed_determinism = {"deterministic", "evidence_bound"}
    unsupported = [
        surface.get("determinism_posture", "unknown")
        for surface in profile.get("behavior_surfaces", [])
        if surface.get("determinism_posture", "unknown") not in allowed_determinism
    ]
    return not unsupported, unsupported


def base_platform_eligibility(profile: dict[str, Any]) -> dict[str, Any]:
    selected_lane_count = int(profile["estimated_cost"]["selected_lane_count"])
    runtime_class = str(profile["estimated_cost"]["runtime_class"])
    deterministic_ok, unsupported_determinism = deterministic_or_evidence_bound(profile)
    command = combined_run_command(profile)
    return {
        "status": profile["status"],
        "escalation_required": bool(profile["escalation"]["required"]),
        "selected_lane_count": selected_lane_count,
        "runtime_class": runtime_class,
        "deterministic_or_evidence_bound": deterministic_ok,
        "unsupported_determinism": unsupported_determinism,
        "command": command,
    }


def platform_candidate(
    *,
    platform: str,
    decision: str,
    reason: str,
    command: str | None = None,
    cache_posture: str,
    cost_posture: str,
    launch_posture: str = "dry_run_only",
    wrapper: str | None = None,
    caveats: list[str] | None = None,
) -> dict[str, Any]:
    candidate: dict[str, Any] = {
        "platform": platform,
        "decision": decision,
        "reason": reason,
        "cache_posture": cache_posture,
        "cost_posture": cost_posture,
        "launch_posture": launch_posture,
    }
    if command:
        candidate["command"] = command
    if wrapper:
        candidate["wrapper"] = wrapper
    if caveats:
        candidate["caveats"] = caveats
    return candidate


def local_platform_candidate(profile: dict[str, Any], eligibility: dict[str, Any]) -> dict[str, Any]:
    if eligibility["status"] not in {"ready_to_run", "no_validation_needed"}:
        return platform_candidate(
            platform="local",
            decision="rejected",
            reason=f"validation profile status {eligibility['status']} is not locally runnable",
            cache_posture="local_target_or_repo_configured",
            cost_posture="no_cloud_cost",
        )
    if eligibility["escalation_required"]:
        return platform_candidate(
            platform="local",
            decision="rejected",
            reason="validation profile requires escalation before local routing",
            cache_posture="local_target_or_repo_configured",
            cost_posture="no_cloud_cost",
        )
    command = eligibility["command"] or "true"
    return platform_candidate(
        platform="local",
        decision="eligible",
        reason="local platform can run the selected validation profile without cloud resources",
        command=command,
        cache_posture="local_target_or_repo_configured",
        cost_posture="no_cloud_cost",
    )


def nessus_platform_candidate(profile: dict[str, Any], eligibility: dict[str, Any], command: str | None = None) -> dict[str, Any]:
    remote_command = command or eligibility["command"]
    if eligibility["status"] != "ready_to_run":
        return platform_candidate(
            platform="nessus",
            decision="rejected",
            reason=f"validation profile status {eligibility['status']} is not remote-runnable",
            cache_posture="remote_target_sccache_warm",
            cost_posture="operator_host_no_cloud_cost",
            wrapper=NESSUS_REMOTE_RUNNER,
        )
    if eligibility["escalation_required"]:
        return platform_candidate(
            platform="nessus",
            decision="rejected",
            reason="validation profile already requires escalation",
            cache_posture="remote_target_sccache_warm",
            cost_posture="operator_host_no_cloud_cost",
            wrapper=NESSUS_REMOTE_RUNNER,
        )
    if eligibility["selected_lane_count"] != 1:
        return platform_candidate(
            platform="nessus",
            decision="rejected",
            reason=f"Nessus routing requires exactly 1 selected lane, observed {eligibility['selected_lane_count']}",
            cache_posture="remote_target_sccache_warm",
            cost_posture="operator_host_no_cloud_cost",
            wrapper=NESSUS_REMOTE_RUNNER,
        )
    if eligibility["runtime_class"] in {"none", "tiny", "escalated"}:
        return platform_candidate(
            platform="nessus",
            decision="rejected",
            reason=f"runtime_class {eligibility['runtime_class']} is not eligible for Nessus remote execution",
            cache_posture="remote_target_sccache_warm",
            cost_posture="operator_host_no_cloud_cost",
            wrapper=NESSUS_REMOTE_RUNNER,
        )
    if not eligibility["deterministic_or_evidence_bound"]:
        return platform_candidate(
            platform="nessus",
            decision="rejected",
            reason="Nessus routing supports deterministic or evidence-bound lanes only",
            cache_posture="remote_target_sccache_warm",
            cost_posture="operator_host_no_cloud_cost",
            wrapper=NESSUS_REMOTE_RUNNER,
            caveats=[f"unsupported_determinism={','.join(eligibility['unsupported_determinism'])}"],
        )
    if not remote_command:
        return platform_candidate(
            platform="nessus",
            decision="rejected",
            reason="Nessus routing requires a validation command",
            cache_posture="remote_target_sccache_warm",
            cost_posture="operator_host_no_cloud_cost",
            wrapper=NESSUS_REMOTE_RUNNER,
        )
    return platform_candidate(
        platform="nessus",
        decision="eligible",
        reason="single-lane non-tiny deterministic profile is eligible for Nessus remote execution",
        command=f"{NESSUS_REMOTE_RUNNER} --command {shell_quote(remote_command)}",
        cache_posture="remote_target_sccache_warm",
        cost_posture="operator_host_no_cloud_cost",
        wrapper=NESSUS_REMOTE_RUNNER,
    )


def aws_spot_platform_candidate(profile: dict[str, Any], eligibility: dict[str, Any]) -> dict[str, Any]:
    command = eligibility["command"]
    if eligibility["status"] != "ready_to_run":
        return platform_candidate(
            platform="aws_spot",
            decision="rejected",
            reason=f"validation profile status {eligibility['status']} is not remote-runnable",
            cache_posture="warm_ebs_cache:/mnt/adl-cache",
            cost_posture="aws_spot_instance_plus_retained_ebs_storage",
            wrapper=AWS_SPOT_REMOTE_RUNNER,
        )
    if eligibility["escalation_required"]:
        return platform_candidate(
            platform="aws_spot",
            decision="rejected",
            reason="validation profile already requires escalation",
            cache_posture="warm_ebs_cache:/mnt/adl-cache",
            cost_posture="aws_spot_instance_plus_retained_ebs_storage",
            wrapper=AWS_SPOT_REMOTE_RUNNER,
        )
    if eligibility["runtime_class"] in {"none", "tiny", "escalated"}:
        return platform_candidate(
            platform="aws_spot",
            decision="rejected",
            reason=f"runtime_class {eligibility['runtime_class']} is not cost-appropriate for AWS Spot",
            cache_posture="warm_ebs_cache:/mnt/adl-cache",
            cost_posture="aws_spot_instance_plus_retained_ebs_storage",
            wrapper=AWS_SPOT_REMOTE_RUNNER,
        )
    if not eligibility["deterministic_or_evidence_bound"]:
        return platform_candidate(
            platform="aws_spot",
            decision="rejected",
            reason="AWS Spot routing supports deterministic or evidence-bound lanes only",
            cache_posture="warm_ebs_cache:/mnt/adl-cache",
            cost_posture="aws_spot_instance_plus_retained_ebs_storage",
            wrapper=AWS_SPOT_REMOTE_RUNNER,
            caveats=[f"unsupported_determinism={','.join(eligibility['unsupported_determinism'])}"],
        )
    if not command:
        return platform_candidate(
            platform="aws_spot",
            decision="rejected",
            reason="AWS Spot routing requires a validation command",
            cache_posture="warm_ebs_cache:/mnt/adl-cache",
            cost_posture="aws_spot_instance_plus_retained_ebs_storage",
            wrapper=AWS_SPOT_REMOTE_RUNNER,
        )
    return platform_candidate(
        platform="aws_spot",
        decision="eligible",
        reason="non-tiny deterministic profile can use the warm-EBS AWS Spot lane when credentials and cache are available",
        command=(
            f"{AWS_SPOT_REMOTE_RUNNER} --command {shell_quote(command)} "
            "--instance-type m7a.2xlarge --print-command"
        ),
        cache_posture="warm_ebs_cache:/mnt/adl-cache",
        cost_posture="aws_spot_instance_plus_retained_ebs_storage",
        wrapper=AWS_SPOT_REMOTE_RUNNER,
        caveats=[
            "requires Agent Logic AWS profile or GitHub OIDC role",
            "requires retained EBS cache attachment for warm-cache proof",
            "requires explicit --run outside validation-manager to launch paid resources",
        ],
    )


def codebuild_platform_candidate(profile: dict[str, Any], eligibility: dict[str, Any]) -> dict[str, Any]:
    wrapper_exists = AWS_CODEFRIEND_BUILD_RUNNER_PATH.exists()
    if not wrapper_exists:
        return platform_candidate(
            platform="codebuild",
            decision="rejected",
            reason="CodeBuild wrapper is not present on this branch; merge or rebase the #4838 CodeFriend build lane first",
            cache_posture="stable_local_target_cache_plus_s3_sccache_when_wrapper_available",
            cost_posture="aws_codebuild_compute_minutes",
            wrapper=AWS_CODEFRIEND_BUILD_RUNNER,
            caveats=["dependency_issue=#4838", "dependency_pr=#4865"],
        )
    if eligibility["status"] != "ready_to_run":
        return platform_candidate(
            platform="codebuild",
            decision="rejected",
            reason=f"validation profile status {eligibility['status']} is not CodeBuild-runnable",
            cache_posture="stable_local_target_cache_plus_s3_sccache",
            cost_posture="aws_codebuild_compute_minutes",
            wrapper=AWS_CODEFRIEND_BUILD_RUNNER,
        )
    if eligibility["escalation_required"]:
        return platform_candidate(
            platform="codebuild",
            decision="rejected",
            reason="validation profile already requires escalation",
            cache_posture="stable_local_target_cache_plus_s3_sccache",
            cost_posture="aws_codebuild_compute_minutes",
            wrapper=AWS_CODEFRIEND_BUILD_RUNNER,
        )
    return platform_candidate(
        platform="codebuild",
        decision="eligible",
        reason="profile can use the scalable CodeFriend CodeBuild lane when the wrapper, project, builder image, and caches are available",
        command=(
            f"{AWS_CODEFRIEND_BUILD_RUNNER} --project-name adl-codefriend-build "
            "--dry-run --print-command "
            "--env ADL_CODEFRIEND_BUILD_COMMAND='bash adl/tools/run_pr_fast_test_lane.sh'"
        ),
        cache_posture="stable_local_target_cache_plus_s3_sccache",
        cost_posture="aws_codebuild_compute_minutes",
        wrapper=AWS_CODEFRIEND_BUILD_RUNNER,
        caveats=[
            "requires Agent Logic AWS CodeBuild project and service role",
            "requires builder image and S3 sccache to be configured",
            "requires explicit live trigger outside validation-manager",
        ],
    )


def wuji_platform_candidate(profile: dict[str, Any], eligibility: dict[str, Any]) -> dict[str, Any]:
    if eligibility["status"] not in {"ready_to_run", "no_validation_needed"}:
        reason = f"validation profile status {eligibility['status']} is not wuji-runnable"
    elif eligibility["escalation_required"]:
        reason = "validation profile requires escalation before wuji routing"
    else:
        reason = "wuji is ARM and requires a separate arm64 builder image before scheduler routing can claim parity"
    return platform_candidate(
        platform="wuji",
        decision="rejected",
        reason=reason,
        cache_posture="linked_target_cache_warm_arm64",
        cost_posture="operator_host_no_cloud_cost",
        caveats=["arm64_builder_image_gap", "do_not_claim_x86_64_parity"],
    )


def platform_routing_decision(profile: dict[str, Any], args: argparse.Namespace) -> dict[str, Any] | None:
    if not args.platform_routing and not args.validation_platform:
        return None
    requested = args.validation_platform or "auto"
    eligibility = base_platform_eligibility(profile)
    candidates = [
        local_platform_candidate(profile, eligibility),
        nessus_platform_candidate(profile, eligibility),
        aws_spot_platform_candidate(profile, eligibility),
        codebuild_platform_candidate(profile, eligibility),
        wuji_platform_candidate(profile, eligibility),
    ]
    by_platform = {candidate["platform"]: candidate for candidate in candidates}

    selected: dict[str, Any] | None = None
    if requested != "auto":
        candidate = by_platform[requested]
        selected = candidate if candidate["decision"] == "eligible" else None
    else:
        lane_ids = [item.get("lane_id") for item in profile.get("run", [])]
        if by_platform["local"]["decision"] == "eligible" and eligibility["runtime_class"] in {"none", "tiny"}:
            selected = by_platform["local"]
        elif "aws_remote_validation_tooling" in lane_ids and by_platform["aws_spot"]["decision"] == "eligible":
            selected = by_platform["aws_spot"]
        elif eligibility["selected_lane_count"] > 1 and by_platform["codebuild"]["decision"] == "eligible":
            selected = by_platform["codebuild"]
        elif by_platform["nessus"]["decision"] == "eligible":
            selected = by_platform["nessus"]
        elif by_platform["aws_spot"]["decision"] == "eligible":
            selected = by_platform["aws_spot"]
        elif by_platform["local"]["decision"] == "eligible":
            selected = by_platform["local"]

    decision = "selected" if selected else "rejected"
    reason = (
        selected["reason"]
        if selected
        else by_platform[requested]["reason"]
        if requested != "auto"
        else "no eligible validation platform candidate for this profile"
    )
    return {
        "schema_version": "adl.validation_platform_routing.v1",
        "requested_platform": requested,
        "decision": decision,
        "selected_platform": selected["platform"] if selected else None,
        "reason": reason,
        "no_launch": True,
        "launch_policy": "validation-manager only emits routing decisions and dry-run commands; live cloud runs require platform wrappers with explicit --run",
        "candidates": candidates,
    }


def remote_runner_decision(profile: dict[str, Any], args: argparse.Namespace) -> dict[str, Any] | None:
    if not args.remote_runner and not args.remote_command:
        return None
    if bool(args.remote_runner) != bool(args.remote_command):
        fail("remote runner selection requires both --remote-runner and --remote-command")
    if args.remote_runner != "nessus":
        fail(f"unsupported remote runner: {args.remote_runner}")

    eligibility = base_platform_eligibility(profile)
    candidate = nessus_platform_candidate(profile, eligibility, args.remote_command)
    if candidate["decision"] != "eligible":
        rejected = {
            "requested": "nessus",
            "decision": "rejected",
            "reason": candidate["reason"],
        }
        if candidate.get("caveats"):
            rejected["caveats"] = candidate["caveats"]
        return rejected

    remote_command = candidate["command"]
    if args.remote_artifact_dir:
        remote_command += f" --local-artifact-dir {shell_quote(str(args.remote_artifact_dir.resolve()))}"
    return {
        "requested": "nessus",
        "decision": "selected",
        "reason": candidate["reason"],
        "command": remote_command,
    }


def print_text(profile: dict[str, Any]) -> None:
    print("Validation profile")
    print(f"  selected_profile={profile['selected_profile']}")
    print(f"  status={profile['status']}")
    print(f"  selector_aggregate_status={profile['selector_aggregate_status']}")
    print(f"  pr_publication_sufficient={str(profile['pr_publication_sufficient']).lower()}")
    if profile["run"]:
        print("  run:")
        for item in profile["run"]:
            print(f"    - lane={item['lane_id']} reason={item['reason']}")
            print(f"      command={item['command']}")
    else:
        print("  run: []")
    if profile.get("remote_runner"):
        remote = profile["remote_runner"]
        print("  remote_runner:")
        print(f"    requested={remote['requested']}")
        print(f"    decision={remote['decision']}")
        print(f"    reason={remote['reason']}")
        if remote.get("command"):
            print(f"    command={remote['command']}")
    if profile.get("platform_routing"):
        routing = profile["platform_routing"]
        print("  platform_routing:")
        print(f"    requested_platform={routing['requested_platform']}")
        print(f"    decision={routing['decision']}")
        print(f"    selected_platform={routing['selected_platform']}")
        print(f"    reason={routing['reason']}")
        for candidate in routing["candidates"]:
            print(
                f"    - platform={candidate['platform']} decision={candidate['decision']} "
                f"cache={candidate['cache_posture']}"
            )
            if candidate.get("command"):
                print(f"      command={candidate['command']}")
    if profile["escalation"]["required"]:
        print("  escalation:")
        for reason in profile["escalation"]["reasons"]:
            print(f"    - lane={reason['lane_id']} status={reason['status']} reason={reason['reason']}")
            if reason.get("triggering_surface"):
                print(f"      triggering_surface={reason['triggering_surface']}")
            if reason.get("manifest_rule"):
                print(f"      manifest_rule={reason['manifest_rule']}")
            if reason.get("remediation_hint"):
                print(f"      remediation_hint={reason['remediation_hint']}")
    if profile["diagnostics"]:
        print("  diagnostics:")
        for item in profile["diagnostics"]:
            print(f"    - code={item['code']} lane={item['lane_id']}")
            print(f"      message={item['message']}")
            if item.get("triggering_surface"):
                print(f"      triggering_surface={item['triggering_surface']}")
            print(f"      manifest_rule={item['manifest_rule']}")
            print(f"      remediation_hint={item['remediation_hint']}")
    if profile["behavior_surfaces"]:
        print("  behavior_surfaces:")
        for behavior in profile["behavior_surfaces"]:
            print(f"    - id={behavior['id']} risk={behavior['risk_class']} source={behavior['source']}")
    if profile.get("slow_proof_families"):
        print("  slow_proof_families:")
        for family in profile["slow_proof_families"]:
            print(
                f"    - id={family['id']} feature={family['feature']} selection_mode={family['selection_mode']}"
            )
    print(
        "  estimated_cost="
        f"{profile['estimated_cost']['runtime_class']} "
        f"lanes={profile['estimated_cost']['selected_lane_count']}"
    )


def write_report(path: Path, profile: dict[str, Any]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(profile, indent=2, sort_keys=True) + "\n")


def utc_now_iso() -> str:
    return datetime.now(timezone.utc).replace(microsecond=0).isoformat().replace("+00:00", "Z")


def repo_relative(path: Path) -> str:
    resolved = path.resolve()
    try:
        return resolved.relative_to(ROOT).as_posix()
    except ValueError:
        return str(resolved)


def default_build_action_log_dir() -> Path:
    run_id = f"{datetime.now(timezone.utc).strftime('%Y%m%dT%H%M%SZ')}-{os.getpid()}"
    return ROOT / ".adl/logs/build-actions/validation-manager" / run_id


def safe_packet_stem(lane_id: str, index: int) -> str:
    safe = "".join(ch if ch.isalnum() or ch in {"-", "_"} else "-" for ch in lane_id)
    safe = safe.strip("-_") or "lane"
    return f"{index:02d}-{safe}"


def build_action_log_root(args: argparse.Namespace) -> Path:
    configured = args.build_action_log_dir or os.environ.get("ADL_BUILD_ACTION_LOG_DIR")
    if configured:
        return Path(configured).expanduser().resolve()
    return default_build_action_log_dir()


def write_build_action_manifest(log_root: Path, packets: list[dict[str, Any]]) -> Path:
    manifest_path = log_root / "manifest.json"
    manifest = {
        "schema_version": BUILD_ACTION_LOG_MANIFEST_SCHEMA,
        "runner": "validation_manager",
        "packet_count": len(packets),
        "packets": [packet["packet_ref"] for packet in packets],
        "created_at": utc_now_iso(),
    }
    manifest_path.write_text(json.dumps(manifest, indent=2, sort_keys=True) + "\n")
    return manifest_path


def run_validation_action_with_log(
    item: dict[str, Any],
    *,
    log_root: Path,
    index: int,
    json_mode: bool,
) -> tuple[int, dict[str, Any]]:
    log_root.mkdir(parents=True, exist_ok=True)
    stem = safe_packet_stem(str(item.get("lane_id", "lane")), index)
    stdout_path = log_root / f"{stem}.stdout.log"
    stderr_path = log_root / f"{stem}.stderr.log"
    packet_path = log_root / f"{stem}.build-action-log.json"
    command = str(item["command"])
    started_at = utc_now_iso()
    started = time.monotonic()
    with stdout_path.open("w") as stdout_file, stderr_path.open("w") as stderr_file:
        result = subprocess.run(
            command,
            cwd=ROOT,
            shell=True,
            text=True,
            stdout=stdout_file,
            stderr=stderr_file,
        )
    ended_at = utc_now_iso()
    elapsed_ms = int((time.monotonic() - started) * 1000)

    stdout_text = stdout_path.read_text(errors="replace")
    stderr_text = stderr_path.read_text(errors="replace")
    if stdout_text:
        stdout_stream = sys.stderr if json_mode else sys.stdout
        print(stdout_text, end="", file=stdout_stream)
    if stderr_text:
        print(stderr_text, end="", file=sys.stderr)

    packet = {
        "schema_version": BUILD_ACTION_LOG_SCHEMA,
        "runner": "validation_manager",
        "lane_id": item.get("lane_id"),
        "reason": item.get("reason"),
        "command": command,
        "command_sha256": hashlib.sha256(command.encode("utf-8")).hexdigest(),
        "cwd": ".",
        "binary_path": "shell",
        "cache_posture": "local_target_or_repo_configured",
        "platform": {
            "system": platform.system(),
            "machine": platform.machine(),
            "python": platform.python_version(),
        },
        "started_at": started_at,
        "ended_at": ended_at,
        "elapsed_ms": elapsed_ms,
        "exit_code": result.returncode,
        "status": "passed" if result.returncode == 0 else "failed",
        "stdout_ref": repo_relative(stdout_path),
        "stderr_ref": repo_relative(stderr_path),
        "packet_ref": repo_relative(packet_path),
        "redaction_status": "not_redacted_local_private_log",
        "retention": "local_workflow_evidence",
    }
    packet_path.write_text(json.dumps(packet, indent=2, sort_keys=True) + "\n")
    return result.returncode, packet


def run_profile(profile: dict[str, Any], args: argparse.Namespace) -> int:
    platform_routing = profile.get("platform_routing")
    if platform_routing:
        selected_platform = platform_routing.get("selected_platform")
        if platform_routing.get("decision") != "selected":
            print_text(profile)
            print("validation_manager: refusing --run because the requested validation platform is not eligible", file=sys.stderr)
            return 1
        if selected_platform != "local":
            print_text(profile)
            print("validation_manager: refusing --run because platform routing is dry-run only for non-local platforms", file=sys.stderr)
            return 1
    remote_runner = profile.get("remote_runner")
    if remote_runner and remote_runner.get("decision") != "selected":
        print_text(profile)
        print("validation_manager: refusing --run because the requested remote runner is not eligible", file=sys.stderr)
        return 1
    if profile["status"] not in {"ready_to_run", "no_validation_needed"}:
        print_text(profile)
        print("validation_manager: refusing --run for non-runnable profile", file=sys.stderr)
        return 1
    failed = False
    packets: list[dict[str, Any]] = []
    log_root = build_action_log_root(args)
    for index, item in enumerate(profile["run"], start=1):
        print(f"==> {item['lane_id']}: {item['command']}", file=sys.stderr)
        returncode, packet = run_validation_action_with_log(item, log_root=log_root, index=index, json_mode=args.json)
        packets.append(packet)
        item["run_status"] = packet["status"]
        item["build_action_log"] = packet["packet_ref"]
        if returncode != 0:
            failed = True
    manifest_path = write_build_action_manifest(log_root, packets)
    profile["build_action_logs"] = {
        "schema_version": BUILD_ACTION_LOG_MANIFEST_SCHEMA,
        "manifest_ref": repo_relative(manifest_path),
        "packet_count": len(packets),
        "packets": [packet["packet_ref"] for packet in packets],
    }
    profile["run_status"] = "failed" if failed else "passed"
    return 1 if failed else 0


def parse_args(argv: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    source = parser.add_mutually_exclusive_group()
    source.add_argument("--changed-files", type=Path)
    source.add_argument("--include-working-tree", action="store_true")
    parser.add_argument("--manifest", type=Path, default=DEFAULT_MANIFEST)
    parser.add_argument("--base", default="origin/main")
    parser.add_argument("--head", default="HEAD")
    parser.add_argument("--max-selected-lanes", type=int, default=8)
    parser.add_argument("--json", action="store_true")
    parser.add_argument("--report-out", type=Path)
    parser.add_argument("--run", action="store_true")
    parser.add_argument("--build-action-log-dir", type=Path)
    parser.add_argument("--remote-runner", choices=["nessus"])
    parser.add_argument("--remote-command")
    parser.add_argument("--remote-artifact-dir", type=Path)
    parser.add_argument("--platform-routing", action="store_true")
    parser.add_argument("--validation-platform", choices=VALIDATION_PLATFORMS)
    return parser.parse_args(argv)


def main(argv: list[str]) -> int:
    args = parse_args(argv)
    plan = selector_plan(args)
    manifest_path = args.manifest.resolve() if args.manifest else DEFAULT_MANIFEST
    guardrails = manager_guardrails(load_manifest(manifest_path), args.max_selected_lanes)
    profile = build_profile(plan, guardrails, manifest_path)
    platform_routing = platform_routing_decision(profile, args)
    if platform_routing:
        profile["platform_routing"] = platform_routing
    remote = remote_runner_decision(profile, args)
    if remote:
        profile["remote_runner"] = remote
        if remote["decision"] == "selected":
            profile["run"] = [
                {
                    "lane_id": "nessus_remote_validation",
                    "command": remote["command"],
                    "reason": remote["reason"],
                    "matched_paths": profile["changed_paths"],
                    "vpp_record": None,
                    "local_run": profile["run"],
                }
            ]
    exit_code = 0
    if args.run:
        exit_code = run_profile(profile, args)
    if args.report_out:
        write_report(args.report_out, profile)
    if args.json:
        print(json.dumps(profile, indent=2, sort_keys=True))
    else:
        print_text(profile)
    return exit_code


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
