# Structured Review Prompt

Template: 1.0.0

Issue: 5655

Repository: danielbaustin/agent-design-language

Card: srp

Status: pre_phase

## Scope

Review the exact Rust GitHub action command, token boundary, request schemas, mutation reconciliation, focused tests, and operator contract.

## Prompts

- Does one Rust command surface cover every declared issue mutation without connector or wrapper fallback?
- Are ambiguous remote outcomes reconciled before retry or local state mutation?
- Are repository, issue, operation key, labels, assignees, comments, and close identity checked exactly?
- Are tokens bounded and never emitted?
- Do tests prove failure behavior rather than only happy paths?

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
