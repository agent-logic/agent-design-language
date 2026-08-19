#!/usr/bin/env python3
"""Run one pre- or post-recovery UTS phase for all six #268 residents."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import pathlib
import subprocess
import sys
import tempfile
from typing import Any


ROOT = pathlib.Path(__file__).resolve().parents[2]
DEFAULT_PLAN = ROOT / "adl/tools/issue268_six_resident_uts_plan.json"
DEFAULT_TASK_PANEL = ROOT / "adl/tools/benchmark/uts_33_task_panel.json"
DEFAULT_RUNNER = ROOT / "adl/tools/uts_benchmark_runner.py"


def digest_file(path: pathlib.Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def canonical_digest(value: Any) -> str:
    return hashlib.sha256(
        json.dumps(value, separators=(",", ":"), sort_keys=True).encode("utf-8")
    ).hexdigest()


def atomic_json(path: pathlib.Path, value: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    temporary = path.with_suffix(path.suffix + ".tmp")
    temporary.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    os.replace(temporary, path)


def safe_id(value: str) -> str:
    if not value or any(character not in "abcdefghijklmnopqrstuvwxyz0123456789-" for character in value):
        raise SystemExit(f"unsafe resident id: {value!r}")
    return value


def selected_task(tasks: dict[str, dict[str, Any]], task_id: str) -> dict[str, Any]:
    task = tasks.get(task_id)
    if task is None:
        raise SystemExit(f"UTS task is absent from canonical panel: {task_id}")
    return task


def validate_report(path: pathlib.Path, agent_id: str, task_id: str) -> dict[str, Any]:
    report = json.loads(path.read_text(encoding="utf-8"))
    if report.get("schema_version") != "uts_benchmark_runner.v1":
        raise SystemExit(f"{agent_id}: unexpected UTS report schema")
    if report.get("deterministic_self_check", {}).get("passed") is not True:
        raise SystemExit(f"{agent_id}: deterministic UTS self-check failed")
    models = report.get("models") or []
    if len(models) != 1 or models[0].get("candidate_id") != agent_id:
        raise SystemExit(f"{agent_id}: exact resident report identity mismatch")
    lanes = models[0].get("lanes") or {}
    if set(lanes) != {"regular", "uts_only", "uts_acc"}:
        raise SystemExit(f"{agent_id}: exact UTS lane set missing")
    for lane_name, lane in lanes.items():
        if lane.get("status") != "evaluated":
            raise SystemExit(f"{agent_id}: {lane_name} did not execute: {lane.get('status')}")
        case_ids = [case.get("task_id") or case.get("id") for case in lane.get("cases") or []]
        if case_ids != [task_id]:
            raise SystemExit(f"{agent_id}: {lane_name} case denominator mismatch: {case_ids}")
    serialized = json.dumps(report, sort_keys=True)
    if any(marker in serialized for marker in ("API_KEY=", "Authorization: Bearer", "BEGIN PRIVATE KEY")):
        raise SystemExit(f"{agent_id}: retained report contains a secret marker")
    return report


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--phase", required=True, choices=("pre", "post"))
    parser.add_argument("--state", required=True, type=pathlib.Path)
    parser.add_argument("--evidence-dir", required=True, type=pathlib.Path)
    parser.add_argument("--plan", type=pathlib.Path, default=DEFAULT_PLAN)
    parser.add_argument("--task-panel", type=pathlib.Path, default=DEFAULT_TASK_PANEL)
    parser.add_argument("--runner", type=pathlib.Path, default=DEFAULT_RUNNER)
    args = parser.parse_args()

    plan = json.loads(args.plan.read_text(encoding="utf-8"))
    task_panel = json.loads(args.task_panel.read_text(encoding="utf-8"))
    tasks = {task["id"]: task for task in task_panel.get("tasks", [])}
    residents = plan.get("residents") or []
    if len(residents) != 6:
        raise SystemExit("six-resident plan is required")

    if args.phase == "pre":
        if args.state.exists():
            raise SystemExit("pre-recovery UTS state already exists; refusing completed-case replay")
        state: dict[str, Any] = {
            "schema": "adl.issue268.six_resident_uts_state.v1",
            "plan_sha256": digest_file(args.plan),
            "task_panel_sha256": digest_file(args.task_panel),
            "phase": "pre_in_progress",
            "residents": {},
        }
    else:
        if not args.state.is_file():
            raise SystemExit("post-recovery phase requires retained pre-recovery state")
        state = json.loads(args.state.read_text(encoding="utf-8"))
        if state.get("schema") != "adl.issue268.six_resident_uts_state.v1":
            raise SystemExit("retained UTS state schema mismatch")
        if state.get("plan_sha256") != digest_file(args.plan) or state.get("task_panel_sha256") != digest_file(args.task_panel):
            raise SystemExit("retained UTS state input digest mismatch")
        if state.get("phase") != "pre_complete":
            raise SystemExit("post-recovery phase requires exactly completed pre phase")
        if set((state.get("residents") or {}).keys()) != {row["agent_id"] for row in residents}:
            raise SystemExit("retained UTS resident population mismatch")
        state["phase"] = "post_in_progress"
        atomic_json(args.state, state)

    args.evidence_dir.mkdir(parents=True, exist_ok=True)
    for resident in residents:
        agent_id = safe_id(resident["agent_id"])
        task_id = resident["pre_recovery_case" if args.phase == "pre" else "post_recovery_case"]
        task = selected_task(tasks, task_id)
        retained = (state.get("residents") or {}).get(agent_id)
        if args.phase == "post":
            if retained is None or retained.get("completed_case_ids") != [resident["pre_recovery_case"]]:
                raise SystemExit(f"{agent_id}: retained completed-case state mismatch")
            if retained.get("pending_case_ids") != [task_id]:
                raise SystemExit(f"{agent_id}: retained pending-case state mismatch")

        with tempfile.TemporaryDirectory(prefix=f"issue268-{agent_id}-") as temporary:
            temp = pathlib.Path(temporary)
            model_panel = temp / "models.json"
            task_subset = temp / "tasks.json"
            model_list = temp / "models.txt"
            model_panel.write_text(
                json.dumps(
                    {
                        "schema_version": "uts_benchmark_model_panel.v1",
                        "models": [
                            {
                                "id": agent_id,
                                "tier": "issue268-resident",
                                "provider_kind": "local",
                                "provider": "ollama-local",
                                "model_id": resident["model"],
                                "notes": resident["role"],
                            }
                        ],
                    },
                    indent=2,
                )
                + "\n",
                encoding="utf-8",
            )
            task_subset.write_text(
                json.dumps({"schema_version": task_panel.get("schema_version"), "tasks": [task]}, indent=2) + "\n",
                encoding="utf-8",
            )
            model_list.write_text(agent_id + "\n", encoding="utf-8")
            report_path = args.evidence_dir / f"{args.phase}-{agent_id}.json"
            environment = os.environ.copy()
            environment.setdefault("ADL_UTS_LOCAL_TEST_TIMEOUT_SECONDS", "600")
            environment.setdefault("ADL_UTS_LOCAL_NUM_PREDICT", "128")
            environment.setdefault("ADL_UTS_LOCAL_NUM_CTX", "32768")
            environment.setdefault("ADL_UTS_OLLAMA_KEEP_ALIVE", "-1")
            command = [
                sys.executable,
                str(args.runner),
                "local",
                str(model_list),
                str(report_path),
                "--panel-file",
                str(model_panel),
                "--task-panel-file",
                str(task_subset),
                "--self-check-task-panel-file",
                str(args.task_panel),
                "--include-governed",
            ]
            completed = subprocess.run(command, cwd=ROOT, env=environment, check=False)
            if completed.returncode != 0:
                if report_path.is_file():
                    failed_report = json.loads(report_path.read_text(encoding="utf-8"))
                    for model in failed_report.get("models", []):
                        for lane_name, lane in (model.get("lanes") or {}).items():
                            if lane.get("status") != "evaluated":
                                print(
                                    f"{agent_id}: lane={lane_name} status={lane.get('status')} "
                                    f"failure_kind={lane.get('provider_failure_kind')} "
                                    f"note={lane.get('note')}",
                                    file=sys.stderr,
                                )
                self_check_path = report_path.with_name(f"{report_path.stem}_self_check.json")
                if self_check_path.is_file():
                    self_check = json.loads(self_check_path.read_text(encoding="utf-8"))
                    print(
                        f"{agent_id}: deterministic self-check failures: "
                        f"{self_check.get('failures', [])}",
                        file=sys.stderr,
                    )
                raise SystemExit(f"{agent_id}: UTS runner failed with {completed.returncode}")
            report = validate_report(report_path, agent_id, task_id)

        report_sha256 = digest_file(report_path)
        if args.phase == "pre":
            role_digest = canonical_digest(
                {"agent_id": agent_id, "role": resident["role"]}
            )
            tool_authority_digest = canonical_digest(
                {"agent_id": agent_id, "tool_authority": resident["tool_authority"]}
            )
            checkpoint_lineage = canonical_digest(
                {
                    "agent_id": agent_id,
                    "generation": 0,
                    "pre_recovery_case": task_id,
                    "post_recovery_case": resident["post_recovery_case"],
                }
            )
            state["residents"][agent_id] = {
                "role": resident["role"],
                "model": resident["model"],
                "role_digest": role_digest,
                "tool_authority_digest": tool_authority_digest,
                "sequence": 1,
                "completed_case_ids": [task_id],
                "pending_case_ids": [resident["post_recovery_case"]],
                "uts_report_sha256": report_sha256,
                "continuation_request_sha256": hashlib.sha256(
                    resident["post_recovery_case"].encode("utf-8")
                ).hexdigest(),
                "checkpoint_lineage": [checkpoint_lineage],
                "lane_results": {
                    name: {
                        "passed_count": lane.get("passed_count"),
                        "total_cases": lane.get("total_cases"),
                        "full_support": lane.get("full_support"),
                    }
                    for name, lane in report["models"][0]["lanes"].items()
                },
            }
        else:
            previous_lineage = retained["checkpoint_lineage"][-1]
            retained["sequence"] = 2
            retained["completed_case_ids"].append(task_id)
            retained["pending_case_ids"] = []
            retained["post_restore_uts_report_sha256"] = report_sha256
            retained["checkpoint_lineage"].append(
                canonical_digest(
                    {
                        "agent_id": agent_id,
                        "generation": 1,
                        "previous": previous_lineage,
                        "completed_case_ids": retained["completed_case_ids"],
                        "post_restore_uts_report_sha256": report_sha256,
                    }
                )
            )
        atomic_json(args.state, state)

    state["phase"] = "pre_complete" if args.phase == "pre" else "post_complete"
    state["resident_count"] = 6
    state["all_pending_empty"] = all(
        not value["pending_case_ids"] for value in state["residents"].values()
    )
    atomic_json(args.state, state)
    print(json.dumps({"status": "pass", "phase": state["phase"], "resident_count": 6}, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
