# Structured Intent Prompt

Template: 1.0.0

Issue: 122

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

After the distributed Runtime is terminal and a separate operator authorizes AWS, expose the exact Observatory and matching Runtime gateway through bounded business-account infrastructure.

## Required Outcome

Route53, ACM, S3, CloudFront, WAF, API Gateway, DNS delegation, and origin configuration provide ordinary browser trust, exact revision parity, governed access, rollback, and ownership proof for the permanent non-compute public edge; operator-authorized disposable Spot EC2 and ALB Runtime-origin smoke proof remains bounded by receipt and teardown evidence.

## Scope

- infra/aws/csm-public-edge/**
- adl/tools/validate_csm_public_edge_static.sh
- adl/tools/validate_csm_public_edge_live.sh
- docs/milestones/post-v0.92/features/CSM_PUBLIC_EDGE_TERRAFORM.md
- .csdlc/issues/122/**
- .csdlc/prepared/issues/122/**
- .csdlc/evidence/122/**

## Authority

- Issue and code authority are agent-logic/agent-design-language#122
- Issue #122 is deferred beyond v0.92 and is not a gate for #83 or #111-#117
- Execution requires terminal distributed Runtime proof plus separate operator AWS authorization
- AWS activity must use the approved Agent Logic business profile; permanent public-edge scope remains non-compute while disposable Spot EC2 and ALB origin proof requires operator authorization and teardown evidence
- Public reachability never grants Runtime write authority or permits private agent-state exposure

## Assumptions

- none

## Operator Constraints

- Do not push, open a PR, publish, merge, close, or mutate #83, #110, or #111-#117 without separate lifecycle authority
- Verify the approved business profile resolves to the Agent Logic business account before any AWS action
- Limit permanent AWS public-edge scope to Route53, ACM, S3, CloudFront, WAF, API Gateway, DNS delegation, and origin configuration
- Allow disposable Spot EC2 and ALB Runtime-origin resources only for explicit operator-authorized smoke proof with external receipt evidence and teardown/empty-state proof
- Never use CodeBuild, NAT, GPU, Kubernetes, containers, or unapproved permanent Runtime compute for this issue
