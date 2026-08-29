# Structured Review Prompt

Template: 1.0.0

Issue: 495

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

Issue #495 XCL-01 paths only: infra/runtime-portable, infra/aws/runtime, infra/gcp/workloads, docs/milestones/v0.92.1/evidence/cloud/xcl-01, .csdlc/prepared/issues/495, .csdlc/evidence/495, and narrow issue-owned validator/proof surfaces.

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

- none

## Review Result

Revision: None

Reviewer: None

Result: pre_review
