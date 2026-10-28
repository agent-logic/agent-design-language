# Z.ai Native Provider Proof - Issue #5025

Date: 2026-07-07

Issue: #5025 `[v0.91.7][provider][z-ai] Implement native Z.ai provider adapter and tests`

## Scope

This issue adds a first-class Z.ai hosted provider path for ADL runtime provider execution:

- Provider kind: `z_ai`
- Accepted aliases: `zai`, `zhipu`
- Default endpoint: `https://open.bigmodel.cn/api/paas/v4/chat/completions`
- Default credential environment variable: `ZAI_API_KEY`
- Provider profile selector: `z_ai:glm-5`
- Stable model reference: `hosted:adl-z-ai:glm-5`
- Provider model id: `glm-5`

The implementation routes Z.ai through the hosted HTTP provider family, provider profiles, provider substrate identity/capability inference, provider setup CLI, and provider adapter runtime path.

## Local Validation

The following focused validation passed in the issue worktree:

```bash
cargo fmt --manifest-path adl/Cargo.toml
cargo test --manifest-path adl/Cargo.toml zai -- --nocapture
cargo test --manifest-path adl/Cargo.toml provider_substrate -- --nocapture
cargo test --manifest-path adl/Cargo.toml provider_setup -- --nocapture
cargo test --manifest-path adl/Cargo.toml provider_adapter -- --nocapture
git diff --check
```

The repo PR-fast lane classified the touched Rust/provider surface as requiring
full nextest escalation. The escalation lane also passed:

```bash
ADL_PR_FAST_ALLOW_FULL_NEXTEST=1 \
bash adl/tools/run_pr_fast_test_lane.sh \
  --changed-files .adl/local-artifacts/provider-zai-5025/changed-files-5025.txt
```

Result: nextest completed with exit code 0; 20,312 tests passed, 2 leaky tests
reported, and 18 tests skipped in 470.816 seconds.

The repo binary setup surface also passed against a repo-relative local artifact directory:

```bash
adl/target/debug/adl provider setup z_ai \
  --out .adl/local-artifacts/provider-zai-5025/setup-smoke/z_ai \
  --force
```

Covered surfaces include:

- Z.ai HTTP-family request translation, credential failure behavior, response extraction, and redacted invocation records.
- Provider adapter runtime execution for mocked Z.ai hosted calls.
- Provider registry construction through `build_provider_for_id`.
- Provider profile expansion for `z_ai:glm-5`.
- Provider setup CLI family coverage for `z_ai`.
- Provider substrate vendor, transport, and model identity inference.

## UTS Harness Result

The UTS deterministic self-check passed:

```bash
python3 tools/benchmark/deterministic_self_check.py
```

The hosted UTS runner was then invoked from the sibling `universal-tool-schema` checkout with the issue worktree as `ADL_HOME`:

```bash
ADL_HOME=<issue-worktree> \
python3 tools/uts_benchmark_runner.py hosted \
  <issue-worktree>/.adl/local-artifacts/provider-zai-5025/uts/zai_selector.txt \
  <issue-worktree>/.adl/local-artifacts/provider-zai-5025/uts/zai_uts_result.json \
  --lanes regular,uts_only \
  --run-id issue-5025-zai-uts \
  --overwrite
```

Result:

- UTS runner completed and wrote the result artifact.
- Deterministic self-check inside the runner passed.
- The ADL hosted provider adapter resolved provider identity as `z_ai` and provider model id `glm-5`.
- Initial #5025 UTS execution was credential-blocked in that command
  environment.
- That initial credential-blocked result is superseded by #5026's provider
  acceptance proof: the approved operator key file under `$HOME/keys/` was
  mapped command-locally to `ZAI_API_KEY`, the live Z.ai smoke passed, and the
  UTS regular and UTS-only lanes both passed through the native ADL provider
  adapter path.

Local artifact summary:

- `.adl/local-artifacts/provider-zai-5025/uts/zai_uts_result.json`
- `.adl/local-artifacts/provider-zai-5025/uts/zai_uts_result_summary.md`
- `.adl/local-artifacts/provider-zai-5025/uts/zai_uts_result_details.md`

## Live Provider Credential State

The original #5025 run was credential-blocked before live provider proof. That
state is no longer the current provider-sprint truth.

Current truth after #5026:

- An operator-approved Z.ai key file exists under `$HOME/keys/`.
- The key file was mapped command-locally to `ZAI_API_KEY` for the live adapter
  smoke and UTS panel.
- The tracked acceptance packet is
  `docs/milestones/v0.91.7/review/provider/Z_AI_ACCEPTANCE_5026.md`.
- The shared provider acceptance matrix is
  `docs/milestones/v0.91.7/review/provider/PROVIDER_ACCEPTANCE_MATRIX_5026.md`.

Prepared local smoke request:

- `.adl/local-artifacts/provider-zai-5025/zai_live_smoke_request.json`

No credential contents were printed, copied, committed, or scanned.

## Non-Claims

- This issue's initial local UTS run did not claim a successful live Z.ai API
  call because credentials were not present in that command environment.
- Current provider-sprint acceptance for live Z.ai API and UTS support is
  claimed by #5026, not by inventing new #5025-only proof.
- This issue does not add a canonical UTS panel member; the UTS invocation used an ad-hoc hosted selector to verify the ADL provider adapter path.
