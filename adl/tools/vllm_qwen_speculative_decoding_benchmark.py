#!/usr/bin/env python3
"""Run a bounded vLLM speculative-decoding benchmark.

This script is meant to run inside the vllm/vllm-openai container. It records
public vLLM metrics after generation, including speculative draft and accepted
token counters when vLLM exposes them through LLM.get_metrics().
"""

from __future__ import annotations

import argparse
import json
import hashlib
import math
import platform
import statistics
import time
from dataclasses import asdict, dataclass
from contextlib import ExitStack
from pathlib import Path
from typing import Any


PROMPTS = [
    "Write a concise Rust function that validates a non-empty provider id.",
    "Explain how speculative decoding preserves target-model correctness.",
    "Given a failed CI check, list the next three debugging steps.",
    "Summarize why tokenizer compatibility matters for draft verification.",
    "Draft a small JSON schema for a provider benchmark result.",
]


def measured_attempt(call, journal, *, phase: str, prompt_index=None, repeat_index=None):
    """Retain started and terminal states without copying prompts or exception text."""
    identity = {"phase": phase, "prompt_index": prompt_index, "repeat_index": repeat_index}
    def record(**values):
        journal.write(json.dumps({**identity, **values}, allow_nan=False) + "\n")
        journal.flush()
    record(status="started")
    start = time.perf_counter()
    try:
        value = call()
    except BaseException as error:
        record(status="failed", elapsed_seconds=time.perf_counter() - start,
               error_class=type(error).__name__)
        raise
    elapsed = time.perf_counter() - start
    record(status="completed", elapsed_seconds=elapsed)
    return value, elapsed


@dataclass
class RunResult:
    prompt_index: int
    repeat_index: int
    elapsed_seconds: float
    output_tokens: int
    tokens_per_second: float
    output_sha256: str = ""


def nonnegative_counter(value: Any) -> int:
    if isinstance(value, bool) or not isinstance(value, (int, float)):
        raise ValueError("counter must be a finite nonnegative integer")
    if not math.isfinite(value) or value < 0 or int(value) != value:
        raise ValueError("counter must be a finite nonnegative integer")
    return int(value)


def output_identity(token_ids: list[int]) -> str:
    if not token_ids:
        raise ValueError("empty generation cannot establish correctness")
    for token in token_ids:
        nonnegative_counter(token)
    return hashlib.sha256(json.dumps(token_ids, separators=(",", ":")).encode()).hexdigest()


def compare_runs(baseline: list[RunResult], speculative: list[RunResult],
                 *, prompt_count: int, repeats: int) -> dict[str, Any]:
    """Compare a complete paired corpus; caller must separately prove provenance.

    Token identity equality is applicable only with the same pinned tokenizer,
    model, corpus and deterministic sampling. This function cannot authenticate
    a Runtime execution or turn caller-provided records into hardware proof.
    """
    summarize(baseline)
    summarize(speculative)
    left = {(r.prompt_index, r.repeat_index): r for r in baseline}
    right = {(r.prompt_index, r.repeat_index): r for r in speculative}
    if nonnegative_counter(prompt_count) == 0 or nonnegative_counter(repeats) == 0:
        raise ValueError("declared corpus and repeats must be nonempty")
    expected = {(p, r) for p in range(prompt_count) for r in range(repeats)}
    if left.keys() != expected or right.keys() != expected:
        raise ValueError("paired run denominator mismatch")
    for key in left:
        for run in [left[key], right[key]]:
            if len(run.output_sha256) != 64 or any(c not in "0123456789abcdef" for c in run.output_sha256):
                raise ValueError("missing or invalid output identity")
        if left[key].output_sha256 != right[key].output_sha256 or left[key].output_tokens != right[key].output_tokens:
            raise ValueError("output equivalence failed")
    baseline_seconds = sum(r.elapsed_seconds for r in baseline)
    speculative_seconds = sum(r.elapsed_seconds for r in speculative)
    return {"paired_runs": len(left), "output_identity_equal": True,
            "baseline_seconds": baseline_seconds, "speculative_seconds": speculative_seconds,
            "wall_time_speedup": baseline_seconds / speculative_seconds,
            "proves_current_runtime_route": False}


def positive_int(value: str) -> int:
    parsed = int(value)
    if parsed <= 0:
        raise argparse.ArgumentTypeError("must be positive")
    return parsed


def summarize(runs: list[RunResult]) -> dict[str, Any]:
    if not runs:
        raise ValueError("no completed runs")
    identities = [(run.prompt_index, run.repeat_index) for run in runs]
    if len(set(identities)) != len(identities):
        raise ValueError("duplicate measured run")
    for run in runs:
        if not math.isfinite(run.elapsed_seconds) or run.elapsed_seconds <= 0:
            raise ValueError("elapsed time must be finite and positive")
        if nonnegative_counter(run.output_tokens) == 0:
            raise ValueError("empty output")
        expected = run.output_tokens / run.elapsed_seconds
        if not math.isfinite(run.tokens_per_second) or not math.isclose(run.tokens_per_second, expected):
            raise ValueError("inconsistent throughput")
    elapsed = [run.elapsed_seconds for run in runs]
    throughput = [run.tokens_per_second for run in runs]
    output_tokens = [run.output_tokens for run in runs]
    return {
        "runs": len(runs),
        "elapsed_seconds": {
            "min": min(elapsed),
            "median": statistics.median(elapsed),
            "mean": statistics.mean(elapsed),
            "max": max(elapsed),
        },
        "tokens_per_second": {
            "min": min(throughput),
            "median": statistics.median(throughput),
            "mean": statistics.mean(throughput),
            "max": max(throughput),
        },
        "output_tokens": {
            "min": min(output_tokens),
            "median": statistics.median(output_tokens),
            "mean": statistics.mean(output_tokens),
            "max": max(output_tokens),
        },
    }


def metric_payload(metric: Any) -> dict[str, Any]:
    payload = {
        "name": getattr(metric, "name", ""),
        "labels": getattr(metric, "labels", {}),
        "type": type(metric).__name__,
    }
    for field in ("value", "values", "count", "sum"):
        if hasattr(metric, field):
            payload[field] = getattr(metric, field)
    return payload


def extract_spec_metrics(metrics: list[Any]) -> dict[str, Any]:
    names = [getattr(metric, "name", "") for metric in metrics
             if getattr(metric, "name", "").startswith("vllm:spec_decode")]
    if len(names) != len(set(names)):
        raise ValueError("ambiguous duplicate speculative metric series")
    by_name = {getattr(metric, "name", ""): metric for metric in metrics}

    def counter(name: str) -> int | None:
        metric = by_name.get(name)
        value = getattr(metric, "value", None)
        return nonnegative_counter(value) if value is not None else None

    def vector(name: str) -> list[int] | None:
        metric = by_name.get(name)
        values = getattr(metric, "values", None)
        return [nonnegative_counter(value) for value in values] if values is not None else None

    num_drafts = counter("vllm:spec_decode_num_drafts")
    draft_tokens = counter("vllm:spec_decode_num_draft_tokens")
    accepted_tokens = counter("vllm:spec_decode_num_accepted_tokens")
    accepted_per_pos = vector("vllm:spec_decode_num_accepted_tokens_per_pos")
    if accepted_tokens is not None and draft_tokens is not None and accepted_tokens > draft_tokens:
        raise ValueError("accepted tokens exceed proposed tokens")

    acceptance_rate = None
    if accepted_tokens is not None and draft_tokens:
        acceptance_rate = accepted_tokens / draft_tokens

    mean_acceptance_length = None
    if accepted_tokens is not None and num_drafts:
        mean_acceptance_length = 1.0 + accepted_tokens / num_drafts

    return {
        "num_drafts": num_drafts,
        "num_draft_tokens": draft_tokens,
        "num_accepted_tokens": accepted_tokens,
        "num_accepted_tokens_per_pos": accepted_per_pos,
        "draft_acceptance_rate": acceptance_rate,
        "mean_acceptance_length_including_bonus": mean_acceptance_length,
    }


def subtract_spec_metrics(
    after: dict[str, Any], before: dict[str, Any]
) -> dict[str, Any]:
    num_drafts = subtract_optional_int(after["num_drafts"], before["num_drafts"])
    draft_tokens = subtract_optional_int(
        after["num_draft_tokens"], before["num_draft_tokens"]
    )
    accepted_tokens = subtract_optional_int(
        after["num_accepted_tokens"], before["num_accepted_tokens"]
    )
    accepted_per_pos = subtract_optional_vector(
        after["num_accepted_tokens_per_pos"],
        before["num_accepted_tokens_per_pos"],
    )
    if accepted_tokens is not None and draft_tokens is not None and accepted_tokens > draft_tokens:
        raise ValueError("accepted delta exceeds proposed delta")

    acceptance_rate = None
    if accepted_tokens is not None and draft_tokens:
        acceptance_rate = accepted_tokens / draft_tokens

    mean_acceptance_length = None
    if accepted_tokens is not None and num_drafts:
        mean_acceptance_length = 1.0 + accepted_tokens / num_drafts

    return {
        "num_drafts": num_drafts,
        "num_draft_tokens": draft_tokens,
        "num_accepted_tokens": accepted_tokens,
        "num_accepted_tokens_per_pos": accepted_per_pos,
        "draft_acceptance_rate": acceptance_rate,
        "mean_acceptance_length_including_bonus": mean_acceptance_length,
    }


def subtract_optional_int(after: int | None, before: int | None) -> int | None:
    if after is None or before is None:
        return None
    delta = nonnegative_counter(after) - nonnegative_counter(before)
    if delta < 0:
        raise ValueError("counter reset during measured interval")
    return delta


def subtract_optional_vector(
    after: list[int] | None, before: list[int] | None
) -> list[int] | None:
    if after is None or before is None:
        return None
    if len(after) != len(before):
        raise ValueError("counter vector shape changed")
    return [subtract_optional_int(a, b) for a, b in zip(after, before)]


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--mode", choices=["target_only", "speculative"], required=True)
    parser.add_argument("--target-model", default="Qwen/Qwen2.5-1.5B-Instruct")
    parser.add_argument("--draft-model", default="Qwen/Qwen2.5-0.5B-Instruct")
    parser.add_argument("--model-family", default="qwen")
    parser.add_argument("--spec-tokens", type=positive_int, default=3)
    parser.add_argument("--out", required=True)
    parser.add_argument("--max-new-tokens", type=positive_int, default=96)
    parser.add_argument("--warmup-runs", type=positive_int, default=1)
    parser.add_argument("--repeats", type=positive_int, default=2)
    parser.add_argument("--prompt-limit", type=positive_int, default=3)
    parser.add_argument("--gpu-memory-utilization", type=float, default=0.50)
    args = parser.parse_args()

    with ExitStack() as stack:
        out_path = Path(args.out)
        journal_path = out_path.with_suffix(out_path.suffix + ".attempts.jsonl")
        # Historical results may predate journals. Refuse either occupied identity
        # before importing/loading an engine; exclusive opens also reject races.
        if out_path.exists() or journal_path.exists():
            raise FileExistsError("benchmark output identity already exists")
        out_path.parent.mkdir(parents=True, exist_ok=True)
        result_stream = stack.enter_context(out_path.open("x", encoding="utf-8"))
        journal = stack.enter_context(journal_path.open("x", encoding="utf-8"))

        def load_engine():
            import torch
            import vllm
            from vllm import LLM, SamplingParams
            return torch, vllm, LLM, SamplingParams
        (torch, vllm, LLM, SamplingParams), _ = measured_attempt(
            load_engine, journal, phase="dependency_setup")

        selected_prompts = PROMPTS[: min(args.prompt_limit, len(PROMPTS))]
        llm_kwargs: dict[str, Any] = {
            "model": args.target_model,
            "max_model_len": 512,
            "gpu_memory_utilization": args.gpu_memory_utilization,
            "disable_log_stats": False,
        }
        if args.mode == "speculative":
            llm_kwargs.update(
                {
                    "spec_model": args.draft_model,
                    "spec_tokens": args.spec_tokens,
                }
            )

        llm, init_elapsed = measured_attempt(lambda: LLM(**llm_kwargs), journal, phase="initialization")

        sampling = SamplingParams(
            max_tokens=args.max_new_tokens,
            temperature=0.0,
            seed=0,
        )

        def generate(prompt: str) -> tuple[int, float, str]:
            start = time.perf_counter()
            outputs = llm.generate([prompt], sampling, use_tqdm=False)
            elapsed = time.perf_counter() - start
            token_ids = outputs[0].outputs[0].token_ids
            return len(token_ids), elapsed, output_identity(list(token_ids))

        for _ in range(args.warmup_runs):
            measured_attempt(lambda: generate(selected_prompts[0]), journal,
                             phase="warmup", prompt_index=0, repeat_index=_)

        warmup_metrics = extract_spec_metrics(llm.get_metrics())

        runs: list[RunResult] = []
        for repeat_index in range(args.repeats):
            for prompt_index, prompt in enumerate(selected_prompts):
                generated, _ = measured_attempt(lambda: generate(prompt), journal,
                                               phase="measured", prompt_index=prompt_index,
                                               repeat_index=repeat_index)
                output_tokens, elapsed, identity = generated
                runs.append(
                    RunResult(
                        prompt_index=prompt_index,
                        repeat_index=repeat_index,
                        elapsed_seconds=elapsed,
                        output_tokens=output_tokens,
                        tokens_per_second=output_tokens / elapsed if elapsed > 0 else 0.0,
                        output_sha256=identity,
                    )
                )

        metrics = llm.get_metrics()
        cumulative_spec_metrics = extract_spec_metrics(metrics)
        measured_spec_metrics = subtract_spec_metrics(
            cumulative_spec_metrics, warmup_metrics
        )
        raw_spec_metrics = [
            metric_payload(metric)
            for metric in metrics
            if getattr(metric, "name", "").startswith("vllm:spec_decode")
        ]

        payload = {
            "schema_version": "adl.vllm_qwen_speculative_decoding_benchmark.v1",
            "issue_number": 905,
            "provenance": {
                "harness_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
                "corpus_sha256": hashlib.sha256(json.dumps(selected_prompts, separators=(",", ":")).encode()).hexdigest(),
                "model_revisions_verified": False,
                "sampling": {"temperature": 0.0, "seed": 0, "max_tokens": args.max_new_tokens},
            },
            "measured_at_utc": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
            "runtime": {
                "platform": platform.platform(),
                "python": platform.python_version(),
                "torch": torch.__version__,
                "vllm": vllm.__version__,
                "cuda_available": torch.cuda.is_available(),
                "cuda_device": torch.cuda.get_device_name(0)
                if torch.cuda.is_available()
                else None,
                "container_image": None,
            },
            "mode": args.mode,
            "models": {
                "target": args.target_model,
                "draft": args.draft_model if args.mode == "speculative" else None,
                "same_family": args.model_family,
            },
            "benchmark": {
                "init_elapsed_seconds": init_elapsed,
                "max_new_tokens": args.max_new_tokens,
                "warmup_runs": args.warmup_runs,
                "repeats": args.repeats,
                "prompt_count": len(selected_prompts),
                "spec_tokens": args.spec_tokens if args.mode == "speculative" else 0,
            },
            "summary": summarize(runs),
            "runs": [asdict(run) for run in runs],
            "speculative_metrics": measured_spec_metrics,
            "warmup_speculative_metrics": warmup_metrics,
            "cumulative_speculative_metrics": cumulative_spec_metrics,
            "raw_speculative_metrics": raw_spec_metrics,
            "claims": {
                "proves_current_runtime_route": False,
                "proves_output_equivalence": False,
                "proves_vllm_generation": True,
                "proves_vllm_speculative_mode": False,
                "speculative_mode_requested": args.mode == "speculative",
                "speculative_counters_exposed": any(
                    value is not None for value in measured_spec_metrics.values()
                ),
                "proves_dspark_backend": False,
            },
        }

        result_stream.write(json.dumps(payload, indent=2, allow_nan=False))
        result_stream.close()
        journal.close()
        print(out_path)
        return 0


if __name__ == "__main__":
    raise SystemExit(main())
