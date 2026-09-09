# Structured Intent Prompt

Template: 1.0.0

Issue: 761

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Prevent global evidence sampling from starving required review lanes.

## Required Outcome

Complete auditable lane denominators with category-valid samples and fail-closed validation.

## Scope

- adl/tools/skills/repo-packet-builder
- adl/tools/skills/repo-review-synthesis/references/output-contract.md

## Authority

- Typed v2 exception authorized by operator while #749 native repair is in flight.

## Assumptions

- none

## Operator Constraints

- Focused Python validation; preserve exact #520 denominator.
