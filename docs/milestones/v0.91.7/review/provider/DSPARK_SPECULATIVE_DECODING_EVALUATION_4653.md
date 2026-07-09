# DSpark Speculative Decoding Evaluation - Issue #4653

Issue: `#4653 [v0.91.7][models][dspark] Evaluate dspark speculative decoding with Qwen and Gemma`

Date: 2026-07-07

## Scope

This issue evaluates whether DSpark-style speculative decoding can be treated as
an accepted ADL provider/model path for Qwen and Gemma in the v0.91.7 provider
mini-sprint.

The answer is intentionally conservative: Qwen and Gemma are plausible
same-family candidates, but they are not accepted model paths in v0.91.7 until a
serving backend exposes real DSpark-style draft generation, target
verification, accepted-token counts, fallback counts, tokenizer compatibility,
latency, and throughput evidence.

## Inputs

- External DSpark source checked on 2026-07-07:
  `https://arxiv.org/abs/2607.05147`
- Existing ADL deterministic speculative-decoding commit-boundary proof:
  `docs/milestones/v0.91.2/review/speculative_decoding/speculative_decoding_prototype_packet.md`
- Provider sprint anchor: `#5027`
- Live GPU smoke follow-on: `#4654`
- Shared provider acceptance gate: `#5026`

## Implementation

- Added `adl/src/dspark_speculative_decoding_evaluation.rs`.
- Added `adl/src/bin/demo_v0917_dspark_speculative_decoding_evaluation.rs`.
- Added `adl/tools/native_qwen_assisted_generation_benchmark.py`.
- Added deterministic JSON report:
  `docs/milestones/v0.91.7/review/provider/DSPARK_SPECULATIVE_DECODING_EVALUATION_4653.json`.
- Added live native Windows benchmark artifact:
  `docs/milestones/v0.91.7/review/provider/NATIVE_QWEN_ASSISTED_GENERATION_BENCHMARK_4653.json`.
- Added offline/cache-mode native Windows benchmark artifact:
  `docs/milestones/v0.91.7/review/provider/NATIVE_QWEN_ASSISTED_GENERATION_BENCHMARK_OFFLINE_4653.json`.
- Added Gemma target + Qwen assistant probe artifact:
  `docs/milestones/v0.91.7/review/provider/GEMMA_QWEN_TOKENIZER_PROBE_4653.json`.
- Added native Windows vLLM availability probe artifact:
  `docs/milestones/v0.91.7/review/provider/VLLM_NATIVE_WINDOWS_AVAILABILITY_PROBE_4653.json`.
- Added vLLM target-only benchmark artifact:
  `docs/milestones/v0.91.7/review/provider/VLLM_QWEN_TARGET_ONLY_BENCHMARK_4653.json`.
- Added vLLM draft-model speculative benchmark artifact:
  `docs/milestones/v0.91.7/review/provider/VLLM_QWEN_SPECULATIVE_BENCHMARK_4653.json`.
- Added vLLM Gemma same-family speculative smoke artifact:
  `docs/milestones/v0.91.7/review/provider/VLLM_GEMMA_SPECULATIVE_SMOKE_4653.json`.

## Evaluation Result

| Row | Disposition | Reason |
| --- | --- | --- |
| Qwen same-family candidate | `blocked_until_backend_exists` | Same-family Qwen is plausible, but ADL has no DSpark/Qwen draft-verify backend evidence yet. |
| Gemma same-family candidate | `blocked_until_backend_exists` | Same-family Gemma is plausible, but current Gemma evidence covers model usefulness, not speculative acceptance or throughput. |
| Qwen/Gemma cross-family pairing | `reject_cross_family_pairing` | ADL's existing speculative-decoding proof treats tokenizer mismatch as non-proving. |
| DeepSeek-V4 DSpark lane | `route_to_live_gpu_smoke` | The DSpark result is tied to DeepSeek-V4 serving; live proof belongs in #4654 with AWS guard and teardown evidence. |

Provider sprint acceptance:

- `accepted_for_v0917_provider_sprint: false`

## Gemma Target + Qwen Assistant Probe

The issue's cross-family Gemma+Qwen path was probed first as a tokenizer/model
access prerequisite for a live assisted-generation benchmark:

- target: `google/gemma-2-2b-it`
- assistant: `Qwen/Qwen2.5-0.5B-Instruct`
- runtime: Nessus native Windows Python with Transformers 4.57.1

Result: the Hugging Face Gemma target probe failed before benchmark execution
because `google/gemma-2-2b-it` is gated and returned a `401` without an
authenticated account that has accepted the model terms. Nessus does have local
Ollama Gemma models, including `gemma4:e2b`, `gemma4:e4b`, `gemma4:26b`, and
`gemma4:31b`, but the Ollama path does not expose Transformers assisted
generation, accepted draft token counts, fallback counts, or DSpark
confidence-scheduled verification telemetry.

Disposition: Gemma target + Qwen assistant remains rejected/non-accepted for
v0.91.7. The blocker is not just access to one gated model; cross-family
speculative decoding also still needs tokenizer compatibility and
target-verified commit evidence from a backend that exposes acceptance
telemetry.

## Supplemental Same-Family Qwen Assisted-Generation Proof

Ran on 2026-07-09 on Nessus native Windows Python with CUDA visible on the
RTX 3090:

- target: `Qwen/Qwen2.5-1.5B-Instruct`
- assistant: `Qwen/Qwen2.5-0.5B-Instruct`
- runtime: Python 3.11.3, torch 2.5.1+cu121, Transformers 4.57.1
- cache mode: `HF_HUB_OFFLINE=1`, `TRANSFORMERS_OFFLINE=1`; existing
  Hugging Face snapshots under the Nessus user cache were reused
- benchmark shape: 5 prompts, 3 measured repeats per mode, 1 warmup, 160
  generated tokens per measured run

| Mode | Runs | Median elapsed | Mean throughput |
| --- | ---: | ---: | ---: |
| target-only | 15 | 6.020s | 26.292 tok/s |
| assisted | 15 | 7.976s | 20.018 tok/s |

Result: native Transformers assisted generation executed on GPU, but it was
slower than target-only generation for this Qwen2.5 1.5B/0.5B pair. This is a
valid supplemental same-family invocation proof and a useful negative result.
It does not answer the Gemma+Qwen cross-family row and is not DSpark backend
proof because the script does not expose accepted draft token counts, fallback
counts, confidence scheduling, or backend target-verification telemetry.

## Native vLLM Availability Probe

The reusable Nessus environment for this issue is the native Windows venv at
the issue-local artifact location. It has Torch 2.5.1+cu121, Transformers
4.57.1, CUDA access, and the RTX 3090. It also reuses the existing Hugging Face
Qwen snapshots from the Nessus user cache, so the offline benchmark does not
redownload those model weights.

That venv does not currently support native Windows vLLM:

```text
C:\adl-local-artifacts\issue-4653\native-venv\Scripts\python.exe -m pip install --dry-run --only-binary=:all: vllm
```

Result on Nessus native Windows: `BLOCKED`; pip returned
`No matching distribution found for vllm`.

## vLLM Qwen Draft-Model Speculative Benchmark

Ran on 2026-07-09 on Nessus with the existing `vllm/vllm-openai:latest` image,
CUDA visible on the RTX 3090, and the existing Hugging Face Qwen cache mounted
read-only into the container path used by vLLM. No model weight redownload was
required. A persistent vLLM compile cache mount was used for the second run.

- target: `Qwen/Qwen2.5-1.5B-Instruct`
- draft: `Qwen/Qwen2.5-0.5B-Instruct`
- vLLM: 0.24.0
- mode: `SpeculativeConfig(method='draft_model', num_spec_tokens=3)`
- benchmark shape: 3 prompts, 2 measured repeats per mode, 1 warmup, 96
  generated tokens per measured run

| Mode | Runs | Median elapsed | Mean throughput |
| --- | ---: | ---: | ---: |
| vLLM target-only | 6 | 0.483s | 199.190 tok/s |
| vLLM speculative | 6 | 1.317s | 72.735 tok/s |

Measured speculative counters, excluding warmup:

| Counter | Value |
| --- | ---: |
| drafts | 204 |
| draft tokens | 612 |
| accepted tokens | 378 |
| draft acceptance rate | 61.765% |
| mean acceptance length including bonus | 2.853 |
| accepted tokens by draft position | `[156, 120, 102]` |

Result: vLLM draft-model speculative decoding works for the same-family Qwen
target/draft pair and exposes real draft/accepted-token counters. It is still a
negative acceleration result for this exact configuration: speculative decoding
was slower than target-only. This is vLLM draft-model speculative evidence, not
DSpark confidence-scheduled backend proof.

## vLLM Gemma Draft-Model Speculative Smoke

Ran on 2026-07-09 on Nessus with the existing `vllm/vllm-openai:latest` image,
CUDA visible on the RTX 3090, and the local Hugging Face token mapped only into
the container environment for the command.

- target: `google/gemma-3-1b-it`
- draft: `google/gemma-3-270m-it`
- vLLM: 0.24.0
- mode: `SpeculativeConfig(method='draft_model', num_spec_tokens=3)`
- benchmark shape requested: 1 prompt, 1 measured repeat, 1 warmup, 32
  generated tokens

Observed progress before failure:

| Event | Value |
| --- | ---: |
| target download time | 539.739s |
| target checkpoint size | 1.86 GiB |
| target load time | 40.55s |
| draft download time | 143.341s |
| draft checkpoint size | 0.50 GiB |
| draft load time | 10.82s |
| combined model loading time | 737.074s |
| target torch compile time | 31.70s |
| draft torch compile time | 17.89s |

Result: the authenticated Gemma same-family vLLM smoke resolved both
`Gemma3ForCausalLM` models, downloaded and loaded both weights, and configured
draft-model speculative decoding. It failed before generation during KV-cache
initialization with:

```text
AssertionError: All drafting layers should belong to the same kv cache group
```

This is a useful negative backend-compatibility result. It proves access,
model resolution, and load for the Gemma target/draft pair, but it does not
prove vLLM Gemma speculative execution, accepted-token counters, DSpark
behavior, or speedup.

## Decision

Do not claim Qwen or Gemma DSpark acceleration as accepted in v0.91.7 from
planning evidence, native assisted-generation invocation, or the current vLLM
probes. Keep Qwen and Gemma as same-family candidates with measured
negative/blocked results, reject cross-family Qwen/Gemma speculative pairings,
and route actual DeepSeek-V4 DSpark live proof to #4654.

## Boundaries

- No provider secret was used or printed.
- No AWS resource was created by this issue.
- No live Qwen, Gemma, or DeepSeek DSpark backend speedup is claimed.
- The Gemma target + Qwen assistant path did not reach benchmark execution
  because the Hugging Face Gemma target was gated and the available Ollama
  Gemma path lacks the needed assisted-generation/counter telemetry.
- The live Qwen benchmark proves native Transformers assisted generation ran on
  Nessus, but it does not prove acceleration for the measured Qwen pair and
  does not substitute for Gemma+Qwen evidence.
- Native Windows vLLM execution is not proven; the reusable native venv cannot
  resolve an installable vLLM binary distribution.
- The Docker vLLM path proves vLLM draft-model speculative decoding and exposes
  accepted-token counters, but it does not prove DSpark confidence scheduling
  and does not prove speedup for the measured Qwen pair.
- The Docker vLLM Gemma same-family smoke proves authenticated Gemma target and
  draft model resolution/load, but vLLM failed before generation with a KV-cache
  grouping assertion; it does not prove Gemma speculative execution or speedup.
- No tool, mutation, merge, or side-effect authority is granted by speculative
  decoding.
- #5026 must consume only provider/model rows with live proof or accepted
  blocked dispositions.

## Validation

Passed on 2026-07-07 in the issue-bound worktree:

```text
python3 -m py_compile adl/tools/native_qwen_assisted_generation_benchmark.py
```

Result: `PASS`.

```text
set HF_HUB_OFFLINE=1 && set TRANSFORMERS_OFFLINE=1 && C:\adl-local-artifacts\issue-4653\native-venv\Scripts\python.exe C:\adl-local-artifacts\issue-4653\native_qwen_assisted_generation_benchmark.py --out C:\adl-local-artifacts\issue-4653\native-qwen-assisted-offline-20260709.json --repeats 3 --prompt-limit 5 --warmup-runs 1 --max-new-tokens 160
```

Result on Nessus native Windows: `PASS`; wrote
`C:\adl-local-artifacts\issue-4653\native-qwen-assisted-offline-20260709.json`,
copied into the retained offline/cache-mode proof artifact listed above.

```text
C:\adl-local-artifacts\issue-4653\native-venv\Scripts\python.exe -c "from transformers import AutoTokenizer; import json; target='google/gemma-2-2b-it'; assistant='Qwen/Qwen2.5-0.5B-Instruct'; tt=AutoTokenizer.from_pretrained(target); at=AutoTokenizer.from_pretrained(assistant); print(json.dumps({'target':target,'assistant':assistant,'target_vocab':len(tt),'assistant_vocab':len(at),'same_class':tt.__class__.__name__==at.__class__.__name__,'target_class':tt.__class__.__name__,'assistant_class':at.__class__.__name__}))"
```

Result on Nessus native Windows: `BLOCKED`; Hugging Face returned
`GatedRepoError` / `401` for `google/gemma-2-2b-it`, so the Gemma target
tokenizer/model prerequisite could not be loaded for a live Gemma+Qwen
assisted-generation benchmark.

```text
C:\adl-local-artifacts\issue-4653\native-venv\Scripts\python.exe -m pip install --dry-run --only-binary=:all: vllm
```

Result on Nessus native Windows: `BLOCKED`; pip returned
`No matching distribution found for vllm`.

```text
docker run --rm --gpus all -e HF_HUB_OFFLINE=1 -e TRANSFORMERS_OFFLINE=1 -v C:\Users\danie\.cache\huggingface:/root/.cache/huggingface -v C:\adl-local-artifacts\issue-4653\vllm-cache:/root/.cache/vllm -v C:\adl-local-artifacts\issue-4653:/work --entrypoint python3 vllm/vllm-openai:latest /work/vllm_qwen_speculative_decoding_benchmark.py --mode target_only --out /work/vllm-qwen-target-full-20260709.json --max-new-tokens 96 --warmup-runs 1 --repeats 2 --prompt-limit 3 --spec-tokens 3 --gpu-memory-utilization 0.50
```

Result on Nessus Docker/vLLM: `PASS`; wrote
`C:\adl-local-artifacts\issue-4653\vllm-qwen-target-full-20260709.json`,
copied into the retained target-only vLLM artifact listed above.

```text
docker run --rm --gpus all -e HF_HUB_OFFLINE=1 -e TRANSFORMERS_OFFLINE=1 -v C:\Users\danie\.cache\huggingface:/root/.cache/huggingface -v C:\adl-local-artifacts\issue-4653\vllm-cache:/root/.cache/vllm -v C:\adl-local-artifacts\issue-4653:/work --entrypoint python3 vllm/vllm-openai:latest /work/vllm_qwen_speculative_decoding_benchmark.py --mode speculative --out /work/vllm-qwen-speculative-full-20260709.json --max-new-tokens 96 --warmup-runs 1 --repeats 2 --prompt-limit 3 --spec-tokens 3 --gpu-memory-utilization 0.50
```

Result on Nessus Docker/vLLM: `PASS`; wrote
`C:\adl-local-artifacts\issue-4653\vllm-qwen-speculative-full-20260709.json`,
copied into the retained speculative vLLM artifact listed above. The run
reported measured draft/accepted counters after subtracting warmup.

```text
docker run --rm --gpus all -e HF_TOKEN=<redacted> -v C:\Users\danie\.cache\huggingface:/root/.cache/huggingface -v C:\adl-local-artifacts\issue-4653\vllm-cache:/root/.cache/vllm -v C:\adl-local-artifacts\issue-4653:/work --entrypoint python3 vllm/vllm-openai:latest /work/vllm_qwen_speculative_decoding_benchmark.py --mode speculative --target-model google/gemma-3-1b-it --draft-model google/gemma-3-270m-it --model-family gemma --out /work/vllm-gemma-spec-smoke-20260709.json --max-new-tokens 32 --warmup-runs 1 --repeats 1 --prompt-limit 1 --spec-tokens 3 --gpu-memory-utilization 0.50
```

Result on Nessus Docker/vLLM: `FAIL`; both Gemma models resolved and loaded,
but vLLM failed during engine initialization before generation with
`AssertionError: All drafting layers should belong to the same kv cache group`.
The retained negative artifact listed above records the failure without any
provider secret value.

```text
CARGO_INCREMENTAL=0 cargo test --manifest-path adl/Cargo.toml --lib dspark_speculative_decoding_evaluation -- --nocapture
```

Result: `4 passed; 0 failed`.

```text
CARGO_INCREMENTAL=0 cargo test --manifest-path adl/Cargo.toml --bin demo_v0917_dspark_speculative_decoding_evaluation -- --nocapture
```

Result: `2 passed; 0 failed`.

```text
CARGO_INCREMENTAL=0 cargo run --manifest-path adl/Cargo.toml --bin demo_v0917_dspark_speculative_decoding_evaluation -- docs/milestones/v0.91.7/review/provider/DSPARK_SPECULATIVE_DECODING_EVALUATION_4653.json
```

Result: regenerated the deterministic JSON report at the path above.

```text
git diff --check
```

Result: `PASS`.

An earlier non-proving cargo invocation omitted the scoped `--lib` / `--bin`
selectors and attempted to compile a broader target set, exhausting local disk
space during compilation. The generated `adl/target` directory in the issue
worktree was removed, and validation was rerun with the scoped commands listed
above.
