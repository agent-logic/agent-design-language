# Fable 5 UTS Acceptance Proof - Issue #5044

Issue: `#5044 [v0.91.7][provider][anthropic] Run Fable 5 UTS provider acceptance panel`

Date: 2026-07-07

## Scope

This issue makes Claude Fable 5 repeatable as an ADL provider-adapter UTS
acceptance target.

- Provider route: `hosted:adl-anthropic:claude-fable-5`
- Provider model id: `claude-fable-5`
- Credential source used for live proof: operator-approved
  `$HOME/keys/claude2.key`, mapped command-locally to `ANTHROPIC_API_KEY`
- Output-token budget used by the ADL adapter request: `1024`
- UTS lanes: `regular,uts_only`

## Implementation Surface

- Added `adl/tools/adl_provider_adapter_with_budget.py`, a generic wrapper that
  inserts `max_output_tokens` into a UTS-owned ADL provider request before
  invoking `adl-provider-adapter`.
- Added `adl/tools/run_fable5_uts_acceptance.sh`, a reusable Fable 5 UTS runner
  that writes the ad-hoc selector, runs deterministic self-check, optionally
  runs hosted availability probe, and runs the regular plus UTS-only panel.
- Added `adl/tools/test_fable5_uts_acceptance.sh`, a non-live wrapper contract
  test proving selector generation and request-budget injection.
- Updated Anthropic response normalization so an Anthropic HTTP 200 response
  with `stop_reason: "refusal"` and no text content is recorded as a model
  refusal JSON instead of a provider empty-output failure.

## Focused Validation

Commands run from the issue worktree:

```text
cargo fmt --manifest-path adl/Cargo.toml
bash adl/tools/test_fable5_uts_acceptance.sh
cargo test --manifest-path adl/Cargo.toml anthropic_provider_complete_normalizes_empty_refusal_response -- --nocapture
cargo test --manifest-path adl/Cargo.toml claude_hosted_adapter_normalizes_empty_refusal_response -- --nocapture
cargo test --manifest-path adl/Cargo.toml optional_output_token_budget_maps_to_provider_native_fields -- --nocapture
git diff --check
ADL_PR_FAST_ALLOW_FULL_NEXTEST=1 bash adl/tools/run_pr_fast_test_lane.sh --changed-files .adl/local-artifacts/provider-fable5-5044-changed-files.txt
```

Focused result: passed.

Broad lane result: non-proving, failed outside the Fable 5 provider surface.
The PR-fast opt-in lane escalated to full nextest for unmapped Rust-surface
coverage and stopped at
`finish_validation_profile_classifies_bounded_cav_tokio_paths`. The failure
reported missing validation-lane coverage for an unrelated bounded CAV/tokio
selector fixture involving `adl/tools/observability.sh`, after `9992` tests had
passed and `18` were skipped. This issue does not claim a green full-nextest
run; publication relies on the focused provider proof above plus the live UTS
acceptance run.

## Live UTS Proof

Command shape:

```text
ADL_PROVIDER_ADAPTER_BIN=<ADL_WORKTREE>/adl/target/debug/adl-provider-adapter \
ADL_ANTHROPIC_API_KEY_FILE=$HOME/keys/claude2.key \
UTS_HOSTED_MAX_ATTEMPTS=2 \
UTS_HOSTED_RETRY_BACKOFF_SECONDS=1 \
bash adl/tools/run_fable5_uts_acceptance.sh \
  --artifact-dir .adl/local-artifacts/provider-fable5-5044-normalized \
  --key-file $HOME/keys/claude2.key \
  --max-output-tokens 1024 \
  --run-id issue-5044-fable5-uts-normalized
```

Availability probe:

- Model: `claude-fable-5`
- Route: `adl-anthropic`
- Classification: `listed_and_invokable`
- HTTP: `200`
- Note: model listing is not supported for the ADL Anthropic route.

UTS panel result:

| Model | Route | Regular | UTS-only | Provider failures | Semantic failures |
| --- | --- | ---: | ---: | ---: | ---: |
| `claude-fable-5` | `hosted:adl-anthropic:claude-fable-5` | `10/11` | `11/11` | `0` | `1` |

The single regular-lane semantic failure was `update_inventory_basic`: Fable 5
returned a model refusal for the ordinary tool-call prompt. The same task passed
in the UTS-only lane because the UTS proposal format carries an explicit
`dry_run_requested: true` boundary.

## Artifacts

Retained local artifacts:

- `.adl/local-artifacts/provider-fable5-5044-normalized/fable5_selector.txt`
- `.adl/local-artifacts/provider-fable5-5044-normalized/fable5_self_check.json`
- `.adl/local-artifacts/provider-fable5-5044-normalized/fable5_probe_results.json`
- `.adl/local-artifacts/provider-fable5-5044-normalized/fable5_probe_results_summary.md`
- `.adl/local-artifacts/provider-fable5-5044-normalized/fable5_uts_results.json`
- `.adl/local-artifacts/provider-fable5-5044-normalized/fable5_uts_results_summary.md`
- `.adl/local-artifacts/provider-fable5-5044-normalized/fable5_uts_results_provider_status.json`
- `.adl/local-artifacts/provider-fable5-5044-normalized/fable5_uts_results_details.md`
- `.adl/local-artifacts/provider-fable5-5044-normalized/fable5_uts_results_run.jsonl`
- `.adl/local-artifacts/provider-fable5-5044-normalized/fable5_uts_results_self_check.json`

Diagnostic local artifacts from earlier runs are retained under
`.adl/local-artifacts/provider-fable5-5044*` and show why the Anthropic refusal
normalization was necessary. They are not claimed as the final accepted run.

## Boundaries

- No provider secret was printed, copied, committed, or retained in tracked
  artifacts.
- This proof does not claim broad Fable 5 quality, benchmark superiority,
  release authority, merge authority, or autonomous repository mutation ability.
- The row is an ad-hoc model acceptance target, not a canonical UTS model-panel
  member.
- The useful result is that Fable 5 is invokable through ADL, has zero provider
  failures after refusal normalization, and performs better with the UTS dry-run
  boundary than with ordinary tool-call prompting on mutation-shaped tasks.
