# Structured Review Prompt

Template: 1.0.0

Issue: 707

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

adl-runtime-kernel/src/control.rs

## Prompts

- Is receipt identity byte-stable across independently resolved package graphs?
- Can any mismatched init, generation, executable, or receipt pass?
- Does live proof distinguish operator reply from a separately addressed and received Ember work item?
- Is rollback preserved throughout deployment?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- The focused regression invokes the greeting helper directly; HTTP scheduling and non-blocking behavior are verified by source inspection.
- The fire-and-forget greeting is not durable across process termination after admission persistence.

## Review Result

Revision: Some("git-blake3:f3b8d952b5b6f00a5bb9e5105da3da47c4703405:5f096ec5806e6a0d4fb5390caecb0424b5ef252a9c29143cd537678a5169f7f5")

Reviewer: Some("subagent:/root/review_711_admission_greeting")

Result: pass
