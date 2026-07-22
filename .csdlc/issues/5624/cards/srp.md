# Structured Review Prompt

Template: 1.0.0

Issue: 5624

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope

adl/tools/run_cargo_validation.sh
adl/tools/test_run_cargo_validation.sh
csdlc-v2/src/operator.rs
csdlc-v2/src/proof.rs
csdlc-v2/src/readiness.rs
csdlc-v2/tests/gate10a.rs
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

Revision: Some("git-blake3:9c303ba0cbd0a88eaf0963e23419e82f606c97aa:d5c09700d1e2586e383ae287f54a8c6486cb76d0a11df4b49b67a6f41f2f9280")

Reviewer: Some("subagent:codex-5624-ci-repair-review")

Result: pass
