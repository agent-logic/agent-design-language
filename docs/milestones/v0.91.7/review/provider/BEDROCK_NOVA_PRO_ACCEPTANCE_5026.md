# Bedrock Nova Pro Provider Acceptance Proof

Issue: #5026
Milestone: v0.91.7
Date: 2026-07-09
Provider: AWS Bedrock
AWS profile: `agent-logic-admin`
Region: `us-west-2`
Model target: `us.amazon.nova-pro-v1:0`

## Summary

Bedrock Nova Pro worked through the ADL provider-adapter path when invoked by
the system inference profile `us.amazon.nova-pro-v1:0`.

Direct on-demand invocation with `amazon.nova-pro-v1:0` failed with a Bedrock
validation error because Nova Pro does not support on-demand throughput in this
account/region. Bedrock inference-profile discovery returned an active system
profile named `US Nova Pro` with id `us.amazon.nova-pro-v1:0`; using that id as
the Bedrock `modelId` produced a successful live adapter smoke and UTS benchmark
run. The built-in `bedrock:nova-pro-v1` profile was aligned to that accepted
inference-profile route during conflict repair so profile expansion does not
select the rejected on-demand id.

## Results

| Surface | Result | Evidence |
| --- | --- | --- |
| AWS profile verification | PASS | Sanitized STS proof recorded only account/user digests and ARN shape. |
| Direct Nova Pro model id | EXPECTED_BLOCKED | `amazon.nova-pro-v1:0` rejected on-demand invocation; use inference profile. |
| Nova Pro inference profile discovery | PASS | `us.amazon.nova-pro-v1:0` was active in `us-west-2`. |
| Built-in Nova Pro profile route | PASS | `bedrock:nova-pro-v1` expands to the accepted `us.amazon.nova-pro-v1:0` inference-profile id. |
| Live adapter smoke | PASS | `final_status: ok`, one attempt, HTTP 200, output text `bedrock nova pro ok`. |
| UTS regular lane | PASS | `11/11`, full support true, zero provider failures. |
| UTS-only lane | PASS | `11/11`, full support true, zero provider failures. |
| UTS+ACC governed lane | NOT_RUN | Runner requires `--include-governed`; #5026 required regular and UTS-only provider acceptance. |

## Timing Snapshot

| Measurement | Value |
| --- | --- |
| UTS wall clock | ~40 seconds (`2026-07-09T18:24:54Z` to `2026-07-09T18:25:34Z`) |
| Regular lane summed provider duration | 20.852 seconds |
| UTS-only lane summed provider duration | 19.172 seconds |
| Slowest regular task | `search_contacts_basic`, 3.165 seconds |
| Slowest UTS-only task | `batch_weather_lookup_basic`, 2.495 seconds |

## Local Evidence

Raw run artifacts are local ignored files under:

- `.adl/local-artifacts/provider-acceptance-5026/aws/sts_agent_logic_admin_sanitized.json`
- `.adl/local-artifacts/provider-acceptance-5026/aws/bedrock_nova_inference_profiles_us_west_2.json`
- `.adl/local-artifacts/provider-acceptance-5026/bedrock-nova-pro/live/bedrock_nova_pro_issue5026_result.json`
- `.adl/local-artifacts/provider-acceptance-5026/bedrock-nova-pro/live/bedrock_nova_pro_issue5026_log.jsonl`
- `.adl/local-artifacts/provider-acceptance-5026/bedrock-nova-pro/uts/bedrock_nova_pro_issue5026_results.json`
- `.adl/local-artifacts/provider-acceptance-5026/bedrock-nova-pro/uts/bedrock_nova_pro_issue5026_results_run.log.jsonl`
- `.adl/local-artifacts/provider-acceptance-5026/bedrock-nova-pro/uts/bedrock_nova_pro_issue5026_results_adapter_evidence/`

## Commands Run

```bash
AWS_PROFILE=agent-logic-admin AWS_REGION=us-west-2 \
  aws bedrock list-inference-profiles --region us-west-2 --output json
```

```bash
env -u ADL_AWS_REGION -u AWS_REGION -u AWS_DEFAULT_REGION \
  ADL_AWS_PROFILE=agent-logic-admin \
  AWS_PROFILE=agent-logic-admin \
  <ADL_5026_WORKTREE>/adl/target/debug/adl-provider-adapter \
  --request .adl/local-artifacts/provider-acceptance-5026/bedrock-nova-pro/live/bedrock_nova_pro_inference_profile_request.json \
  --out .adl/local-artifacts/provider-acceptance-5026/bedrock-nova-pro/live/bedrock_nova_pro_issue5026_result.json \
  --log .adl/local-artifacts/provider-acceptance-5026/bedrock-nova-pro/live/bedrock_nova_pro_issue5026_log.jsonl
```

```bash
env -u ADL_AWS_REGION -u AWS_REGION -u AWS_DEFAULT_REGION \
  ADL_HOME=<ADL_5026_WORKTREE> \
  ADL_PROVIDER_ADAPTER_BIN=<ADL_5026_WORKTREE>/adl/target/debug/adl-provider-adapter \
  ADL_AWS_PROFILE=agent-logic-admin \
  AWS_PROFILE=agent-logic-admin \
  python3 <ADL_5026_WORKTREE>/adl/tools/uts_benchmark_runner.py \
  --panel-file <ADL_5026_WORKTREE>/.adl/local-artifacts/provider-acceptance-5026/bedrock-nova-pro/uts/bedrock_nova_pro_model_panel.json \
  hosted \
  <ADL_5026_WORKTREE>/.adl/local-artifacts/provider-acceptance-5026/bedrock-nova-pro/uts/bedrock_nova_pro_selector.txt \
  <ADL_5026_WORKTREE>/.adl/local-artifacts/provider-acceptance-5026/bedrock-nova-pro/uts/bedrock_nova_pro_issue5026_results.json
```

## Redaction Notes

- No AWS credentials, authorization headers, raw account id, or secret values are
  recorded in this tracked packet.
- The STS record stores SHA-256 digests only.
- UTS result records use redacted response excerpts and retained adapter
  evidence paths.
- The successful Nova Pro route is the inference profile id
  `us.amazon.nova-pro-v1:0`, not the on-demand model id.

## Non-Claims

- This packet does not claim Nova Pro on-demand support.
- This packet does not claim the governed UTS+ACC lane was run.
- This packet does not claim byte-stable live provider output.
- Z.ai live proof is recorded separately in
  `docs/milestones/v0.91.7/review/provider/Z_AI_ACCEPTANCE_5026.md`.
