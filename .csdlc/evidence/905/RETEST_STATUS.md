# #905 current speculative requalification checkpoint

Status: **incomplete; deterministic harness work only**. No keep/repair/retire performance recommendation, actual Runtime comparison, hardware smoke or fallback execution is claimed.

## Current route and environment

Inspected source candidate: `6ff1f023f62a4c0e1044483506ff0137626607b1` (bounded harness changes still uncommitted at capture). Current `adl-runtime-kernel/src/assembly.rs` dispatches conversation work through `control::invoke_provider_conversation` / `invoke_provider_model`. In `adl-runtime-kernel/src/control.rs`, `invoke_provider_model` accepts only `ollama` for live generation; Vertex AI validates then returns live-call-deferred and other providers fail. No speculative/draft-model controls were located in provider/kernel/execute source. The separate `adl-provider-adapter --request --out --log` invokes the Rust provider adapter, but that CLI alone does not establish this production Runtime route.

The historical benchmark directly constructs vLLM.LLM, uses historical engine arguments and unpinned model defaults, and does not call current Runtime. Default local Python on arm64 has neither vllm nor torch, and nvidia-smi is unavailable. Root separately owns Apple/MLX investigation; no model was loaded/downloaded or service changed here. The historical vLLM path is not silently replaced by an MLX benchmark. No approved current engine/model/tokenizer pair, comparable Runtime routes, sampling/seed control, hardware resource ceiling or controlled draft-failure execution has been selected.

## Bounded changes and proof

`adl/tools/vllm_qwen_speculative_decoding_benchmark.py` now rejects nonfinite/fractional/negative token counters, duplicate ambiguous metric series, counter resets, changed vector lengths and accepted counts above proposals. Run summaries reject empty/duplicate runs, invalid elapsed time, empty output and inconsistent throughput. Token-output SHA-256 provides an identity without publishing generated text. A paired comparison helper requires the complete declared prompt/repeat grid and exact token-output identity, reports null/negative speed benefit honestly, and explicitly denies Runtime execution proof. Same model/tokenizer/corpus/sampling provenance is a separate mandatory prerequisite; caller-provided hashes do not authenticate execution.

Focused command: `python3 adl/tools/test_vllm_qwen_speculative_decoding_benchmark.py` — thirteen deterministic test methods pass. PVF: required local CPU accounting/correctness negative contracts, no engine imports, network, inference or accelerator. `git diff --check` passes. No numerical line/branch coverage or hosted CI claimed. These tests do not prove real model fallback or output equivalence of actual engines.

## Remaining execution gates

1. Select an available approved engine and compatible pinned target/draft/tokenizer snapshots; bound time/memory/resource cost and sampling/repetitions.
2. Demonstrate a current production Runtime route for both modes with comparable settings. A missing route is a blocker; #905 limits code changes to the retest harness, not a provider integration repair.
3. Dependency setup, initialization, warmup and measured generation now append started/completed/failed records to a create-only attempt journal, retaining exception class without exception text. Interrupted started-only attempts remain censored. Final execution still needs source/model/engine/corpus provenance, actual resource/cost measurements; raw legacy output is not sufficient for final qualification.
4. Execute both routes and controlled draft failure/incompatibility with verified healthy normal fallback. Review actual correctness/benefit including no-benefit results before a keep/repair/retire recommendation.

No implementation PR should claim #905 complete while these gates remain. Native setup was refreshed through doctor/edit/validate generation6; full acceptance criteria remain unchanged. Parent and sibling #904 received exact source/environment blockers. No shared provider/kernel edits, cloud provisioning, credential access or paid execution performed.

Independent partial review by sprint8_720 and sprint8_908 identified historical output overwrite when no journal existed. Both output and journal identities are now exclusively reserved before engine import/initialization, with collision regression preserving historical bytes and no engine initialization. Dependency failures are journaled without exception text. Requested speculative mode is recorded separately from activation (not proved); unverified container identity is null, model revisions remain explicitly unverified, and current harness/corpus hashes plus sampling are retained. Fake-engine output tests prove record semantics only, not inference.
