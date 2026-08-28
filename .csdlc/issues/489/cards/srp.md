# Structured Review Prompt

Template: 1.0.0

Issue: 489

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.gitignore
.csdlc/issues/489
.csdlc/prepared/issues/489
.csdlc/evidence/489
infra/aws/runtime
docs/operations/cloud/aws/runtime-platform/README.md
docs/milestones/v0.92.1/evidence/cloud/aws-f/aws-f-runtime-platform-proof.md

## Prompts

- Does the design keep #489 to private AWS Runtime platform modules without absorbing #122, #488, #496, #495, or production cutover work?
- Are no-direct-public-ingress, edge/network/build/node state separation, disposable cleanup, cost/deadline, rollback, and observability specified with machine-checkable proof?
- Does the live AWS readback plan avoid mutation and avoid printing or retaining credentials/sensitive values?
- Are #122 public edge authority and #488 adoption-register authority consumed read-only rather than reclassified?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Review did not perform live AWS readbacks, Terraform apply/plan against AWS, or GitHub publication.
- Live disposable deployment, ALB target-health, external request receipt, and zero-residue teardown remain operator-authorized live-proof gates; #489 records static Terraform/runtime-platform readiness.

## Review Result

Revision: Some("git-blake3:cd3819e661ad0625e2bcda5cedf04cdfa362dc57:04873ca361130cec80538e7c5dcbe43e47339cf0781ee45132c94965e0fbf1d8")

Reviewer: Some("fresh-session:2d46c7d9-3e7b-4892-97c2-ff8941aa41e5")

Result: pass
