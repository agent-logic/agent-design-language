# Z.ai Provider Acceptance Proof

Issue: #5026
Milestone: v0.91.7
Date: 2026-07-09
Provider: Z.ai
Model target: `glm-5`
ADL route: `hosted:adl-z-ai:glm-5`
Credential source: operator-approved key file under `$HOME/keys/`, mapped
command-locally to `ZAI_API_KEY`

## Summary

Z.ai live proof reached the native ADL Z.ai provider-adapter path with an
approved key file mapped command-locally to `ZAI_API_KEY`. After the operator
repaired the account balance/resource package, the live adapter smoke passed and
the UTS benchmark passed both requested provider acceptance lanes.

The final accepted live smoke returned HTTP 200 with `final_status: ok`, one
attempt, and exact output text `ADL Z.ai provider adapter smoke ok`. The UTS
run evaluated the canonical `hosted:adl-z-ai:glm-5` route through the ADL
provider adapter with
regular `11/11`, UTS-only `11/11`, and zero provider failures. The governed
UTS+ACC lane was not run because #5026 required the regular and UTS-only
provider acceptance lanes.

## Results

| Surface | Result | Evidence |
| --- | --- | --- |
| Approved key-file presence | PASS | Operator-approved key file under `$HOME/keys/` existed and was non-empty. |
| Native adapter route | PASS | Request used provider `z_ai`, model `glm-5`, canonical ADL model ref `hosted:adl-z-ai:glm-5`; the retained UTS selector id used the historical `hosted:adl-z_ai:glm-5` alias. |
| Live adapter smoke | PASS | `final_status: ok`, one attempt, HTTP 200, exact output text `ADL Z.ai provider adapter smoke ok`. |
| UTS regular lane | PASS | `11/11`, full support true, zero provider failures. |
| UTS-only lane | PASS | `11/11`, full support true, zero provider failures. |
| UTS+ACC governed lane | NOT_RUN | Runner requires `--include-governed`; #5026 required regular and UTS-only provider acceptance. |

## Timing Snapshot

| Measurement | Value |
| --- | --- |
| UTS wall clock | ~6 minutes 49 seconds (`2026-07-09T18:24:18Z` to `2026-07-09T18:31:07Z`) |
| Regular lane summed provider duration | 120.484 seconds |
| UTS-only lane summed provider duration | 288.489 seconds |
| Slowest regular task | `get_weather_basic`, 37.746 seconds |
| Slowest UTS-only task | `update_inventory_basic`, 85.835 seconds |

## Local Evidence

Raw run artifacts are local ignored files under:

- `.adl/local-artifacts/provider-acceptance-5026/z-ai/live/zai_live_smoke_request.json`
- `.adl/local-artifacts/provider-acceptance-5026/z-ai/live/zai_live_smoke_issue5026_zai_impl_result.json`
- `.adl/local-artifacts/provider-acceptance-5026/z-ai/live/zai_live_smoke_issue5026_zai_impl_log.jsonl`
- `.adl/local-artifacts/provider-acceptance-5026/z-ai/uts/zai_selector.txt`
- `.adl/local-artifacts/provider-acceptance-5026/z-ai/uts/zai_model_panel.json`
- `.adl/local-artifacts/provider-acceptance-5026/z-ai/uts/zai_issue5026_results.json`
- `.adl/local-artifacts/provider-acceptance-5026/z-ai/uts/zai_issue5026_results_run.log.jsonl`
- `.adl/local-artifacts/provider-acceptance-5026/z-ai/uts/zai_issue5026_results_adapter_evidence/`

## Commands Run

```bash
test -s "<operator-approved-z-ai-key-file-under-$HOME/keys>"
```

```bash
ZAI_API_KEY="redacted" \
  <ADL_5026_WORKTREE>/adl/target/debug/adl-provider-adapter \
  --request .adl/local-artifacts/provider-acceptance-5026/z-ai/live/zai_live_smoke_request.json \
  --out .adl/local-artifacts/provider-acceptance-5026/z-ai/live/zai_live_smoke_issue5026_zai_impl_result.json \
  --log .adl/local-artifacts/provider-acceptance-5026/z-ai/live/zai_live_smoke_issue5026_zai_impl_log.jsonl
```

```bash
ZAI_API_KEY="redacted" \
  ADL_HOME=<ADL_5026_WORKTREE> \
  ADL_PROVIDER_ADAPTER_BIN=<ADL_5026_WORKTREE>/adl/target/debug/adl-provider-adapter \
  python3 <ADL_5026_WORKTREE>/adl/tools/uts_benchmark_runner.py \
  --panel-file .adl/local-artifacts/provider-acceptance-5026/z-ai/uts/zai_model_panel.json \
  hosted \
  .adl/local-artifacts/provider-acceptance-5026/z-ai/uts/zai_selector.txt \
  .adl/local-artifacts/provider-acceptance-5026/z-ai/uts/zai_issue5026_results.json
```

## Redaction Notes

- The tracked packet records only that an operator-approved key file under
  `$HOME/keys/` was used; it does not record the key filename or contents.
- The live command mapped the key file into `ZAI_API_KEY` only for the adapter
  process.
- The retained provider log redacts provider diagnostic fields.
- No authorization headers or raw secret values are recorded here.

## Non-Claims

- This packet does not claim the governed UTS+ACC lane was run.
- This packet does not claim byte-stable live provider output.
