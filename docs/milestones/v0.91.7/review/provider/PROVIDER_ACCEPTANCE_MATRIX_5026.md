# Provider Acceptance Matrix - Issue #5026

Issue: `#5026 [v0.91.7][provider][proof] Run Z.ai and Bedrock provider acceptance smoke tests`

Date: 2026-07-09

## Summary

This packet is the #5026 shared acceptance matrix for the provider mini-sprint.
It rolls up the provider/model rows that #5026 depends on without overstating
rows owned by separate issues.

Z.ai GLM-5 and AWS Bedrock Nova Pro are accepted live provider paths through
the #5026-local ADL provider adapter. Fable 5 is accepted as an ad-hoc ADL
provider-adapter UTS target through #5044, with one ordinary regular-lane
semantic refusal recorded. DSpark is not accepted as an acceleration/provider
path yet: #4653 produced a conservative candidate evaluation and #4654 failed
closed before EC2 launch because the Agent Logic account had zero P-family
quota and no exact single-node 2xH100 shape was available.

## Matrix

| Row | Provider route | Provider model id | Status | Proof | Notes |
| --- | --- | --- | --- | --- | --- |
| Z.ai GLM-5 | `hosted:adl-z-ai:glm-5` | `glm-5` | `implemented_and_integrated` | `Z_AI_ACCEPTANCE_5026.md` | Live smoke PASS; UTS regular `11/11`; UTS-only `11/11`; provider failures `0`; slow provider latency recorded. |
| AWS Bedrock Nova Pro | `hosted:adl-bedrock:us.amazon.nova-pro-v1:0` | `us.amazon.nova-pro-v1:0` | `implemented_and_integrated` | `BEDROCK_NOVA_PRO_ACCEPTANCE_5026.md` | Live smoke PASS through the `US Nova Pro` system inference profile; UTS regular `11/11`; UTS-only `11/11`; direct on-demand id `amazon.nova-pro-v1:0` rejected as expected. |
| Fable 5 | `hosted:adl-anthropic:claude-fable-5` | `claude-fable-5` | `accepted_model_path_with_recorded_semantic_caveat` | `FABLE5_UTS_ACCEPTANCE_5044.md` | Live provider route listed and invokable; UTS regular `10/11`; UTS-only `11/11`; provider failures `0`; one ordinary mutation-shaped regular task produced a refusal. |
| DSpark Qwen/Gemma candidates | not accepted as a provider route | not applicable | `candidate_only_not_accepted_model_path` | `DSPARK_SPECULATIVE_DECODING_EVALUATION_4653.md` | Same-family Qwen/Gemma rows remain candidates blocked on a real DSpark backend; cross-family rows are rejected as non-proving. |
| DeepSeek V4 Flash DSpark GPU smoke | not launched | `deepseek-ai/DeepSeek-V4-Flash-DSpark` | `blocked_external_capacity` | `dspark_gpu_smoke_4654/README.md` | Live preflight found public model metadata, but EC2 P-family On-Demand and Spot quotas were `0` vCPUs and quota requests were pending; no EC2 GPU resource was launched. |

## Selection And Runtime Boundary

- Z.ai and Bedrock are selected through provider profile/registry expansion and
  invoked through the ADL provider-adapter boundary, not bespoke one-off HTTP
  scripts.
- Bedrock Nova Pro profile expansion now maps `bedrock:nova-pro-v1` to the
  accepted inference-profile id `us.amazon.nova-pro-v1:0`.
- Z.ai profile expansion maps `z_ai:glm-5` to the canonical model ref
  `hosted:adl-z-ai:glm-5` and provider model id `glm-5`.
- Fable 5 is recorded as an ad-hoc ADL provider-adapter UTS target rather than
  a canonical provider profile.
- DSpark remains outside scheduler-accepted provider routing until a real
  backend proves draft/target behavior and live capacity constraints are
  cleared.

## Negative Cases

| Case | Disposition |
| --- | --- |
| Bedrock direct Nova Pro on-demand id | `EXPECTED_BLOCKED`; `amazon.nova-pro-v1:0` was rejected for on-demand throughput, so the accepted path is `us.amazon.nova-pro-v1:0`. |
| Bedrock profile/account guardrail | Covered by the native Bedrock provider proof and #5026 AWS profile use; AWS work used `agent-logic-admin`. |
| Z.ai credential handling | Key contents and exact filename are not recorded; proof commands map an operator-approved key file under `$HOME/keys/` command-locally to `ZAI_API_KEY`. |
| Fable 5 mutation-shaped ordinary task | Recorded semantic caveat: `update_inventory_basic` refused in the regular lane while passing in the UTS-only dry-run lane. |
| DSpark AWS GPU capacity | `BLOCKED`; #4654 failed closed before launch because quota/shape gates were not satisfied. |

## Non-Claims

- This packet does not claim governed UTS+ACC lanes were run for #5026.
- This packet does not claim DSpark acceleration is proven.
- This packet does not claim Fable 5 has broad quality superiority or unrestricted
  autonomous repository-mutation suitability.
- This packet does not claim byte-stable live provider output.
