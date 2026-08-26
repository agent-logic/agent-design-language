# AWS Resource Ownership Inventory

Issue: #484 / AWS-A

Scope: Agent Logic business AWS account, read-only inventory only

Profile: `agent-logic-admin`

Account: `713332525889`

Caller principal: `arn:aws:iam::713332525889:user/daniel.austin.admin`

Enabled/available region denominator: 17

## Disposition vocabulary

- `owned` — Agent Logic/ADL-owned resource or resource family.
- `externally-owned` — known non-ADL resource that must not be changed by ADL.
- `frozen-unknown` — discovered but not yet safely attributable; preserve until classified.
- `not-observed` — read-only census found no resource on that inspected surface.
- `read-failed` — read-only inventory for this surface failed and must be retried before mutation or cleanup.

## Evidence packet

- Readbacks: `docs/milestones/v0.92.1/evidence/cloud/aws-a/readbacks/`
- Command manifest: `docs/milestones/v0.92.1/evidence/cloud/aws-a/readbacks/command-manifest.md`

## Resource inventory

| Surface | Region | Resource | Disposition | Evidence |
| --- | --- | --- | --- | --- |
| account | global | `713332525889` | owned | `readbacks/account-identity.json` |
| regions | all | `17 enabled/available regions` | owned | `readbacks/regions.json` |
| s3-bucket | global | `adl-aws-remote-tool-cache-agentlogic` | frozen-unknown | `readbacks/s3-buckets.json` |
| s3-bucket | global | `adl-aws-remote-validation-cache-713332525889-us-west-2` | frozen-unknown | `readbacks/s3-buckets.json` |
| s3-bucket | global | `adl-codefriend-build-cache` | frozen-unknown | `readbacks/s3-buckets.json` |
| s3-bucket | global | `adl-shepherd-model-artifacts-b05e1f4379b5c745-us-west-2` | frozen-unknown | `readbacks/s3-buckets.json` |
| s3-bucket | global | `adl-wp08-obsmem-community-archive-b05e1f4379b5c745-us-west-2` | frozen-unknown | `readbacks/s3-buckets.json` |
| s3-bucket | global | `agent-logic-ai-origin-agentlogic` | frozen-unknown | `readbacks/s3-buckets.json` |
| s3-bucket | global | `agent-logic-podcast-archive-agentlogic` | frozen-unknown | `readbacks/s3-buckets.json` |
| s3-bucket | global | `agent-logic-strategic-cognitive-reserve-agentlogic` | frozen-unknown | `readbacks/s3-buckets.json` |
| s3-bucket | global | `codefriend-ai-origin-agentlogic` | frozen-unknown | `readbacks/s3-buckets.json` |
| s3-bucket | global | `csm-wuji-dev-observatory-assets` | frozen-unknown | `readbacks/s3-buckets.json` |
| s3-bucket | global | `scr-agent-logic-ai-origin-agentlogic` | frozen-unknown | `readbacks/s3-buckets.json` |
| route53-zone | global | `csm.agent-logic.ai.` | frozen-unknown | `readbacks/route53-hosted-zones.json` |
| cloudfront-distribution | global | `E1QMUEXPA12TDK` | frozen-unknown | `readbacks/cloudfront-distributions.json` |
| cloudfront-distribution | global | `E3C29FMX32KDDU` | frozen-unknown | `readbacks/cloudfront-distributions.json` |
| cloudfront-distribution | global | `E34IBPFTBM0242` | frozen-unknown | `readbacks/cloudfront-distributions.json` |
| cloudfront-distribution | global | `E33B60VD3JG6BI` | frozen-unknown | `readbacks/cloudfront-distributions.json` |
| cloudfront-distribution | global | `E2P8CMPYZNLKVX` | frozen-unknown | `readbacks/cloudfront-distributions.json` |
| cloudfront-distribution | global | `E2A4Y69MBQG519` | frozen-unknown | `readbacks/cloudfront-distributions.json` |
