#!/usr/bin/env python3
from __future__ import annotations

import http.server
import importlib.util
import json
import pathlib
import subprocess
import sys
import tempfile
import threading


ROOT = pathlib.Path(__file__).resolve().parents[2]
WARMUP = ROOT / "adl/tools/warm_issue268_ollama_models.py"

SPEC = importlib.util.spec_from_file_location("issue268_warmup", WARMUP)
MODULE = importlib.util.module_from_spec(SPEC)
assert SPEC.loader
SPEC.loader.exec_module(MODULE)
assert MODULE.WARMUP_HTTP_TIMEOUT_SECONDS > MODULE.MODEL_LOAD_TIMEOUT_SECONDS


def main() -> None:
    observed: list[dict] = []

    class Handler(http.server.BaseHTTPRequestHandler):
        def log_message(self, *_args: object) -> None:
            return

        def do_POST(self) -> None:  # noqa: N802
            length = int(self.headers.get("Content-Length", "0"))
            observed.append(json.loads(self.rfile.read(length)))
            body = json.dumps({"response": "OK"}).encode()
            self.send_response(200)
            self.send_header("Content-Type", "application/json")
            self.send_header("Content-Length", str(len(body)))
            self.end_headers()
            self.wfile.write(body)

        def do_GET(self) -> None:  # noqa: N802
            body = json.dumps({"models": [{"name": model} for model in ("llama3.1:8b", "qwen3:8b", "phi4-mini:latest")]}).encode()
            self.send_response(200)
            self.send_header("Content-Type", "application/json")
            self.send_header("Content-Length", str(len(body)))
            self.end_headers()
            self.wfile.write(body)

    server = http.server.ThreadingHTTPServer(("127.0.0.1", 0), Handler)
    thread = threading.Thread(target=server.serve_forever, daemon=True)
    thread.start()
    try:
        with tempfile.TemporaryDirectory(prefix="issue268-warmup-") as temporary:
            root = pathlib.Path(temporary)
            plan = json.loads((ROOT / "adl/tools/issue268_six_resident_uts_plan.json").read_text())
            plan["materialization"] = {"configuration_contract": {"context_tokens": 32768, "max_loaded_models": 3}}
            plan_path = root / "plan.json"
            receipt = root / "receipt.json"
            plan_path.write_text(json.dumps(plan) + "\n")
            subprocess.run(
                [sys.executable, str(WARMUP), "--plan", str(plan_path), "--ollama-url", f"http://127.0.0.1:{server.server_port}", "--receipt", str(receipt)],
                check=True,
            )
            result = json.loads(receipt.read_text())
            assert result["status"] == "passed" and result["resident_model_count"] == 3
            assert len(observed) == 3
            assert all("keep_alive" not in row for row in observed)
            assert all(row["options"]["num_ctx"] == 32768 for row in observed)
    finally:
        server.shutdown()
        server.server_close()
        thread.join(timeout=5)
    print("PASS: issue268 Ollama models warm once and remain resident")


if __name__ == "__main__":
    main()
