# Structured Review Prompt

Template: 1.0.0

Issue: 5624

Repository: danielbaustin/agent-design-language

Card: srp

Status: pre_phase

## Scope

csdlc-v2/src/readiness.rs
csdlc-v2/tests/gate7.rs
csdlc-v2/tests/gate7_lifecycle.rs
.csdlc/issues/5624
.csdlc/prepared/issues/5624
.csdlc/evidence/5624

## Prompts

- Can `.` validate any checkout other than the exact current terminal branch worktree?
- Can two repositories with the same relative worktree suffix collide?
- Do malformed, missing, wrong, and dirty candidates fail with unsafe_checkout?
- Can validation alter the terminal record or retained receipt?
- Does command-level proof exercise the same path as issue 5340?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:e970ffa2eaa4ece145af4960bd82250fb181901e:3ce7f05bc4c2508c984fdbefa80727d2c69375f85829978105afb53c111433f4")

Reviewer: Some("subagent:codex-5624-exact-review")

Result: pass
