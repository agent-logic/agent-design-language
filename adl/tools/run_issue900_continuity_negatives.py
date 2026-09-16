#!/usr/bin/env python3
"""Run #900 restore-integrity negatives against isolated production-state copies."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import pathlib
import shutil
import subprocess
from typing import Any, Callable


def read_json(path: pathlib.Path) -> Any:
    return json.loads(path.read_text(encoding="utf-8"))


def write_json(path: pathlib.Path, value: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def sha256_file(path: pathlib.Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def run(binary: pathlib.Path, command: str, input_path: pathlib.Path, runtime_root: pathlib.Path, output: pathlib.Path) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        [str(binary), command, "--input", str(input_path), "--runtime-root", str(runtime_root), "--output", str(output)],
        check=False,
        capture_output=True,
        text=True,
    )


def rewrite_dehydration_input(source: dict[str, Any], runtime_root: pathlib.Path) -> dict[str, Any]:
    rewritten = dict(source)
    rewritten["retained_runtime_root"] = str(runtime_root)
    rewritten["existing_agent_specs"] = [
        str(runtime_root / "agent-specs" / resident["agent_id"] / "agent.continuity.json")
        for resident in source["residents"]
    ]
    return rewritten


def tamper_signature(runtime_root: pathlib.Path, restore_input: dict[str, Any]) -> None:
    manifest_path = runtime_root / "live-kernel" / "generation-2" / "manifest.json"
    manifest = read_json(manifest_path)
    signature = manifest["signature"]
    manifest["signature"] = ("0" if signature[0] != "0" else "1") + signature[1:]
    write_json(manifest_path, manifest)


def tamper_payload(runtime_root: pathlib.Path, restore_input: dict[str, Any]) -> None:
    payload = runtime_root / "live-kernel" / "generation-2" / "0000-live_kernel.bin"
    data = bytearray(payload.read_bytes())
    data[len(data) // 2] ^= 1
    payload.write_bytes(data)


def omit_resident(runtime_root: pathlib.Path, restore_input: dict[str, Any]) -> None:
    restore_input["residents"] = restore_input["residents"][:-1]


def substitute_configuration(runtime_root: pathlib.Path, restore_input: dict[str, Any]) -> None:
    restore_input["residents"][0]["configuration_sha256"] = "0" * 64


def substitute_provider(runtime_root: pathlib.Path, restore_input: dict[str, Any]) -> None:
    restore_input["residents"][0]["provider_id"] = "substituted_provider"


def substitute_lineage(runtime_root: pathlib.Path, restore_input: dict[str, Any]) -> None:
    restore_input["residents"][0]["completed_task_sha256"] = "0" * 64


def stale_snapshot(runtime_root: pathlib.Path, restore_input: dict[str, Any]) -> None:
    receipt_path = runtime_root / "dehydration-receipt.json"
    receipt = read_json(receipt_path)
    receipt["generation"] = 1
    write_json(receipt_path, receipt)


SCENARIOS: tuple[tuple[str, Callable[[pathlib.Path, dict[str, Any]], None]], ...] = (
    ("changed_signature", tamper_signature),
    ("changed_payload", tamper_payload),
    ("removed_resident", omit_resident),
    ("substituted_provider", substitute_provider),
    ("substituted_configuration", substitute_configuration),
    ("substituted_lineage", substitute_lineage),
    ("stale_snapshot", stale_snapshot),
)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--continuity-bin", required=True, type=pathlib.Path)
    parser.add_argument("--source-runtime-root", required=True, type=pathlib.Path)
    parser.add_argument("--dehydration-input", required=True, type=pathlib.Path)
    parser.add_argument("--evidence-dir", required=True, type=pathlib.Path)
    args = parser.parse_args()
    for name in (
        "ADL_ISSUE414_SIGNING_KEY_HEX",
        "ADL_CSM_CUSTODY_P256_SIGNING_PRIVATE_KEY_B64",
        "ADL_CSM_CUSTODY_TRUSTED_P256_PUBLIC_KEY_B64",
    ):
        if not os.environ.get(name):
            raise SystemExit(f"required continuity signing environment is absent: {name}")
    if not args.continuity_bin.is_file() or not args.source_runtime_root.is_dir():
        raise SystemExit("production continuity binary and successful source Runtime root are required")

    source_input = read_json(args.dehydration_input)
    results = []
    for name, mutate in SCENARIOS:
        scenario_dir = args.evidence_dir / name
        runtime_root = scenario_dir / "runtime-root"
        shutil.copytree(args.source_runtime_root, runtime_root)
        dehydration_input = rewrite_dehydration_input(source_input, runtime_root)
        dehydration_input_path = scenario_dir / "dehydration-input.json"
        write_json(dehydration_input_path, dehydration_input)
        dehydrated = run(args.continuity_bin, "dehydrate", dehydration_input_path, runtime_root, scenario_dir / "dehydration.json")
        if dehydrated.returncode != 0:
            raise SystemExit(f"{name}: isolated generation-2 dehydration failed: {dehydrated.stderr.strip()}")
        receipt = read_json(scenario_dir / "dehydration.json")
        if receipt.get("generation") != 2 or receipt.get("admission_open") is not False:
            raise SystemExit(f"{name}: isolated negative baseline is not closed generation 2")

        restore_input = {
            "residents": json.loads(json.dumps(dehydration_input["residents"])),
            "retained_runtime_root": str(runtime_root),
            "build_cache_root": dehydration_input["build_cache_root"],
            "runtime_volume_identity_sha256": dehydration_input["runtime_volume_identity_sha256"],
        }
        mutate(runtime_root, restore_input)
        restore_input_path = scenario_dir / "restore-input.json"
        write_json(restore_input_path, restore_input)
        pointer = runtime_root / "active-population.json"
        before_pointer_sha256 = sha256_file(pointer)
        before_generations = sorted(path.name for path in (runtime_root / "restored-populations").glob("generation-*"))
        restored = run(args.continuity_bin, "restore", restore_input_path, runtime_root, scenario_dir / "restore.json")
        after_pointer_sha256 = sha256_file(pointer)
        after_generations = sorted(path.name for path in (runtime_root / "restored-populations").glob("generation-*"))
        admission_open = read_json(pointer).get("admission_open")
        denied = restored.returncode != 0
        no_effect = (
            before_pointer_sha256 == after_pointer_sha256
            and before_generations == after_generations
            and admission_open is False
        )
        result = {
            "scenario": name,
            "restore_denied": denied,
            "exit_code": restored.returncode,
            "admission_open_after": admission_open,
            "pointer_unchanged": before_pointer_sha256 == after_pointer_sha256,
            "restored_generations_unchanged": before_generations == after_generations,
            "no_inappropriate_effect": no_effect,
            "error": restored.stderr.strip().replace(str(args.evidence_dir), "<evidence>"),
        }
        write_json(scenario_dir / "result.json", result)
        if not denied or not no_effect:
            raise SystemExit(f"{name}: restore was not denied without effects")
        results.append(result)

    summary = {
        "schema": "adl.issue900.continuity_negative_summary.v1",
        "status": "passed",
        "scenario_count": len(results),
        "all_restore_denied": all(row["restore_denied"] for row in results),
        "all_no_inappropriate_effect": all(row["no_inappropriate_effect"] for row in results),
        "scenarios": results,
    }
    write_json(args.evidence_dir / "summary.json", summary)
    print(json.dumps(summary, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
