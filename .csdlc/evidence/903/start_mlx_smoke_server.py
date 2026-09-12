"""Launch only after approval of MLX_EXECUTION_PLAN.md; never downloads models."""
import os
import runpy
import signal
import sys
from pathlib import Path

if len(sys.argv) != 2:
    raise SystemExit("exact existing model snapshot path required")
model = Path(sys.argv[1]).resolve(strict=True)
if not (model / "config.json").is_file() or not (model / "tokenizer.json").is_file():
    raise SystemExit("model snapshot is incomplete")
os.environ["HF_HUB_OFFLINE"] = "1"
os.environ["TRANSFORMERS_OFFLINE"] = "1"
import mlx.core as mx
if not mx.metal.is_available():
    raise SystemExit("Apple Metal unavailable")
mx.set_memory_limit(8 * 1024**3)
mx.set_cache_limit(128 * 1024**2)
# SIGALRM terminates this owned process even if a request does not return.
signal.alarm(180)
sys.argv = ["mlx_lm.server", "--model", str(model), "--host", "127.0.0.1",
            "--port", "18093", "--max-tokens", "16", "--decode-concurrency", "1",
            "--prompt-concurrency", "1", "--prompt-cache-size", "1",
            "--prompt-cache-bytes", "64M", "--prefill-step-size", "128"]
runpy.run_module("mlx_lm.server", run_name="__main__")
