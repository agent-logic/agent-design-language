# Structured Output Record

Template: 1.0.0

Issue: 122

Repository: agent-logic/agent-design-language

Card: sor

Status: ready

## Summary

Implement #122 Terraform-only permanent CSM public edge foundation with CloudFront/WAF Observatory, API Gateway HTTP edge, and native WSS custom-origin support, while excluding NAT, compute-node provisioning, CloudFormation, and live AWS apply.

## Artifacts

- infra/aws/csm-public-edge/versions.tf
- infra/aws/csm-public-edge/variables.tf
- infra/aws/csm-public-edge/locals.tf
- infra/aws/csm-public-edge/main.tf
- infra/aws/csm-public-edge/outputs.tf
- infra/aws/csm-public-edge/terraform.tfvars.example
- infra/aws/csm-public-edge/.terraform.lock.hcl
- infra/aws/csm-public-edge/README.md
- docs/milestones/post-v0.92/features/CSM_PUBLIC_EDGE_TERRAFORM.md
- .csdlc/prepared/issues/122/terraform-execution-plan.md
- adl/tools/validate_csm_public_edge_static.sh
- adl/tools/validate_csm_public_edge_live.sh

## Execution

- Add Terraform module infra/aws/csm-public-edge for ACM DNS validation, Route53 names, WAF, private S3 Observatory assets, CloudFront Observatory/API/WSS distributions, API Gateway HTTP proxy, and native WSS custom-origin forwarding.
- Parameterize per-CSM/per-environment names such as api.<csm>.<env>.csm.agent-logic.ai, external Runtime HTTP origins, external or public-ALB WSS origins, WSS Host/SNI forwarding, allowed browser origins, rate limits, log retention, and forwarded WSS query/cookie allowlists.
- Preserve cost boundary by provisioning no NAT Gateway, EC2, Spot, EKS, CodeBuild, VPC, or compute node resources; Runtime compute remains independently attachable later through external HTTPS/WSS origins or a future reviewed ALB/private integration.
- Document the Terraform execution plan and operator runbook boundary, including ACM us-east-1 viewer certificate behavior, WAF scope, no live apply without explicit operator approval, and future adapter boundaries for API Gateway WebSocket API or private ALB integration.
- Add static and live validation helper scripts; static proof validates Terraform formatting/init/validate plus authored-surface guards for Terraform-only/no-compute/no-NAT/native-WSS support, while live proof remains explicitly apply-gated.

## Validation

[
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Run Git diff whitespace hygiene after implementation.",
    "outcome": "passed",
    "evidence_ref": "csm-public-edge-diff-hygiene.log"
  },
  {
    "command": [
      "bash",
      "adl/tools/validate_csm_public_edge_static.sh"
    ],
    "purpose": "Run the issue-owned static Terraform public-edge validator from the bound worktree.",
    "outcome": "passed",
    "evidence_ref": "csm-public-edge-static.log"
  }
]

## Integration

worktree_only

## Publication

Publication: not_published

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
