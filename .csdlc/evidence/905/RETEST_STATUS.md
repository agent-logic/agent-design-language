# #905 speculative-decoding requalification

Status: **complete; recommendation: repair the qualification benchmark and keep speculative decoding unqualified**. The experiment establishes correctness and bounded recovery behavior, but the performance result is not repeatable enough to support keep or retire.

## Current Runtime route and pinned environment

Runtime candidate `1872cf04b6d2641f32ff63d58dd64385f69e5b5e` was built locally and exercised through Runtime v3's Observatory conversation path, provider registry, Ollama chat compatibility fallback, and `/api/generate`. The provider boundary forced `think=false` because the generic Runtime request does not yet carry Ollama's explicit thinking control and reasoning-only responses are invalid normal conversation output.

The final run used Ollama `0.32.14` on an Apple M4 Pro with 14 CPU cores, 20 GPU cores and 64 GB unified memory. Both aliases came from the same immutable GGUF blob (`dec52a44569a2a25341c4e4d3fee25846eed4f6f0b936278e3a3c900bb99d37c`). The harness verified identical full model and tokenizer metadata, the same 15 embedded MTP tensors, and no normalized configuration difference except `draft_num_predict` (`0` baseline, `4` speculative). Temperature 0, seed 905, `num_predict=64`, the two-prompt corpus, and the exact same Runtime agent identity were shared. No model download, cloud call, or paid resource occurred.

## Executed result

Four counterbalanced A/B and B/A blocks produced eight measured conversations per arm after symmetric direct preload and excluded Runtime prewarm at every switch. All eight paired outputs were byte-equivalent and matched their correctness markers. The aggregate was 59.395 seconds baseline and 52.569 seconds speculative, which superficially implies a 12.98% gain. Aggregate decode rates were 13.86 and 16.26 tokens/s.

That aggregate is not robust. Per-block end-to-end benefit was `-12.52%`, `-0.34%`, `+105.18%`, and `-11.57%`: speculative lost three of four blocks, and the median benefit was `-5.96%`. Per-block decode benefit was `-53.40%`, `+130.85%`, `+22.40%`, and `-45.07%`: only two of four blocks won, with a `-11.33%` median. One mid-run regime shift and baseline decode outliers dominate the positive aggregate. The declared robustness gate requires three of four wins in both measures plus positive medians; it fails. Runtime's dynamic envelope differed by only two prompt tokens (`0.013%`), which is retained but does not explain or cure the variance. Ollama did not expose accepted/proposed draft-token counters, so none are claimed.

An alias derived from the valid resident target with `draft_num_predict=invalid` was rejected during configuration before Runtime admission. The harness then admitted the ordinary baseline configuration through the same Runtime route and delivered the exact expected marker. This proves fail-closed invalid-draft handling and healthy operator-selected ordinary recovery; it does not claim automatic Runtime fallback.

## Disposition

Repair the benchmark before requalification. A follow-on should define an explicit stationarity/stabilization criterion, increase the paired block denominator, retain per-block distributions, and require a robust aggregate or confidence interval. Until that proof exists, speculative decoding remains unqualified for the current Runtime path. This issue does not decommission the feature or delete resident models.

## Validation classification

- `python3 adl/tools/test_vllm_qwen_speculative_decoding_benchmark.py`: 13 deterministic accounting and negative-contract tests passed.
- `python3 -m unittest adl/tools/test_issue905_runtime_speculative_retest.py`: 3 deterministic safety regressions passed. They prove source/existing/duplicate aliases fail before model creation, collision failures never delete existing models, and identity/setup failure writes `report.json` while removing only aliases successfully created by that run.
- `issue905_runtime_speculative_retest.py`: actual local Runtime/hardware lane; immutable model/tokenizer checks; eight exact pairs; symmetric preload/prewarm; counterbalanced order; per-block robustness classification; invalid draft rejection; operator-selected ordinary recovery.
- `python3 -m py_compile adl/tools/issue905_runtime_speculative_retest.py`: passed.
- `git diff --check`: passed.
- Local evidence is hardware-dependent. Hosted CI validates repository bytes; it cannot replace model execution or repair statistical instability.

## PR #1004 review remediation

The post-publication review found that arbitrary aliases could overwrite and later delete an existing model, and that setup failures before the original reporting scope could leave created aliases without a failure report. The harness now canonicalizes all three temporary aliases, requires them to be nonempty, mutually distinct, different from the source model, and absent from the pre-run Ollama inventory before any creation. Cleanup tracks successful `ollama create` calls and removes only that run-owned list in reverse order. Binary checks, alias creation, identity checks, Runtime setup, execution, failure recording, cleanup, and final report writing now share one guarded lifecycle.

These changes harden the harness and do not alter or rerun the retained hardware measurements. The eight matching output pairs, variable per-block performance, and `repair_inconclusive` disposition remain unchanged.
