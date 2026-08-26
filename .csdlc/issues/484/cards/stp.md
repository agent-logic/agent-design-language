# Structured Task Prompt

Template: 1.0.0

Issue: 484

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Exactly one read-only AWS resource ownership inventory and retained redacted evidence packet.

## Deliverables

- Approved account and region basis
- All-region read-only discovery denominator
- Owner and lifecycle disposition table for discovered resources
- Separate classification for website Terraform and issue evidence
- Redaction and no-mutation validation proof
- Fresh exact-head review and closing PR

## Acceptance

1. AC-1: The approved business account and regions are exact.
2. AC-2: Every discovered resource has an owner or frozen-unknown disposition.
3. AC-3: Website Terraform and issue evidence remain separately classified.
4. AC-4: EBS and other retained assets are not inferred disposable.
5. AC-5: Evidence contains no credentials, secrets, raw token contents, or mutation commands.
6. AC-6: Fresh exact-head review has zero actionable findings and publication truth closes only issue #484.

## Dependencies

- none

## Inputs

- https://github.com/agent-logic/agent-design-language/issues/484
- docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml#AWS-A
- docs/milestones/v0.92.1/features/AWS_ACCOUNT_MOVE_IN_v0.92.1.md
- .csdlc/prepared/issues/484/design.md

## Non Goals

- Resource import
- Cleanup or deletion
- Terraform apply
- Billing baseline
- IAM changes
- CloudFormation retirement
