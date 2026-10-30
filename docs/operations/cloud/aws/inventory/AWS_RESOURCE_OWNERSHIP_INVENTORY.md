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
| tagged-resource | global | `arn:aws:ec2:us-west-2:713332525889:security-group/sg-0e1c37b8b950925aa` | frozen-unknown | `readbacks/global-tagged-resources.json` |
| tagged-resource | global | `arn:aws:ec2:us-west-2:713332525889:security-group/sg-0d4b1eb7765fd8ff3` | frozen-unknown | `readbacks/global-tagged-resources.json` |
| tagged-resource | global | `arn:aws:ec2:us-west-2:713332525889:security-group/sg-09cfab23732c43a4d` | frozen-unknown | `readbacks/global-tagged-resources.json` |
| tagged-resource | global | `arn:aws:ec2:us-west-2:713332525889:security-group/sg-011d94b6aa33d5239` | frozen-unknown | `readbacks/global-tagged-resources.json` |
| tagged-resource | global | `arn:aws:ec2:us-west-2:713332525889:security-group/sg-09fc52684a96ead6a` | frozen-unknown | `readbacks/global-tagged-resources.json` |
| tagged-resource | global | `arn:aws:s3:::codefriend-ai-origin-agentlogic` | frozen-unknown | `readbacks/global-tagged-resources.json` |
| tagged-resource | global | `arn:aws:s3:::agent-logic-ai-origin-agentlogic` | frozen-unknown | `readbacks/global-tagged-resources.json` |
| tagged-resource | global | `arn:aws:ssm:us-west-2:713332525889:managed-instance/mi-0dd41a2b1cad222a0` | frozen-unknown | `readbacks/global-tagged-resources.json` |
| tagged-resource | global | `arn:aws:ec2:us-west-2:713332525889:security-group/sg-0833ffb665cedaf0d` | frozen-unknown | `readbacks/global-tagged-resources.json` |
| tagged-resource | global | `arn:aws:ec2:us-west-2:713332525889:security-group/sg-01474cd05a7109f1d` | frozen-unknown | `readbacks/global-tagged-resources.json` |
| tagged-resource | global | `arn:aws:ec2:us-west-2:713332525889:snapshot/snap-09003211cc2600dff` | frozen-unknown | `readbacks/global-tagged-resources.json` |
| tagged-resource | global | `arn:aws:s3:::scr-agent-logic-ai-origin-agentlogic` | frozen-unknown | `readbacks/global-tagged-resources.json` |
| tagged-resource | global | `arn:aws:s3:::agent-logic-podcast-archive-agentlogic` | frozen-unknown | `readbacks/global-tagged-resources.json` |
| tagged-resource | global | `arn:aws:codebuild:us-west-2:713332525889:project/adl-codefriend-build` | frozen-unknown | `readbacks/global-tagged-resources.json` |
| tagged-resource | global | `arn:aws:logs:us-west-2:713332525889:log-group:/aws/apigateway/csm-wuji-dev-runtime-http` | frozen-unknown | `readbacks/global-tagged-resources.json` |
| tagged-resource | global | `arn:aws:ec2:us-west-2:713332525889:security-group/sg-0a271b4781cd616e8` | frozen-unknown | `readbacks/global-tagged-resources.json` |
| tagged-resource | global | `arn:aws:ec2:us-west-2:713332525889:security-group/sg-0e230571cdd126919` | frozen-unknown | `readbacks/global-tagged-resources.json` |
| tagged-resource | global | `arn:aws:ec2:us-west-2:713332525889:security-group/sg-0d8e37e4f26741263` | frozen-unknown | `readbacks/global-tagged-resources.json` |
| tagged-resource | global | `arn:aws:events:us-west-2:713332525889:event-bus/adl-csm` | frozen-unknown | `readbacks/global-tagged-resources.json` |
| tagged-resource | global | `arn:aws:ec2:us-west-2:713332525889:security-group/sg-052e1b4273335e5f7` | frozen-unknown | `readbacks/global-tagged-resources.json` |
| tagged-resource | global | `arn:aws:ec2:us-west-2:713332525889:security-group/sg-01a98c8e210b62c85` | frozen-unknown | `readbacks/global-tagged-resources.json` |
| tagged-resource | global | `arn:aws:events:us-west-2:713332525889:rule/adl-ec2-instance-age-12h-hourly` | frozen-unknown | `readbacks/global-tagged-resources.json` |
| tagged-resource | global | `arn:aws:s3:::agent-logic-strategic-cognitive-reserve-agentlogic` | frozen-unknown | `readbacks/global-tagged-resources.json` |
| tagged-resource | global | `arn:aws:apigateway:us-west-2::/apis/tb485bn6j4/stages/$default` | frozen-unknown | `readbacks/global-tagged-resources.json` |
| tagged-resource | global | `arn:aws:ec2:us-west-2:713332525889:security-group/sg-0b7ccf07b35efa40f` | frozen-unknown | `readbacks/global-tagged-resources.json` |
| tagged-resource | global | `arn:aws:sns:us-west-2:713332525889:adl-v0917-csm-governed-notice-4998` | frozen-unknown | `readbacks/global-tagged-resources.json` |
| tagged-resource | global | `arn:aws:ec2:us-west-2:713332525889:security-group/sg-01df232252551f607` | frozen-unknown | `readbacks/global-tagged-resources.json` |
| tagged-resource | global | `arn:aws:ssm:us-west-2:713332525889:managed-instance/mi-0538c965d11eae809` | frozen-unknown | `readbacks/global-tagged-resources.json` |
| tagged-resource | global | `arn:aws:ec2:us-west-2:713332525889:security-group/sg-05df85829752c6cd6` | frozen-unknown | `readbacks/global-tagged-resources.json` |
| tagged-resource | global | `arn:aws:ec2:us-west-2:713332525889:security-group/sg-0f71e25c9b6b29862` | frozen-unknown | `readbacks/global-tagged-resources.json` |
| tagged-resource | global | `arn:aws:ec2:us-west-2:713332525889:security-group/sg-046ff6902fb0f8cc0` | frozen-unknown | `readbacks/global-tagged-resources.json` |
| tagged-resource | global | `arn:aws:ec2:us-west-2:713332525889:security-group/sg-063bdaf36c2e3b0d2` | frozen-unknown | `readbacks/global-tagged-resources.json` |
| tagged-resource | global | `arn:aws:events:us-west-2:713332525889:rule/adl-ebs-unattached-age-5d-daily` | frozen-unknown | `readbacks/global-tagged-resources.json` |
| tagged-resource | global | `arn:aws:lambda:us-west-2:713332525889:function:adl-csm-5039-api-gateway-bridge` | frozen-unknown | `readbacks/global-tagged-resources.json` |
| tagged-resource | global | `arn:aws:apigateway:us-west-2::/apis/8fej3k7qt5` | frozen-unknown | `readbacks/global-tagged-resources.json` |
| tagged-resource | global | `arn:aws:ec2:us-west-2:713332525889:security-group/sg-003d8cb7246067499` | frozen-unknown | `readbacks/global-tagged-resources.json` |
| tagged-resource | global | `arn:aws:ec2:us-west-2:713332525889:security-group/sg-0535963eaec330b12` | frozen-unknown | `readbacks/global-tagged-resources.json` |
| tagged-resource | global | `arn:aws:ec2:us-west-2:713332525889:security-group/sg-08ea8f565ec5d4985` | frozen-unknown | `readbacks/global-tagged-resources.json` |
| tagged-resource | global | `arn:aws:ec2:us-west-2:713332525889:security-group/sg-09571038c2d412bb2` | frozen-unknown | `readbacks/global-tagged-resources.json` |
| tagged-resource | global | `arn:aws:ec2:us-west-2:713332525889:security-group/sg-041f08aeea5391e02` | frozen-unknown | `readbacks/global-tagged-resources.json` |
| tagged-resource | global | `arn:aws:acm:us-west-2:713332525889:certificate/8ba373ca-c226-4de3-bfb5-9da44c28f338` | frozen-unknown | `readbacks/global-tagged-resources.json` |
| tagged-resource | global | `arn:aws:ec2:us-west-2:713332525889:instance/i-027183bbc454a62e3` | frozen-unknown | `readbacks/global-tagged-resources.json` |
| tagged-resource | global | `arn:aws:ec2:us-west-2:713332525889:security-group/sg-0ef287ea7be2cbdf7` | frozen-unknown | `readbacks/global-tagged-resources.json` |
| tagged-resource | global | `arn:aws:ec2:us-west-2:713332525889:security-group/sg-0108df7bb1ab37ffd` | frozen-unknown | `readbacks/global-tagged-resources.json` |
| tagged-resource | global | `arn:aws:events:us-west-2:713332525889:event-bus/adl-csm-notice-bus-4998` | frozen-unknown | `readbacks/global-tagged-resources.json` |
| tagged-resource | global | `arn:aws:events:us-west-2:713332525889:rule/adl-csm-notice-bus-4998/adl-csm-notice-to-lambda-4998` | frozen-unknown | `readbacks/global-tagged-resources.json` |
| tagged-resource | global | `arn:aws:lambda:us-west-2:713332525889:function:adl-ebs-unattached-age-5d-alert` | frozen-unknown | `readbacks/global-tagged-resources.json` |
| tagged-resource | global | `arn:aws:sns:us-west-2:713332525889:adl-v0917-wp08-acip-sns-4685` | frozen-unknown | `readbacks/global-tagged-resources.json` |
| tagged-resource | global | `arn:aws:ec2:us-west-2:713332525889:security-group/sg-0c5c2cdb816a3ee36` | frozen-unknown | `readbacks/global-tagged-resources.json` |
| tagged-resource | global | `arn:aws:ec2:us-west-2:713332525889:security-group/sg-0010f099186c8f9eb` | frozen-unknown | `readbacks/global-tagged-resources.json` |
| tagged-resource | global | `arn:aws:apigateway:us-west-2::/apis/tb485bn6j4` | frozen-unknown | `readbacks/global-tagged-resources.json` |
| tagged-resource | global | `arn:aws:cloudformation:us-west-2:713332525889:stack/adl-ebs-unattached-age-alert/c7192a10-a0a1-11f1-8445-0a1da76cb771` | frozen-unknown | `readbacks/global-tagged-resources.json` |
| tagged-resource | global | `arn:aws:ec2:us-west-2:713332525889:security-group/sg-0d790aabae60b5d7a` | frozen-unknown | `readbacks/global-tagged-resources.json` |
| tagged-resource | global | `arn:aws:events:us-west-2:713332525889:rule/adl-wp5795-gpu-deadline-reaper` | frozen-unknown | `readbacks/global-tagged-resources.json` |
| tagged-resource | global | `arn:aws:ec2:us-west-2:713332525889:security-group/sg-03e9677b1cb483f25` | frozen-unknown | `readbacks/global-tagged-resources.json` |
| tagged-resource | global | `arn:aws:ec2:us-west-2:713332525889:security-group/sg-03bf9803b091d0bfb` | frozen-unknown | `readbacks/global-tagged-resources.json` |
| tagged-resource | global | `arn:aws:events:us-west-2:713332525889:rule/adl-csm/adl-csm-api-gateway-bridge-5039` | frozen-unknown | `readbacks/global-tagged-resources.json` |
| tagged-resource | global | `arn:aws:lambda:us-west-2:713332525889:function:adl-csm-notice-receiver-4998` | frozen-unknown | `readbacks/global-tagged-resources.json` |
| tagged-resource | global | `arn:aws:s3:::csm-wuji-dev-observatory-assets` | frozen-unknown | `readbacks/global-tagged-resources.json` |
| acm-certificate | ap-northeast-1 | `none observed` | not-observed | `readbacks/regions/ap-northeast-1-acm-certificates.json` |
| cloudformation-stack | ap-northeast-1 | `none observed` | not-observed | `readbacks/regions/ap-northeast-1-cloudformation-stacks.json` |
| ec2-instance | ap-northeast-1 | `none observed` | not-observed | `readbacks/regions/ap-northeast-1-ec2-instances.json` |
| ebs-volume | ap-northeast-1 | `none observed` | not-observed | `readbacks/regions/ap-northeast-1-ec2-volumes.json` |
| load-balancer | ap-northeast-1 | `none observed` | not-observed | `readbacks/regions/ap-northeast-1-load-balancers.json` |
| security-group | ap-northeast-1 | `sg-06cd23a4aa3f86bda` | frozen-unknown | `readbacks/regions/ap-northeast-1-security-groups.json` |
| subnet | ap-northeast-1 | `subnet-0783ba9e67dc51036` | frozen-unknown | `readbacks/regions/ap-northeast-1-subnets.json` |
| subnet | ap-northeast-1 | `subnet-0d2d99674060e3def` | frozen-unknown | `readbacks/regions/ap-northeast-1-subnets.json` |
| subnet | ap-northeast-1 | `subnet-0125097d61b922015` | frozen-unknown | `readbacks/regions/ap-northeast-1-subnets.json` |
| vpc | ap-northeast-1 | `vpc-0d2f1b7ed0944d450` | frozen-unknown | `readbacks/regions/ap-northeast-1-vpcs.json` |
| acm-certificate | ap-northeast-2 | `none observed` | not-observed | `readbacks/regions/ap-northeast-2-acm-certificates.json` |
| cloudformation-stack | ap-northeast-2 | `none observed` | not-observed | `readbacks/regions/ap-northeast-2-cloudformation-stacks.json` |
| ec2-instance | ap-northeast-2 | `none observed` | not-observed | `readbacks/regions/ap-northeast-2-ec2-instances.json` |
| ebs-volume | ap-northeast-2 | `none observed` | not-observed | `readbacks/regions/ap-northeast-2-ec2-volumes.json` |
| load-balancer | ap-northeast-2 | `none observed` | not-observed | `readbacks/regions/ap-northeast-2-load-balancers.json` |
| security-group | ap-northeast-2 | `sg-04eb1a19830585fdc` | frozen-unknown | `readbacks/regions/ap-northeast-2-security-groups.json` |
| subnet | ap-northeast-2 | `subnet-01ac92b55af52fcf7` | frozen-unknown | `readbacks/regions/ap-northeast-2-subnets.json` |
| subnet | ap-northeast-2 | `subnet-0f2e0bbc6094cbd2b` | frozen-unknown | `readbacks/regions/ap-northeast-2-subnets.json` |
| subnet | ap-northeast-2 | `subnet-0fa31028d86bc7b43` | frozen-unknown | `readbacks/regions/ap-northeast-2-subnets.json` |
| subnet | ap-northeast-2 | `subnet-0d3489d84c2d34075` | frozen-unknown | `readbacks/regions/ap-northeast-2-subnets.json` |
| vpc | ap-northeast-2 | `vpc-0c72f0309c024d767` | frozen-unknown | `readbacks/regions/ap-northeast-2-vpcs.json` |
| acm-certificate | ap-northeast-3 | `none observed` | not-observed | `readbacks/regions/ap-northeast-3-acm-certificates.json` |
| cloudformation-stack | ap-northeast-3 | `none observed` | not-observed | `readbacks/regions/ap-northeast-3-cloudformation-stacks.json` |
| ec2-instance | ap-northeast-3 | `none observed` | not-observed | `readbacks/regions/ap-northeast-3-ec2-instances.json` |
| ebs-volume | ap-northeast-3 | `none observed` | not-observed | `readbacks/regions/ap-northeast-3-ec2-volumes.json` |
| load-balancer | ap-northeast-3 | `none observed` | not-observed | `readbacks/regions/ap-northeast-3-load-balancers.json` |
| security-group | ap-northeast-3 | `sg-0ed23095a3c5c83d6` | frozen-unknown | `readbacks/regions/ap-northeast-3-security-groups.json` |
| subnet | ap-northeast-3 | `subnet-0d75f181a119c5c75` | frozen-unknown | `readbacks/regions/ap-northeast-3-subnets.json` |
| subnet | ap-northeast-3 | `subnet-034d0cecf57237fe8` | frozen-unknown | `readbacks/regions/ap-northeast-3-subnets.json` |
| subnet | ap-northeast-3 | `subnet-0621bd687c8f21986` | frozen-unknown | `readbacks/regions/ap-northeast-3-subnets.json` |
| vpc | ap-northeast-3 | `vpc-0fc31ed266e9abac8` | frozen-unknown | `readbacks/regions/ap-northeast-3-vpcs.json` |
| acm-certificate | ap-south-1 | `none observed` | not-observed | `readbacks/regions/ap-south-1-acm-certificates.json` |
| cloudformation-stack | ap-south-1 | `none observed` | not-observed | `readbacks/regions/ap-south-1-cloudformation-stacks.json` |
| ec2-instance | ap-south-1 | `none observed` | not-observed | `readbacks/regions/ap-south-1-ec2-instances.json` |
| ebs-volume | ap-south-1 | `none observed` | not-observed | `readbacks/regions/ap-south-1-ec2-volumes.json` |
| load-balancer | ap-south-1 | `none observed` | not-observed | `readbacks/regions/ap-south-1-load-balancers.json` |
| security-group | ap-south-1 | `sg-03762a3d7b95a5228` | frozen-unknown | `readbacks/regions/ap-south-1-security-groups.json` |
| subnet | ap-south-1 | `subnet-0d9334ab6ebe185aa` | frozen-unknown | `readbacks/regions/ap-south-1-subnets.json` |
| subnet | ap-south-1 | `subnet-04f85b223f136a5a6` | frozen-unknown | `readbacks/regions/ap-south-1-subnets.json` |
| subnet | ap-south-1 | `subnet-0ec566141108c8217` | frozen-unknown | `readbacks/regions/ap-south-1-subnets.json` |
| vpc | ap-south-1 | `vpc-0d93be8923c1b1a3e` | frozen-unknown | `readbacks/regions/ap-south-1-vpcs.json` |
| acm-certificate | ap-southeast-1 | `none observed` | not-observed | `readbacks/regions/ap-southeast-1-acm-certificates.json` |
| cloudformation-stack | ap-southeast-1 | `none observed` | not-observed | `readbacks/regions/ap-southeast-1-cloudformation-stacks.json` |
| ec2-instance | ap-southeast-1 | `none observed` | not-observed | `readbacks/regions/ap-southeast-1-ec2-instances.json` |
| ebs-volume | ap-southeast-1 | `none observed` | not-observed | `readbacks/regions/ap-southeast-1-ec2-volumes.json` |
| load-balancer | ap-southeast-1 | `none observed` | not-observed | `readbacks/regions/ap-southeast-1-load-balancers.json` |
| security-group | ap-southeast-1 | `sg-0711168131e34a576` | frozen-unknown | `readbacks/regions/ap-southeast-1-security-groups.json` |
| subnet | ap-southeast-1 | `subnet-019e1e5b8a7ba1b01` | frozen-unknown | `readbacks/regions/ap-southeast-1-subnets.json` |
| subnet | ap-southeast-1 | `subnet-0c384bcf1fef6bec7` | frozen-unknown | `readbacks/regions/ap-southeast-1-subnets.json` |
| subnet | ap-southeast-1 | `subnet-0afcfb785faa2d56c` | frozen-unknown | `readbacks/regions/ap-southeast-1-subnets.json` |
| vpc | ap-southeast-1 | `vpc-091d61081291a092b` | frozen-unknown | `readbacks/regions/ap-southeast-1-vpcs.json` |
| acm-certificate | ap-southeast-2 | `none observed` | not-observed | `readbacks/regions/ap-southeast-2-acm-certificates.json` |
| cloudformation-stack | ap-southeast-2 | `none observed` | not-observed | `readbacks/regions/ap-southeast-2-cloudformation-stacks.json` |
| ec2-instance | ap-southeast-2 | `none observed` | not-observed | `readbacks/regions/ap-southeast-2-ec2-instances.json` |
| ebs-volume | ap-southeast-2 | `none observed` | not-observed | `readbacks/regions/ap-southeast-2-ec2-volumes.json` |
| load-balancer | ap-southeast-2 | `none observed` | not-observed | `readbacks/regions/ap-southeast-2-load-balancers.json` |
| security-group | ap-southeast-2 | `sg-007159583fa8491ed` | frozen-unknown | `readbacks/regions/ap-southeast-2-security-groups.json` |
| subnet | ap-southeast-2 | `subnet-0bf328a3214809420` | frozen-unknown | `readbacks/regions/ap-southeast-2-subnets.json` |
| subnet | ap-southeast-2 | `subnet-04691f7c514c7edd7` | frozen-unknown | `readbacks/regions/ap-southeast-2-subnets.json` |
| subnet | ap-southeast-2 | `subnet-069145ca80bce1477` | frozen-unknown | `readbacks/regions/ap-southeast-2-subnets.json` |
| vpc | ap-southeast-2 | `vpc-06df7eb9148dff694` | frozen-unknown | `readbacks/regions/ap-southeast-2-vpcs.json` |
| acm-certificate | ca-central-1 | `none observed` | not-observed | `readbacks/regions/ca-central-1-acm-certificates.json` |
| cloudformation-stack | ca-central-1 | `none observed` | not-observed | `readbacks/regions/ca-central-1-cloudformation-stacks.json` |
| ec2-instance | ca-central-1 | `none observed` | not-observed | `readbacks/regions/ca-central-1-ec2-instances.json` |
| ebs-volume | ca-central-1 | `none observed` | not-observed | `readbacks/regions/ca-central-1-ec2-volumes.json` |
| load-balancer | ca-central-1 | `none observed` | not-observed | `readbacks/regions/ca-central-1-load-balancers.json` |
| security-group | ca-central-1 | `sg-0234e788758923ff4` | frozen-unknown | `readbacks/regions/ca-central-1-security-groups.json` |
| subnet | ca-central-1 | `subnet-0a8c6918c303d9885` | frozen-unknown | `readbacks/regions/ca-central-1-subnets.json` |
| subnet | ca-central-1 | `subnet-01399da327cdb67ea` | frozen-unknown | `readbacks/regions/ca-central-1-subnets.json` |
| subnet | ca-central-1 | `subnet-04a59127a551bfaf5` | frozen-unknown | `readbacks/regions/ca-central-1-subnets.json` |
| vpc | ca-central-1 | `vpc-03b86a0ec5055a0a5` | frozen-unknown | `readbacks/regions/ca-central-1-vpcs.json` |
| acm-certificate | eu-central-1 | `none observed` | not-observed | `readbacks/regions/eu-central-1-acm-certificates.json` |
| cloudformation-stack | eu-central-1 | `none observed` | not-observed | `readbacks/regions/eu-central-1-cloudformation-stacks.json` |
| ec2-instance | eu-central-1 | `none observed` | not-observed | `readbacks/regions/eu-central-1-ec2-instances.json` |
| ebs-volume | eu-central-1 | `none observed` | not-observed | `readbacks/regions/eu-central-1-ec2-volumes.json` |
| load-balancer | eu-central-1 | `none observed` | not-observed | `readbacks/regions/eu-central-1-load-balancers.json` |
| security-group | eu-central-1 | `sg-03e7e3210c58861ed` | frozen-unknown | `readbacks/regions/eu-central-1-security-groups.json` |
| subnet | eu-central-1 | `subnet-009b5889c7c4bf2bd` | frozen-unknown | `readbacks/regions/eu-central-1-subnets.json` |
| subnet | eu-central-1 | `subnet-0c3df1270155c12df` | frozen-unknown | `readbacks/regions/eu-central-1-subnets.json` |
| subnet | eu-central-1 | `subnet-0e3446be3a9092f9d` | frozen-unknown | `readbacks/regions/eu-central-1-subnets.json` |
| vpc | eu-central-1 | `vpc-042aeb210dbd26445` | frozen-unknown | `readbacks/regions/eu-central-1-vpcs.json` |
| acm-certificate | eu-north-1 | `none observed` | not-observed | `readbacks/regions/eu-north-1-acm-certificates.json` |
| cloudformation-stack | eu-north-1 | `none observed` | not-observed | `readbacks/regions/eu-north-1-cloudformation-stacks.json` |
| ec2-instance | eu-north-1 | `none observed` | not-observed | `readbacks/regions/eu-north-1-ec2-instances.json` |
| ebs-volume | eu-north-1 | `none observed` | not-observed | `readbacks/regions/eu-north-1-ec2-volumes.json` |
| load-balancer | eu-north-1 | `none observed` | not-observed | `readbacks/regions/eu-north-1-load-balancers.json` |
| security-group | eu-north-1 | `sg-0fa95efb51436c15e` | frozen-unknown | `readbacks/regions/eu-north-1-security-groups.json` |
| subnet | eu-north-1 | `subnet-040249b14d105d8ff` | frozen-unknown | `readbacks/regions/eu-north-1-subnets.json` |
| subnet | eu-north-1 | `subnet-0be7da902cb4e2114` | frozen-unknown | `readbacks/regions/eu-north-1-subnets.json` |
| subnet | eu-north-1 | `subnet-0e7077eda3d2e527e` | frozen-unknown | `readbacks/regions/eu-north-1-subnets.json` |
| vpc | eu-north-1 | `vpc-0a7a7d37abd03fd78` | frozen-unknown | `readbacks/regions/eu-north-1-vpcs.json` |
| acm-certificate | eu-west-1 | `none observed` | not-observed | `readbacks/regions/eu-west-1-acm-certificates.json` |
| cloudformation-stack | eu-west-1 | `none observed` | not-observed | `readbacks/regions/eu-west-1-cloudformation-stacks.json` |
| ec2-instance | eu-west-1 | `none observed` | not-observed | `readbacks/regions/eu-west-1-ec2-instances.json` |
| ebs-volume | eu-west-1 | `none observed` | not-observed | `readbacks/regions/eu-west-1-ec2-volumes.json` |
| load-balancer | eu-west-1 | `none observed` | not-observed | `readbacks/regions/eu-west-1-load-balancers.json` |
| security-group | eu-west-1 | `sg-0b8b703f9012c5435` | frozen-unknown | `readbacks/regions/eu-west-1-security-groups.json` |
| subnet | eu-west-1 | `subnet-0054729f8f3992e53` | frozen-unknown | `readbacks/regions/eu-west-1-subnets.json` |
| subnet | eu-west-1 | `subnet-05e6a6230739f6aa1` | frozen-unknown | `readbacks/regions/eu-west-1-subnets.json` |
| subnet | eu-west-1 | `subnet-085af00f0107682bf` | frozen-unknown | `readbacks/regions/eu-west-1-subnets.json` |
| vpc | eu-west-1 | `vpc-0e798b208c23e92e4` | frozen-unknown | `readbacks/regions/eu-west-1-vpcs.json` |
| acm-certificate | eu-west-2 | `none observed` | not-observed | `readbacks/regions/eu-west-2-acm-certificates.json` |
| cloudformation-stack | eu-west-2 | `none observed` | not-observed | `readbacks/regions/eu-west-2-cloudformation-stacks.json` |
| ec2-instance | eu-west-2 | `none observed` | not-observed | `readbacks/regions/eu-west-2-ec2-instances.json` |
| ebs-volume | eu-west-2 | `none observed` | not-observed | `readbacks/regions/eu-west-2-ec2-volumes.json` |
| load-balancer | eu-west-2 | `none observed` | not-observed | `readbacks/regions/eu-west-2-load-balancers.json` |
| security-group | eu-west-2 | `sg-06b930853131e53f2` | frozen-unknown | `readbacks/regions/eu-west-2-security-groups.json` |
| subnet | eu-west-2 | `subnet-01c4d0f339ad66eea` | frozen-unknown | `readbacks/regions/eu-west-2-subnets.json` |
| subnet | eu-west-2 | `subnet-05ee5f11dc5c9d63b` | frozen-unknown | `readbacks/regions/eu-west-2-subnets.json` |
| subnet | eu-west-2 | `subnet-0c34b4566a52e09c8` | frozen-unknown | `readbacks/regions/eu-west-2-subnets.json` |
| vpc | eu-west-2 | `vpc-0dc3cd2ab8226a239` | frozen-unknown | `readbacks/regions/eu-west-2-vpcs.json` |
| acm-certificate | eu-west-3 | `none observed` | not-observed | `readbacks/regions/eu-west-3-acm-certificates.json` |
| cloudformation-stack | eu-west-3 | `none observed` | not-observed | `readbacks/regions/eu-west-3-cloudformation-stacks.json` |
| ec2-instance | eu-west-3 | `none observed` | not-observed | `readbacks/regions/eu-west-3-ec2-instances.json` |
| ebs-volume | eu-west-3 | `none observed` | not-observed | `readbacks/regions/eu-west-3-ec2-volumes.json` |
| load-balancer | eu-west-3 | `none observed` | not-observed | `readbacks/regions/eu-west-3-load-balancers.json` |
| security-group | eu-west-3 | `sg-0b5dfea442222947b` | frozen-unknown | `readbacks/regions/eu-west-3-security-groups.json` |
| subnet | eu-west-3 | `subnet-04ff452222c87972a` | frozen-unknown | `readbacks/regions/eu-west-3-subnets.json` |
| subnet | eu-west-3 | `subnet-01b507e023b0c0412` | frozen-unknown | `readbacks/regions/eu-west-3-subnets.json` |
| subnet | eu-west-3 | `subnet-0adbc5f1b1b87e038` | frozen-unknown | `readbacks/regions/eu-west-3-subnets.json` |
| vpc | eu-west-3 | `vpc-0b0264ba5cbcba99b` | frozen-unknown | `readbacks/regions/eu-west-3-vpcs.json` |
| acm-certificate | sa-east-1 | `none observed` | not-observed | `readbacks/regions/sa-east-1-acm-certificates.json` |
| cloudformation-stack | sa-east-1 | `none observed` | not-observed | `readbacks/regions/sa-east-1-cloudformation-stacks.json` |
| ec2-instance | sa-east-1 | `none observed` | not-observed | `readbacks/regions/sa-east-1-ec2-instances.json` |
| ebs-volume | sa-east-1 | `none observed` | not-observed | `readbacks/regions/sa-east-1-ec2-volumes.json` |
| load-balancer | sa-east-1 | `none observed` | not-observed | `readbacks/regions/sa-east-1-load-balancers.json` |
| security-group | sa-east-1 | `sg-0db32c0b75eac9b4a` | frozen-unknown | `readbacks/regions/sa-east-1-security-groups.json` |
| subnet | sa-east-1 | `subnet-0ea151253c362ef60` | frozen-unknown | `readbacks/regions/sa-east-1-subnets.json` |
| subnet | sa-east-1 | `subnet-0433476b5dd3de78c` | frozen-unknown | `readbacks/regions/sa-east-1-subnets.json` |
| subnet | sa-east-1 | `subnet-0d9f59395762cbd36` | frozen-unknown | `readbacks/regions/sa-east-1-subnets.json` |
| vpc | sa-east-1 | `vpc-0c8b863652c32fac1` | frozen-unknown | `readbacks/regions/sa-east-1-vpcs.json` |
| acm-certificate | us-east-1 | `codefriend.ai` | frozen-unknown | `readbacks/regions/us-east-1-acm-certificates.json` |
| acm-certificate | us-east-1 | `agent-logic.ai` | frozen-unknown | `readbacks/regions/us-east-1-acm-certificates.json` |
| acm-certificate | us-east-1 | `agent-logic.ai` | frozen-unknown | `readbacks/regions/us-east-1-acm-certificates.json` |
| acm-certificate | us-east-1 | `agent-logic.ai` | frozen-unknown | `readbacks/regions/us-east-1-acm-certificates.json` |
| acm-certificate | us-east-1 | `*.wuji.dev.csm.agent-logic.ai` | frozen-unknown | `readbacks/regions/us-east-1-acm-certificates.json` |
| cloudformation-stack | us-east-1 | `none observed` | not-observed | `readbacks/regions/us-east-1-cloudformation-stacks.json` |
| ec2-instance | us-east-1 | `none observed` | not-observed | `readbacks/regions/us-east-1-ec2-instances.json` |
| ebs-volume | us-east-1 | `none observed` | not-observed | `readbacks/regions/us-east-1-ec2-volumes.json` |
| load-balancer | us-east-1 | `none observed` | not-observed | `readbacks/regions/us-east-1-load-balancers.json` |
| security-group | us-east-1 | `sg-0220ad0c1b60b7cf9` | frozen-unknown | `readbacks/regions/us-east-1-security-groups.json` |
| subnet | us-east-1 | `subnet-00880845f602eae26` | frozen-unknown | `readbacks/regions/us-east-1-subnets.json` |
| subnet | us-east-1 | `subnet-00687214eb0d232af` | frozen-unknown | `readbacks/regions/us-east-1-subnets.json` |
| subnet | us-east-1 | `subnet-0294c9cc73ac56b9b` | frozen-unknown | `readbacks/regions/us-east-1-subnets.json` |
| subnet | us-east-1 | `subnet-09ac6d69cd7a660fb` | frozen-unknown | `readbacks/regions/us-east-1-subnets.json` |
| subnet | us-east-1 | `subnet-02fcc183de06919c1` | frozen-unknown | `readbacks/regions/us-east-1-subnets.json` |
| subnet | us-east-1 | `subnet-0783fe9f47478e12b` | frozen-unknown | `readbacks/regions/us-east-1-subnets.json` |
| vpc | us-east-1 | `vpc-080be4051e99f513b` | frozen-unknown | `readbacks/regions/us-east-1-vpcs.json` |
| acm-certificate | us-east-2 | `none observed` | not-observed | `readbacks/regions/us-east-2-acm-certificates.json` |
| cloudformation-stack | us-east-2 | `none observed` | not-observed | `readbacks/regions/us-east-2-cloudformation-stacks.json` |
| ec2-instance | us-east-2 | `none observed` | not-observed | `readbacks/regions/us-east-2-ec2-instances.json` |
| ebs-volume | us-east-2 | `none observed` | not-observed | `readbacks/regions/us-east-2-ec2-volumes.json` |
| load-balancer | us-east-2 | `none observed` | not-observed | `readbacks/regions/us-east-2-load-balancers.json` |
| security-group | us-east-2 | `sg-0615663a786f0183f` | frozen-unknown | `readbacks/regions/us-east-2-security-groups.json` |
| subnet | us-east-2 | `subnet-02817c28b0dc9b3c9` | frozen-unknown | `readbacks/regions/us-east-2-subnets.json` |
| subnet | us-east-2 | `subnet-05ada19c5e220a81e` | frozen-unknown | `readbacks/regions/us-east-2-subnets.json` |
| subnet | us-east-2 | `subnet-02d84a7155270883d` | frozen-unknown | `readbacks/regions/us-east-2-subnets.json` |
| vpc | us-east-2 | `vpc-0f8131426be9225e0` | frozen-unknown | `readbacks/regions/us-east-2-vpcs.json` |
| acm-certificate | us-west-1 | `none observed` | not-observed | `readbacks/regions/us-west-1-acm-certificates.json` |
| cloudformation-stack | us-west-1 | `none observed` | not-observed | `readbacks/regions/us-west-1-cloudformation-stacks.json` |
| ec2-instance | us-west-1 | `none observed` | not-observed | `readbacks/regions/us-west-1-ec2-instances.json` |
| ebs-volume | us-west-1 | `none observed` | not-observed | `readbacks/regions/us-west-1-ec2-volumes.json` |
| load-balancer | us-west-1 | `none observed` | not-observed | `readbacks/regions/us-west-1-load-balancers.json` |
| security-group | us-west-1 | `sg-02e20409541091f36` | frozen-unknown | `readbacks/regions/us-west-1-security-groups.json` |
| subnet | us-west-1 | `subnet-00a88420517e4f3f4` | frozen-unknown | `readbacks/regions/us-west-1-subnets.json` |
| subnet | us-west-1 | `subnet-0db24256e4637bdd2` | frozen-unknown | `readbacks/regions/us-west-1-subnets.json` |
| vpc | us-west-1 | `vpc-0d93501562180ae4a` | frozen-unknown | `readbacks/regions/us-west-1-vpcs.json` |
| acm-certificate | us-west-2 | `origin-smoke.wuji.dev.csm.agent-logic.ai` | frozen-unknown | `readbacks/regions/us-west-2-acm-certificates.json` |
| cloudformation-stack | us-west-2 | `adl-ebs-unattached-age-alert` | frozen-unknown | `readbacks/regions/us-west-2-cloudformation-stacks.json` |
| ec2-instance | us-west-2 | `none observed` | not-observed | `readbacks/regions/us-west-2-ec2-instances.json` |
| ebs-volume | us-west-2 | `none observed` | not-observed | `readbacks/regions/us-west-2-ec2-volumes.json` |
| load-balancer | us-west-2 | `none observed` | not-observed | `readbacks/regions/us-west-2-load-balancers.json` |
| security-group | us-west-2 | `sg-01a98c8e210b62c85` | frozen-unknown | `readbacks/regions/us-west-2-security-groups.json` |
| security-group | us-west-2 | `sg-003d8cb7246067499` | frozen-unknown | `readbacks/regions/us-west-2-security-groups.json` |
| security-group | us-west-2 | `sg-0010f099186c8f9eb` | frozen-unknown | `readbacks/regions/us-west-2-security-groups.json` |
| security-group | us-west-2 | `sg-011d94b6aa33d5239` | frozen-unknown | `readbacks/regions/us-west-2-security-groups.json` |
| security-group | us-west-2 | `sg-046ff6902fb0f8cc0` | frozen-unknown | `readbacks/regions/us-west-2-security-groups.json` |
| security-group | us-west-2 | `sg-0ef287ea7be2cbdf7` | frozen-unknown | `readbacks/regions/us-west-2-security-groups.json` |
| security-group | us-west-2 | `sg-0d790aabae60b5d7a` | frozen-unknown | `readbacks/regions/us-west-2-security-groups.json` |
| security-group | us-west-2 | `sg-0535963eaec330b12` | frozen-unknown | `readbacks/regions/us-west-2-security-groups.json` |
| security-group | us-west-2 | `sg-0d8e37e4f26741263` | frozen-unknown | `readbacks/regions/us-west-2-security-groups.json` |
| security-group | us-west-2 | `sg-041f08aeea5391e02` | frozen-unknown | `readbacks/regions/us-west-2-security-groups.json` |
| security-group | us-west-2 | `sg-0833ffb665cedaf0d` | frozen-unknown | `readbacks/regions/us-west-2-security-groups.json` |
| security-group | us-west-2 | `sg-0e230571cdd126919` | frozen-unknown | `readbacks/regions/us-west-2-security-groups.json` |
| security-group | us-west-2 | `sg-01df232252551f607` | frozen-unknown | `readbacks/regions/us-west-2-security-groups.json` |
| security-group | us-west-2 | `sg-05df85829752c6cd6` | frozen-unknown | `readbacks/regions/us-west-2-security-groups.json` |
| security-group | us-west-2 | `sg-0a271b4781cd616e8` | frozen-unknown | `readbacks/regions/us-west-2-security-groups.json` |
| security-group | us-west-2 | `sg-0c5c2cdb816a3ee36` | frozen-unknown | `readbacks/regions/us-west-2-security-groups.json` |
| security-group | us-west-2 | `sg-08ea8f565ec5d4985` | frozen-unknown | `readbacks/regions/us-west-2-security-groups.json` |
| security-group | us-west-2 | `sg-0b7ccf07b35efa40f` | frozen-unknown | `readbacks/regions/us-west-2-security-groups.json` |
| security-group | us-west-2 | `sg-0d4b1eb7765fd8ff3` | frozen-unknown | `readbacks/regions/us-west-2-security-groups.json` |
| security-group | us-west-2 | `sg-09cfab23732c43a4d` | frozen-unknown | `readbacks/regions/us-west-2-security-groups.json` |
| security-group | us-west-2 | `sg-0108df7bb1ab37ffd` | frozen-unknown | `readbacks/regions/us-west-2-security-groups.json` |
| security-group | us-west-2 | `sg-0e1c37b8b950925aa` | frozen-unknown | `readbacks/regions/us-west-2-security-groups.json` |
| security-group | us-west-2 | `sg-063bdaf36c2e3b0d2` | frozen-unknown | `readbacks/regions/us-west-2-security-groups.json` |
| security-group | us-west-2 | `sg-03bf9803b091d0bfb` | frozen-unknown | `readbacks/regions/us-west-2-security-groups.json` |
| security-group | us-west-2 | `sg-03e9677b1cb483f25` | frozen-unknown | `readbacks/regions/us-west-2-security-groups.json` |
| security-group | us-west-2 | `sg-09571038c2d412bb2` | frozen-unknown | `readbacks/regions/us-west-2-security-groups.json` |
| security-group | us-west-2 | `sg-052e1b4273335e5f7` | frozen-unknown | `readbacks/regions/us-west-2-security-groups.json` |
| security-group | us-west-2 | `sg-09fc52684a96ead6a` | frozen-unknown | `readbacks/regions/us-west-2-security-groups.json` |
| security-group | us-west-2 | `sg-01474cd05a7109f1d` | frozen-unknown | `readbacks/regions/us-west-2-security-groups.json` |
| security-group | us-west-2 | `sg-0f71e25c9b6b29862` | frozen-unknown | `readbacks/regions/us-west-2-security-groups.json` |
| security-group | us-west-2 | `sg-01a09aea28b8e4127` | frozen-unknown | `readbacks/regions/us-west-2-security-groups.json` |
| subnet | us-west-2 | `subnet-002ef5df5a93bbb1b` | frozen-unknown | `readbacks/regions/us-west-2-subnets.json` |
| subnet | us-west-2 | `subnet-0469c259e71ea0e79` | frozen-unknown | `readbacks/regions/us-west-2-subnets.json` |
| subnet | us-west-2 | `subnet-04bf6905b2d945ac5` | frozen-unknown | `readbacks/regions/us-west-2-subnets.json` |
| subnet | us-west-2 | `subnet-03e511b3c7d4550d2` | frozen-unknown | `readbacks/regions/us-west-2-subnets.json` |
| vpc | us-west-2 | `vpc-07dc67492b4aee743` | frozen-unknown | `readbacks/regions/us-west-2-vpcs.json` |
