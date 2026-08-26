#!/usr/bin/env python3
from __future__ import annotations

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
        command = [sys.executable, str(MATERIALIZER), "--tags-json", str(tags_path), "--output", str(output), "--agent-spec-dir", str(specs)]
        subprocess.run(command, cwd=ROOT, check=True)
        plan = json.loads(output.read_text())
        assert len(plan["residents"]) == 6
        assert {row["model_ref_sha256"] for row in plan["residents"]} == {"a" * 64, "b" * 64, "c" * 64}
        assert all(len(row["configuration_sha256"]) == 64 for row in plan["residents"])
        assert plan["materialization"]["source"] == "ollama_api_tags"
        assert plan["materialization"]["configuration_contract"] == {
            "context_tokens": 32768,
            "num_predict": 1024,
            "qualification_num_predict": 128,
            "num_gpu": 0,
            "temperature": 0,
            "max_concurrent_inference": 1,
            "max_loaded_models": 3,
        }
        written_specs = [json.loads((specs / row["agent_id"] / "agent.yaml").read_text()) for row in plan["residents"]]
        assert len(written_specs) == 6
        assert {row["agent_id"] for row in written_specs} == {row["agent_id"] for row in plan["residents"]}
        assert all(row["schema"] == "adl.issue268.resident_agent_spec.v1" for row in written_specs)
        subprocess.run(command, cwd=ROOT, check=True)
        assert json.loads(output.read_text()) == plan

        tags["models"][1]["digest"] = "mutable-tag"
        tags_path.write_text(json.dumps(tags), encoding="utf-8")
        failed = subprocess.run(command, cwd=ROOT, capture_output=True, text=True)
        assert failed.returncode != 0 and "not an exact SHA-256" in failed.stderr
    print("PASS: issue268 exact Ollama plan materialization")


if __name__ == "__main__":
    main()
