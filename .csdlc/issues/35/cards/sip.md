# Structured Intent Prompt

Template: 1.0.0

Issue: 35

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Make Codex background task dispatch terminate with an observable task identity or a bounded typed failure without leaving hidden ownership.

## Required Outcome

Reproduce and classify project discovery and projectless dispatch, retain timing and ownership evidence, define the smallest actionable upstream or local repair boundary, and prove failed dispatch cannot create hidden or duplicate issue ownership.

## Scope

- Codex background task dispatch and project discovery behavior for one bounded ADL issue handoff
- retained dispatch timing, result, and ownership evidence
- operator guidance for safe failure and retry behavior

## Authority

- Codex task creation and project discovery are external application behavior and are not implemented inside ADL
- ADL repository changes are limited to durable evidence and bounded operator guidance unless a concrete repository-owned defect is proven
- no successful task identity means no ownership transfer

## Assumptions

- none

## Operator Constraints

- Use one uniquely identified projectless no-op canary that owns no repository, issue, or worktree and forbids all mutation or child dispatch.
- Never retry a timeout or indeterminate dispatch; retry an explicit typed failure only after complete paginated inventory receipts prove an empty task-ID delta.
- Do not create wrappers, polling state machines, claims, leases, or hidden ownership records.
- Do not use AWS, remote builders, or /private/tmp.
