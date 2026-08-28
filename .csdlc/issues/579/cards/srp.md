# Structured Review Prompt

Template: 1.0.0

Issue: 579

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

infra/aws/modules/csm-runtime-alb/main.tf
infra/aws/modules/csm-runtime-alb/variables.tf
infra/aws/runtime/alb-origin
infra/aws/runtime/private-node
infra/aws/runtime/README.md
docs/operations/cloud/aws/runtime-platform/README.md
docs/milestones/v0.92.1/evidence/cloud/aws-f/aws-f-runtime-platform-proof.md
.csdlc/prepared/issues/579/validate-aws-f-corrective.sh
.csdlc/issues/579

## Prompts

- Does the corrective remove AWS-F public Route53/ACM ownership and leave public exposure to #122?
- Are local/static proof, deferred live AWS proof, deployment, cleanup, rollback, observability, artifact wiring, and cost/deadline claims truthful?
- Does the validator structurally reject forbidden world-open Runtime ingress and avoid egress false exclusions?
- Are backend, locking, account identity, workspace, and key isolation enforced or fail-closed?
- Is one-time Spot instance target behavior bounded as non-production-resilient?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- No AWS, paid, live deployment, or remote-state operation was performed in review.
- Reviewer could not independently run csdlc-validate from PATH; implementation session ran /Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-validate successfully before assignment.

## Review Result

Revision: Some("git-blake3:ed94d768ba9533f9d8315108a604f2f96ac1001d:f003a5319bb138ed2135e1676648e9ed0a9b4ea955a29bf05a2448ee7cffd5b6")

Reviewer: Some("fresh-session:10d98e20-b267-406f-9f64-bf6348d497d1")

Result: pass
