# Structured Intent Prompt

Template: 1.0.0

Issue: 486

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Produce one recoverable AWS Terraform account-foundation bootstrap.

## Required Outcome

One recoverable AWS account-foundation backend and deployment-role bootstrap isolated from existing website and workload states.

## Scope

- AWS account-foundation Terraform backend bootstrap
- Dedicated encrypted and versioned state bucket
- Dedicated Terraform lock table
- Deployment role boundary for account foundation work
- State isolation evidence and operator runbook

## Authority

- Use the Agent Logic business AWS profile agent-logic-admin for any AWS readback or approved apply
- Do not use personal/default AWS account state
- Do not copy, import, or dual-own existing website, DDNS, public-edge, or workload Terraform state
- Do not expose credentials or retained token material

## Assumptions

- #485 is terminal and provides AWS access/billing baseline evidence
- Implementation-owned Terraform/readback paths do not exist before #486 is bound
- The pre-bind packet can prove readiness without performing paid AWS mutations

## Operator Constraints

- Use typed C-SDLC v2 lifecycle routes
- Bind beneath /Volumes/FastWork/adl-worktrees before tracked implementation edits
- Use standard runners only for hosted CI
- Stop if reviewed plan differs before apply
