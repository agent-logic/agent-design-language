#!/usr/bin/env python3
"""Run a native Windows Qwen assisted-generation benchmark.

This script is intentionally dependency-light and emits a JSON proof artifact.
It does not require vLLM. It uses Hugging Face Transformers assisted generation
when available and records latency/throughput rather than inventing draft-token
acceptance counts that the API does not expose.
"""

from __future__ import annotations

import argparse
import json
import platform
import statistics
import time
from dataclasses import asdict, dataclass
from pathlib import Path
from typing import Any


PROMPTS = [
    "Write a concise Rust function that validates a non-empty provider id.",
    "Explain how speculative decoding preserves target-model correctness.",
    "Given a failed CI check, list the next three debugging steps.",
    "Summarize why tokenizer compatibility matters for draft verification.",
    "Draft a small JSON schema for a provider benchmark result.",
]


@dataclass
class RunResult:
    mode: str
    prompt_index: int
    repeat_index: int
    elapsed_seconds: float
    input_tokens: int
    output_tokens: int
    tokens_per_second: float


def positive_int(value: str) -> int:
    parsed = int(value)
    if parsed <= 0:
        raise argparse.ArgumentTypeError("must be positive")
    return parsed


def percentile(values: list[float], pct: float) -> float:
    if not values:
        return 0.0
    ordered = sorted(values)
    if len(ordered) == 1:
        return ordered[0]
    rank = (len(ordered) - 1) * pct
    lower = int(rank)
    upper = min(lower + 1, len(ordered) - 1)
    weight = rank - lower
    return ordered[lower] * (1.0 - weight) + ordered[upper] * weight


def summarize(mode: str, runs: list[RunResult]) -> dict[str, Any]:
    elapsed = [run.elapsed_seconds for run in runs]
    throughput = [run.tokens_per_second for run in runs]
    output_tokens = [run.output_tokens for run in runs]
    return {
        "mode": mode,
        "runs": len(runs),
        "elapsed_seconds": {
            "min": min(elapsed),
            "median": statistics.median(elapsed),
            "mean": statistics.mean(elapsed),
            "p95": percentile(elapsed, 0.95),
            "max": max(elapsed),
        },
        "tokens_per_second": {
            "min": min(throughput),
            "median": statistics.median(throughput),
            "mean": statistics.mean(throughput),
            "p95": percentile(throughput, 0.95),
            "max": max(throughput),
        },
        "output_tokens": {
            "min": min(output_tokens),
            "median": statistics.median(output_tokens),
            "mean": statistics.mean(output_tokens),
            "max": max(output_tokens),
        },
    }


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--target-model", default="Qwen/Qwen2.5-1.5B-Instruct")
    parser.add_argument("--assistant-model", default="Qwen/Qwen2.5-0.5B-Instruct")
    parser.add_argument("--out", required=True)
    parser.add_argument("--max-new-tokens", type=positive_int, default=160)
    parser.add_argument("--warmup-runs", type=positive_int, default=1)
    parser.add_argument("--repeats", type=positive_int, default=3)
    parser.add_argument("--prompt-limit", type=positive_int, default=len(PROMPTS))
    args = parser.parse_args()

    import torch
    from transformers import AutoModelForCausalLM, AutoTokenizer

    device = "cuda" if torch.cuda.is_available() else "cpu"
    dtype = torch.float16 if device == "cuda" else torch.float32
    selected_prompts = PROMPTS[: min(args.prompt_limit, len(PROMPTS))]

    tokenizer = AutoTokenizer.from_pretrained(args.target_model)
    target_model = AutoModelForCausalLM.from_pretrained(
        args.target_model,
        torch_dtype=dtype,
    ).to(device)
    assistant_model = AutoModelForCausalLM.from_pretrained(
        args.assistant_model,
        torch_dtype=dtype,
    ).to(device)
    target_model.eval()
    assistant_model.eval()

    def generate(prompt: str, assistant: bool) -> tuple[int, int, float]:
        messages = [{"role": "user", "content": prompt}]
        if hasattr(tokenizer, "apply_chat_template"):
            text = tokenizer.apply_chat_template(
                messages,
                tokenize=False,
                add_generation_prompt=True,
            )
        else:
            text = prompt
        inputs = tokenizer([text], return_tensors="pt").to(device)
        input_tokens = int(inputs.input_ids.shape[-1])
        kwargs: dict[str, Any] = {
            "max_new_tokens": args.max_new_tokens,
            "do_sample": False,
            "pad_token_id": tokenizer.eos_token_id,
        }
        if assistant:
            kwargs["assistant_model"] = assistant_model
        if device == "cuda":
            torch.cuda.synchronize()
        start = time.perf_counter()
        with torch.inference_mode():
            output = target_model.generate(**inputs, **kwargs)
        if device == "cuda":
            torch.cuda.synchronize()
        elapsed = time.perf_counter() - start
        output_tokens = int(output.shape[-1] - input_tokens)
        return input_tokens, output_tokens, elapsed

    for _ in range(args.warmup_runs):
        generate(selected_prompts[0], assistant=False)
        generate(selected_prompts[0], assistant=True)

    runs: list[RunResult] = []
    for repeat_index in range(args.repeats):
        for prompt_index, prompt in enumerate(selected_prompts):
            for mode, assistant in (("target_only", False), ("assisted", True)):
                input_tokens, output_tokens, elapsed = generate(prompt, assistant)
                tokens_per_second = output_tokens / elapsed if elapsed > 0 else 0.0
                runs.append(
                    RunResult(
                        mode=mode,
                        prompt_index=prompt_index,
                        repeat_index=repeat_index,
                        elapsed_seconds=elapsed,
                        input_tokens=input_tokens,
                        output_tokens=output_tokens,
                        tokens_per_second=tokens_per_second,
                    )
                )

    by_mode = {
        mode: [run for run in runs if run.mode == mode]
        for mode in ("target_only", "assisted")
    }
    payload = {
        "schema_version": "adl.native_qwen_assisted_generation_benchmark.v1",
        "issue_number": 4653,
        "measured_at_utc": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
        "runtime": {
            "platform": platform.platform(),
            "python": platform.python_version(),
            "torch": torch.__version__,
            "transformers": __import__("transformers").__version__,
            "device": device,
            "cuda_available": torch.cuda.is_available(),
            "cuda_device": torch.cuda.get_device_name(0)
            if torch.cuda.is_available()
            else None,
        },
        "models": {
            "target": args.target_model,
            "assistant": args.assistant_model,
            "same_family": "qwen",
        },
        "benchmark": {
            "max_new_tokens": args.max_new_tokens,
            "warmup_runs_per_mode": args.warmup_runs,
            "repeats": args.repeats,
            "prompt_count": len(selected_prompts),
            "modes": ["target_only", "assisted"],
        },
        "summaries": [summarize(mode, by_mode[mode]) for mode in by_mode],
        "runs": [asdict(run) for run in runs],
        "claims": {
            "proves_native_windows_gpu_generation": device == "cuda",
            "proves_transformers_assisted_generation_invocation": True,
            "proves_dspark_backend": False,
            "accepted_draft_token_counts_exposed": False,
            "accepted_draft_token_counts_note": (
                "Transformers assisted generation does not expose backend "
                "accepted-token/fallback counters through this script."
            ),
        },
    }
    out_path = Path(args.out)
    out_path.parent.mkdir(parents=True, exist_ok=True)
    out_path.write_text(json.dumps(payload, indent=2), encoding="utf-8")
    print(out_path)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
