#!/usr/bin/env python3
"""Normalize retained installed-command attempts into one strict retry ledger."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
from typing import Any

import validate_retry_comparison as contract


RAW_SCHEMA = "csdlc.v3.installed_intent_attempts.v1"
MAX_JSON_BYTES = 16 * 1024 * 1024
MAX_RAW_ATTEMPTS = 256
SCENARIO_FILES = {
    "primary-linked-edit": "primary_observation",
    "terminal-journey": "terminal_observation",
}
EXPECTED_GUARD_REASONS = {
    "intent_preview_argument_not_supported",
    "intent_merge_internal_inputs_denied",
    "cleanup_archive_foreign_or_tracked_dirty",
    "intent_cleanup_preview_stale",
}


class NormalizationError(ValueError):
    pass


def read_json(path: Path) -> Any:
    if path.stat().st_size > MAX_JSON_BYTES:
        raise NormalizationError(f"JSON input exceeds {MAX_JSON_BYTES} bytes: {path}")
    return json.loads(path.read_text(encoding="utf-8"))


def sha256_bytes(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def write_atomic(path: Path, value: Any) -> None:
    rendered = json.dumps(value, indent=2, sort_keys=True) + "\n"
    staged = path.with_name(f".{path.name}.next")
    staged.write_text(rendered, encoding="utf-8")
    os.replace(staged, path)


def result_field(attempt: dict[str, Any], key: str) -> Any:
    result = attempt.get("result")
    if not isinstance(result, dict):
        return None
    envelope = result.get("envelope")
    if isinstance(envelope, dict) and envelope.get(key) is not None:
        return envelope[key]
    return result.get(key)


def finding_messages(attempt: dict[str, Any]) -> list[str]:
    result = attempt.get("result")
    if not isinstance(result, dict):
        return []
    findings = result.get("findings")
    if not isinstance(findings, list):
        envelope = result.get("envelope")
        findings = envelope.get("findings") if isinstance(envelope, dict) else None
    if not isinstance(findings, list):
        return []
    return [
        finding.get("message")
        for finding in findings
        if isinstance(finding, dict) and isinstance(finding.get("message"), str)
    ]


def candidate_guard_is_exact(attempt: dict[str, Any]) -> bool:
    result = attempt.get("result")
    envelope = result.get("envelope") if isinstance(result, dict) else None
    effects = envelope.get("effects") if isinstance(envelope, dict) else None
    guard = attempt.get("guard_invariants")
    return (
        attempt.get("exit_code") == 2
        and isinstance(result, dict)
        and result.get("performed_mutation") in {None, False}
        and result.get("native_effect_truth") not in {"performed", "created", "updated"}
        and isinstance(effects, dict)
        and effects.get("outcome") == "unknown"
        and isinstance(guard, dict)
        and guard.get("equal") is True
        and guard.get("before_sha256") == guard.get("after_sha256")
        and guard.get("exit_code") == 2
        and guard.get("effects_outcome") == "unknown"
        and guard.get("performed_mutation") in {None, False}
    )


def classify_outcome(
    attempt: dict[str, Any], variant: str, step_id: str
) -> tuple[str, str, bool, str | None]:
    result = attempt.get("result")
    if result is None:
        return "interrupted", "process_interrupted_without_result", False, None
    if not isinstance(result, dict):
        raise NormalizationError("attempt result must be an object or null")
    reason = result_field(attempt, "reason_code")
    status = result_field(attempt, "status")
    envelope = result.get("envelope")
    correlation_id = None
    if isinstance(envelope, dict):
        correlation_id = envelope.get("correlation_id")
    if correlation_id is None:
        correlation_id = result.get("correlation_id")
    if not isinstance(reason, str) or not reason:
        raise NormalizationError("retained result is missing reason_code")
    if not isinstance(correlation_id, str) or not correlation_id:
        raise NormalizationError("retained result is missing correlation_id")
    if (
        variant == "predecessor"
        and step_id == "merge-internal-input-guard"
        and reason == "github_merge_ineligible"
        and finding_messages(attempt)
        == ["explicit operator merge authorization reference required"]
    ):
        outcome = "expected_guard_denial"
    elif (
        variant == "predecessor"
        and step_id in {"cleanup-foreign-dirty-guard", "cleanup-tracked-dirty-guard"}
        and reason == "command_completed"
        and result.get("performed_mutation") is False
        and isinstance(result.get("result"), dict)
        and isinstance(result["result"].get("cleanup"), dict)
        and result["result"]["cleanup"].get("decision") == "dirty"
        and isinstance(envelope, dict)
        and isinstance(envelope.get("effects"), dict)
        and envelope["effects"].get("outcome") == "none"
        and finding_messages(attempt) == []
    ):
        outcome = "expected_guard_denial"
    elif (
        variant == "predecessor"
        and step_id == "cleanup-stale-preview-guard"
        and reason == "preview_receipt_mismatch"
        and result.get("performed_mutation") is False
        and isinstance(envelope, dict)
        and isinstance(envelope.get("effects"), dict)
        and envelope["effects"].get("outcome") == "none"
        and finding_messages(attempt)
        == ["cleanup removal requires a preview receipt for the same Git registration"]
    ):
        outcome = "expected_guard_denial"
    elif reason in EXPECTED_GUARD_REASONS and (
        variant != "candidate" or candidate_guard_is_exact(attempt)
    ):
        outcome = "expected_guard_denial"
    elif (
        variant == "predecessor"
        and step_id in {"prepare-preview-guard", "linked-edit-preview-guard"}
        and reason == "usage"
        and any(
            message.endswith("; unexpected argument --preview")
            for message in finding_messages(attempt)
        )
    ):
        outcome = "expected_guard_denial"
    elif (
        variant == "predecessor"
        and step_id in {"status-bound-primary", "validate-bound-primary", "primary-edit"}
        and reason == "issue_already_bound"
        and finding_messages(attempt)
        == ["the bound checkout owns this issue; primary preparation and edits are denied"]
    ):
        outcome = "invalid_intent"
    elif (
        variant == "predecessor"
        and step_id == "review"
        and reason == "command_completed"
        and status == "ready"
        and result.get("read_only") is True
        and isinstance(envelope, dict)
        and isinstance(envelope.get("effects"), dict)
        and envelope["effects"].get("outcome") == "none"
        and finding_messages(attempt) == []
    ):
        outcome = "successful_no_op"
    elif reason == "lifecycle_digest_valid":
        outcome = "successful_no_op"
    elif reason == "cleanup_archive_recovery_required":
        outcome = "recovery_required"
    elif reason == "command_failed" and status == "recovery_required":
        outcome = "interrupted"
    elif reason == "command_completed":
        effect = None
        if isinstance(envelope, dict):
            effects = envelope.get("effects")
            if isinstance(effects, dict):
                effect = effects.get("outcome")
        if effect is None:
            effect = result.get("native_effect_truth")
        if effect in {"performed", "created", "updated"}:
            outcome = "transition_applied"
        elif status in {"ready", "blocked"}:
            outcome = "expected_wait"
        else:
            outcome = "successful_no_op"
    elif reason.startswith("intent_"):
        outcome = "invalid_intent"
    else:
        outcome = "unexpected_fault"
    return outcome, reason, True, correlation_id


def canonicalize_argv(argv: Any, variant: str) -> list[str]:
    if not isinstance(argv, list) or not argv or any(not isinstance(v, str) for v in argv):
        raise NormalizationError("attempt argv must be a nonempty string array")
    candidate_root = "$FIXTURE_ROOT/.git/installed-candidate/"
    variant_root = f"$FIXTURE_ROOT/.git/installed-{variant}/"
    return [token.replace(candidate_root, variant_root) for token in argv]


def derived_retry_class(policy: str, previous_outcome: str) -> str:
    if policy == "eligible" and previous_outcome in {"invalid_intent", "unexpected_fault"}:
        return "avoidable"
    if policy in {"eligible", "useful_only"} and previous_outcome in {
        "expected_wait",
        "recovery_required",
        "interrupted",
    }:
        return "useful"
    raise NormalizationError(
        f"retry is not admitted by {policy} after {previous_outcome}"
    )


def validate_raw(
    value: Any, scenario_id: str, expected_blake3: str
) -> list[dict[str, Any]]:
    if not isinstance(value, dict) or value.get("schema") != RAW_SCHEMA:
        raise NormalizationError(f"{scenario_id} raw observation schema is invalid")
    attempts = value.get("attempts")
    if not isinstance(attempts, list) or not attempts:
        raise NormalizationError(f"{scenario_id} raw attempts must be nonempty")
    if len(attempts) > MAX_RAW_ATTEMPTS:
        raise NormalizationError(
            f"{scenario_id} raw attempts exceed {MAX_RAW_ATTEMPTS}"
        )
    if value.get("attempted") != len(attempts):
        raise NormalizationError(f"{scenario_id} raw attempt denominator differs")
    provenance = value.get("provenance")
    if not isinstance(provenance, dict):
        raise NormalizationError(f"{scenario_id} provenance is missing")
    if provenance.get("installed_binary_blake3") != expected_blake3:
        raise NormalizationError(f"{scenario_id} installed binary provenance mismatch")
    return attempts


def validate_raw_issue_identity(
    attempts: list[dict[str, Any]], variant: str, scenario_id: str, expected_issue: int
) -> None:
    def input_issue(value: Any) -> Any | None:
        if not isinstance(value, dict):
            return None
        if isinstance(value.get("issue"), int):
            return value["issue"]
        operation = value.get("operation")
        if not isinstance(operation, dict):
            return None
        request = operation.get("request")
        if isinstance(request, dict) and isinstance(request.get("issue"), int):
            return request["issue"]
        return None

    for index, attempt in enumerate(attempts):
        where = f"{scenario_id} raw attempt[{index}]"
        input_value = attempt.get("input")
        declared_input_issue = input_issue(input_value)
        if variant == "predecessor":
            if declared_input_issue != expected_issue:
                raise NormalizationError(f"{where} input issue differs from relevant facts")
        elif declared_input_issue is not None and declared_input_issue != expected_issue:
            raise NormalizationError(f"{where} input issue differs from relevant facts")
        result = attempt.get("result")
        if not isinstance(result, dict):
            continue
        identities: list[Any] = []
        if "request_issue" in result:
            identities.append(result["request_issue"])
        owner_result = result.get("result")
        if isinstance(owner_result, dict) and "issue" in owner_result:
            identities.append(owner_result["issue"])
        envelope = result.get("envelope")
        if isinstance(envelope, dict):
            envelope_issue = envelope.get("issue")
            if isinstance(envelope_issue, dict) and "number" in envelope_issue:
                identities.append(envelope_issue["number"])
        if any(identity != expected_issue for identity in identities):
            raise NormalizationError(f"{where} result issue differs from relevant facts")


def validate_raw_cleanup_topology(
    raw: dict[str, Any], attempts: list[dict[str, Any]], variant: str, scenario_id: str
) -> None:
    if scenario_id != "terminal-journey":
        return
    topology = raw.get("cleanup_topology")
    if not isinstance(topology, dict) or topology.get("schema") != "csdlc.v3.issue873.cleanup_control_topology.v1":
        raise NormalizationError("terminal-journey cleanup topology is missing")
    try:
        primary = Path(topology["primary_root"])
        active = Path(topology["active_issue_worktree"])
        control = Path(topology["cleanup_control_worktree"])
    except (KeyError, TypeError) as error:
        raise NormalizationError("terminal-journey cleanup topology paths are invalid") from error
    if (
        not all(path.is_absolute() for path in (primary, active, control))
        or len({primary, active, control}) != 3
        or active.parent != control.parent
        or topology.get("active_issue_retained") is not True
        or topology.get("baseline_head") != topology.get("control_head")
        or not isinstance(topology.get("tracked_inventory_sha256"), str)
    ):
        raise NormalizationError("terminal-journey cleanup topology differs from the common facts")
    if variant == "predecessor":
        topology_valid = (
            control.name == "cleanup-control"
            and topology.get("control_status_porcelain") == ""
        )
    else:
        topology_valid = (
            active.name == "cleanup-control"
            and bool(topology.get("active_status_porcelain"))
            and bool(topology.get("control_status_porcelain"))
        )
    if not topology_valid:
        raise NormalizationError("terminal-journey cleanup topology differs from the variant contract")
    registered = topology.get("registered_paths")
    if not isinstance(registered, list) or not {str(primary), str(active), str(control)}.issubset(set(registered)):
        raise NormalizationError("terminal-journey cleanup topology registration is incomplete")
    cleanup_attempts = attempts[-6:]
    if len(cleanup_attempts) != 6:
        raise NormalizationError("terminal-journey cleanup attempt denominator differs")
    for offset, attempt in enumerate(cleanup_attempts):
        value = attempt.get("input")
        if not isinstance(value, dict):
            raise NormalizationError("terminal-journey cleanup input is missing")
        if variant == "predecessor":
            cleanup = value.get("cleanup")
            candidate = cleanup.get("candidate_path") if isinstance(cleanup, dict) else None
        else:
            candidate = value.get("cleanup_candidate")
            if value.get("active_issue_worktree") != str(active):
                raise NormalizationError("candidate cleanup active worktree identity differs")
        if candidate != str(control):
            raise NormalizationError("terminal-journey cleanup target differs from control topology")
        result = attempt.get("result")
        owner = result.get("result") if isinstance(result, dict) else None
        cleanup_result = owner.get("cleanup") if isinstance(owner, dict) else None
        if isinstance(cleanup_result, dict) and cleanup_result.get("path") not in {None, str(control)}:
            raise NormalizationError("terminal-journey cleanup result path differs from control topology")
        if variant == "candidate" and offset in {3, 5}:
            executed = attempt.get("executed_argv")
            if not isinstance(executed, list) or executed[-1:] != [value.get("preview_receipt_digest")]:
                raise NormalizationError("candidate cleanup preview operand was not executed exactly")


def normalize(
    scenario_map: dict[str, Any],
    variant: str,
    observations: dict[str, tuple[Path, Any]],
    binary_sha256: str,
    observation_base: Path | None = None,
) -> dict[str, Any]:
    if variant not in {"predecessor", "candidate"}:
        raise NormalizationError("variant must be predecessor or candidate")
    binary = scenario_map["binaries"][variant]
    if binary_sha256 != binary["sha256"]:
        raise NormalizationError(f"{variant} binary sha256 mismatch")
    ledger_attempts: list[dict[str, Any]] = []
    journeys: list[dict[str, str]] = []
    retained_observations: list[dict[str, str]] = []
    scenario_by_id = {scenario["id"]: scenario for scenario in scenario_map["scenarios"]}
    if set(observations) != set(scenario_by_id):
        raise NormalizationError("observation scenarios differ from the scenario map")
    for scenario in scenario_map["scenarios"]:
        scenario_id = scenario["id"]
        path, raw = observations[scenario_id]
        raw_attempts = validate_raw(raw, scenario_id, binary["blake3"])
        fixture_facts = scenario["relevant_facts"]["initial_fixture_facts"]
        expected_issue = fixture_facts.get("issue_number")
        if not isinstance(expected_issue, int):
            raise ValueError(f"{scenario_id}: shared issue number is missing")
        validate_raw_issue_identity(raw_attempts, variant, scenario_id, expected_issue)
        steps = scenario["semantic_steps"]
        step_index = 0
        active_attempt_id: str | None = None
        active_outcome: str | None = None
        for raw_index, raw_attempt in enumerate(raw_attempts):
            if step_index >= len(steps):
                raise NormalizationError(f"{scenario_id} has an attempt after all mapped steps")
            if not isinstance(raw_attempt, dict):
                raise NormalizationError(f"{scenario_id} raw attempt[{raw_index}] must be an object")
            step = steps[step_index]
            argv = canonicalize_argv(raw_attempt.get("argv"), variant)
            expected_argv = step["argv"][variant]
            if active_attempt_id is not None and step.get("retry_argv") is not None:
                expected_argv = step["retry_argv"][variant]
            if argv != expected_argv:
                raise NormalizationError(
                    f"{scenario_id} step {step['id']} argv mapping drift"
                )
            outcome, reason, envelope_present, correlation_id = classify_outcome(
                raw_attempt, variant, step["id"]
            )
            if outcome not in step["expected_outcomes"]:
                raise NormalizationError(
                    f"{scenario_id} step {step['id']} outcome {outcome} is not declared"
                )
            raw_attempt_id = raw_attempt.get("attempt_id")
            if not isinstance(raw_attempt_id, str) or not raw_attempt_id:
                raise NormalizationError(f"{scenario_id} raw attempt id is missing")
            attempt_id = f"{variant}:{scenario_id}:{raw_attempt_id}"
            if active_attempt_id is None:
                retry_of = None
                retry_class = "not_retry"
            else:
                retry_of = active_attempt_id
                assert active_outcome is not None
                retry_class = derived_retry_class(step["retry_policy"], active_outcome)
            ledger_attempts.append(
                {
                    "attempt_id": attempt_id,
                    "scenario_id": scenario_id,
                    "step_id": step["id"],
                    "relevant_facts_sha256": scenario["relevant_facts_sha256"],
                    "intended_operation": step["intended_operation"],
                    "argv": argv,
                    "outcome_class": outcome,
                    "reason_code": reason,
                    "result_envelope_present": envelope_present,
                    "correlation_id": correlation_id,
                    "retry_of": retry_of,
                    "retry_class": retry_class,
                }
            )
            if outcome in step["completion_outcomes"]:
                step_index += 1
                active_attempt_id = None
                active_outcome = None
            else:
                active_attempt_id = attempt_id
                active_outcome = outcome
        if step_index != len(steps):
            missing = [step["id"] for step in steps[step_index:]]
            raise NormalizationError(f"{scenario_id} missing mapped steps: {missing}")
        validate_raw_cleanup_topology(raw, raw_attempts, variant, scenario_id)
        journeys.append({"scenario_id": scenario_id, "status": "completed"})
        if observation_base is None:
            retained_path = path.as_posix()
        else:
            try:
                retained_path = path.resolve().relative_to(observation_base.resolve()).as_posix()
            except ValueError as error:
                raise NormalizationError(
                    f"{scenario_id} observation is outside the run directory"
                ) from error
        retained_observations.append(
            {
                "scenario_id": scenario_id,
                "path": retained_path,
                "sha256": sha256_bytes(path),
            }
        )
    return {
        "schema": contract.LEDGER_SCHEMA,
        "corpus_id": scenario_map["corpus_id"],
        "scenario_map_sha256": "",
        "variant": variant,
        "binary": binary,
        "observations": retained_observations,
        "journeys": journeys,
        "attempts": ledger_attempts,
    }


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser()
    parser.add_argument("--scenario-map", required=True, type=Path)
    parser.add_argument("--variant", required=True, choices=("predecessor", "candidate"))
    parser.add_argument("--primary-observation", required=True, type=Path)
    parser.add_argument("--terminal-observation", required=True, type=Path)
    parser.add_argument("--binary-sha256", required=True)
    parser.add_argument("--output", required=True, type=Path)
    return parser


def main() -> int:
    args = build_parser().parse_args()
    try:
        scenario_map = contract.validate_map(read_json(args.scenario_map))
        observations = {
            "primary-linked-edit": (
                args.primary_observation,
                read_json(args.primary_observation),
            ),
            "terminal-journey": (
                args.terminal_observation,
                read_json(args.terminal_observation),
            ),
        }
        ledger = normalize(
            scenario_map,
            args.variant,
            observations,
            args.binary_sha256,
            args.output.parent,
        )
        ledger["scenario_map_sha256"] = sha256_bytes(args.scenario_map)
        write_atomic(args.output, ledger)
        contract.validate_ledger(
            ledger,
            args.variant,
            scenario_map,
            ledger["scenario_map_sha256"],
            args.output.parent,
        )
    except (OSError, json.JSONDecodeError, contract.ValidationError, NormalizationError) as error:
        args.output.unlink(missing_ok=True)
        print(json.dumps({"status": "invalid", "finding": str(error)}, sort_keys=True))
        return 2
    print(json.dumps({"status": "completed", "variant": args.variant, "attempts": len(ledger["attempts"])}))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
