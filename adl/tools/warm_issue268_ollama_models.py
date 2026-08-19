#!/usr/bin/env python3
"""Load the exact #268 Ollama model set once and retain it indefinitely."""
from __future__ import annotations

import argparse
import json
import os
import pathlib
import urllib.error
import urllib.request


def request_json(url: str, payload: dict | None = None, timeout: int = 900) -> dict:
    data = None if payload is None else json.dumps(payload).encode("utf-8")
    request = urllib.request.Request(
        url,
        data=data,
        headers={"Content-Type": "application/json"} if data is not None else {},
        method="POST" if data is not None else "GET",
    )
    try:
        with urllib.request.urlopen(request, timeout=timeout) as response:
            return json.load(response)
    except urllib.error.HTTPError as error:
        detail = error.read().decode("utf-8", errors="replace")
        raise RuntimeError(f"Ollama HTTP {error.code}: {detail}") from error


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--plan", required=True, type=pathlib.Path)
    parser.add_argument("--ollama-url", required=True)
    parser.add_argument("--receipt", required=True, type=pathlib.Path)
    args = parser.parse_args()

    plan = json.loads(args.plan.read_text(encoding="utf-8"))
    models = list(dict.fromkeys(row["model"] for row in plan.get("residents") or []))
    if len(models) != 3:
        raise SystemExit(f"issue268: exact three-model resident set required, found {len(models)}")
    contract = (plan.get("materialization") or {}).get("configuration_contract") or {}
    context_tokens = contract.get("context_tokens")
    if context_tokens != 32768 or contract.get("max_loaded_models") != 3:
        raise SystemExit("issue268: usable 32K three-model residency contract required")

    base = args.ollama_url.rstrip("/")
    for model in models:
        result = request_json(
            f"{base}/api/generate",
            {
                "model": model,
                "prompt": "Return exactly: OK",
                "stream": False,
                "think": False,
                "options": {"num_predict": 8, "num_ctx": context_tokens, "temperature": 0},
            },
        )
        if not isinstance(result.get("response"), str):
            raise SystemExit(f"issue268: model warmup returned no response for {model}")

    resident = request_json(f"{base}/api/ps", timeout=30).get("models") or []
    resident_names = {row.get("name") or row.get("model") for row in resident}
    missing = [model for model in models if model not in resident_names]
    if missing:
        raise SystemExit(f"issue268: models did not remain resident: {missing}")
    receipt = {
        "schema": "adl.issue268.ollama_residency.v1",
        "status": "passed",
        "models": models,
        "resident_model_count": len(models),
        "context_tokens": context_tokens,
        "keep_alive": "process_default_indefinite",
        "max_loaded_models": 3,
    }
    args.receipt.parent.mkdir(parents=True, exist_ok=True)
    temporary = args.receipt.with_suffix(args.receipt.suffix + ".tmp")
    temporary.write_text(json.dumps(receipt, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    os.replace(temporary, args.receipt)
    print(json.dumps({"status": "passed", "resident_model_count": len(models)}, sort_keys=True))


if __name__ == "__main__":
    main()
