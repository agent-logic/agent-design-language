# Structured Task Prompt

Template: 1.0.0

Issue: 579

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Issue #579 corrective only; repair #489 AWS-F module/proof/validator truth without absorbing #122 public exposure, #496 CloudFormation retirement, #495 cross-cloud conversion, production cutover, or live paid AWS proof.

## Deliverables

- Repaired AWS-F Terraform/runtime-platform module or examples that no longer create public Route53/ACM exposure
- Repaired AWS-F security validator with regression fixtures for forbidden world-open ingress
- Updated AWS-F runbook/evidence truth for deployment proof, cleanup, rollback, observability, artifact wiring, state isolation, and Spot resilience
- .csdlc/prepared/issues/579 validation artifacts
- .csdlc/evidence/579 proof records
- typed C-SDLC v2 cards proving scope, validation, review, publication, and terminal truth

## Acceptance

1. AC-1: AWS-F no longer exposes executable Route53/ACM public-edge ownership and instead refers public exposure to #122.
2. AC-2: Proof records truthfully separate local/static validation from any operator-authorized live AWS deployment or cleanup proof.
3. AC-3: The security validator rejects forbidden world-open Runtime ingress through a regression fixture.
4. AC-4: State isolation is enforced or explicitly fail-closed for backend, locking, account identity, workspace, and state key boundaries.
5. AC-5: Spot one-time instance target behavior is accurately bounded and not represented as production-resilient.
6. AC-6: Fresh exact-head review has no actionable findings before publication.

## Dependencies

- #489 / PR #577 terminal merge SHA 69ba35e066d1389a9f194659acb066a7dca82a40
- #122 public edge ownership authority

## Inputs

- agent-logic/agent-design-language#579
- agent-logic/agent-design-language#489
- agent-logic/agent-design-language#577
- infra/aws/runtime/
- docs/operations/cloud/aws/runtime-platform/
- docs/milestones/v0.92.1/evidence/cloud/aws-f/
- .csdlc/prepared/issues/489/validate-aws-f-runtime-platform.sh
- .csdlc/prepared/issues/489/run-aws-f-readbacks.sh

## Non Goals

- Rewriting terminal #489
- Public edge ownership
- Paid AWS resource creation without approval
- Production traffic or cutover
- CloudFormation retirement
- Cross-cloud conversion
- Runtime behavior fork
