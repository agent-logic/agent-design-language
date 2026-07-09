# DeepSeek V4 Flash DSpark GPU Smoke - Issue #4654

Issue: `#4654 [v0.91.7][aws][gpu][dspark] Smoke test deepseek-v4-flash-dspark on ephemeral 2xH100 EC2`

Date: 2026-07-07

## Result

Status: `blocked_p_family_quota_zero_or_pending`

The live AWS and model preflight ran under the required Agent Logic business
profile, but the actual EC2 H100 smoke could not be launched safely:

- AWS profile: `agent-logic-admin`
- AWS region: `us-west-2`
- AWS profile resolved: `true`
- AWS account identifier or pseudonym recorded: `false`
- Raw AWS account id recorded: `false`
- Credentials recorded: `false`
- Existing running/stopped P/G GPU instances found: `0`
- On-Demand P quota: `0` vCPUs
- P Spot quota: `0` vCPUs
- On-Demand P quota request for `32` vCPUs: `PENDING`
- P Spot quota request for `32` vCPUs: `PENDING`

## Model Evidence

The target model exists and is publicly discoverable:

- Model id: `deepseek-ai/DeepSeek-V4-Flash-DSpark`
- Public/private gate: `private=false`, `gated=false`, `disabled=false`
- Hugging Face revision observed: `62af8fffb2f7030cac4de2f0169f5b8d1101b646`
- Last modified: `2026-07-04T03:15:12.000Z`
- Library: `transformers`
- Pipeline: `text-generation`
- Retained storage estimate: `166886535336` bytes
- Safetensors shard count: `48`

## EC2 Shape Evidence

Live EC2 shape discovery found:

| Shape | GPUs | vCPUs | Issue Fit |
| --- | ---: | ---: | --- |
| `p5.4xlarge` | 1 H100 | 16 | Two instances satisfy the literal GPU count, but not one shared-memory multi-GPU inference node. |
| `p5.48xlarge` | 8 H100 | 192 | Single-node H100 fallback for a large FP8 DeepSeek V4 smoke, but larger than the issue title and requires 192 P vCPUs. |

No exact single-node `2xH100` EC2 offering was found in `us-west-2`.

## Actions Taken

- Verified the required AWS profile resolves through STS without recording the
  raw account id or a reversible account pseudonym.
- Queried live EC2 P-family instance shapes and availability-zone offerings.
- Queried live EC2 P-family On-Demand and Spot service quotas.
- Confirmed no P/G GPU instances are currently running or stopped in the account.
- Confirmed EC2 P-family quota increase requests for `32` vCPUs are pending:
  - `Running On-Demand P instances`
  - `All P Spot Instance Requests`
- Verified duplicate quota request attempts fail closed with
  `ResourceAlreadyExistsException`, confirming one open request per quota.
- Queried Hugging Face live model metadata for
  `deepseek-ai/DeepSeek-V4-Flash-DSpark`.

## Retained Artifact

Machine-readable proof:

```text
docs/milestones/v0.91.7/review/provider/dspark_gpu_smoke_4654/preflight_summary.json
```

Tooling:

```text
adl/tools/provider/run_dspark_gpu_smoke_preflight.sh
```

## Validation

```text
ADL_AWS_PROFILE=agent-logic-admin AWS_REGION=us-west-2 \
  bash adl/tools/provider/run_dspark_gpu_smoke_preflight.sh \
  --request-quota \
  --out docs/milestones/v0.91.7/review/provider/dspark_gpu_smoke_4654/preflight_summary.json
```

Result: `PASS`, retained JSON decision
`blocked_p_family_quota_zero_or_pending`.

## Non-Claims

- This issue did not launch EC2 GPU instances.
- This issue did not run DeepSeek V4 Flash DSpark inference.
- This issue does not claim the 2xH100 smoke passed.
- The pending `32` vCPU quota requests are sufficient for two `p5.4xlarge`
  instances, but not for the likely single-node `p5.48xlarge` fallback.

## Required Follow-Up

After quota approval, choose the operator-approved runtime shape:

- Literal two-H100 interpretation: two `p5.4xlarge` instances, with the caveat
  that this is not a single-node tensor-parallel inference environment.
- Practical single-node fallback: one `p5.48xlarge`, requiring a larger `192`
  P-vCPU quota request.

Only after the quota and shape are approved should an agent launch ephemeral
resources, run a bounded model load/generation smoke, retain logs, and terminate
all created resources.
