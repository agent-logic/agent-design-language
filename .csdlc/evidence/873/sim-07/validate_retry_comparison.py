#!/usr/bin/env python3
"""Validate and adjudicate one SIM-07 same-corpus retry comparison."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
from typing import Any


MAP_SCHEMA = "adl.csdlc.issue873.retry_scenario_map.v1"
LEDGER_SCHEMA = "adl.csdlc.issue873.retry_ledger.v1"
OUTPUT_SCHEMA = "adl.csdlc.issue873.retry_comparison.v1"
OUTCOMES = {
    "transition_applied",
    "successful_no_op",
    "expected_wait",
    "invalid_intent",
    "expected_guard_denial",
    "unexpected_fault",
    "recovery_required",
    "interrupted",
}
RETRY_CLASSES = {"not_retry", "avoidable", "useful"}
POLICIES = {
    "eligible",
    "useful_only",
    "excluded_expected_guard",
    "excluded_injected_interruption",
    "excluded_required_observation",
}


class ValidationError(ValueError):
    pass


def read_json(path: Path) -> Any:
    return json.loads(path.read_text(encoding="utf-8"))


def sha256_bytes(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def write_atomic(path: Path, rendered: str) -> None:
    staged = path.with_name(f".{path.name}.next")
    staged.write_text(rendered, encoding="utf-8")
    os.replace(staged, path)


def require_exact_keys(value: dict[str, Any], expected: set[str], where: str) -> None:
    actual = set(value)
    if actual != expected:
        raise ValidationError(
            f"{where} keys differ: missing={sorted(expected - actual)} "
            f"unexpected={sorted(actual - expected)}"
        )


def require_hex(value: Any, where: str) -> str:
    if not isinstance(value, str) or len(value) != 64:
        raise ValidationError(f"{where} must be a 64-character lowercase hex digest")
    if any(character not in "0123456789abcdef" for character in value):
        raise ValidationError(f"{where} must be a 64-character lowercase hex digest")
    return value


def validate_map(value: Any) -> dict[str, Any]:
    if not isinstance(value, dict):
        raise ValidationError("scenario map must be an object")
    require_exact_keys(value, {"schema", "corpus_id", "binaries", "scenarios"}, "scenario map")
    if value["schema"] != MAP_SCHEMA:
        raise ValidationError("unsupported scenario-map schema")
    if not isinstance(value["corpus_id"], str) or not value["corpus_id"]:
        raise ValidationError("scenario map corpus_id must be nonempty")
    binaries = value["binaries"]
    if not isinstance(binaries, dict):
        raise ValidationError("scenario map binaries must be an object")
    require_exact_keys(binaries, {"predecessor", "candidate"}, "scenario map binaries")
    for variant in ("predecessor", "candidate"):
        binary = binaries[variant]
        if not isinstance(binary, dict):
            raise ValidationError(f"{variant} binary must be an object")
        require_exact_keys(binary, {"revision", "sha256", "blake3"}, f"{variant} binary")
        if not isinstance(binary["revision"], str) or not binary["revision"]:
            raise ValidationError(f"{variant} revision must be nonempty")
        require_hex(binary["sha256"], f"{variant} sha256")
        require_hex(binary["blake3"], f"{variant} blake3")
    scenarios = value["scenarios"]
    if not isinstance(scenarios, list) or not scenarios:
        raise ValidationError("scenario map must contain scenarios")
    scenario_ids: set[str] = set()
    for scenario_index, scenario in enumerate(scenarios):
        where = f"scenario[{scenario_index}]"
        if not isinstance(scenario, dict):
            raise ValidationError(f"{where} must be an object")
        require_exact_keys(
            scenario,
            {"id", "relevant_facts", "relevant_facts_sha256", "semantic_steps"},
            where,
        )
        scenario_id = scenario["id"]
        if not isinstance(scenario_id, str) or not scenario_id:
            raise ValidationError(f"{where}.id must be nonempty")
        if scenario_id in scenario_ids:
            raise ValidationError(f"duplicate scenario id: {scenario_id}")
        scenario_ids.add(scenario_id)
        if not isinstance(scenario["relevant_facts"], dict) or not scenario["relevant_facts"]:
            raise ValidationError(f"{where}.relevant_facts must be a nonempty object")
        declared_facts_digest = require_hex(
            scenario["relevant_facts_sha256"], f"{where}.relevant_facts_sha256"
        )
        actual_facts_digest = hashlib.sha256(
            json.dumps(
                scenario["relevant_facts"], sort_keys=True, separators=(",", ":")
            ).encode("utf-8")
        ).hexdigest()
        if declared_facts_digest != actual_facts_digest:
            raise ValidationError(f"{where}.relevant_facts_sha256 does not bind relevant_facts")
        steps = scenario["semantic_steps"]
        if not isinstance(steps, list) or not steps:
            raise ValidationError(f"{where}.semantic_steps must be nonempty")
        step_ids: set[str] = set()
        for step_index, step in enumerate(steps):
            step_where = f"{where}.semantic_steps[{step_index}]"
            if not isinstance(step, dict):
                raise ValidationError(f"{step_where} must be an object")
            require_exact_keys(
                step,
                {
                    "id",
                    "retry_policy",
                    "intended_operation",
                    "expected_outcomes",
                    "completion_outcomes",
                    "argv",
                },
                step_where,
            )
            step_id = step["id"]
            if not isinstance(step_id, str) or not step_id:
                raise ValidationError(f"{step_where}.id must be nonempty")
            if step_id in step_ids:
                raise ValidationError(f"duplicate semantic step in {scenario_id}: {step_id}")
            step_ids.add(step_id)
            if step["retry_policy"] not in POLICIES:
                raise ValidationError(f"unsupported retry policy at {step_where}")
            if not isinstance(step["intended_operation"], str) or not step["intended_operation"]:
                raise ValidationError(f"{step_where}.intended_operation must be nonempty")
            expected_outcomes = step["expected_outcomes"]
            if (
                not isinstance(expected_outcomes, list)
                or not expected_outcomes
                or any(outcome not in OUTCOMES for outcome in expected_outcomes)
                or len(set(expected_outcomes)) != len(expected_outcomes)
            ):
                raise ValidationError(f"{step_where}.expected_outcomes are invalid")
            completion_outcomes = step["completion_outcomes"]
            if (
                not isinstance(completion_outcomes, list)
                or not completion_outcomes
                or any(outcome not in expected_outcomes for outcome in completion_outcomes)
                or len(set(completion_outcomes)) != len(completion_outcomes)
            ):
                raise ValidationError(f"{step_where}.completion_outcomes are invalid")
            argv = step["argv"]
            if not isinstance(argv, dict):
                raise ValidationError(f"{step_where}.argv must be an object")
            require_exact_keys(argv, {"predecessor", "candidate"}, f"{step_where}.argv")
            for variant in ("predecessor", "candidate"):
                tokens = argv[variant]
                if (
                    not isinstance(tokens, list)
                    or not tokens
                    or any(not isinstance(token, str) or not token for token in tokens)
                ):
                    raise ValidationError(f"{step_where}.argv.{variant} must be nonempty tokens")
    return value


def validate_ledger(
    value: Any,
    variant: str,
    scenario_map: dict[str, Any],
    map_sha256: str,
) -> dict[str, Any]:
    if not isinstance(value, dict):
        raise ValidationError(f"{variant} ledger must be an object")
    require_exact_keys(
        value,
        {
            "schema",
            "corpus_id",
            "scenario_map_sha256",
            "variant",
            "binary",
            "journeys",
            "attempts",
        },
        f"{variant} ledger",
    )
    if value["schema"] != LEDGER_SCHEMA:
        raise ValidationError(f"unsupported {variant} ledger schema")
    if value["variant"] != variant:
        raise ValidationError(f"ledger variant mismatch: expected {variant}")
    if value["corpus_id"] != scenario_map["corpus_id"]:
        raise ValidationError(f"{variant} corpus mismatch")
    if value["scenario_map_sha256"] != map_sha256:
        raise ValidationError(f"{variant} scenario-map digest mismatch")
    binary = value["binary"]
    if not isinstance(binary, dict):
        raise ValidationError(f"{variant} binary must be an object")
    require_exact_keys(binary, {"revision", "sha256", "blake3"}, f"{variant} ledger binary")
    if binary != scenario_map["binaries"][variant]:
        raise ValidationError(f"{variant} binary provenance mismatch")

    scenario_by_id = {scenario["id"]: scenario for scenario in scenario_map["scenarios"]}
    step_by_key = {
        (scenario["id"], step["id"]): step
        for scenario in scenario_map["scenarios"]
        for step in scenario["semantic_steps"]
    }
    required_steps = set(step_by_key)

    journeys = value["journeys"]
    if not isinstance(journeys, list):
        raise ValidationError(f"{variant} journeys must be an array")
    journey_status: dict[str, str] = {}
    for index, journey in enumerate(journeys):
        if not isinstance(journey, dict):
            raise ValidationError(f"{variant} journey[{index}] must be an object")
        require_exact_keys(journey, {"scenario_id", "status"}, f"{variant} journey[{index}]")
        scenario_id = journey["scenario_id"]
        if scenario_id not in scenario_by_id:
            raise ValidationError(f"unknown {variant} journey scenario: {scenario_id}")
        if scenario_id in journey_status:
            raise ValidationError(f"duplicate {variant} journey scenario: {scenario_id}")
        if journey["status"] not in {"completed", "failed", "not_proven"}:
            raise ValidationError(f"invalid {variant} journey status")
        journey_status[scenario_id] = journey["status"]
    if set(journey_status) != set(scenario_by_id):
        raise ValidationError(f"{variant} journey denominator differs from the scenario map")

    attempts = value["attempts"]
    if not isinstance(attempts, list) or not attempts:
        raise ValidationError(f"{variant} attempts must be nonempty")
    attempt_by_id: dict[str, dict[str, Any]] = {}
    final_attempt_by_step: dict[tuple[str, str], dict[str, Any]] = {}
    observed_steps: set[tuple[str, str]] = set()
    next_step_index = {scenario_id: 0 for scenario_id in scenario_by_id}
    current_step = {scenario_id: None for scenario_id in scenario_by_id}
    for index, attempt in enumerate(attempts):
        where = f"{variant} attempt[{index}]"
        if not isinstance(attempt, dict):
            raise ValidationError(f"{where} must be an object")
        require_exact_keys(
            attempt,
            {
                "attempt_id",
                "scenario_id",
                "step_id",
                "relevant_facts_sha256",
                "intended_operation",
                "argv",
                "outcome_class",
                "reason_code",
                "result_envelope_present",
                "correlation_id",
                "retry_of",
                "retry_class",
            },
            where,
        )
        attempt_id = attempt["attempt_id"]
        if not isinstance(attempt_id, str) or not attempt_id:
            raise ValidationError(f"{where}.attempt_id must be nonempty")
        if attempt_id in attempt_by_id:
            raise ValidationError(f"duplicate {variant} attempt id: {attempt_id}")
        key = (attempt["scenario_id"], attempt["step_id"])
        if key not in step_by_key:
            raise ValidationError(f"unknown {variant} semantic step: {key}")
        scenario_id, step_id = key
        if key not in observed_steps:
            declared_steps = scenario_by_id[scenario_id]["semantic_steps"]
            declared_index = next_step_index[scenario_id]
            expected_step_id = declared_steps[declared_index]["id"]
            if step_id != expected_step_id:
                raise ValidationError(
                    f"{where} step order differs: expected {expected_step_id}, observed {step_id}"
                )
            next_step_index[scenario_id] += 1
            current_step[scenario_id] = step_id
        elif current_step[scenario_id] != step_id:
            raise ValidationError(f"{where} returns to a noncontiguous semantic step")
        observed_steps.add(key)
        scenario = scenario_by_id[attempt["scenario_id"]]
        step = step_by_key[key]
        if attempt["relevant_facts_sha256"] != scenario["relevant_facts_sha256"]:
            raise ValidationError(f"{where} relevant facts differ from the scenario map")
        if attempt["outcome_class"] not in OUTCOMES:
            raise ValidationError(f"{where} has unsupported outcome_class")
        if attempt["outcome_class"] not in step["expected_outcomes"]:
            raise ValidationError(f"{where} outcome differs from the scenario map")
        if attempt["intended_operation"] != step["intended_operation"]:
            raise ValidationError(f"{where} intended operation differs from the scenario map")
        if attempt["argv"] != step["argv"][variant]:
            raise ValidationError(f"{where} argv differs from the scenario map")
        if not isinstance(attempt["reason_code"], str) or not attempt["reason_code"]:
            raise ValidationError(f"{where}.reason_code must be nonempty")
        if not isinstance(attempt["result_envelope_present"], bool):
            raise ValidationError(f"{where}.result_envelope_present must be boolean")
        if attempt["result_envelope_present"]:
            if not isinstance(attempt["correlation_id"], str) or not attempt["correlation_id"]:
                raise ValidationError(f"{where} result envelope requires correlation_id")
        elif attempt["outcome_class"] != "interrupted" or attempt["correlation_id"] is not None:
            raise ValidationError(f"{where} missing result envelope is allowed only for interruption")
        if attempt["retry_class"] not in RETRY_CLASSES:
            raise ValidationError(f"{where} has unsupported retry_class")
        retry_of = attempt["retry_of"]
        prior_same_step = next(
            (
                prior
                for prior in reversed(list(attempt_by_id.values()))
                if (prior["scenario_id"], prior["step_id"]) == key
            ),
            None,
        )
        if retry_of is None:
            if attempt["retry_class"] != "not_retry":
                raise ValidationError(f"{where} classifies a first attempt as a retry")
            if prior_same_step is not None:
                raise ValidationError(f"{where} repeats a semantic step without retry_of")
        else:
            if not isinstance(retry_of, str) or retry_of not in attempt_by_id:
                raise ValidationError(f"{where}.retry_of must name an earlier attempt")
            previous = attempt_by_id[retry_of]
            if (previous["scenario_id"], previous["step_id"]) != key:
                raise ValidationError(f"{where} retries a different semantic step")
            if previous["relevant_facts_sha256"] != attempt["relevant_facts_sha256"]:
                raise ValidationError(f"{where} retry changed relevant facts")
            if prior_same_step is not previous:
                raise ValidationError(f"{where}.retry_of must name the immediately preceding same-step attempt")
            policy = step["retry_policy"]
            previous_outcome = previous["outcome_class"]
            if policy == "eligible" and previous_outcome in {"invalid_intent", "unexpected_fault"}:
                expected_retry_class = "avoidable"
            elif policy in {"eligible", "useful_only"} and previous_outcome in {
                "expected_wait",
                "recovery_required",
                "interrupted",
            }:
                expected_retry_class = "useful"
            else:
                raise ValidationError(
                    f"{where} retry is not admitted by {policy} after {previous_outcome}"
                )
            if attempt["retry_class"] != expected_retry_class:
                raise ValidationError(
                    f"{where} retry_class must be derived as {expected_retry_class}"
                )
        attempt_by_id[attempt_id] = attempt
        final_attempt_by_step[key] = attempt
    if observed_steps != required_steps:
        missing = sorted(required_steps - observed_steps)
        extra = sorted(observed_steps - required_steps)
        raise ValidationError(f"{variant} semantic-step denominator differs: missing={missing} extra={extra}")
    for scenario_id, scenario in scenario_by_id.items():
        completed = all(
            final_attempt_by_step[(scenario_id, step["id"])]["outcome_class"]
            in step["completion_outcomes"]
            for step in scenario["semantic_steps"]
        )
        derived_status = "completed" if completed else "failed"
        if journey_status[scenario_id] != derived_status:
            raise ValidationError(
                f"{variant} journey {scenario_id} status must be derived as {derived_status}"
            )
    return value


def compare(
    scenario_map: dict[str, Any],
    predecessor: dict[str, Any],
    candidate: dict[str, Any],
) -> dict[str, Any]:
    predecessor_avoidable = sum(
        attempt["retry_class"] == "avoidable" for attempt in predecessor["attempts"]
    )
    candidate_avoidable = sum(
        attempt["retry_class"] == "avoidable" for attempt in candidate["attempts"]
    )
    predecessor_useful = sum(
        attempt["retry_class"] == "useful" for attempt in predecessor["attempts"]
    )
    candidate_useful = sum(
        attempt["retry_class"] == "useful" for attempt in candidate["attempts"]
    )
    journeys_complete = all(
        journey["status"] == "completed"
        for ledger in (predecessor, candidate)
        for journey in ledger["journeys"]
    )
    if not journeys_complete or predecessor_avoidable == 0:
        status = "not_proven"
        reduction = None
    else:
        reduction = (predecessor_avoidable - candidate_avoidable) / predecessor_avoidable
        status = "pass" if reduction >= 0.5 else "fail"
    return {
        "schema": OUTPUT_SCHEMA,
        "corpus_id": scenario_map["corpus_id"],
        "status": status,
        "comparability": {
            "same_scenario_map": True,
            "same_relevant_facts": True,
            "same_fake_remote_schedule": True,
            "version_specific_argv_bound": True,
            "intended_operations_bound": True,
            "expected_outcomes_bound": True,
            "all_declared_steps_observed": True,
            "journeys_complete": journeys_complete,
        },
        "denominators": {
            "predecessor_attempts": len(predecessor["attempts"]),
            "candidate_attempts": len(candidate["attempts"]),
            "predecessor_avoidable_retries": predecessor_avoidable,
            "candidate_avoidable_retries": candidate_avoidable,
            "predecessor_useful_retries": predecessor_useful,
            "candidate_useful_retries": candidate_useful,
        },
        "avoidable_retry_reduction_fraction": reduction,
        "target_fraction": 0.5,
        "non_claim": "A valid comparator result is evidence input; it does not activate writers or independently qualify SIM-07.",
    }


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser()
    parser.add_argument("--scenario-map", required=True, type=Path)
    parser.add_argument("--predecessor-ledger", required=True, type=Path)
    parser.add_argument("--candidate-ledger", required=True, type=Path)
    parser.add_argument("--output", type=Path)
    return parser


def main() -> int:
    args = build_parser().parse_args()
    try:
        scenario_map = validate_map(read_json(args.scenario_map))
        map_sha256 = sha256_bytes(args.scenario_map)
        predecessor = validate_ledger(
            read_json(args.predecessor_ledger), "predecessor", scenario_map, map_sha256
        )
        candidate = validate_ledger(
            read_json(args.candidate_ledger), "candidate", scenario_map, map_sha256
        )
        result = compare(scenario_map, predecessor, candidate)
    except (OSError, json.JSONDecodeError, ValidationError) as error:
        rendered = json.dumps(
            {"schema": OUTPUT_SCHEMA, "status": "invalid", "finding": str(error)},
            sort_keys=True,
        ) + "\n"
        if args.output:
            try:
                write_atomic(args.output, rendered)
            except OSError:
                args.output.unlink(missing_ok=True)
        print(rendered, end="")
        return 2
    rendered = json.dumps(result, indent=2, sort_keys=True) + "\n"
    if args.output:
        write_atomic(args.output, rendered)
    print(rendered, end="")
    return 0 if result["status"] == "pass" else 1


if __name__ == "__main__":
    raise SystemExit(main())
