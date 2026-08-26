# Structured Intent Prompt

Template: 1.0.0

Issue: 268

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Execute one bounded six-hour production Runtime v3 qualification on Agent Logic Spot and retain exact-head, failure, cleanup, and zero-instance evidence.

## Required Outcome

One Spot-only attempt provides at least 21,600 monotonic seconds of production exposure or fails truthfully within USD 20, retains all evidence, cleans exact task resources, and proves zero remaining task-owned instances.

## Scope

- Fixed six-hour production suite in adl-runtime-lifecycle-soak
- Issue-owned Spot qualification wrapper and contract tests
- Exact run evidence, review, publication, finish, and cleanup

## Authority

- Agent Logic business AWS account through agent-logic-admin only
- One Spot-only attempt; no GPU, On-Demand fallback, second run, or #269 execution
- Hard total cost ceiling USD 20 and exact-owner cleanup

## Assumptions

- none

## Operator Constraints

- Operator authorized #268 on 2026-08-17 with total budget USD 20
- Do not execute #269 without a separate operator decision
- Preserve every failed, interrupted, or cancelled attempt
- Never expose AWS credentials or full private identifiers
