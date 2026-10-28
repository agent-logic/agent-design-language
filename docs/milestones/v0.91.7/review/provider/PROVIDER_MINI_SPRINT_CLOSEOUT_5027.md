# Provider Mini-Sprint Closeout - Issue #5027

Issue: `#5027 [v0.91.7][provider][sprint] Implement native Z.ai and AWS Bedrock providers`

Date: 2026-07-10

## Summary

The #5027 provider mini-sprint is ready to close. Its implementation and proof
children are closed, and the shared acceptance matrix records the accepted
provider/model rows without overstating DSpark, Gemma, or broad model-quality
claims.

The canonical provider acceptance matrix is
`docs/milestones/v0.91.7/review/provider/PROVIDER_ACCEPTANCE_MATRIX_5026.md`.

## Child Issue Closeout Truth

| Issue | State | Result |
| --- | --- | --- |
| #5024 | closed | Native AWS Bedrock provider adapter landed with Agent Logic AWS profile guardrails, provider registry/profile integration, and Nova Pro proof. |
| #5025 | closed | Native Z.ai provider adapter landed with provider registry/profile integration and GLM-5 proof. |
| #5044 | closed | Fable 5 UTS acceptance path landed as an ad-hoc Anthropic provider-adapter target. |
| #5026 | closed | Shared provider acceptance matrix landed and rolls up accepted provider/model rows. |
| #4653 | closed | DSpark speculative-decoding evaluation landed as conservative candidate evidence. |
| #4654 | closed | DeepSeek V4 Flash DSpark GPU smoke failed closed before launch on AWS quota/shape gates. |
| #5075 | closed | Follow-up Qwen/Gemma vLLM proof publication landed after #4653 had already closed. |

## Accepted Provider Rows

| Row | Provider route | Status | Proof |
| --- | --- | --- | --- |
| Z.ai GLM-5 | `hosted:adl-z-ai:glm-5` | accepted live provider path | `Z_AI_ACCEPTANCE_5026.md` |
| AWS Bedrock Nova Pro | `hosted:adl-bedrock:us.amazon.nova-pro-v1:0` | accepted live provider path | `BEDROCK_NOVA_PRO_ACCEPTANCE_5026.md` |
| Fable 5 | `hosted:adl-anthropic:claude-fable-5` | accepted ad-hoc provider-adapter UTS target with semantic caveat | `FABLE5_UTS_ACCEPTANCE_5044.md` |

## Non-Accepted Rows

- DSpark Qwen/Gemma remains candidate evidence only. Qwen vLLM speculative
  execution worked and exposed accepted-token counters, but was slower than
  target-only generation for the measured pair. Gemma authenticated, loaded, and
  compiled, then failed before generation during vLLM KV-cache initialization.
- DeepSeek V4 Flash DSpark GPU smoke remains blocked by external AWS quota and
  instance-shape constraints. #4654 did not launch an EC2 GPU resource.

## Validation And Proof Surfaces

- Bedrock native provider proof:
  `docs/milestones/v0.91.7/review/provider/AWS_BEDROCK_NATIVE_PROVIDER_PROOF_5024.md`
- Z.ai native provider proof:
  `docs/milestones/v0.91.7/review/provider/Z_AI_NATIVE_PROVIDER_PROOF_5025.md`
- Fable 5 UTS proof:
  `docs/milestones/v0.91.7/review/provider/FABLE5_UTS_ACCEPTANCE_5044.md`
- Shared provider acceptance matrix:
  `docs/milestones/v0.91.7/review/provider/PROVIDER_ACCEPTANCE_MATRIX_5026.md`
- Shared release-gate disposition:
  `docs/milestones/v0.91.7/review/provider/PROVIDER_ACCEPTANCE_RELEASE_GATE_DISPOSITION_5026.yaml`
- Qwen/Gemma follow-up proof:
  `docs/milestones/v0.91.7/review/provider/QWEN_GEMMA_VLLM_SPECULATIVE_PROOF_5075.md`
- DSpark GPU blocked proof:
  `docs/milestones/v0.91.7/review/provider/dspark_gpu_smoke_4654/README.md`

Focused closeout checks:

```text
git diff --check
adl tooling validate-structured-prompt --type sor --phase execution --input .adl/v0.91.7/tasks/issue-5027__v0-91-7-provider-sprint-implement-native-z-ai-and-aws-bedrock-providers/sor.md
ADL_GITHUB_TOKEN_FILE=$HOME/keys/github.token bash adl/tools/pr.sh issue search --query "repo:danielbaustin/agent-design-language is:issue is:open label:area:provider label:version:v0.91.7" --json
```

The open-provider issue search returned only #5027 before this closeout PR.

## Redaction And Non-Claims

- No provider secret value, AWS credential, account secret, or authorization
  header is retained in this closeout packet.
- Credential sources are described only as approved command-local inputs under
  `$HOME/keys` or the Agent Logic AWS profile policy.
- This sprint does not claim broad benchmark superiority, release authority,
  merge authority, autonomous repository mutation authority, DSpark
  acceleration, Gemma speculative-generation success, or byte-stable live
  provider output.
