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
- Added deterministic JSON report:
  `docs/milestones/v0.91.7/review/provider/DSPARK_SPECULATIVE_DECODING_EVALUATION_4653.json`.

## Evaluation Result

| Row | Disposition | Reason |
| --- | --- | --- |
| Qwen same-family candidate | `blocked_until_backend_exists` | Same-family Qwen is plausible, but ADL has no DSpark/Qwen draft-verify backend evidence yet. |
| Gemma same-family candidate | `blocked_until_backend_exists` | Same-family Gemma is plausible, but current Gemma evidence covers model usefulness, not speculative acceptance or throughput. |
| Qwen/Gemma cross-family pairing | `reject_cross_family_pairing` | ADL's existing speculative-decoding proof treats tokenizer mismatch as non-proving. |
| DeepSeek-V4 DSpark lane | `route_to_live_gpu_smoke` | The DSpark result is tied to DeepSeek-V4 serving; live proof belongs in #4654 with AWS guard and teardown evidence. |

Provider sprint acceptance:

- `accepted_for_v0917_provider_sprint: false`

## Decision

Do not claim Qwen or Gemma DSpark acceleration as accepted in v0.91.7 from
planning evidence alone. Keep Qwen and Gemma as same-family candidates, reject
cross-family Qwen/Gemma speculative pairings, and route actual DeepSeek-V4
DSpark live proof to #4654.

## Boundaries

- No provider secret was used or printed.
- No AWS resource was created by this issue.
- No live Qwen, Gemma, or DeepSeek DSpark backend speedup is claimed.
- No tool, mutation, merge, or side-effect authority is granted by speculative
  decoding.
- #5026 must consume only provider/model rows with live proof or accepted
  blocked dispositions.

## Validation

Passed on 2026-07-07 in the issue-bound worktree:

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
