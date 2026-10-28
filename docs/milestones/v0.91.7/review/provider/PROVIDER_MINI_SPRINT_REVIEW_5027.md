# Provider Mini-Sprint Review - Issue #5027

Issue: `#5027 [v0.91.7][provider][sprint] Implement native Z.ai and AWS Bedrock providers`

Date: 2026-07-10

## Findings

No active blocking findings remain after this review repair.

Previously recorded review-record finding:

| Severity | Finding | Disposition |
| --- | --- | --- |
| P2 | The local `.adl` sprint review placeholder still said the #5027 review was not started, even though the sprint closeout packet and child issue evidence were already retained under `docs/milestones/v0.91.7/review/provider/`. | Fixed by this tracked review packet and the sprint-review register update. Release review must consume this tracked packet plus `PROVIDER_MINI_SPRINT_CLOSEOUT_5027.md`, not the ignored local placeholder. |

## Scope Reviewed

This packet reviews the #5027 provider mini-sprint closeout and retained
evidence. It does not execute new provider work, rerun live provider calls, or
expand the provider acceptance claims beyond the already retained packets.

Reviewed sprint children and follow-up:

| Issue | PR | State | Reviewed result |
| --- | ---: | --- | --- |
| #5024 | #5048 | closed / merged | Native AWS Bedrock adapter, profile guardrails, setup support, and Nova proof were retained. |
| #5025 | #5047 | closed / merged | Native Z.ai adapter, profile integration, setup support, and superseding #5026 live proof were retained. |
| #5044 | #5050 | closed / merged | Fable 5 UTS acceptance retained with the regular-lane semantic caveat visible. |
| #4653 | #5051 | closed / merged | DSpark Qwen/Gemma speculative-decoding evaluation retained as candidate-only evidence. |
| #4654 | #5052 | closed / merged | DeepSeek V4 Flash DSpark GPU smoke failed closed before launch on AWS quota and shape gates. |
| #5026 | #5061 | closed / merged | Shared provider acceptance matrix retained accepted Z.ai, Bedrock, and Fable rows. |
| #5075 | #5077 | closed / merged | Qwen/Gemma vLLM proof publication retained after #4653 closeout. |
| #5027 | #5078 | closed / merged | Sprint closeout packet retained and umbrella closed. |

## Evidence Summary

Retained review and proof packets:

- `docs/milestones/v0.91.7/review/provider/PROVIDER_MINI_SPRINT_CLOSEOUT_5027.md`
- `docs/milestones/v0.91.7/review/provider/PROVIDER_ACCEPTANCE_MATRIX_5026.md`
- `docs/milestones/v0.91.7/review/provider/PROVIDER_ACCEPTANCE_RELEASE_GATE_DISPOSITION_5026.yaml`
- `docs/milestones/v0.91.7/review/provider/AWS_BEDROCK_NATIVE_PROVIDER_PROOF_5024.md`
- `docs/milestones/v0.91.7/review/provider/Z_AI_NATIVE_PROVIDER_PROOF_5025.md`
- `docs/milestones/v0.91.7/review/provider/FABLE5_UTS_ACCEPTANCE_5044.md`
- `docs/milestones/v0.91.7/review/provider/QWEN_GEMMA_VLLM_SPECULATIVE_PROOF_5075.md`
- `docs/milestones/v0.91.7/review/provider/dspark_gpu_smoke_4654/README.md`

The retained acceptance matrix records:

- Z.ai GLM-5 accepted through `hosted:adl-z-ai:glm-5`.
- AWS Bedrock Nova Pro accepted through
  `hosted:adl-bedrock:us.amazon.nova-pro-v1:0`.
- Fable 5 accepted as an ad-hoc provider-adapter UTS target with one
  regular-lane semantic refusal.
- DSpark Qwen/Gemma as candidate evidence only, not an accepted provider route.
- DeepSeek V4 Flash DSpark GPU smoke as blocked before EC2 launch by AWS
  quota and instance-shape constraints.

## Review Coverage

| Lane | Status | Notes |
| --- | --- | --- |
| Gap analysis | no new finding | Required sprint outcomes are represented by closed child issues and retained proof packets. DSpark and DeepSeek rows are intentionally not accepted. |
| Code surface | no new docs-repair finding | Code review is consumed through merged child PRs #5047, #5048, #5050, #5051, #5052, #5061, #5077, and #5078. This repair changes review documentation only and does not re-review implementation code. |
| Docs and release truth | fixed | The stale local placeholder is superseded by this tracked packet and the register update. |
| Tests and validation adequacy | no new finding, residuals retained | Focused provider proofs are retained. Broad lane failures/non-proving attempts are recorded as outside the provider acceptance surface where applicable. |
| Evidence and closeout | no new finding | Child issue, PR, and umbrella closure truth agree with the retained closeout packet. |
| Redaction and secret hygiene | no new finding in retained docs | Packets describe approved key/profile sources without retaining provider secrets, AWS credentials, raw account ids, or authorization headers. |

## Residual Risk

- Full-repo test absence is not claimed. The provider acceptance disposition
  records a full-nextest attempt that failed outside the provider surface and
  relies on focused provider proof plus GitHub CI for merged PR heads.
- Fable 5 is not promoted to broad model-quality superiority or autonomous
  repository-mutation suitability.
- DSpark acceleration, Gemma speculative generation, and DeepSeek V4 Flash
  DSpark live inference remain unproven.
- The ignored local `.adl` placeholder may still exist in some operator
  checkouts. It is not release-consumed evidence.

## Non-Claims

- This review does not claim v0.91.7 release readiness.
- This review does not claim new live provider invocation beyond the retained
  proof packets.
- This review does not create or close issues.
- This review does not approve future AWS resource launch after quota approval.
- This review does not treat candidate-only DSpark evidence as accepted
  provider readiness.
