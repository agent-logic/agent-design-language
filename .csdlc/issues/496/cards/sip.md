# Structured Intent Prompt

Template: 1.0.0

Issue: 496

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Produce one accepted AWS CloudFormation retirement decision for the bounded issue #194/#268 template denominator.

## Required Outcome

One accepted retirement ledger that inventories the issue #194 and #268 CloudFormation templates, classifies every active or planned consumer/reference path, binds Terraform replacement and rollback evidence, and records a truthful disposition without deleting historical evidence or abandoning live stacks.

## Scope

- AWS-G retirement ledger and proof packet under docs/milestones/v0.92.1/evidence/cloud/aws-g
- Read-only inventory of adl/tools/issue194_private_network.cloudformation.json and adl/tools/issue268_runtime_qualification.cloudformation.yaml
- Read-only consumer census across current repo consumer/reference paths to the two templates
- Consumption of merged #489 AWS-F and #495 XCL-01 Terraform replacement evidence
- Explicit rollback and retained-evidence disposition for the CloudFormation templates
- Live-stack readback plan and non-claim boundary when no live stack deletion is authorized

## Authority

- Do not delete CloudFormation templates or historical evidence in #496
- Do not force retirement if Terraform parity or live-stack ownership is ambiguous
- Do not re-open #489 AWS-F or #495 XCL-01 implementation scope
- Do not create, update, or destroy AWS resources without separate live-cloud authorization
- Do not print, copy, commit, or expose cloud credentials or sensitive account data
- Do not absorb corporate, distributed Runtime, GCP, Observatory, or release-tail scope

## Assumptions

- none

## Operator Constraints

- Use typed C-SDLC v2 lifecycle routes
- Use the dedicated FastWork worktree and do not write tracked issue work on primary main
- Use standard runners only for hosted CI
- Preserve primary main cleanliness
- Closeout/cleanup is asynchronous and not a child execution gate
- Keep #496 as a decision/ledger issue unless live retirement receives explicit operator authorization
