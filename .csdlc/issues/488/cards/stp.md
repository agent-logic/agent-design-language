# Structured Task Prompt

Template: 1.0.0

Issue: 488

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Issue #488 AWS-E only; produce the accepted AWS resource adoption register and focused proof. Do not implement AWS Runtime modules, cross-cloud abstraction, CloudFormation retirement, production cutover, website rewrite, or speculative resource cleanup.

## Deliverables

- docs/operations/cloud/aws/adoption resource-adoption register and operator notes
- docs/milestones/v0.92.1/evidence/cloud/aws-e retained reconciliation evidence
- issue-owned validator/readback scripts for live-state reconciliation, authority exclusivity, retention/deletion gates, tag/lifecycle evidence, and rollback/follow-on routing
- typed C-SDLC v2 cards proving dependency, scope, validation, review, publication, and terminal truth

## Acceptance

1. AC-1: Every durable AWS resource in the admitted denominator has exactly one management authority and disposition
2. AC-2: Website and historical evidence ownership is preserved and not silently reclassified
3. AC-3: Cleanup requires exact non-use evidence, retention recovery, and deletion authority
4. AC-4: Live and declared state agree or the discrepancy is assigned a truthful frozen-unknown or follow-on disposition
5. AC-5: Fresh exact-head review has no actionable findings before publication

## Dependencies

- AWS-D #487 terminal/merged audit and security baseline; observed merge 1d31016a8df3cf07a4c3f2e6acd2694bd10570c2

## Inputs

- docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml#AWS-E
- docs/operations/cloud/aws/
- infra/aws/
- docs/milestones/v0.92.1/evidence/cloud/aws-d/
- docs/milestones/v0.92.1/evidence/cloud/aws-a/
- docs/milestones/v0.92.1/evidence/cloud/aws-b/
- docs/milestones/v0.92.1/evidence/cloud/aws-c/

## Non Goals

- Speculative cleanup
- Website rewrite
- CloudFormation retirement
- AWS Runtime platform module implementation
- Cross-cloud Runtime Terraform conversion
- Production cutover
- Credential disclosure
