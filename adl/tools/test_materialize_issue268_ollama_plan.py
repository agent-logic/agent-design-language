#!/usr/bin/env python3
from __future__ import annotations

import hashlib
import json
import pathlib
import subprocess
import sys
import tempfile


ROOT = pathlib.Path(__file__).resolve().parents[2]
MATERIALIZER = ROOT / "adl/tools/materialize_issue268_ollama_plan.py"


def main() -> None:
    with tempfile.TemporaryDirectory(prefix="issue268-materialize-") as temporary:
        root = pathlib.Path(temporary)
        tags = {
            "models": [
                {"name": "llama3.1:8b", "digest": "sha256:" + "a" * 64, "details": {"quantization_level": "Q4_K_M"}},
                {"name": "qwen3:8b", "digest": "b" * 64, "details": {"quantization_level": "Q4_K_M"}},
                {"name": "phi4-mini:latest", "digest": "c" * 64, "details": {"quantization_level": "Q4_K_M"}},
            ]
        }
        tags_path = root / "tags.json"
        tags_path.write_text(json.dumps(tags), encoding="utf-8")
        output = root / "materialized.json"
        specs = root / "agents"
        command = [
            sys.executable,
            str(MATERIALIZER),
            "--tags-json",
            str(tags_path),
            "--output",
            str(output),
            "--agent-spec-dir",
            str(specs),
            "--max-loaded-models",
            "1",
        ]
        subprocess.run(command, cwd=ROOT, check=True)
        plan = json.loads(output.read_text())
        assert len(plan["residents"]) == 6
        assert {row["model_ref_sha256"] for row in plan["residents"]} == {"a" * 64, "b" * 64, "c" * 64}
        assert all(len(row["configuration_sha256"]) == 64 for row in plan["residents"])
        assert plan["materialization"]["source"] == "ollama_api_tags"
        assert plan["materialization"]["configuration_contract"] == {
            "context_tokens": 32768,
            "num_predict": 128,
            "gpu_placement": "ollama_server_default",
            "temperature": 0,
            "max_concurrent_inference": 1,
            "max_loaded_models": 1,
        }
        assert plan["host"]["max_loaded_models"] == 1
        qwen = next(row for row in plan["residents"] if row["model"] == "qwen3:8b")
        expected_qwen_configuration = {
            "provider_id": "local_ollama",
            "provider_kind": "ollama",
            "model": "qwen3:8b",
            "artifact_sha256": qwen["model_ref_sha256"],
            "quantization": qwen["quantization"],
            "context_tokens": 32768,
            "num_predict": 128,
            "gpu_placement": "ollama_server_default",
            "temperature": 0,
            "max_concurrent_inference": 1,
            "max_loaded_models": 1,
            "qwen_think": "ollama_server_default",
        }
        assert qwen["configuration_sha256"] == hashlib.sha256(
            json.dumps(expected_qwen_configuration, separators=(",", ":"), sort_keys=True).encode()
        ).hexdigest()
        written_specs = [json.loads((specs / row["agent_id"] / "agent.yaml").read_text()) for row in plan["residents"]]
        assert len(written_specs) == 6
        assert {row["agent_id"] for row in written_specs} == {row["agent_id"] for row in plan["residents"]}
        assert all(row["schema"] == "adl.issue268.resident_agent_spec.v1" for row in written_specs)
        assert all(row["provider_id"] == "local_ollama" for row in written_specs)
        subprocess.run(command, cwd=ROOT, check=True)
        assert json.loads(output.read_text()) == plan

        tags["models"][1]["digest"] = "mutable-tag"
        tags_path.write_text(json.dumps(tags), encoding="utf-8")
        failed = subprocess.run(command, cwd=ROOT, capture_output=True, text=True)
        assert failed.returncode != 0 and "not an exact SHA-256" in failed.stderr
    print("PASS: issue268 exact Ollama plan materialization")


if __name__ == "__main__":
    main()
