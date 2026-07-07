# AWS Bedrock Native Provider Proof - Issue #5024

Issue: `#5024 [v0.91.7][provider][bedrock] Implement native AWS Bedrock provider adapter and tests`

Date: 2026-07-07

## Implementation Surface

- Added Rust-native AWS Bedrock provider support through the provider family path.
- Added provider profile presets for:
  - `bedrock:nova-lite-v1` -> `amazon.nova-lite-v1:0`
  - `bedrock:nova-pro-v1` -> `amazon.nova-pro-v1:0`
- Integrated `bedrock` / `aws_bedrock` into provider substrate inference, provider construction, hosted adapter routing, and `adl provider setup bedrock`.
- Added profile/region guardrails:
  - default AWS profile: `agent-logic-admin`
  - default AWS region: `us-west-2`
  - non-`agent-logic-admin` profiles fail before live Bedrock calls.
- Bedrock calls use the AWS SDK credential chain, STS profile verification, and Bedrock Runtime `InvokeModel`.
- Request shape uses the Amazon Nova messages schema with `schemaVersion: "messages-v1"`.
- Diagnostics classify Bedrock auth, model-unavailable, rate-limit, timeout, and generic provider failures without logging credentials or raw account identifiers.

## Local Focused Validation

Commands run from the issue worktree:

```text
cargo fmt --manifest-path adl/Cargo.toml
cargo test --manifest-path adl/Cargo.toml bedrock -- --nocapture
cargo test --manifest-path adl/Cargo.toml provider_adapter -- --nocapture
cargo test --manifest-path adl/Cargo.toml provider_setup -- --nocapture
cargo test --manifest-path adl/Cargo.toml provider_substrate -- --nocapture
git diff --check
```

Results:

- `bedrock`: passed, including local request/response shape tests, profile guardrail tests, and provider profile integration tests.
- `provider_adapter`: passed, including Bedrock hosted adapter request construction and profile guardrail tests.
- `provider_setup`: passed, including `adl provider setup bedrock` support.
- `provider_substrate`: passed, including Bedrock substrate inference.
- `git diff --check`: passed.

Broad lane note:

- `ADL_PR_FAST_ALLOW_FULL_NEXTEST=1 bash adl/tools/run_pr_fast_test_lane.sh --changed-files .adl/local-artifacts/provider-bedrock-5024/changed-files-5024.txt` compiled the full lane and ran 8,952 tests before failing in two `adl-pr-doctor` validation-profile tests.
- The failed tests passed when rerun directly:
  - `cargo test --manifest-path adl/Cargo.toml --bin adl-pr-doctor finish_validation_profile_classifies_slow_proof_family_split_slice -- --nocapture`
  - `cargo test --manifest-path adl/Cargo.toml --bin adl-pr-doctor finish_validation_profile_classifies_sprint_shell_helper_tests_as_small_binary_focused -- --nocapture`
- Classification: broad-lane parallel validation-manager/temp-file anomaly, not a Bedrock provider regression.

## Live AWS Verification

AWS profile used: `agent-logic-admin`.

Profile verification was performed with STS and recorded without account id disclosure:

- Artifact: `.adl/local-artifacts/provider-bedrock-5024/aws/sts_agent_logic_admin_sanitized.json`
- Recorded fields include `profile`, `sts_verified`, `account_id_sha256`, redacted ARN shape, and `user_id_present`.
- No AWS credentials, raw account id, authorization headers, or secret material are recorded.

Model discovery:

- `us-east-1` artifact: `.adl/local-artifacts/provider-bedrock-5024/aws/bedrock_nova_models_us_east_1.json`
- `us-west-2` artifact: `.adl/local-artifacts/provider-bedrock-5024/aws/bedrock_nova_models_us_west_2.json`
- `amazon.nova-lite-v1:0` was visible in both regions.
- `us-east-1` returned a live daily-token throttle for the tiny smoke call: `Too many tokens per day, please wait before trying again.`
- `us-west-2` successfully invoked the same Nova Lite model, so `us-west-2` is the default Bedrock region in this implementation.

## Live Adapter Smoke

Command shape:

```text
env -u ADL_AWS_REGION -u AWS_REGION -u AWS_DEFAULT_REGION \
  ADL_AWS_PROFILE=agent-logic-admin \
  AWS_PROFILE=agent-logic-admin \
  adl/target/debug/adl-provider-adapter \
    --request .adl/local-artifacts/provider-bedrock-5024/live/bedrock_nova_lite_request.json \
    --out .adl/local-artifacts/provider-bedrock-5024/live/bedrock_nova_lite_result_default_region.json \
    --log .adl/local-artifacts/provider-bedrock-5024/live/bedrock_nova_lite_log_default_region.jsonl
```

Result:

- `final_status`: `ok`
- attempts: `1`
- HTTP status: `200`
- output text: `bedrock ok`
- default region path: proven, because all region env vars were unset.

Artifacts:

- Request: `.adl/local-artifacts/provider-bedrock-5024/live/bedrock_nova_lite_request.json`
- Result: `.adl/local-artifacts/provider-bedrock-5024/live/bedrock_nova_lite_result_default_region.json`
- Log: `.adl/local-artifacts/provider-bedrock-5024/live/bedrock_nova_lite_log_default_region.jsonl`

## Provider Setup Smoke

Command:

```text
adl/target/debug/adl provider setup bedrock --out .adl/local-artifacts/provider-bedrock-5024/setup-smoke/bedrock --force
```

Result:

- Generated provider family: `bedrock`
- Generated model: `amazon.nova-lite-v1:0`
- Generated profile: `agent-logic-admin`
- Generated region: `us-west-2`

Artifacts:

- `.adl/local-artifacts/provider-bedrock-5024/setup-smoke/bedrock/provider.adl.yaml`
- `.adl/local-artifacts/provider-bedrock-5024/setup-smoke/bedrock/env.example`
- `.adl/local-artifacts/provider-bedrock-5024/setup-smoke/bedrock/README.md`

## UTS Benchmark Proof

Command shape:

```text
env -u ADL_AWS_REGION -u AWS_REGION -u AWS_DEFAULT_REGION \
  ADL_HOME=<ADL_WORKTREE> \
  ADL_AWS_PROFILE=agent-logic-admin \
  AWS_PROFILE=agent-logic-admin \
  python3 tools/uts_benchmark_runner.py hosted \
    <ADL_WORKTREE>/.adl/local-artifacts/provider-bedrock-5024/uts/bedrock_nova_lite_selector.txt \
    <ADL_WORKTREE>/.adl/local-artifacts/provider-bedrock-5024/uts/bedrock_nova_lite_results.json \
    --lanes regular,uts_only
```

Result:

- Model: `amazon.nova-lite-v1:0`
- Route: `hosted:adl-bedrock:amazon.nova-lite-v1:0`
- Regular lane: `11/11`
- UTS-only lane: `11/11`
- Provider failures: `0`
- Semantic failures: `0`
- Deterministic self-check: passed

Artifacts:

- Summary: `.adl/local-artifacts/provider-bedrock-5024/uts/bedrock_nova_lite_results_summary.md`
- Results JSON: `.adl/local-artifacts/provider-bedrock-5024/uts/bedrock_nova_lite_results.json`
- Provider status: `.adl/local-artifacts/provider-bedrock-5024/uts/bedrock_nova_lite_results_provider_status.json`
- Details: `.adl/local-artifacts/provider-bedrock-5024/uts/bedrock_nova_lite_results_details.md`
- Run log: `.adl/local-artifacts/provider-bedrock-5024/uts/bedrock_nova_lite_results_run.jsonl`
- Self-check: `.adl/local-artifacts/provider-bedrock-5024/uts/bedrock_nova_lite_results_self_check.json`

## Residual Truth

- Nova Pro profile support is implemented, but live UTS proof for this issue used Nova Lite only to keep live Bedrock usage bounded.
- `us-east-1` model visibility was proven, but the live `us-east-1` smoke hit a daily token throttle. The operational default is therefore `us-west-2`, which passed the default-region live smoke and full UTS panel.
