# Structured Review Prompt

Template: 1.0.0

Issue: 496

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

docs/milestones/v0.92.1/evidence/cloud/aws-g

## Prompts

- Does the design keep #496 to a retirement decision ledger without deleting templates or reimplementing #489/#495?
- Are the #194 and #268 template denominators and every current repo consumer/reference classified truthfully?
- Does the ledger require Terraform parity, rollback authority, retained evidence, and live-stack disposition before any retirement claim?
- Does the plan avoid cloud mutation and credential disclosure while preserving a clear future live readback route?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- No live AWS mutation/readback was run for #496; this issue records CloudFormation retirement decision truth only and preserves live-stack retirement as non-claim/deferred authority.

## Review Result

Revision: Some("git-blake3:1105455ad15f95a2f0b9103b336692154ad0a41e:40b5d66d53ffcaf7790939a091c79967838bbbb7c9a711f8d9a800b8fa65ff9c")

Reviewer: Some("fresh-session:53b89b9a-e9e8-4f14-b86a-fb33599d239e")

Result: pass
