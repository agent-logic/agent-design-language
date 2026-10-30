# AWS-E resource adoption register

Issue: #488
Sprint: v0.92.1 Sprint 2
Profile for live readback: `agent-logic-admin`
Dependency: #487 terminal merge `1d31016a8df3cf07a4c3f2e6acd2694bd10570c2`

This register adopts the read-only AWS-A inventory denominator and records the
management authority for every admitted durable AWS resource surface without
performing any import, delete, tag, rewrite, or retirement action.

`dependency_487_terminal=true`

## Disposition vocabulary

- `retain` — preserve under current durable authority; no cleanup.
- `import` — candidate for a later Terraform import issue; not imported here.
- `replace` — candidate for later replacement; no replacement here.
- `retire-later` — candidate for later retirement with explicit authority.
- `ephemeral` — non-durable or disposable; deletion still requires proof.
- `frozen-unknown` — observed but not safely attributable; preserve until a
  later issue resolves ownership.

## One management authority invariant

Every row has exactly one management authority, exactly one current authority,
and one intended authority. When
authority is ambiguous, this register assigns `frozen-unknown` and prevents
cleanup rather than accepting dual management.

## Adopted denominator

The authoritative denominator is the AWS-A inventory at
`docs/operations/cloud/aws/inventory/AWS_RESOURCE_OWNERSHIP_INVENTORY.md`,
backed by readbacks in
`docs/milestones/v0.92.1/evidence/cloud/aws-a/readbacks/`.

Rows explicitly listed below are the high-signal durable groups needed by
Sprint 2. Every AWS-A resource row not explicitly listed inherits the
catch-all row `aws-a-inventory-remainder`, whose disposition is
`frozen-unknown` and whose deletion authority is `none`.

## Adoption rows

| Stable identity | Service | Region | Observed source | Current authority | Intended authority | Disposition | Evidence reference | Deletion authority | Retention recovery | Follow-on |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `account/713332525889` | account | global | AWS-A inventory | Agent Logic business account | Agent Logic cloud baseline | retain | `aws-a/readbacks/account-identity.json`; #487 AWS-D | none | not applicable | none |
| `regions/enabled-or-available-17` | regions | global | AWS-A inventory | AWS account denominator | AWS account denominator | retain | `aws-a/readbacks/regions.json` | none | not applicable | none |
| `infra/aws/bootstrap` | Terraform backend foundation | us-west-2 | #486 Terraform bootstrap | #486 Terraform bootstrap | #486 Terraform bootstrap | retain | `infra/aws/bootstrap/**`; `aws-c/state-isolation-register.md` | none | state recovery through #486 runbook | none |
| `infra/aws/account-foundation` | audit/security baseline | us-west-2/global | #487 Terraform/account foundation | #487 AWS-D | #487 AWS-D | retain | `infra/aws/account-foundation/**`; `aws-d/README.md` | none | audit bucket/KMS/CloudTrail recovery through #487 | none |
| `infra/aws/csm-public-edge` | public edge | us-west-2/global | #122 public exposure | #122 public exposure | #122 public exposure | retain | `infra/aws/csm-public-edge/**`; `aws-a/readbacks/global-tagged-resources.json` | none | Route53/ACM/API/CloudFront recovery through #122 | none |
| `infra/aws/csm-runtime-alb` | runtime ALB | us-west-2 | #122 runtime ALB root | #122 public exposure reference | #489 AWS Runtime platform modules | import | `infra/aws/csm-runtime-alb/**` | none in #488 | preserve until #489 owns platform modules | #489 |
| `infra/aws/csm-runtime-spot` | runtime spot instance root | us-west-2 | #122 runtime spot root | #122 public exposure reference | #489 AWS Runtime platform modules | import | `infra/aws/csm-runtime-spot/**` | none in #488 | preserve until #489 owns platform modules | #489 |
| `modules/csm-runtime-alb` | Terraform module | us-west-2 | #122 module source | #122 public exposure reference | #489/#495 module consumers | retain | `infra/aws/modules/csm-runtime-alb/**` | none | module source in Git | #489/#495 |
| `modules/csm-runtime-spot` | Terraform module | us-west-2 | #122 module source | #122 public exposure reference | #489/#495 module consumers | retain | `infra/aws/modules/csm-runtime-spot/**` | none | module source in Git | #489/#495 |
| `s3/adl-aws-remote-tool-cache-agentlogic` | S3 bucket | global | AWS-A inventory | frozen pending owner proof | frozen pending owner proof | frozen-unknown | `aws-a/readbacks/s3-buckets.json` | none | require bucket inventory and object-retention review | follow-on if cleanup requested |
| `s3/adl-aws-remote-validation-cache-713332525889-us-west-2` | S3 bucket | global | AWS-A inventory | frozen pending owner proof | frozen pending owner proof | frozen-unknown | `aws-a/readbacks/s3-buckets.json` | none | require bucket inventory and object-retention review | follow-on if cleanup requested |
| `s3/adl-codefriend-build-cache` | S3 bucket | global | AWS-A inventory | frozen pending CodeFriend proof | frozen pending CodeFriend proof | frozen-unknown | `aws-a/readbacks/s3-buckets.json` | none | require CodeFriend build-cache retention review | follow-on if cleanup requested |
| `s3/adl-shepherd-model-artifacts-b05e1f4379b5c745-us-west-2` | S3 bucket | global | AWS-A inventory | frozen pending Shepherd proof | frozen pending Shepherd proof | frozen-unknown | `aws-a/readbacks/s3-buckets.json` | none | require model-artifact retention review | follow-on if cleanup requested |
| `s3/adl-wp08-obsmem-community-archive-b05e1f4379b5c745-us-west-2` | S3 bucket | global | live AWS-E readback | frozen pending Observatory/Memory proof | frozen pending Observatory/Memory proof | frozen-unknown | `aws-e-live-readback-summary.log`; `aws-a/readbacks/s3-buckets.json` | none | require Observatory/Memory archive retention review | follow-on if cleanup requested |
| `s3/agent-logic-ai-origin-agentlogic` | S3 bucket | global | AWS-A inventory | website/publication authority | website/publication authority | retain | `aws-a/readbacks/s3-buckets.json`; `aws-a/readbacks/global-tagged-resources.json` | none | website origin retention required | none |
| `s3/codefriend-ai-origin-agentlogic` | S3 bucket | global | AWS-A inventory | website/publication authority | website/publication authority | retain | `aws-a/readbacks/s3-buckets.json`; `aws-a/readbacks/global-tagged-resources.json` | none | website origin retention required | none |
| `s3/scr-agent-logic-ai-origin-agentlogic` | S3 bucket | global | AWS-A inventory | website/publication authority | website/publication authority | retain | `aws-a/readbacks/s3-buckets.json`; `aws-a/readbacks/global-tagged-resources.json` | none | website origin retention required | none |
| `s3/agent-logic-podcast-archive-agentlogic` | S3 bucket | global | AWS-A inventory | podcast/archive authority | podcast/archive authority | retain | `aws-a/readbacks/s3-buckets.json`; `aws-a/readbacks/global-tagged-resources.json` | none | archive retention required | none |
| `s3/agent-logic-strategic-cognitive-reserve-agentlogic` | S3 bucket | global | AWS-A inventory | strategic-cognitive-reserve authority | strategic-cognitive-reserve authority | retain | `aws-a/readbacks/s3-buckets.json`; `aws-a/readbacks/global-tagged-resources.json` | none | archive retention required | none |
| `s3/csm-wuji-dev-observatory-assets` | S3 bucket | global | AWS-A inventory | CSM/Observatory public edge | CSM/Observatory public edge | retain | `aws-a/readbacks/s3-buckets.json`; `aws-a/readbacks/global-tagged-resources.json` | none | Observatory asset retention required | none |
| `route53/csm.agent-logic.ai.` | Route53 hosted zone | global | AWS-A inventory | CSM public DNS | CSM public DNS | retain | `aws-a/readbacks/route53-hosted-zones.json` | none | DNS continuity required | none |
| `cloudfront/all-observed` | CloudFront distributions | global | AWS-A inventory | website/public-edge authority by alias/origin | website/public-edge authority by alias/origin | retain | `aws-a/readbacks/cloudfront-distributions.json` | none | alias/origin proof before any change | follow-on if ambiguous |
| `tagged/api-gateway-runtime-http` | API Gateway | us-west-2 | AWS-A tagged resources; #122 | #122 public edge | #122 public edge | retain | `aws-a/readbacks/global-tagged-resources.json`; `infra/aws/csm-public-edge/**` | none | public edge continuity required | none |
| `tagged/lambda-eventbridge-sns-notice` | Lambda/EventBridge/SNS notice resources | us-west-2 | AWS-A tagged resources; historical evidence | historical runtime/shepherd evidence | historical runtime/shepherd evidence | frozen-unknown | `aws-a/readbacks/global-tagged-resources.json` | none | preserve until owning issue proves non-use | follow-on if cleanup requested |
| `tagged/security-groups` | EC2 security groups | us-west-2 and regional defaults | AWS-A tagged and regional readbacks | mixed runtime/default/foundation authority | mixed runtime/default/foundation authority | frozen-unknown | `aws-a/readbacks/global-tagged-resources.json`; regional security-group readbacks | none | preserve until VPC/ALB/instance owner proves exact non-use | #489/#496 as applicable |
| `tagged/ssm-managed-instances` | SSM managed instances | us-west-2 | AWS-A tagged resources | runtime/shepherd host authority pending proof | runtime/shepherd host authority pending proof | frozen-unknown | `aws-a/readbacks/global-tagged-resources.json` | none | require host owner and dehydrate/retention proof | follow-on if cleanup requested |
| `tagged/ec2-instance-i-027183bbc454a62e3` | EC2 instance | us-west-2 | AWS-A tagged resources | runtime/spot smoke authority pending proof | runtime/spot smoke authority pending proof | frozen-unknown | `aws-a/readbacks/global-tagged-resources.json` | none | require instance purpose, volume, and log retention proof | #489 if platform-owned |
| `regional-default-networking` | default VPC/subnet/security-group surfaces | all enabled regions | AWS-A regional readbacks | AWS account regional default authority | AWS account regional default authority | frozen-unknown | `aws-a/readbacks/regions/*-{vpcs,subnets,security-groups}.json` | none | do not delete without regional dependency proof | follow-on if cleanup requested |
| `aws-a-inventory-remainder` | all other AWS-A rows | mixed | AWS-A inventory | frozen pending owner proof | frozen pending owner proof | frozen-unknown | `docs/operations/cloud/aws/inventory/AWS_RESOURCE_OWNERSHIP_INVENTORY.md` | none | each row requires exact owner/non-use/retention proof | follow-on if cleanup requested |

## Downstream boundaries

- Runtime platform modules remain #489.
- Cross-cloud Runtime Terraform conversion remains #495.
- CloudFormation retirement remains #496.
- This register is an input to those issues; it does not perform their actions.

## Cleanup gate

No row in this register grants deletion authority. A future cleanup issue must
prove all of the following for the exact stable identity:

1. exact non-use evidence;
2. retained evidence or state recovery path;
3. explicit operator deletion authority;
4. no website, public-edge, historical-evidence, Runtime platform, or
   CloudFormation-retirement ownership conflict.
