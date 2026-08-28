# Structured Intent Prompt

Template: 1.0.0

Issue: 488

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Produce one accepted AWS resource adoption register reconciled with live state.

## Required Outcome

One accepted adoption register mapping every durable resource to retain, import, replace, retire-later, ephemeral, or frozen-unknown ownership.

## Scope

- AWS durable resource denominator discovery for the Agent Logic business AWS account
- One adoption register with exactly one management authority and disposition per durable resource
- Reconciliation between live AWS readbacks, Terraform declarations, CloudFormation templates, scripts, tags, lifecycle rules, retained website resources, and historical evidence resources
- Explicit cleanup/deletion authority gates requiring exact non-use evidence, retention recovery, and operator authorization
- Follow-on routing for imports, replacements, retirement, or frozen-unknown resources

## Authority

- Use the Agent Logic business AWS account through the approved AWS profile only
- Read live AWS state only through redacted read-only inventory commands unless a later typed operation explicitly authorizes mutation
- Do not delete, import, tag, or modify live AWS resources as part of this issue unless the accepted register and operator authority explicitly permit the narrow action
- Do not re-own #487 audit/security baseline implementation
- Do not implement #489 AWS Runtime platform modules, #495 cross-cloud Terraform conversion, or #496 CloudFormation retirement

## Assumptions

- none

## Operator Constraints

- Use typed C-SDLC v2 lifecycle routes
- Bind beneath /Volumes/FastWork/adl-worktrees before tracked implementation edits
- Use standard runners only for hosted CI
- Preserve primary main cleanliness
- Do not print, copy, commit, or expose cloud credentials or sensitive account data
- Keep #488 scoped to adoption-register truth and evidence, not speculative cleanup
