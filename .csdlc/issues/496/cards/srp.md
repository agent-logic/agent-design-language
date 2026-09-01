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

Revision: Some("git-blake3:e16a5499bdda3d9025fec5f4ea91955233abf3db:4d6623234961f7f30115b1235b3c4ddf55355530388723a49090e1a79af487eb")

Reviewer: Some("fresh-session:c1b4c95e-27f4-4a9a-a839-2c51632e9b04")

Result: pass
