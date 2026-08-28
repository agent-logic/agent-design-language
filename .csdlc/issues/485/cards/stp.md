# Structured Task Prompt

Template: 1.0.0

Issue: 485

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Exactly one accepted access-and-billing baseline; individual readbacks are evidence inputs and do not close independently.

## Deliverables

- Corporate recovery and administrator-continuity baseline
- Human, workload, and agent-initiated identity census
- Agent Toolkit for AWS approved Codex path and AWS CLI version check
- Agent IAM guardrail and read-only default posture specification
- CloudWatch and CloudTrail attribution readbacks
- Billing, budget, anomaly, export, and cost-attribution readbacks
- Credential redaction and no-mutation validation proof
- Fresh exact-head review and closing PR

## Acceptance

1. AC-1: Corporate recovery does not depend on one personal factor.
2. AC-2: Human workload and agent-initiated identities are distinguishable.
3. AC-3: Agent Toolkit for AWS is configured for the approved Codex path with AWS CLI 2.35 or newer.
4. AC-4: IAM context policies bind agent actions with read-only default posture.
5. AC-5: CloudWatch metrics and CloudTrail requests are attributable.
6. AC-6: Billing and budget ownership is visible.
7. AC-7: Existing administrator access remains until replacement is proven.
8. AC-8: Evidence contains no credentials, secrets, raw token contents, or unintended mutation commands.
9. AC-9: Fresh exact-head review has zero actionable findings and publication truth closes only issue #485.

## Dependencies

- #484

## Inputs

- https://github.com/agent-logic/agent-design-language/issues/485
- https://github.com/agent-logic/agent-design-language/issues/484
- https://github.com/agent-logic/agent-design-language/pull/556
- docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml#AWS-B
- docs/milestones/v0.92.1/features/AWS_ACCOUNT_MOVE_IN_v0.92.1.md
- .csdlc/prepared/issues/485/design.md

## Non Goals

- Organizations rollout
- Workload deployment
- Unrestricted agent mutation
- Removing break-glass access
- Terraform apply
- Resource cleanup
- Public WSS or Route53/ACM exposure
