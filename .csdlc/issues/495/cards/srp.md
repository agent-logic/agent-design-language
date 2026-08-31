# Structured Review Prompt

Template: 1.0.0

Issue: 495

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/issues/495
.csdlc/prepared/issues/495
.csdlc/evidence/495
infra/runtime-portable
infra/aws/runtime/xcl-01
infra/gcp/workloads/xcl-01
docs/milestones/v0.92.1/evidence/cloud/xcl-01

## Prompts

- Does the design keep #495 to XCL-01 cross-cloud Terraform conversion without absorbing AWS-G #496, GCP-E #494, DRT-D, or production cutover?
- Does the design inventory the exact #194/#268 CloudFormation-template denominator and preserve CloudFormation rollback authority until AWS-G?
- Are provider-neutral contract claims backed by explicit AWS/GCP security, identity, networking, state, and cleanup differences?
- Does the proof plan truthfully separate static validation from paid/live apply/destroy parity and avoid credential disclosure?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- No live AWS/GCP plan/apply/destroy proof was run for #495; the proof is limited to static contract validation, Terraform fmt, provider schema validation, and diff hygiene.
- CloudFormation rollback authority remains until #496 makes the separate retirement decision.

## Review Result

Revision: Some("git-blake3:99b9b271cc92901b189ea2090444e49a903cf0d6:97fcf2a996985ce59b03d497f149ed4765379d9c14a71b048621818b25f6dcf8")

Reviewer: Some("fresh-session:fddca265-27c7-4a97-ba90-9d33754a2b01")

Result: pass
