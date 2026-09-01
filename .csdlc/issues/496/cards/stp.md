# Structured Task Prompt

Template: 1.0.0

Issue: 496

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Issue #496 AWS-G only; decide the retirement disposition for the issue #194/#268 CloudFormation template denominator. Do not delete templates, abandon live stacks, mutate cloud resources, or implement #489/#495 replacement work.

## Deliverables

- docs/milestones/v0.92.1/evidence/cloud/aws-g/aws-g-cloudformation-retirement-ledger.md
- .csdlc/prepared/issues/496/validate-aws-g-cloudformation-retirement.sh
- typed C-SDLC v2 cards proving dependency, scope, validation, review, publication, and terminal truth

## Acceptance

1. AC-1: Issue 194 and 268 templates are inventoried
2. AC-2: Every consumer/reference path and retained evidence path has a disposition
3. AC-3: Retirement requires proven Terraform parity and rollback
4. AC-4: No active stack is silently abandoned
5. AC-5: Fresh exact-head review has no actionable findings before publication

## Dependencies

- AWS-F #489 closed by merged PR with merge sha 69ba35e066d1389a9f194659acb066a7dca82a40
- XCL-01 #495 closed by merged PR #590 with merge sha c78c60f5a45a87a96159d4910a831b69b62b042c; no local derived-terminal cache was present at bootstrap, so #496 must consume live GitHub/ancestry truth explicitly

## Inputs

- docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml#AWS-G
- docs/milestones/v0.92.1/features/AWS_ACCOUNT_MOVE_IN_v0.92.1.md
- docs/milestones/v0.92.1/features/CROSS_CLOUD_TERRAFORM_CONVERSION_v0.92.1.md
- adl/tools/issue194_private_network.cloudformation.json
- adl/tools/issue268_runtime_qualification.cloudformation.yaml
- infra/aws/runtime
- infra/runtime-portable
- docs/milestones/v0.92.1/evidence/cloud/aws-f
- docs/milestones/v0.92.1/evidence/cloud/xcl-01

## Non Goals

- Deleting historical evidence
- Forced retirement
- Website Terraform changes
- Cloud resource mutation
- Reimplementing AWS-F or XCL-01
- Production cutover
- Credential disclosure
