# Structured Review Prompt

Template: 1.0.0

Issue: 496

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

Exact commit ea9dfe3377359792c62c5c73bc141b37a78bdb8e
Issue #496 AWS-G only
docs/milestones/v0.92.1/evidence/cloud/aws-g/**
.csdlc/issues/496/**
.csdlc/prepared/issues/496/**
.csdlc/evidence/496/**
Read-only denominator references to adl/tools/issue194_private_network.cloudformation.json and adl/tools/issue268_runtime_qualification.cloudformation.yaml

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

Revision: Some("git-blake3:ea9dfe3377359792c62c5c73bc141b37a78bdb8e:b7ef7083f6ad849eb93bdc42690aa52cf92f089facd8b93cfabc05c19052682e")

Reviewer: Some("fresh-session:6830be43-0ec4-43e2-810f-772d9bc35f77")

Result: pass
