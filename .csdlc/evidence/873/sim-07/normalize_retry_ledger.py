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


def classify_outcome(attempt: dict[str, Any]) -> tuple[str, str, bool, str | None]:
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
    if reason in EXPECTED_GUARD_REASONS:
        outcome = "expected_guard_denial"
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
            if argv != step["argv"][variant]:
                raise NormalizationError(
                    f"{scenario_id} step {step['id']} argv mapping drift"
                )
            outcome, reason, envelope_present, correlation_id = classify_outcome(raw_attempt)
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
