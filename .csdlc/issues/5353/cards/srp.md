# Structured Review Prompt

Template: 1.0.0

Issue: 5353

Repository: danielbaustin/agent-design-language

Card: srp

Status: pre_phase

## Scope

csdlc-v2/src/lifecycle.rs
csdlc-v2/src/store.rs
csdlc-v2/tests/gate2.rs

## Prompts

- Verify issue-local paths cannot create a false existing-record condition.
- Verify both design and diagram digests refresh atomically.
- Verify tests do not widen into ADL or Runtime code.

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:ea5d5bc939bd445811169d236cba5d4e9d0aa349:baba78f10c5f093a9f53bf6e0cd028a7cee515a84677f4aa4fee36c865088b58")

Reviewer: Some("subagent-reviewer")

Result: pass
