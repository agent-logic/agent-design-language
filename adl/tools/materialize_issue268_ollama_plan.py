#!/usr/bin/env python3
"""Bind the reviewed #268 plan template to exact local Ollama artifacts."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import pathlib
import urllib.request
from typing import Any


ROOT = pathlib.Path(__file__).resolve().parents[2]
DEFAULT_PLAN = ROOT / "adl/tools/issue268_six_resident_uts_plan.json"


def canonical_digest(value: Any) -> str:
    encoded = json.dumps(value, separators=(",", ":"), sort_keys=True).encode("utf-8")
    return hashlib.sha256(encoded).hexdigest()


def exact_digest(value: object, label: str) -> str:
    if not isinstance(value, str):
        raise SystemExit(f"{label} is absent")
    digest = value.removeprefix("sha256:")
    if len(digest) != 64 or any(character not in "0123456789abcdef" for character in digest):
        raise SystemExit(f"{label} is not an exact SHA-256")
    return digest


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--template", type=pathlib.Path, default=DEFAULT_PLAN)
    parser.add_argument("--output", required=True, type=pathlib.Path)
    parser.add_argument("--agent-spec-dir", type=pathlib.Path)
    parser.add_argument("--tags-json", type=pathlib.Path)
    parser.add_argument("--ollama-url", default=os.environ.get("OLLAMA_HOST", "http://127.0.0.1:11434"))
    args = parser.parse_args()
    if args.output.resolve() == args.template.resolve():
        raise SystemExit("materialized plan must not overwrite the reviewed template")

    if args.tags_json:
        tags = json.loads(args.tags_json.read_text(encoding="utf-8"))
    else:
        request = urllib.request.Request(args.ollama_url.rstrip("/") + "/api/tags", method="GET")
        with urllib.request.urlopen(request, timeout=10) as response:
            tags = json.load(response)
    by_name: dict[str, dict[str, Any]] = {}
    for metadata in tags.get("models") or []:
        for name in (metadata.get("name"), metadata.get("model")):
            if isinstance(name, str):
                by_name[name] = metadata

    plan = json.loads(args.template.read_text(encoding="utf-8"))
    for resident in plan.get("residents") or []:
        model = resident["model"]
        metadata = by_name.get(model)
        if metadata is None and not model.endswith(":latest"):
            metadata = by_name.get(model + ":latest")
        if metadata is None:
            raise SystemExit(f"required local Ollama model is absent: {model}")
        artifact = exact_digest(metadata.get("digest"), f"{model} registry digest")
        quantization = (metadata.get("details") or {}).get("quantization_level")
        if not isinstance(quantization, str) or not quantization.startswith("Q4"):
            raise SystemExit(f"{model} is not a reviewed Q4 quantization")
        configuration = {
            "model": model,
            "artifact_sha256": artifact,
            "quantization": quantization,
            "context_tokens": 32768,
            "num_predict": 1024,
            "num_gpu": 0,
            "temperature": 0,
            "max_concurrent_inference": 1,
            "max_loaded_models": 3,
            "qwen_think": False if model == "qwen3:8b" else "unsupported",
        }
        resident["model_ref_sha256"] = artifact
        resident["quantization"] = quantization
        resident["configuration_sha256"] = canonical_digest(configuration)

    plan["materialization"] = {
        "schema": "adl.issue268.ollama_plan_materialization.v1",
        "template_sha256": hashlib.sha256(args.template.read_bytes()).hexdigest(),
        "source": "ollama_api_tags",
        "configuration_contract": {
            "context_tokens": 32768,
            "num_predict": 1024,
            "qualification_num_predict": 128,
            "num_gpu": 0,
            "temperature": 0,
            "max_concurrent_inference": 1,
            "max_loaded_models": 3,
        },
    }
    args.output.parent.mkdir(parents=True, exist_ok=True)
    temporary = args.output.with_suffix(args.output.suffix + ".tmp")
    temporary.write_text(json.dumps(plan, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    os.replace(temporary, args.output)
    if args.agent_spec_dir:
        for resident in plan["residents"]:
            spec = {
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
            target = args.agent_spec_dir / resident["agent_id"] / "agent.yaml"
            target.parent.mkdir(parents=True, exist_ok=True)
            spec_temporary = target.with_suffix(".tmp")
            spec_temporary.write_text(json.dumps(spec, indent=2, sort_keys=True) + "\n", encoding="utf-8")
            os.replace(spec_temporary, target)
    print(json.dumps({"schema": "adl.issue268.ollama_plan_materialization.v1", "status": "pass", "resident_count": 6}, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
